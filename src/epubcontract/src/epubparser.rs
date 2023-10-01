use authcomp::{Computation,AuthType,AuthT,AuthContainer,AuthTContainer,ProofStream};
use authcomp::{Encode,Decode,Serialize,Deserialize,DeserializeOwned,DecodeOwned};
use authcomp::{UnAuth,UnAuthMut,AuthUpdate};

use authdoc::Node;
use authselect::ElementRef;
use indextree::Arena;
use nanoserde::SerJson;

use authparser::{read_auth_xml,AuthDocument};


use crate::authpub::publication::*;
use crate::authpub::metadata::*;
use crate::authpub::mediaoverlay::*;
use crate::authpub::types::*;

use crate::epub::opf::{Title,Metafield,AsAuthor,Author,ManifestItem};
use crate::epub::ncx::{NavPoint};

use crate::epubarchive::{EPubArchive,Epub};

use std::collections::BTreeMap;
use std::string::String;
use std::rc::Rc;
use std::vec::Vec;
use std::borrow::Cow;
use std::borrow::ToOwned;

use core::cell::RefCell;
use core::marker::PhantomData;
use core::fmt::Debug;
use core::ops::Not;
use core::str;

use anyhow::{anyhow,Result,Error};

//Internal keys
use typed_key::{typed_key,Key};

const TYPE_KEY : Key<String> = typed_key!("type");
const ROOT_FILE : Key<String> = typed_key!("rootfile");

const EPUB3 : &str = "3.0";
const EPUB31 : &str = "3.1";
const EPUB2 : &str = "2.0";
const EPUB201 : &str = "2.0.1";
const AUTO_META : &str = "auto";
const NONE_META : &str = "none";
const REFLOWABLE_META : &str = "reflowable";
const MEDIAOVERLAY_URL : &str = "media-overlay?resource=";

#[derive(Debug)]
pub struct EPubParser<C>
    where       
        C:Computation,
        C:AuthType<String>,
        C:AuthType<Vec<MediaOverlayNode<C>>>,
        C:AuthType<Metadata<C>>,
        C:AuthType<Link<C>>,
        C:AuthType<PublicationCollection<C>>,
        C:AuthType<Contributor<C>>,
        C:AuthType<MediaOverlayNode<C>>,
        C:AuthType<u32>,
        C:AuthType<f32>,
        C:AuthType<bool>,
        C:AuthType<Meta<C>>,
        C:AuthType<Vec<AuthT<Meta<C>,C>>>,
        C:AuthType<MultiLanguage<C>>,
        C:AuthType<BTreeMap<AuthT<String,C>, AuthT<String,C>>>,
        C:AuthType<BelongsTo<C>>,
        C:AuthType<Collection<C>>,
        C:AuthType<Subject<C>>,
        C:AuthType<Properties<C>>,
        C:AuthType<Contributor<C>>,
        C:AuthType<Vec<AuthT<Collection<C>,C>>>,    
        C:AuthContainer<MediaOverlayNode<C>>,
        C:AuthContainer<Meta<C>>,
        C:AuthContainer<PublicationCollection<C>>,
        C:AuthContainer<Link<C>>,

        C:AuthType<Vec<Rc<RefCell< AuthT<PublicationCollection<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Link<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Meta<C>,C>>>>>,

        C:AuthType<Vec<Rc<RefCell< AuthT<Contributor<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Subject<C>,C>>>>>,

        C:AuthType<Vec<Rc<RefCell<AuthT<String,C>>>>>,
        C:AuthType<Vec<u8>>,
        C:AuthType<BTreeMap<AuthT<Vec<u8>,C>, (AuthT<Vec<u8>,C>,Option<ElementRef<C>>)>>,
        C:AuthType<Node>,
        C:AuthType<Arena<AuthT<Node,C>>>

{

    _marker : PhantomData<C>

    
}

