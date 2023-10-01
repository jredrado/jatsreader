
use std::string::String;


#[cfg(test)]
#[no_mangle]
pub extern "C" fn test_all() {
    test_main();

    
    unsafe {
       let msg = String::from("Ok");

       print(&msg);

    }
    

}

#[cfg(test)]
#[start]
#[no_mangle]
pub extern "C" fn _start() {
    

}

mod epubarchive;
//mod epubdoc;
mod api;
//mod wasm_api;

mod epub;

mod publication;
mod authpub;

mod epubparser;

pub use authpub::publication::Publication;
pub use epubparser::EPubParser;

// Contract API ¿?
//pub use authallocator::alloc;
//pub use authallocator::dealloc;

//pub use wasm_api::wapc_init;

pub use api::*;

//pub use epubarchive::EPubArchive;

#[cfg(test)]
mod test {
    use core::str;
    use alloc::format;
    use crate::epubarchive::EPubArchive;
    use authallocator::print;


    use crate::alloc::borrow::ToOwned;

    /*
    // 
    fn read_test () {

        use authparser::Document;
        use authcomp::Prover;
        use crate::epubdoc::EPubDoc;

        let source = include_bytes!("../assets/childrens-literature.epub");

        let epub = EPubArchive::new(source).unwrap();

        let container_file = epub.get_container_file().unwrap();
        print(&format!("{}",str::from_utf8(container_file).unwrap()));


        let document = Document::new_from_xml(container_file);

        print(&format!("{:?}",document));

        let root_file = document.find_element("rootfile").unwrap();

        let package_file = root_file.attr("full-path").unwrap();
        print(&format!("{:?}",package_file));

        let package_file_vec = package_file.as_bytes().to_vec();
        let package_file_content = epub.get(&package_file_vec);

        print(&format!("{:?}",str::from_utf8(package_file_content.unwrap())));

        let package_document = Document::new_from_xml(package_file_content.unwrap());

        print(&format!("{:?}",package_document));

        //let spine = EPubDoc::<Prover::<()>>::new_spine(&package_document);
        
    }
    

    // 
    fn epubdoc_read_test () {

        use authcomp::Prover;
        use crate::epubdoc::EPubDoc;

        let epubsource = include_bytes!("../assets/childrens-literature.epub");
        let epubdoc = EPubDoc::<Prover::<()>>::new(epubsource);
        print(&format!("{:?}",epubdoc));
    }
    */
  
     
    fn strong_xml_test () {

        use alloc::borrow::Cow;
        use strong_xml::{XmlRead, XmlResult, XmlWrite};

        #[derive(PartialEq,Debug)]

        pub struct Metafield<'a> {

            pub content: Cow<'a, str>,

            pub dir: Option<Cow<'a, str>>,

            pub refines: Option<Cow<'a, str>>,

            pub property: Option<Cow<'a, str>>,

            pub id:  Option<Cow<'a, str>>,

            pub lang: Option<Cow<'a, str>>,

            pub scheme: Option<Cow<'a, str>>,    
        
            //Legacy attributes 2.0
            

            pub name_legacy: Option<Cow<'a, str>>,   

            pub content_legacy: Option<Cow<'a, str>>,      
            
        }

        impl <'__input: 'a, 'a> strong_xml::XmlRead<'__input> for
        Metafield<'a> {
           fn from_reader(mut reader: &mut strong_xml::XmlReader<'__input>)
            -> strong_xml::XmlResult<Self> {
               use strong_xml::xmlparser::{ElementEnd, Token, Tokenizer};
               use strong_xml::XmlError;
               ;
               let __self_content;
               let mut __self_dir = None;
               let mut __self_refines = None;
               let mut __self_property = None;
               let mut __self_id = None;
               let mut __self_lang = None;
               let mut __self_scheme = None;
               let mut __self_name_legacy = None;
               let mut __self_content_legacy = None;

               print(&format!("read_till_element_start"));
               reader.read_till_element_start("meta")?;
               print(&format!("find_attribute"));
               while let Some((__key, __value)) = reader.find_attribute()? {
                   match __key {
                       "dir" => { ; __self_dir = Some(__value); ; }
                       "refines" => { ; __self_refines = Some(__value); ; }
                       "property" => { ; __self_property = Some(__value); ; }
                       "id" => { ; __self_id = Some(__value); ; }
                       "lang" => { ; __self_lang = Some(__value); ; }
                       "scheme" => { ; __self_scheme = Some(__value); ; }
                       "name" => { ; __self_name_legacy = Some(__value); ; }
                       "content" => {
                           ;
                           __self_content_legacy = Some(__value);
                           ;
                       }
                       key => { ; }
                   }
               }
               ;
               print(&format!("read_text"));
               let __value = reader.read_text("meta")?;
               print(&format!("get content"));
               __self_content = Some(__value);
               ;
               let __res =
                   Metafield{content:
                                 __self_content.ok_or(XmlError::MissingField{name:
                                                                                 "Metafield".to_owned(),
                                                                             field:
                                                                                 "content".to_owned(),})?,
                             dir: __self_dir,
                             refines: __self_refines,
                             property: __self_property,
                             id: __self_id,
                             lang: __self_lang,
                             scheme: __self_scheme,
                             name_legacy: __self_name_legacy,
                             content_legacy: __self_content_legacy,};
               ;
               return Ok(__res);
           }
       }

        print(&format!("Strong xml test"));

        let m = Metafield::from_str(r#"<meta name="cover" content="cover"/>"#).unwrap();

        print(&format!("Strong xml test {:?}",m));

     

    }

     
    fn opfs_xml_test () {
        print(&format!("Opfs test from source"));

        use crate::epub::opf::*;
        use strong_xml::XmlRead;
        use alloc::borrow::Cow;
        use alloc::vec::Vec;
        
        //let m = Metadata::from_str(r#"<metadata><dc:identifier id="id">http://www.gutenberg.org/ebooks/25545</dc:identifier><dc:subject>Una prueba</dc:subject><dc:subject>Otra prueba</dc:subject></metadata>"#).unwrap();

        let source = str::from_utf8(include_bytes!("../assets/childrens-literature/EPUB/package.opf")).unwrap();

        let m = Opf::from_str(source);
        //assert_eq!(m,Metadata { identifier: Vec::default(),subject: Vec::from([Subject { data: Cow::from("Una prueba") }, Subject { data: Cow::from("Otra prueba") }]) });
        print(&format!("{:?}",m));

    }

    
    fn ncx_xml_test () {
        print(&format!("NCX test from source"));

        use crate::epub::ncx::*;
        use strong_xml::XmlRead;
        use alloc::borrow::Cow;
        use alloc::vec::Vec;
        
        //let m = Metadata::from_str(r#"<metadata><dc:identifier id="id">http://www.gutenberg.org/ebooks/25545</dc:identifier><dc:subject>Una prueba</dc:subject><dc:subject>Otra prueba</dc:subject></metadata>"#).unwrap();

        let source = str::from_utf8(include_bytes!("../assets/childrens-literature/EPUB/toc.ncx")).unwrap();

        let m = Ncx::from_str(source).unwrap();
        //assert_eq!(m,Metadata { identifier: Vec::default(),subject: Vec::from([Subject { data: Cow::from("Una prueba") }, Subject { data: Cow::from("Otra prueba") }]) });
        print(&format!("{:?}",m));

    }    

    
    fn nav_point_test() {
        print(&format!("NavPoint"));

        use crate::epub::ncx::*;
        use strong_xml::XmlRead;
        use alloc::borrow::Cow;
        use alloc::vec::Vec;

        let source = r#"<navPoint id="np-313">
			<navLabel>
				<text>SECTION IV FAIRY STORIES—MODERN FANTASTIC TALES</text>
			</navLabel>
			<content src="s04.xhtml#pgepubid00492"/>
			<navPoint id="np-315">
				<navLabel>
					<text>BIBLIOGRAPHY</text>
				</navLabel>
				<content src="s04.xhtml#pgepubid00495"/>
			</navPoint>
			<navPoint id="np-317">
				<navLabel>
					<text>INTRODUCTORY</text>
				</navLabel>
				<content src="s04.xhtml#pgepubid00498"/>
			</navPoint>
        </navPoint>"#;

        let m = NavPoint::from_str(source).unwrap();
        print(&format!("{:?}",m));

    }
    