impl<C> EPubParser<C>
    where       
        C:Computation,
        C:AuthType<String>,
        C:AuthType<Vec<MediaOverlayNode<C>>>,
        C:AuthType<Metadata<C>>,
        C:AuthType<Link<C>>,
        C:AuthType<PublicationCollection<C>>,
        C:AuthType<Contributor<C>>,
        C:AuthType<MediaOverlayNode<C>>,
        C:AuthType<u32>,
        C:AuthType<f32>,
        C:AuthType<bool>,
        C:AuthType<Meta<C>>,
        C:AuthType<Vec<AuthT<Meta<C>,C>>>,
        C:AuthType<MultiLanguage<C>>,
        C:AuthType<BTreeMap<AuthT<String,C>, AuthT<String,C>>>,
        C:AuthType<BelongsTo<C>>,
        C:AuthType<Collection<C>>,
        C:AuthType<Subject<C>>,
        C:AuthType<Properties<C>>,
        C:AuthType<Contributor<C>>,
        C:AuthType<Vec<AuthT<Collection<C>,C>>>,    
        C:AuthContainer<MediaOverlayNode<C>>,
        C:AuthContainer<Meta<C>>,
        C:AuthContainer<PublicationCollection<C>>,
        C:AuthContainer<Link<C>>,

        C:AuthType<Vec<Rc<RefCell< AuthT<PublicationCollection<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Link<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Meta<C>,C>>>>>,

        C:AuthType<Vec<Rc<RefCell< AuthT<Contributor<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Subject<C>,C>>>>>,

        C:AuthType<Vec<Rc<RefCell<AuthT<String,C>>>>>,
        C:AuthType<Vec<u8>>,
        C:AuthType<BTreeMap<AuthT<Vec<u8>,C>, (AuthT<Vec<u8>,C>,Option<ElementRef<C>>)>>,
        C:AuthType<Node>,
        C:AuthType<Arena<AuthT<Node,C>>>,

        C: Serialize + Clone + Default+Debug+PartialEq+Encode+DecodeOwned+DeserializeOwned,

        C: Computation
{

    pub fn parse (source: &[u8]) -> Result<PublicationWithInternal<C>>
            where 
                C:Computation, 
                <C as AuthType<String>>::AuthT: Ord, <C as AuthType<Vec<u8>>>::AuthT: Ord ,
                BTreeMap<<C as AuthType<std::string::String>>::AuthT, <C as AuthType<std::string::String>>::AuthT> : SerJson,
                BTreeMap<<C as AuthType<Vec<u8>>>::AuthT, (<C as AuthType<Vec<u8>>>::AuthT, std::option::Option<ElementRef<C>>)>: SerJson

    {
        let mut publication = PublicationWithInternal::<C>::default();

        let epubarchive = EPubArchive::new(source)?;

        let book = epubarchive.epub()?;

        publication.add_to_internal(TYPE_KEY,String::from("epub"));

        (&mut *publication.publication.context.unauth().borrow_mut())
                    .push(Rc::new(RefCell::new( C::auth( String::from( "https://readium.org/webpub-manifest/context.jsonld" )) )));

        (&mut publication.publication.context).update();

        (&mut *publication.publication.metadata.unauth().borrow_mut()).rdftype = C::auth( String::from("http://schema.org/Book"));

        let epub_version = Self::get_epub_version (&book);

        publication.add_to_internal(ROOT_FILE,String::from(book.container.rootfiles.rootfile.full_path.as_ref()));

        Self::add_title(&mut publication,&book);

        Self::add_language(&mut publication,&book);

        Self::add_identifier(&mut publication,&book);

        Self::add_rights(&mut publication,&book);

        Self::add_description(&mut publication,&book);
        
        Self::add_publisher(&mut publication,&book);

        Self::add_source(&mut publication,&book);
        Self::add_page_progression(&mut publication,&book);

        Self::add_contributors(&mut publication,&book);

        Self::fill_spine_and_resource(&mut publication,&book);
        Self::add_presentation(&mut publication,&book);
        Self::add_cover_rel(&mut publication,&book);

        if (&*publication.publication.toc.unauth().borrow()).is_empty() {
            publication.publication.toc = C::auth ( Self::fill_toc_from_ncx(&book) );
            publication.publication.page_list = C::auth ( Self::fill_toc_from_ncx(&book) );
            publication.publication.landmarks = C::auth ( Self::fill_landmarks_from_guide(&book) );
        }

        Self::fill_subject(&mut publication,&book);
        Self::fill_publication_date(&mut publication,&book);

        let resources :BTreeMap<AuthT<Vec<u8>,C>,(AuthT<Vec<u8>,C>,Option<ElementRef<C>>)> = epubarchive.get_data().iter().map(|(name,resource)| {

            if let Ok(href_name) = str::from_utf8(name) {
                if let Ok(link) = publication.publication.search_link_by_href(href_name){
                    if let Some(type_link) = &(&*link.borrow()).type_link{
                        let link_type_ref = type_link.unauth();
                        if &*link_type_ref.borrow() == "application/xhtml+xml" {

                            let mut auth_document = AuthDocument::<C>::default();

                            let root = read_auth_xml::<C>(resource,&mut auth_document);

                            let document = ElementRef::<C> {
                                    id: root,
                                    doc: Rc::new(auth_document)
                            };

                            return (C::auth(name.to_owned()), (C::auth(resource.to_owned()),Some(document)) )
                        }                        
                    }
                }
            }
            

            (C::auth(name.to_owned()), (C::auth(resource.to_owned()),None) )
        }
        ).collect();

        publication.publication.raw_resources = C::auth ( resources );

        Ok(publication)
    }


    fn get_epub_version<'b> (book: &'b Epub) -> Option<&'b str> {
        if let Some(version) = &book.container.rootfiles.rootfile.version  {
            return Some(version)
        } else if let Some(version) = &book.opf.version {
            return Some(version)
        }
        return None
    }


    fn is_epub30_or_more (book: & Epub) -> bool {
        let version = Self::get_epub_version(book);

        match version {
            Some(EPUB3) => true,
            Some(EPUB31) => true,
            _ => false
        }
    }


    fn add_title (publication: &mut PublicationWithInternal<C>, book: &Epub ) 
            where 
                <C as AuthType<String>>::AuthT: Ord ,
                BTreeMap<<C as AuthType<std::string::String>>::AuthT, <C as AuthType<std::string::String>>::AuthT> : SerJson
    {
        if Self::is_epub30_or_more(book) {
            let mut main_title = &Title::default();

            if book.opf.metadata.title.len() > 1 {
                for titletag in &book.opf.metadata.title {
                    for metatag in &book.opf.metadata.meta {
                        if let Some(id) = &titletag.id {
                            if metatag.refines ==  Some(Cow::Borrowed(&("#".to_owned()+&id))) {
                                if metatag.content.as_ref() == "main"  {
                                    main_title = titletag;
                                }
                            }
                        }
                    }
                }
            }else {
                main_title = &book.opf.metadata.title[0];
            }

            let meta_alt = Self::find_all_meta_by_refine_and_property(book, &main_title.id.as_ref().unwrap_or(&Cow::Borrowed("")) , "alternate-script");

            if !meta_alt.is_empty() {

                let mut hashmap = BTreeMap::new();

                hashmap.insert ( C::auth(String::from (main_title.lang.as_ref().unwrap_or(&Cow::Borrowed("")).as_ref() )), 
                                 C::auth(String::from (main_title.content.as_ref())) );

                for m in &meta_alt {
                        if let Some(lang) = &m.lang {
                            hashmap.insert( C::auth(String::from(lang.as_ref())) , C::auth(String::from(m.content.as_ref() )) );
                        }
                }

                (&mut *publication.publication.metadata.unauth().borrow_mut()).title = C::auth ( MultiLanguage::MultiString ( C::auth(hashmap) ) );

            }else {

                (&mut *publication.publication.metadata.unauth().borrow_mut()).title = C::auth ( MultiLanguage::SingleString ( C::auth( String::from( main_title.content.as_ref() ) )) );
            }
        } else {
            if !book.opf.metadata.title.is_empty() {
                (&mut *publication.publication.metadata.unauth().borrow_mut()).title = C::auth ( MultiLanguage::SingleString ( C::auth( String::from( book.opf.metadata.title[0].content.as_ref() ) )) );
            }
        }

    }

    fn find_all_meta_by_refine_and_property<'a>(book: &'a Epub, id: &str, property: &str) -> Vec<&'a Metafield<'a>> {

        let mut metas = Vec::new();

        for metatag in &book.opf.metadata.meta {
            if metatag.refines == Some(Cow::Borrowed(&("#".to_owned()+id))) && metatag.property.as_ref().filter(|s| s.as_ref() == property).is_some() {
                metas.push(metatag)
            }

        }

        metas
    }

    fn find_meta_by_refine_and_property<'a>(book: &'a Epub, id: &str, property: &str) -> Option<&'a Metafield<'a>> {
        for metatag in &book.opf.metadata.meta {
            if metatag.refines == Some(Cow::Borrowed(&("#".to_owned()+id))) && metatag.property.as_ref().filter(|s| s.as_ref() == property).is_some() {
                return Some(metatag)
            }

        }
        return None
    }

    fn add_identifier (publication: &mut PublicationWithInternal<C>, book: &Epub ) {

        if book.opf.metadata.identifier.len() > 1 {
            let unique_id = Some(&book.opf.unique_identifier);
            for ident in &book.opf.metadata.identifier {
                if ident.id.as_ref() == unique_id{
                    (&mut *publication.publication.metadata.unauth().borrow_mut()).identifier = Some (C::auth ( String::from(ident.content.as_ref()) ))
                }
            }

        } else {
            if !book.opf.metadata.identifier.is_empty() {
                (&mut *publication.publication.metadata.unauth().borrow_mut()).identifier = Some (C::auth ( String::from(book.opf.metadata.identifier[0].content.as_ref()) ))
            }
        }
    }

    fn add_language (publication: &mut PublicationWithInternal<C>, book: &Epub ) {

        let language : Vec<Rc<RefCell<AuthT<String,C>>>> = book.opf.metadata.language.iter().map ( |l| Rc::new(RefCell::new( C::auth (String::from(l.as_ref())))) ).collect();

        if !language.is_empty() {
            (&mut *publication.publication.metadata.unauth().borrow_mut()).language = Some(C::auth ( language ));
        }
    }

    fn add_rights (publication: &mut PublicationWithInternal<C>, book: &Epub ) {
        let rights : String = book.opf.metadata.rights.iter()
                                    .map (|s| s.as_ref() )
                                    .zip(std::iter::repeat(" "))
                                    .flat_map(|(a, sep)| vec![sep, a])
                                    .skip(1)
                                    .collect::<String>();

        if !rights.is_empty(){
            (&mut *publication.publication.metadata.unauth().borrow_mut()).rights = Some(C::auth ( rights ));
        }
    }

    fn add_description (publication: &mut PublicationWithInternal<C>, book: &Epub ) {
        if !book.opf.metadata.description.is_empty() {
            (&mut *publication.publication.metadata.unauth().borrow_mut()).description = Some(C::auth ( String::from (book.opf.metadata.description[0].as_ref()) ));
        }
    }

    fn add_source (publication: &mut PublicationWithInternal<C>, book: &Epub ) {
        if !book.opf.metadata.source.is_empty() {
            (&mut *publication.publication.metadata.unauth().borrow_mut()).source = Some(C::auth ( String::from (book.opf.metadata.source[0].as_ref()) ));
        }
    }


    fn add_publisher (publication: &mut PublicationWithInternal<C>, book: &Epub ) {

        let publisher = book.opf.metadata.publisher.iter()
                                    .map( |p| Rc::new(RefCell::new (C::auth ( 
                                                        Contributor { 
                                                                name : C::auth ( MultiLanguage::SingleString ( C::auth( String::from( p.as_ref() )))),
                                                                sort_as : None,
                                                                identifier : None,
                                                                role : None
                                                        }  
                                    )))).collect::<Vec<Rc<RefCell<AuthT<Contributor<C>,C>>>>>();

        if !publisher.is_empty() {
                (&mut *publication.publication.metadata.unauth().borrow_mut()).publisher = Some(C::auth ( publisher ));
        }
    }


    fn add_page_progression (publication: &mut PublicationWithInternal<C>, book: &Epub ) {

        if let Some(page_progression) = &book.opf.spine.page_progression {
            if page_progression != "" {
                (&mut *publication.publication.metadata.unauth().borrow_mut()).direction = Some(C::auth ( String::from (page_progression.as_ref()) ))
            }
        }
    }


    fn add_contributors (publication: &mut PublicationWithInternal<C>, book: &Epub ) 
                where 
                    <C as AuthType<String>>::AuthT: Ord ,
                    BTreeMap<<C as AuthType<std::string::String>>::AuthT, <C as AuthType<std::string::String>>::AuthT> : SerJson
    {

        for cont in &book.opf.metadata.contributor {
                Self::add_contributor(publication,book,cont,"")
        }

        for creator in &book.opf.metadata.creator {
                Self::add_contributor(publication,book,creator,"aut")
        }

        if Self::is_epub30_or_more(book) {
            Self::add_contributor_in_meta(publication,book)
        }
    }


    fn add_contributor_in_meta (publication: &mut PublicationWithInternal<C>, book: &Epub ) 
        where 
            <C as AuthType<String>>::AuthT: Ord,
            BTreeMap<<C as AuthType<std::string::String>>::AuthT, <C as AuthType<std::string::String>>::AuthT> : SerJson
    {
        for meta in &book.opf.metadata.meta {
            if meta.property.as_ref().filter(|s| s.as_ref() == "dcterms:creator").is_some() || meta.property.as_ref().filter( |s| s.as_ref() == "dcterms:contributor").is_some() {
                let author = Author {
                    content : Cow::Borrowed(meta.content.as_ref()),
                    id: meta.id.clone(),
                    file_as: None,
                    role: None
                };
                Self::add_contributor(publication,book,&author,"")
            }
        }
    }

    fn add_contributor<'a> (publication: &mut PublicationWithInternal<C>, book: &Epub, cont: &'a dyn AsAuthor<'a>,forced_role: &str ) 
                where 
                    <C as AuthType<String>>::AuthT: Ord ,
                    BTreeMap<<C as AuthType<std::string::String>>::AuthT, <C as AuthType<std::string::String>>::AuthT>: SerJson
    {

        let mut role : &str = "";
        let mut contributor : Contributor<C> = Contributor::<C>::default();

        if Self::is_epub30_or_more(book) {
            if cont.id().is_some() {
                let meta = Self::find_meta_by_refine_and_property(book,cont.id().unwrap(),"role");
                if meta.is_some() {
                    role = meta.unwrap().content.as_ref();
                }
            }

            if role.is_empty() && !forced_role.is_empty() {
                role = forced_role;
            }

            if cont.id().is_some() {
                let meta_alt = Self::find_all_meta_by_refine_and_property(book,cont.id().unwrap(),"alternate-script");

                if !meta_alt.is_empty(){
                    let mut hashmap = BTreeMap::new();

                    let metadata_ref = publication.publication.metadata.unauth();
                    
                    let langs_ref = metadata_ref.borrow_mut().language.unauth_mut();

                    let l = &*langs_ref.borrow()[0];


                    hashmap.insert ( (*l.borrow()).clone() , 
                                    C::auth(String::from (cont.content())) );

                    for m in &meta_alt {
                            if let Some(lang) = &m.lang {
                                hashmap.insert( C::auth(String::from(lang.as_ref())) , C::auth(String::from(m.content.as_ref())) );
                            }
                    }

                    contributor.name = C::auth ( MultiLanguage::MultiString ( C::auth(hashmap) ) );

                }else {
                    contributor.name = C::auth ( MultiLanguage::SingleString ( C::auth( String::from( cont.content() ) )));
                }
            }
        }else {

            contributor.name = C::auth ( MultiLanguage::SingleString ( C::auth( String::from( cont.content() ) )));

            if cont.role().is_some() {
                role = cont.role().unwrap();
            }

            if role.is_empty() && !forced_role.is_empty() {
                role = forced_role;
            }            
        }

        match role {
            "aut" => {
                            let metadata_ref = publication.publication.metadata.unauth();
                            
                            let author_ref = metadata_ref.borrow_mut().author.unauth_mut();

                            (&mut *author_ref.borrow_mut()).push(Rc::new(RefCell::new(C::auth(contributor))));

                            //C::auth_update::<Vec<Rc<RefCell<AuthT<Contributor<C>,C>>>>>(&mut (&mut *metadata_ref.borrow_mut()).author.as_mut().unwrap());
                            (&mut *metadata_ref.borrow_mut()).author.update();
                            //C::auth_update::<Metadata<C>>(&mut publication.publication.metadata);
                            (&mut publication.publication.metadata).update();
                    }

            "trl" => {
                        let metadata_ref = publication.publication.metadata.unauth();
                        
                        let translator_ref = metadata_ref.borrow_mut().translator.unauth_mut();

                        (&mut *translator_ref.borrow_mut()).push(Rc::new(RefCell::new(C::auth(contributor))));

                        //C::auth_update::<Vec<Rc<RefCell<AuthT<Contributor<C>,C>>>>>(&mut (&mut *metadata_ref.borrow_mut()).translator);
                        (&mut *metadata_ref.borrow_mut()).translator.update();

                        //C::auth_update::<Metadata<C>>(&mut publication.publication.metadata);
                        (&mut publication.publication.metadata).update();
                    }
            "art" => {
                        let metadata_ref = publication.publication.metadata.unauth();
                        
                        let artist_ref = metadata_ref.borrow_mut().artist.unauth_mut();

                        (&mut *artist_ref.borrow_mut()).push(Rc::new(RefCell::new(C::auth(contributor))));

                        //C::auth_update::<Vec<Rc<RefCell<AuthT<Contributor<C>,C>>>>>(&mut (&mut *metadata_ref.borrow_mut()).artist);
                        (&mut *metadata_ref.borrow_mut()).artist.update();

                        //C::auth_update::<Metadata<C>>(&mut publication.publication.metadata);
                        (&mut publication.publication.metadata).update();
                    }

            "edt" => {
                        let metadata_ref = publication.publication.metadata.unauth();
                        
                        let editor_ref = metadata_ref.borrow_mut().editor.unauth_mut();

                        (&mut *editor_ref.borrow_mut()).push(Rc::new(RefCell::new(C::auth(contributor))));

                        //C::auth_update::<Vec<Rc<RefCell<AuthT<Contributor<C>,C>>>>>(&mut (&mut *metadata_ref.borrow_mut()).editor);
                        (&mut *metadata_ref.borrow_mut()).editor.update();

                        //C::auth_update::<Metadata<C>>(&mut publication.publication.metadata);
                        (&mut publication.publication.metadata).update();
                    }
            "ill" => {
                        let metadata_ref = publication.publication.metadata.unauth();
                        
                        let illustrator_ref = metadata_ref.borrow_mut().illustrator.unauth_mut();

                        (&mut *illustrator_ref.borrow_mut()).push(Rc::new(RefCell::new(C::auth(contributor))));

                        //C::auth_update::<Vec<Rc<RefCell<AuthT<Contributor<C>,C>>>>>(&mut (&mut *metadata_ref.borrow_mut()).illustrator);
                        (&mut *metadata_ref.borrow_mut()).illustrator.update();

                        //C::auth_update::<Metadata<C>>(&mut publication.publication.metadata);
                        (&mut publication.publication.metadata).update();
            }

            "clr" => {
                        let metadata_ref = publication.publication.metadata.unauth();
                        
                        let colorist_ref = metadata_ref.borrow_mut().colorist.unauth_mut();

                        (&mut *colorist_ref.borrow_mut()).push(Rc::new(RefCell::new(C::auth(contributor))));

                        //C::auth_update::<Vec<Rc<RefCell<AuthT<Contributor<C>,C>>>>>(&mut (&mut *metadata_ref.borrow_mut()).colorist);
                        (&mut *metadata_ref.borrow_mut()).colorist.update();

                        //C::auth_update::<Metadata<C>>(&mut publication.publication.metadata);
                        (&mut publication.publication.metadata).update();
                    }
            "nrt" => {
                        let metadata_ref = publication.publication.metadata.unauth();
                        
                        let narrator_ref = metadata_ref.borrow_mut().narrator.unauth_mut();

                        (&mut *narrator_ref.borrow_mut()).push(Rc::new(RefCell::new(C::auth(contributor))));

                        //C::auth_update::<Vec<Rc<RefCell<AuthT<Contributor<C>,C>>>>>(&mut (&mut *metadata_ref.borrow_mut()).narrator);
                        (&mut *metadata_ref.borrow_mut()).narrator.update();
                        //C::auth_update::<Metadata<C>>(&mut publication.publication.metadata);
                        (&mut publication.publication.metadata).update();
                    }

            "pbl" => {
                        let metadata_ref = publication.publication.metadata.unauth();
                        
                        let publisher_ref = metadata_ref.borrow_mut().publisher.unauth_mut();

                        (&mut *publisher_ref.borrow_mut()).push(Rc::new(RefCell::new(C::auth(contributor))));

                        //C::auth_update::<Vec<Rc<RefCell<AuthT<Contributor<C>,C>>>>>(&mut (&mut *metadata_ref.borrow_mut()).publisher);
                        (&mut *metadata_ref.borrow_mut()).publisher.update();
                        //C::auth_update::<Metadata<C>>(&mut publication.publication.metadata);
                        (&mut publication.publication.metadata).update();
                    }
            _ => {
                    //authallocator::print(&alloc::format!("Contributor {:?}",&contributor));

                    if !role.is_empty(){
                        contributor.role = Some(C::auth ( String::from (role)));
                    }

                    let metadata_ref = publication.publication.metadata.unauth();
                            
                    let contributor_ref = metadata_ref.borrow_mut().contributor.unauth_mut();

                    (&mut *contributor_ref.borrow_mut()).push(Rc::new(RefCell::new(C::auth(contributor))));  
                    
                    //C::auth_update::<Vec<Rc<RefCell<AuthT<Contributor<C>,C>>>>>(&mut (&mut *metadata_ref.borrow_mut()).contributor);
                    (&mut *metadata_ref.borrow_mut()).contributor.update();
                    //C::auth_update::<Metadata<C>>(&mut publication.publication.metadata);
                    (&mut publication.publication.metadata).update();
            }
        }

    }


    fn find_in_spine_by_ref<'a> (publication: &'a PublicationWithInternal<C>, href: &str ) -> Option<Rc<RefCell<Link<C>>>> {

        let reading_order_ref = C::unauth2::<Vec<Rc<RefCell< AuthT<Link<C>,C>>>>>(&publication.publication.reading_order);

        for l in &*reading_order_ref.borrow() {
            let l_ref = C::unauth2::<Link<C>>(&*l.borrow());
            let href_ref = C::unauth2::<String>(&(&*l_ref.borrow()).href);
            if &*href_ref.borrow() == href {
                return Some(Rc::clone(&l_ref))
            }
        }

        None
    }

    fn find_in_manifest_by_id (book : &Epub, id: &str) -> Option<Link<C>> {

        for item in &book.opf.manifest.items {
            if item.id == id {
                let mut link_item = Link::<C>::default();
                if !item.mediatype.as_ref().is_empty(){
                    link_item.type_link = Some(C::auth ( String::from ( item.mediatype.as_ref() )));
                }
                link_item.add_href_absolute(&item.href,&book.container.rootfiles.rootfile.full_path);
                Self::add_rel_and_properties_to_link(&mut link_item,item,book);
                Self::add_media_overlay(&mut link_item,item,book);
                return Some(link_item);
            }
        }

        None
    }

    fn add_link_properties_from_string (link : &mut Link<C>, properties_string: &str ){

        {

            //let mut properties_struct_ref = C::unauth2::<Properties<C>>(&link.properties);
            let properties_struct_ref =link.properties.unauth_mut();
            let properties_struct = &mut *properties_struct_ref.borrow_mut();
            
            for property in properties_string.split(" ") {

            
                match property {
                    "cover-image" => link.add_rel("cover"),
                    "nav" => link.add_rel("contents"),
                    "scripted" => {
                        
                        //let contains_ref = C::unauth2::<Vec<Rc<RefCell<AuthT<String,C>>>>>(&properties_struct.contains);
                        let contains_ref = properties_struct.contains.unauth_mut();
                        (&mut *contains_ref.borrow_mut()).push( Rc::new ( RefCell::new (C::auth(String::from("js"))) ) );

                        //C::auth_update::<Vec<Rc<RefCell<AuthT<String,C>>>>>(&mut properties_struct.contains);
                        properties_struct.contains.update();
                    },
                    
                    "mathml" => {
                        //let contains_ref = C::unauth2::<Vec<Rc<RefCell<AuthT<String,C>>>>>(&properties_struct.contains);
                        let contains_ref = properties_struct.contains.unauth_mut();
                        (&mut *contains_ref.borrow_mut()).push( Rc::new ( RefCell::new (C::auth(String::from("mathml"))) ) );

                        //C::auth_update::<Vec<Rc<RefCell<AuthT<String,C>>>>>(&mut properties_struct.contains);
                        properties_struct.contains.update();
                    },
                    "onix-record" => {
                        //let contains_ref = C::unauth2::<Vec<Rc<RefCell<AuthT<String,C>>>>>(&properties_struct.contains);
                        let contains_ref = properties_struct.contains.unauth_mut();
                        (&mut *contains_ref.borrow_mut()).push( Rc::new ( RefCell::new (C::auth(String::from("onix"))) ) );

                        //C::auth_update::<Vec<Rc<RefCell<AuthT<String,C>>>>>(&mut properties_struct.contains);
                        properties_struct.contains.update();
                    },
                    "svg" => {
                        //let contains_ref = C::unauth2::<Vec<Rc<RefCell<AuthT<String,C>>>>>(&properties_struct.contains);
                        let contains_ref = properties_struct.contains.unauth_mut();
                        (&mut *contains_ref.borrow_mut()).push( Rc::new ( RefCell::new (C::auth(String::from("svg"))) ) );

                        //C::auth_update::<Vec<Rc<RefCell<AuthT<String,C>>>>>(&mut properties_struct.contains);
                        properties_struct.contains.update();
                    },    

                    "xmp-record" => {
                        //let contains_ref = C::unauth2::<Vec<Rc<RefCell<AuthT<String,C>>>>>(&properties_struct.contains);
                        let contains_ref = properties_struct.contains.unauth_mut();
                        (&mut *contains_ref.borrow_mut()).push( Rc::new ( RefCell::new (C::auth(String::from("xmp"))) ) );

                        //C::auth_update::<Vec<Rc<RefCell<AuthT<String,C>>>>>(&mut properties_struct.contains);
                        properties_struct.contains.update();
                    },
                    "remote-resources" => {
                        //let contains_ref = C::unauth2::<Vec<Rc<RefCell<AuthT<String,C>>>>>(&properties_struct.contains);
                        let contains_ref = properties_struct.contains.unauth_mut();
                        (&mut *contains_ref.borrow_mut()).push( Rc::new ( RefCell::new (C::auth(String::from("remote-resources"))) ) );

                        //C::auth_update::<Vec<Rc<RefCell<AuthT<String,C>>>>>(&mut properties_struct.contains);
                        properties_struct.contains.update();
                    },
                    "page-spread-left" => {
                        properties_struct.page = Some(C::auth ( String::from ("left")));
                    },
                    "page-spread-right" => {
                        properties_struct.page = Some(C::auth ( String::from ("right")));
                    },
                    "page-spread-center" => {
                        properties_struct.page = Some(C::auth ( String::from ("center")));
                    },
                    "rendition:spread-none" => {
                        properties_struct.spread = Some(C::auth ( String::from (NONE_META)));
                    },
                    "rendition:spread-auto" => {    
                        properties_struct.spread = Some(C::auth ( String::from (AUTO_META)));
                    },
                    "rendition:spread-landscape" => {
                        properties_struct.spread = Some(C::auth ( String::from ( "landscape" )));
                    },
                    "rendition:spread-portrait" => {
                        properties_struct.spread = Some(C::auth ( String::from ( "both" )));
                    },
                    "rendition:spread-both" => {
                        properties_struct.spread = Some(C::auth ( String::from ( "both" )));
                    },
                    "rendition:layout-reflowable" => {
                        properties_struct.layout = Some(C::auth ( String::from ( REFLOWABLE_META )));
                    },
                    "rendition:layout-pre-paginated" => {
                        properties_struct.layout = Some(C::auth ( String::from ( "fixed" )));
                    },
                    "rendition:orientation-auto" => {
                        properties_struct.orientation = Some(C::auth ( String::from ("auto")));
                    },
                    "rendition:orientation-landscape" => {
                        properties_struct.orientation = Some(C::auth ( String::from ("landscape")));
                    },
                    "rendition:orientation-portrait" => {
                        properties_struct.orientation = Some(C::auth ( String::from ("portrait")));
                    },
                    "rendition:flow-auto" => {
                        properties_struct.overflow = Some(C::auth ( String::from ( AUTO_META )));
                    },
                    "rendition:flow-paginated" => {
                        properties_struct.overflow = Some(C::auth ( String::from ( "paginated" )));
                    },
                    "rendition:flow-scrolled-continuous" => {
                        properties_struct.overflow = Some(C::auth ( String::from ("scrolled-continuous")));
                    },
                    "rendition:flow-scrolled-doc" => {
                        properties_struct.overflow = Some(C::auth ( String::from ( "scrolled" )));
                    },
                    
                    _ => {}
                }
            }
        } //This block allows to release *properties_struct_ref.borrow_mut()
        //Update 
        
        //C::auth_update::<Properties<C>>(&mut link.properties);
        link.properties.update();
       

    }

    fn find_properties_in_spine_for_manifest<'a> ( manifest_item : &'a ManifestItem<'a>, book: &'a Epub) -> Option<&'a str> 
    {
        for item in &book.opf.spine.items {
                if item.idref == manifest_item.id {
                    return item.properties.as_ref().map ( |s| s.as_ref())
                }
        }

        None
    }

    
    fn add_rel_and_properties_to_link<'a> (link: &mut Link<C>, manifest_item: &'a ManifestItem<'a>, book: &'a Epub) 
    {
        if manifest_item.properties.is_some() {
            Self::add_link_properties_from_string(link,manifest_item.properties.as_ref().unwrap().as_ref());
        }

        let spine_properties = Self::find_properties_in_spine_for_manifest(manifest_item,book);

        if spine_properties.is_some() {
            Self::add_link_properties_from_string(link,spine_properties.unwrap().as_ref());
        }

    }


    fn add_media_overlay<'a> (link: &mut Link<C>, manifest_item: &'a ManifestItem<'a>, book: &'a Epub) 
    {
        if manifest_item.mediaoverlay.is_some() {
            let meta = Self::find_meta_by_refine_and_property(book,manifest_item.mediaoverlay.as_ref().unwrap(),"media:duration");
            if let Some(m) = meta {
                link.duration = Some(C::auth ( String::from (Self::smil_time_to_seconds(m.content.as_ref()))))
            }
        }
    }

    // TO FIX
    fn  smil_time_to_seconds (smile_time: &str) -> &str {
        smile_time
    }
    

    fn fill_spine_and_resource (publication: &mut PublicationWithInternal<C>, book: &Epub ) {

        //authallocator::print(&alloc::format!("Spine items{:?}",&book.opf.spine.items));

        for item in &book.opf.spine.items {
            let linear = match &item.linear {
                            Some(l) => { l == "yes" || l == "" },
                            None => true
            };

            if linear {
                    let link_item_opt = Self::find_in_manifest_by_id (book,&item.idref);
                    if let Some(link_item) = link_item_opt {
                        let href = C::unauth2::<String> (&link_item.href);

                        if &*href.borrow() != "" {
                            //let reading_order_ref = C::unauth2::<Vec<Rc<RefCell< AuthT<Link<C>,C>>>>>(&publication.publication.reading_order);
                            let reading_order_ref = publication.publication.reading_order.unauth();
                            (&mut *reading_order_ref .borrow_mut()).push(Rc::new(RefCell::new(C::auth( link_item))));

                        }

                    }

 
            }
        }

        publication.publication.reading_order.update();

        //authallocator::print(&alloc::format!("Manifest items{:?}",&book.opf.manifest.items));

        for manifest_item in &book.opf.manifest.items {

            let mut link_item = Link::<C>::default();

            if !manifest_item.mediatype.as_ref().is_empty(){
                link_item.type_link = Some(C::auth ( String::from (manifest_item.mediatype.as_ref()) ));
            }
            
            link_item.add_href_absolute(&manifest_item.href,&book.container.rootfiles.rootfile.full_path);

            let href_ref = link_item.href.unauth();
            let link_spine_opt = Self::find_in_spine_by_ref(publication,&*href_ref.borrow());

            if link_spine_opt.is_none() {
                {
                    Self::add_rel_and_properties_to_link(&mut link_item,manifest_item,book);
                    Self::add_media_overlay(&mut link_item,manifest_item,book);

                    let resources_ref = publication.publication.resources.unauth();

                    (&mut *resources_ref.borrow_mut()).push(Rc::new(RefCell::new(C::auth(link_item))));

                   
                }

                //C::auth_update::<Vec<Rc<RefCell<AuthT<Link<C>,C>>>>>(&mut publication.publication.resources );
                publication.publication.resources.update();
            }

        }
    }

    
    fn fill_toc_from_ncx (book: &Epub) -> Vec<Rc<RefCell<AuthT<Link<C>,C>>>>
    {
        let mut nodes = Vec::new();

        for point in &book.ncx.map.points {
            let mut n = Self::fill_toc_from_navpoint(book,point);
            nodes.append(&mut n);
        }

        nodes
    }
    

    fn fill_toc_from_navpoint (book: &Epub, point: &NavPoint) -> Vec<Rc<RefCell<AuthT<Link<C>,C>>>>
    {

        let mut nodes = Vec::new();

        let mut link = Link::<C>::default();
        link.add_href_absolute(&point.content.src,&book.ncx_path);

        if !point.nav_label.text.as_ref().is_empty(){
            link.title = Some (C::auth ( String::from(point.nav_label.text.as_ref()) ));
        }

        for p in &point.nav_points {
            let mut n = Self::fill_toc_from_navpoint(book,p);
            nodes.append(&mut n);
        }

        nodes.push( Rc::new ( RefCell::new ( C::auth ( link ) )) );
        nodes
    }

    fn fill_page_list_from_ncx ( book: &Epub ) -> Vec<Rc<RefCell<AuthT<Link<C>,C>>>>
    {

        let mut nodes = Vec::new();

        if book.ncx.page_list.is_some() {
            for page_target in &book.ncx.page_list.as_ref().unwrap().page_targets {
                let mut link = Link::<C>::default();
                link.add_href_absolute(&page_target.content.src,&book.ncx_path);
                if !page_target.nav_label.text.as_ref().is_empty() {
                    link.title = Some(C::auth ( String::from(page_target.nav_label.text.as_ref()) ));
                }
                nodes.push( Rc::new ( RefCell::new ( C::auth ( link ) )) );
            }
        }
        nodes
    }

    fn fill_landmarks_from_guide (book: &Epub) -> Vec<Rc<RefCell<AuthT<Link<C>,C>>>> 
    {
        let mut nodes = Vec::new();

        if let Some(guide) = &book.opf.guide {
            for reference in &guide.references {
                if reference.href.as_ref() != "" {
                        let mut link = Link::<C>::default();
                        link.add_href_absolute(&reference.href,&book.container.rootfiles.rootfile.full_path);
                        link.title = reference.title.as_ref().map (|s| C::auth (String::from(s.as_ref())) );
                        nodes.push ( Rc::new (RefCell::new ( C::auth (link) )));
                }
            }
        }

        nodes
    }

    fn add_presentation (publication: &mut PublicationWithInternal<C>, book: &Epub) {

        let mut presentation = Properties::<C>::default();
        let mut is_set = false;

        for meta in &book.opf.metadata.meta {
            match meta.property.as_ref().map(|s| s.as_ref() ).unwrap_or_default() {
                "rendition:layout" => {
                                    match meta.content.as_ref() {
                                        "pre-paginated" => {presentation.layout = Some(C::auth( String::from("fixed") )); is_set = true;},
                                        "reflowable" => {presentation.layout = Some(C::auth( String::from("reflowable") )); is_set = true;},
                                        _ => {}
                                    }
                },
                "rendition:orientation" => { presentation.orientation = Some(C::auth( String::from(meta.content.as_ref() ) )); is_set = true;},
                "rendition:spread" => { presentation.spread = Some(C::auth( String::from(meta.content.as_ref()) )); is_set = true;},
                "rendition:flow" => { presentation.overflow = Some(C::auth( String::from(meta.content.as_ref()) )); is_set = true;},
                _ => {}
            }
        }

        if is_set {
            (*publication.publication.metadata.unauth().borrow_mut()).presentation = Some(C::auth ( presentation ));
            publication.publication.metadata.update();
        }
    }

    fn add_cover_rel (publication: &mut PublicationWithInternal<C>, book: &Epub) 
    {

        //Legacy meta with name and content attributes

        let mut cover_id = "";

        
        let meta = book.opf.metadata.meta.iter().find ( |m| m.name_legacy.as_ref().filter( |s| s.as_ref() =="cover").is_some() );
        if let Some(m) = meta {
            if let Some(c) = &m.content_legacy {
                cover_id = c.as_ref();
            }
        }
        

        if !cover_id.is_empty(){
            if let Some(manifest_info) = Self::find_in_manifest_by_id(&book,cover_id){
                let manifest_href_ref = manifest_info.href.unauth();
                let manifest_href = &*manifest_href_ref.borrow();

                let resources_ref = publication.publication.resources.unauth();
                let resources = &*resources_ref.borrow();

                for item in resources {
                        let link_auth = &mut *item.borrow_mut();
                        let link_ref =  link_auth.unauth();
                        let link = &mut *link_ref.borrow_mut();
                        
                        let href_ref = link.href.unauth();
                        let href = &*href_ref.borrow();

                        if href == manifest_href {
                            link.add_rel("cover");
                        }                        

                }

                publication.publication.resources.update();
            }

        }
        
    }

    fn fill_subject(publication: &mut PublicationWithInternal<C>, book: &Epub) {
        let mut subjects = Vec::new();

        for subject in &book.opf.metadata.subject {
                let new_subject = Subject::<C> {
                        name: C::auth ( String::from (subject.content.as_ref()) ),
                        code: subject.term.as_ref().map(|s| C::auth(String::from(s.as_ref()))),
                        scheme: subject.authority.as_ref().map ( |s|  C::auth(String::from(s.as_ref()))),
                        sort_as: None

                };

                subjects.push(Rc::new (RefCell::new (C::auth (new_subject) )));
        }

        if !subjects.is_empty() {
            (&mut *publication.publication.metadata.unauth().borrow_mut()).subject = Some(C::auth (subjects));
            publication.publication.metadata.update();
        }

    }



    fn fill_publication_date(publication: &mut PublicationWithInternal<C>, book: &Epub){
        /*
        use chrono::{DateTime,NaiveDateTime,Date,NaiveDate};
        use alloc::string::ToString;
        
        //authallocator::print(&alloc::format!("fill_publication_date {:?}",&book.opf.metadata.date));

        if !book.opf.metadata.date.is_empty() {
            if Self::is_epub30_or_more(&book) {
             
                //authallocator::print(&alloc::format!("fill_publication_date {:?}",DateTime::parse_from_rfc3339(&book.opf.metadata.date[0].content)));
                //authallocator::print(&alloc::format!("fill_publication_date {:?}",NaiveDate::parse_from_str(&book.opf.metadata.date[0].content,"%Y-%m-%d")));

                if let Ok(date) = DateTime::parse_from_rfc3339(&book.opf.metadata.date[0].content){
                    (&mut *publication.publication.metadata.unauth().borrow_mut()).publication_date = Some(C::auth (date.to_rfc3339()));
                    publication.publication.metadata.update();
                }else if let Ok(date) = NaiveDate::parse_from_str(&book.opf.metadata.date[0].content,"%Y-%m-%d"){
                    (&mut *publication.publication.metadata.unauth().borrow_mut()).publication_date = Some(C::auth (date.to_string()));
                    publication.publication.metadata.update();
                }
            }

        }

        for date in &book.opf.metadata.date {
            if let Some(ref event) = date.event {
                if event.contains("publication") {

                    if let Ok(d) = DateTime::parse_from_rfc3339(&date.content){
                        (&mut *publication.publication.metadata.unauth().borrow_mut()).publication_date = Some(C::auth (d.to_rfc3339()));
                        publication.publication.metadata.update();
                        return
                    }else if let Ok(d) = NaiveDate::parse_from_str(&date.content,"%Y-%m-%d"){
                        (&mut *publication.publication.metadata.unauth().borrow_mut()).publication_date = Some(C::auth (d.to_string()));
                        publication.publication.metadata.update();
                        return
                    }else if let Ok(d) = NaiveDate::parse_from_str(&date.content,"%Y-%m"){
                        (&mut *publication.publication.metadata.unauth().borrow_mut()).publication_date = Some(C::auth (d.to_string()));
                        publication.publication.metadata.update();
                        return
                    }else if let Ok(d) = NaiveDate::parse_from_str(&date.content,"%Y"){
                        (&mut *publication.publication.metadata.unauth().borrow_mut()).publication_date = Some(C::auth (d.to_string()));
                        publication.publication.metadata.update();
                        return
                    }
                }
            }
        }
        */
    }
}