    fn page_list_test() {
        print(&format!("PageList"));

        use crate::epub::ncx::*;
        use strong_xml::XmlRead;
        use alloc::borrow::Cow;
        use alloc::vec::Vec;

        let source = r#"<pageList>
                <navInfo>
                    <text>Pages</text>
                </navInfo>
                <pageTarget type="normal">
                    <navLabel>
                        <text>169</text>
                    </navLabel>
                    <content src="s04.xhtml#Page_169"/>
                </pageTarget>
                <pageTarget type="normal">
                    <navLabel>
                        <text>170</text>
                    </navLabel>
                    <content src="s04.xhtml#Page_170"/>
                </pageTarget>
            </pageList>"#;

        let m = PageList::from_str(source).unwrap();
        print(&format!("{:?}",m));

    }

   
    fn container_xml_test () {
        print(&format!("Container test from source"));

        use crate::epub::container::*;
        use strong_xml::XmlRead;
        use alloc::borrow::Cow;
        use alloc::vec::Vec;
        
        
        let source = str::from_utf8(include_bytes!("../assets/childrens-literature/META-INF/container.xml")).unwrap();

        let m = Container::from_str(source).unwrap();
       
        print(&format!("{:?}",m));

    }        

   
    fn structure_test () {

        print(&format!("Structure test from source"));

        let source = include_bytes!("../assets/childrens-literature.epub");

        let epubarchive = EPubArchive::new(source).unwrap();

        let epub = epubarchive.epub();

        print(&format!("{:?}",epub));

    }

    
    /*
    fn serialize_test () {
        
        use authcomp::{Computation,Prover,AuthTProver,AuthT,Error};
        
        use crate::authpub::{publication::*, error::Error as PubError };

        use crate::api::ApiResponse;
        use alloc::string::String;
        use alloc::vec::Vec;
        use alloc::vec;
        use alloc::rc::Rc;
        use core::cell::RefCell;
        
        use minicbor::{Encode,Encoder};

        print(&format!("Serialize test"));

        let s_bytes = Prover::<ApiResponse,()>::shallow(&String::from("Test string"));
        let s : &str = Prover::<ApiResponse,()>::unshallow(&s_bytes).unwrap();

        print(&format!("String: {:?}",s));

        let mut epub = Publication::<Prover<ApiResponse,()>>::default();

        epub.add_link::<Prover<(),()>>(  String::from("stylesheet"),&vec![String::from("cover")],
                        String::from("http://test.com"),
                        false,None);

        let l = epub.get_cover::<Prover< Rc<RefCell<Link<Prover<ApiResponse,()>>>>,PubError> >(None);

        print(&format!("Cover {:?}",l.get_proofs()));

        let aepub = Prover::<ApiResponse,()>::auth(epub);

        
        print(&format!("Aepub {:?}",&aepub));

        let bytes = Prover::<ApiResponse,()>::shallow(&aepub);
        print(&format!("Bytes {:?}",&bytes));
        
        let ret : Result<AuthTProver<Publication<Prover<ApiResponse,()>>>, Error> = Prover::<ApiResponse,()>::unshallow(&bytes);

        print(&format!("Are equal? {:?}", ret.unwrap() == aepub));
        
        let mut e = Encoder::new(Vec::new());
        let encoded_epub = aepub.encode(&mut e);
        let r = e.into_inner();

        print(&format!("Encoded Bytes {:?}",&r));

        let decoded_epub : Result<AuthTProver<Publication<Prover<ApiResponse,()>>>, minicbor::decode::Error>=  minicbor::decode(&r);

        print(&format!("Decoded {:?}",&decoded_epub));

        print(&format!("Are equal? {:?}", decoded_epub.unwrap() == aepub));

    }

    */
    
    fn verifier_signature_test () {
        use authcomp::{Computation,Prover,Verifier,AuthTProver,AuthTVerifier,AuthT,Error};
        use authcomp::AuthType;

        use crate::authpub::{publication::*, error::Error as PubError };

        use crate::api::ApiResponse;
        use alloc::string::String;
        use alloc::vec::Vec;
        use alloc::vec;
        use alloc::rc::Rc;
        use core::cell::RefCell;

        print(&format!("Verifier signature test"));

        let mut epub = Publication::<Prover<ApiResponse,()>>::default();

        let aepub_prover = Prover::<(),()>::auth(epub);

        let prover_signature = Prover::<Publication::<Prover<ApiResponse,()>>,()>::signature(&aepub_prover);

        print(&format!("Prover {:?}",prover_signature));

        let mut epub = Publication::<Verifier<ApiResponse,()>>::default();

        let aepub_verifier = Verifier::<(),()>::auth(epub);

        let verifier_signature = Verifier::<Publication::<Verifier<ApiResponse,()>>,()>::signature(&aepub_verifier);
        print(&format!("Verifier {:?}",verifier_signature));

        assert_eq!(prover_signature,verifier_signature);
    }    
    
    
    /*
    fn verifier_test () {
        use authcomp::{Computation,Prover,Verifier,AuthTProver,AuthTVerifier,AuthT,Error};
        use authcomp::AuthType;

        use crate::authpub::{publication::*, error::Error as PubError };

        use crate::api::ApiResponse;
        use alloc::string::String;
        use alloc::vec::Vec;
        use alloc::vec;
        use alloc::rc::Rc;
        use core::cell::RefCell;

        print(&format!("Verifier test"));

        let mut epub = Publication::<Prover<ApiResponse,()>>::default();

        print(&format!("Prover pub {:?}",&epub)); 

        let ac_prover = epub.add_link::<Prover<(),()>>(  String::from("stylesheet"),&vec![String::from("cover")],
                        String::from("http://test.com"),
                        false,None);

        print(&format!("Prover pub after {:?}",&epub)); 
        print(&format!("Prover proofs {:?}",ac_prover.get_proofs()));

        let aepub_prover = Prover::<(),()>::auth(epub);
        let prover_signature = Prover::<Publication::<Prover<ApiResponse,()>>,()>::signature(&aepub_prover);

        //let l_prover = epub.get_cover::<Prover< Rc<RefCell<Link<Prover<ApiResponse,()>>>>,PubError> >(Some(ac_prover.get_proofs()));

        //print(&format!("Prover proofs {:?}",l_prover.get_proofs()));

        let mut epub = Publication::<Verifier<ApiResponse,()>>::default();

        print(&format!("Verifier pub {:?}",&epub)); 

        let ac_verifier = epub.add_link::<Verifier<(),()>>(  String::from("stylesheet"),&vec![String::from("cover")],
                        String::from("http://test.com"),
                        false,Some(ac_prover.get_proofs()));

        print(&format!("Verifier pub after {:?}",&epub)); 

        print(&format!("Verifier proofs {:?}",ac_verifier.get_proofs()));                        

        let aepub_verifier = Verifier::<(),()>::auth(epub);

        let verifier_signature = Verifier::<Publication::<Verifier<ApiResponse,()>>,()>::signature(&aepub_verifier);
        

        print(&format!("Prover signature {:?}",&prover_signature));
        print(&format!("Verifier signature {:?}",&verifier_signature));

        assert_eq!(prover_signature,verifier_signature);

        //let l_verifier = epub.get_cover::<Verifier< Rc<RefCell<Link<Verifier<ApiResponse,()>>>>,PubError> >(Some(ac_verifier.get_proofs()));

        //print(&format!("Verifier proofs {:?}",l_verifier.get_proofs()));
    }

     
    fn verifier_test_2 () {
        use authcomp::{Computation,Prover,Verifier,AuthTProver,AuthTVerifier,AuthT,Error};
        use authcomp::AuthType;

        use crate::authpub::{publication::*, error::Error as PubError };

        use crate::api::ApiResponse;
        use alloc::string::String;
        use alloc::vec::Vec;
        use alloc::vec;
        use alloc::rc::Rc;
        use core::cell::RefCell;

        print(&format!("Verifier test 2"));

        let mut epub = Publication::<Prover<ApiResponse,()>>::default();

        print(&format!("Prover pub {:?}",&epub)); 

        let ac_prover = epub.add_link::<Prover<(),()>>(  String::from("stylesheet"),&vec![String::from("cover")],
                        String::from("http://test.com"),
                        false,None);

        print(&format!("Prover pub after {:?}",&epub)); 
        print(&format!("Prover proofs {:?}",ac_prover.get_proofs()));

        let l_prover = epub.get_cover::<Prover< Rc<RefCell<Link<Prover<ApiResponse,()>>>>,PubError> >(Some(ac_prover.get_proofs()));

        print(&format!("Prover proofs {:?}",l_prover.get_proofs()));

        let mut epub = Publication::<Verifier<ApiResponse,()>>::default();

        print(&format!("Verifier pub {:?}",&epub)); 

        let ac_verifier = epub.add_link::<Verifier<(),()>>(  String::from("stylesheet"),&vec![String::from("cover")],
                        String::from("http://test.com"),
                        false,Some(l_prover.get_proofs()));

        print(&format!("Verifier pub after {:?}",&epub)); 

        print(&format!("Verifier proofs {:?}",ac_verifier.get_proofs()));                        
        

        let l_verifier = epub.get_cover::<Verifier< Rc<RefCell<Link<Verifier<ApiResponse,()>>>>,PubError> >(Some(ac_verifier.get_proofs()));

        print(&format!("Verifier proofs {:?}",l_verifier.get_proofs()));

        assert_eq!(l_verifier.get_proofs().len(), 0);
    }    
    */

   
    fn verifier_test_3 () {
        use authcomp::{Computation,Prover,Verifier,AuthTProver,AuthTVerifier,AuthT,Error};
        use authcomp::{AuthType,AuthContainer};
        use authcomp::{Encode,Decode,DecodeOwned,Serialize,DeserializeOwned};

        use crate::authpub::{publication::*, metadata::*, mediaoverlay::*, error::Error as PubError };

        use crate::api::ApiResponse;
        use alloc::string::String;
        use alloc::vec::Vec;
        use alloc::vec;
        use alloc::rc::Rc;
        use core::cell::RefCell;
        use alloc::collections::BTreeMap;
        use core::fmt::Debug;
        use is_type::Is;
        
        print(&format!("Verifier test 3"));


        fn test_3<C> () -> Vec<Rc<RefCell<AuthT<MediaOverlayNode<C>,C>>>>
            where   
                C:Computation,
                C:AuthType<String>,
                //C:AuthType<Vec<AuthT<String,C>>>,
                //C:AuthType<Vec<AuthT<Link<C>,C>>>,
                //C:AuthType<Vec<AuthT<PublicationCollection<C>,C>>>,
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
                C:AuthType<Vec<AuthT<String,C>>>,
                C:AuthType<Vec<AuthT<Collection<C>,C>>>,
                //C:AuthType<Vec<AuthT<Subject<C>,C>>>,
                //C:AuthType<Vec<AuthT<Contributor<C>,C>>>,        
                C:AuthContainer<MediaOverlayNode<C>>,
                C:AuthContainer<Meta<C>>,
                C:AuthContainer<PublicationCollection<C>>,
                C:AuthContainer<Link<C>>,
                C:DecodeOwned,
                C:Encode,
                C:Default,
                C: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq,
                //C: AuthContainer<Vec<<C as AuthType<MediaOverlayNode<C>>>::AuthT>>,
                C: AuthType<Vec<Rc<RefCell<<C as AuthType<MediaOverlayNode<C>>>::AuthT>>>>,
                <C as AuthContainer<MediaOverlayNode<C>>>::AuthT : Is< Type=<C as AuthType<Vec<Rc<RefCell< <C as AuthType<MediaOverlayNode<C>>>::AuthT>>>>>::AuthT >,

                C:AuthType<Vec<Rc<RefCell< AuthT<PublicationCollection<C>,C>>>>>,
                C:AuthType<Vec<Rc<RefCell< AuthT<Link<C>,C>>>>>,
                C:AuthType<Vec<Rc<RefCell< AuthT<Meta<C>,C>>>>>,
        
                C:AuthType<Vec<Rc<RefCell< AuthT<Contributor<C>,C>>>>>,
                C:AuthType<Vec<Rc<RefCell< AuthT<Subject<C>,C>>>>>,
        
                C:AuthType<Vec<Rc<RefCell<AuthT<String,C>>>>>,
                C:AuthType<Vec<u8>>,
                C:AuthType<BTreeMap<AuthT<Vec<u8>,C>, AuthT<Vec<u8>,C>>>,
                C:AuthType<BTreeMap<AuthT<Vec<u8>,C>, ( AuthT<Vec<u8>,C>, Option<authselect::ElementRef<C>>)>>,
                C:AuthType<authdoc::Node>,
                C:AuthType<indextree::Arena<AuthT<authdoc::Node,C>>>
        {
            let mut epub = Publication::<C>::default();
    
            epub.add_link2(  String::from("stylesheet"),&vec![String::from("cover")],
                            String::from("http://test.com"),
                            false);
    
            epub.get_cover();

            epub.find_all_mediaoverlay()
        }

        
        let prover = Prover::<Vec<Rc<RefCell<<Prover<(),()> as AuthType<MediaOverlayNode<Prover<(),()>>>>::AuthT>>>,PubError>::run (|| {
            Ok(test_3::<Prover<(),()>>())
        });

        //print(&format!("Prover link {:?}",prover.get().unwrap().borrow().href));

        let verifier = Verifier::<Vec<Rc<RefCell<<Verifier<(),()> as AuthType<MediaOverlayNode<Verifier<(),()>>>>::AuthT>>>,PubError>::run_with_proofs (prover.transfer_proofs(), || {
            Ok(test_3::<Verifier<(),()>>())
        });

        print(&format!("Verifier proofs: {:?}",verifier.get_proofs()));
        print(&format!("Verifier result {:?}",*verifier));

        assert_eq!(verifier.get_proofs().is_empty(),true);
    }        

    
    fn heterogeneus_list_test () {

        use typed_key::{typed_key,Key};
        use crate::authpub::types::*;
        use alloc::string::String;

        let q  = EmptySQ;
        let q = q.push(5);
        //let q = q.push("dadf");

        print(&format!("Q: {:?}",q));

        let list = cons(8, cons("Hello!!!", cons(4.3, Nil)));

        print(&format!("List length: {:?}",list.len()));

        let mut iter = list.iter();
        let (item, mut iter) = iter.next(); 
        print(&format!("Item: {:?}",item));
        let (item, mut iter) = iter.next(); 
        print(&format!("Item: {:?}",item));
        let (item, mut iter) = iter.next(); 
        print(&format!("Item: {:?}",item));

        print(&format!("List: {:?}",list));

        let i : Key<u8> = typed_key!("test");
        let k : Key<String> = typed_key!("test2");

        let mut h = HashMap::new();
        h.insert(i,5);
        h.insert(k,String::from("ddd"));

        print(&format!("Map: {:?}",h));

        print(&format!("get(i): {:?}",h.get(i)));
        print(&format!("get(k): {:?}",h.get(k)));
    }

    
    fn add_rel_test () {


        use authcomp::{Computation,Prover,Verifier,AuthTProver,AuthTVerifier,AuthT,Error};
        use authcomp::AuthType;

        use crate::authpub::{publication::*, error::Error as PubError };

        use crate::api::ApiResponse;
        use alloc::string::String;
        use alloc::vec::Vec;
        use alloc::vec;
        use alloc::rc::Rc;
        use core::cell::RefCell;

        print(&format!("add_rel test"));
        
        let mut link = Link::<Prover<ApiResponse,()>>::default();

        link.add_rel("test rel");
        link.add_rel("test rel");
        link.add_rel("Another one");

        print(&format!("Link: {:?}", &link.rel));

        let v_rel_ref = Prover::<ApiResponse,()>::unauth2::<Vec<Rc<RefCell<AuthT<String,Prover<ApiResponse,()>>>>>>(&link.rel);

        assert_eq!((&*v_rel_ref.borrow()).len(),2);

    }

    
    fn add_title_test () {

        use authcomp::{Computation,Prover,Verifier,AuthTProver,AuthTVerifier,AuthT,Error};
        use authcomp::AuthType;

        use crate::authpub::{publication::*, error::Error as PubError };
        use crate::epubparser::EPubParser;

        use crate::api::ApiResponse;
        use alloc::string::String;
        use alloc::vec::Vec;
        use alloc::vec;
        use alloc::rc::Rc;
        use core::cell::RefCell;

        print(&format!("Add Title from source"));


        let source = include_bytes!("../assets/childrens-literature3.epub");

        let publication = EPubParser::<Prover<(),()>>::parse (source);

        print(&format!("Publication: {:?}",publication));
    }

    
    fn manifest_test () {
        use authcomp::{Computation,Prover,Verifier,AuthTProver,AuthTVerifier,AuthT,Error};
        use authcomp::AuthType;
        use authcomp::ToJSON;

        use crate::authpub::{publication::*, error::Error as PubError };
        use crate::epubparser::EPubParser;

        use crate::api::ApiResponse;
        use alloc::string::String;
        use alloc::vec::Vec;
        use alloc::vec;
        use alloc::rc::Rc;
        use core::cell::RefCell;

        print(&format!("Manifest test"));


        let source = include_bytes!("../assets/childrens-literature3.epub");

        let publication = EPubParser::<Prover<(),()>>::parse (source).unwrap();

        print(&format!("Publication: {}",publication.publication.serialize_json()));
    }

    #[test_case]
    fn selector_test () {

        
        use authcomp::{Computation,Prover};

        use authparser::{read_auth_xml,AuthDocument};
        use authselect::ElementRef;
        use alloc::rc::Rc;
        use alloc::vec::Vec;
        use alloc::string::String;

        print(&format!("Selector test"));

        let s = br#"<!DOCTYPE html>
                <html lang="en">
                    <head>
                        <meta charset="UTF-8"/>
                        <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                        <title>Document</title>
                        <style>
                            body {
                                background: black;
                            }
                    
                            h1 {
                                color: white;
                            }
                        </style>
                    </head>
                    <body>
                        <h1 id="test" class="test">Hello world</h1>
                        <!-- There should be more text here -->
                        <script class="test">
                            const title = document.querySelector("h1")
                            title.innerText = "Hello from script"
                        </script>
                    </body>
                </html>        
            "#;

        let comp = Prover::<Vec<String>,()>::run (|| {

            let mut auth_document = AuthDocument::<Prover<(),()>>::default();

            let root = read_auth_xml::<Prover<(),()>>(s,&mut auth_document);
            print(&format!("Root: {:?}",root));
            print(&format!("Document: {:?}",auth_document));

            let document = ElementRef::<Prover<(),()>> {
                    id: root,
                    doc: Rc::new(auth_document)
            };

            let result = document.select_fmt("script");

            print(&format!("Result: {:?}",result));

            Ok(result)
        });

        print(&format!("Computation: {:?}",comp.get()));
        print(&format!("Computation Proofs Length: {}",comp.get_proofs().len()));
    }

    #[test_case] 
    fn another_selector_test () {

        
        use authcomp::{Computation,Prover};

        use authparser::{read_auth_xml,AuthDocument};
        use authselect::ElementRef;
        use authselect::{DOMRange,DOMIndex};
        use alloc::rc::Rc;
        use alloc::vec::Vec;
        use alloc::string::String;
        use indextree::NodeId;
        use wapc_guest as guest;
        use core::cell::RefCell;

        print(&format!("Another Selector test -->"));

        let s = include_bytes!("../assets/childrens-literature/EPUB/s04.xhtml");

        let comp = Prover::<Vec<Rc<RefCell<authdoc::Node>>>,()>::run (|| {

            let mut auth_document = AuthDocument::<Prover<(),()>>::default();

            let root = read_auth_xml::<Prover<(),()>>(s,&mut auth_document);
            print(&format!("Root: {:?}",root));
            //guest::console_log(&format!("Document: {:?}",auth_document));
            
            let document = ElementRef::<Prover<(),()>> {
                    id: root,
                    doc: Rc::new(auth_document)
            };

            print(&format!("Selecting..."));
            //let result = document.select_with_path("html body #pgepubid00492 #pgepubid00498 div");

            //let result = document.select_fmt("span");
            //guest::console_log(&format!("Result: {:?}",result));

            let range = DOMRange {
                start: DOMIndex {
                    css_selector: String::from("html body #pgepubid00492 #pgepubid00498 div"),
                    text_node_index:None,
                    offset:None
                },
                end: DOMIndex {
                    css_selector: String::from("html body #pgepubid00492 #pgepubid00501 h3"),
                    text_node_index:None,
                    offset:None
                }
            };

            let result = document.select_nodes_with_range(&range);

            Ok(result)
            
        });

        print(&format!("Computation: {:?}",comp.get()));
        //guest::console_log(&format!("Computation Proofs Length: {:?}",comp.get_proofs().len()));
    }

    
    fn another_cfi_test () {

        
        use authcomp::{Computation,Prover};

        use authparser::{read_auth_xml,AuthDocument};
        use authselect::{ElementRef,CFIComponent,CFIComponentList};
        use alloc::rc::Rc;
        use alloc::vec::Vec;
        use alloc::string::String;
        use indextree::NodeId;
        use wapc_guest as guest;
        use core::cell::RefCell;

        print(&format!("CFI Selector test -->"));

        let s = include_bytes!("../assets/childrens-literature/EPUB/s04.xhtml");

        let comp = Prover::<Option<Rc<RefCell<authdoc::Node>>>,()>::run (|| {

            let mut auth_document = AuthDocument::<Prover<(),()>>::default();

            let root = read_auth_xml::<Prover<(),()>>(s,&mut auth_document);
            print(&format!("Root: {:?}",root));
            //guest::console_log(&format!("Document: {:?}",auth_document));
            
            let document = ElementRef::<Prover<(),()>> {
                    id: root,
                    doc: Rc::new(auth_document)
            };

            print(&format!("Selecting..."));

            let mut list = CFIComponentList::new();

            list.push ( CFIComponent {
                node_index: 2,
                qualifier : None,
                character_offset : None,
                temporal_offset: None,
                text_qualifier: None
            });

            list.push ( CFIComponent {
                node_index: 4,
                qualifier : None,
                character_offset : None,
                temporal_offset: None,
                text_qualifier: None
            });

            list.push ( CFIComponent {
                node_index: 2,
                qualifier : None,
                character_offset : None,
                temporal_offset: None,
                text_qualifier: None
            });

            list.push ( CFIComponent {
                node_index: 4,
                qualifier : None,
                character_offset : None,
                temporal_offset: None,
                text_qualifier: None
            });

            list.push ( CFIComponent {
                node_index: 1,
                qualifier : None,
                character_offset : None,
                temporal_offset: None,
                text_qualifier: None
            });
            let result = document.select_cfi(&list);

            //let result = document.select_fmt("span");
            //guest::console_log(&format!("Result: {:?}",result));

            Ok(result)
            
        });

        print(&format!("Computation: {:?}",comp.get()));
        //guest::console_log(&format!("Computation Proofs Length: {:?}",comp.get_proofs().len()));
    }

    
    fn range_cfi_test () {

        
        use authcomp::{Computation,Prover};

        use authparser::{read_auth_xml,AuthDocument};
        use authselect::{ElementRef,CFIComponent,CFIComponentList};
        use alloc::rc::Rc;
        use alloc::vec::Vec;
        use alloc::string::String;
        use indextree::NodeId;
        use wapc_guest as guest;
        use core::cell::RefCell;

        print(&format!("CFI Selector test -->"));

        let s = include_bytes!("../assets/childrens-literature/EPUB/s04.xhtml");

        let comp = Prover::<Vec<Rc<RefCell<authdoc::Node>>>,()>::run (|| {

            let mut auth_document = AuthDocument::<Prover<(),()>>::default();

            let root = read_auth_xml::<Prover<(),()>>(s,&mut auth_document);
            print(&format!("Root: {:?}",root));
            //guest::console_log(&format!("Document: {:?}",auth_document));
            
            let document = ElementRef::<Prover<(),()>> {
                    id: root,
                    doc: Rc::new(auth_document)
            };

            print(&format!("Selecting..."));

            let mut from= CFIComponentList::new();

            from.push ( CFIComponent {
                node_index: 2,
                qualifier : None,
                character_offset : None,
                temporal_offset: None,
                text_qualifier: None
            });

            from.push ( CFIComponent {
                node_index: 4,
                qualifier : None,
                character_offset : None,
                temporal_offset: None,
                text_qualifier: None
            });

            from.push ( CFIComponent {
                node_index: 2,
                qualifier : None,
                character_offset : None,
                temporal_offset: None,
                text_qualifier: None
            });

            from.push ( CFIComponent {
                node_index: 4,
                qualifier : None,
                character_offset : None,
                temporal_offset: None,
                text_qualifier: None
            });

            from.push ( CFIComponent {
                node_index: 1,
                qualifier : None,
                character_offset : None,
                temporal_offset: None,
                text_qualifier: None
            });


            let mut to= CFIComponentList::new();

            to.push ( CFIComponent {
                node_index: 2,
                qualifier : None,
                character_offset : None,
                temporal_offset: None,
                text_qualifier: None
            });

            to.push ( CFIComponent {
                node_index: 4,
                qualifier : None,
                character_offset : None,
                temporal_offset: None,
                text_qualifier: None
            });

            to.push ( CFIComponent {
                node_index: 2,
                qualifier : None,
                character_offset : None,
                temporal_offset: None,
                text_qualifier: None
            });

            to.push ( CFIComponent {
                node_index: 6,
                qualifier : None,
                character_offset : None,
                temporal_offset: None,
                text_qualifier: None
            });

            to.push ( CFIComponent {
                node_index: 6,
                qualifier : None,
                character_offset : None,
                temporal_offset: None,
                text_qualifier: None
            });

            to.push ( CFIComponent {
                node_index: 2,
                qualifier : None,
                character_offset : None,
                temporal_offset: None,
                text_qualifier: None
            });

            let result = document.select_cfi_range(&from,&to);

            //let result = document.select_fmt("span");
            //guest::console_log(&format!("Result: {:?}",result));

            Ok(result)
            
        });

        print(&format!("Computation: {:?}",comp.get()));
        //guest::console_log(&format!("Computation Proofs Length: {:?}",comp.get_proofs().len()));
    }
}