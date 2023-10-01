use logos::Logos;
use logos::Lexer;

extern crate alloc;

use alloc::borrow::ToOwned;
use smallvec::{SmallVec,smallvec};

#[derive(Logos, Debug, PartialEq,Eq,Clone,Copy)]

enum Html<'source> {
   
    #[regex(r"<!--([^->]*)-->", |lex| lex.slice())]
    HtmlComment(&'source str),

    //#[regex(r"<\?xml ([^?]*)\?>", |lex| lex.slice())]
    #[regex(r"<\?([^?]*)\?>", |lex| lex.slice())]
    XmlDeclarationOrPi(&'source str),

    #[regex(r"<!\[CDATA\[([^\]]*)\]\]>", |lex| lex.slice())]
    CdataDeclaration(&'source str),

    #[regex(r"<!\[([^\]]*)\]>", |lex| lex.slice())]
    HtmlConditionalComment(&'source str),

    #[regex(r"<!DOCTYPE([^>]*)>", |lex| lex.slice())]
    DtdDeclaration(&'source str),

    #[regex("<%([^%>]*)%>", |lex| lex.slice())]
    ScripletDeclaration(&'source str),

    //#[regex("((' '|\t|\r)?\n)+", |lex| lex.slice())]
    #[regex("(' '|\t|\r|\n)+", logos::skip)]
    SeaWS,

    #[regex("<script([^>]*)>", |lex| lex.slice())]
    ScriptOpen(&'source str),

    #[regex("<style([^>]*)>", |lex| lex.slice())]
    StyleOpen(&'source str),

    #[token("<", |lex| lex.slice())]
    TagOpen(&'source str),

    #[regex("[^<]+", |lex| lex.slice())]
    HtmlText(&'source str),

    #[error]
    Error,
}

/// A result type wrapping a token with start and end locations.

pub type Location = usize;
pub type Span<T, E> = Result<(Location, T, Location), E>;

/// A lexer error.
#[derive(Debug)]
pub enum LexicalError {
    UnexpectedConversionToken,
    NoToken
}


fn html_to_token(h: Html,start: Location, end: Location) -> Span<Tokens,LexicalError> {
    match h {
        Html::HtmlComment(t) => Ok((start,Tokens::HtmlConditionalComment(t),end)),
        Html::XmlDeclarationOrPi(t) => Ok((start,Tokens::XmlDeclarationOrPi(t),end)),
        Html::CdataDeclaration(t) => Ok((start,Tokens::CdataDeclaration(t),end)),
        Html::HtmlConditionalComment(t) => Ok((start,Tokens::HtmlConditionalComment(t),end)),
        Html::DtdDeclaration(t) => Ok((start,Tokens::DtdDeclaration(t),end)),
        Html::ScripletDeclaration(t) => Ok((start,Tokens::ScripletDeclaration(t),end)),
        Html::SeaWS => Err(LexicalError::UnexpectedConversionToken),
        Html::ScriptOpen(t) => Ok((start,Tokens::ScriptOpen(t),end)),
        Html::StyleOpen(t) => Ok((start,Tokens::StyleOpen(t),end)),
        Html::TagOpen(t) => Ok((start,Tokens::TagOpen(t),end)),
        Html::HtmlText(t) => Ok((start,Tokens::HtmlText(t),end)),
        Html::Error => Err(LexicalError::UnexpectedConversionToken),
    }

}

fn tag_to_token(t: Tag,start: Location, end: Location) -> Span<Tokens,LexicalError> {
    match t {
        Tag::TagClose(t) => Ok((start,Tokens::TagClose(t),end)),
        Tag::TagSlashClose(t) => Ok((start,Tokens::TagSlashClose(t),end)),
        Tag::TagSlash(t) => Ok((start,Tokens::TagSlash(t),end)),
        Tag::TagEquals(t) => Ok((start,Tokens::TagEquals(t),end)),
        Tag::TagName(t) => Ok((start,Tokens::TagName(t),end)),
        Tag::TagWhiteSpace => Ok((start,Tokens::TagWhiteSpace,end)),
        Tag::Error => Err(LexicalError::UnexpectedConversionToken),
    }
}

    
fn script_to_token (s: Script,start: Location, end: Location) ->Span<Tokens,LexicalError>{
    match s {
        Script::ScriptBody(t) => Ok((start,Tokens::ScriptBody(t),end)),
        Script::ScriptShortBody(t) => Ok((start,Tokens::ScriptShortBody(t),end)),
        Script::Error => Err(LexicalError::UnexpectedConversionToken),
    }
}

//Style mode
fn style_to_token (s: Style,start: Location, end: Location) -> Span<Tokens,LexicalError> {
    match s {
        Style::StyleBody(t) => Ok((start,Tokens::StyleBody(t),end)),
        Style::StyleShortBody(t) => Ok((start,Tokens::StyleShortBody(t),end)),
        Style::Error => Err(LexicalError::UnexpectedConversionToken),
    }
}

//Attribute mode
fn att_to_token (a: Attribute,start: Location, end: Location) -> Span<Tokens,LexicalError> {
    match a {
        Attribute::Attribute(t) => Ok((start,Tokens::AttributeValue(t.trim_start_matches("\"").trim_end_matches("\"")),end)),
        Attribute::Error => Err(LexicalError::UnexpectedConversionToken),
    }
}

macro_rules! pop_mode {
    ($self:ident,$lexer:ident) => {
        {
 
            
            $self.lexers_stack.pop();
        
            //Get current mode
            let mode = $self.lexers_stack[$self.lexers_stack.len()-1];
        
            match mode {
                ModesType::Html => {
                    $self.current_mode = Modes::Html($lexer.to_owned().morph());
                },
                ModesType::Tag => {
                    $self.current_mode = Modes::Tag($lexer.to_owned().morph());
                },
                ModesType::Style => {
                    $self.current_mode = Modes::Style($lexer.to_owned().morph());
                },
                ModesType::Script => {
                    $self.current_mode = Modes::Script($lexer.to_owned().morph());
                },
                ModesType::Attribute => {
                    $self.current_mode = Modes::Attribute($lexer.to_owned().morph());
                },
                ModesType::None=> panic!("The adapter has not been initializated or the mode is wrong")
            }
            }
        }
}


#[derive(Logos, Debug, PartialEq,Eq,Clone,Copy)]

enum Tag<'source> {

    #[token(">", |lex| lex.slice())]
    TagClose(&'source str),

    #[token(r"/>", |lex| lex.slice())]
    TagSlashClose(&'source str),

    #[token("/", |lex| lex.slice())]
    TagSlash(&'source str),

    #[token("=", |lex| lex.slice())]
    TagEquals(&'source str),

    #[regex(r#"([a-zA-Z]|[\u2070-\u218F]|[\u2C00-\u2FEF]|[\u3001-\uD7FF]|[\uF900-\uFDCF]|[\uFDF0-\uFFFD])(([a-zA-Z]|[\u2070-\u218F]|[\u2C00-\u2FEF]|[\u3001-\uD7FF]|[\uF900-\uFDCF]|[\uFDF0-\uFFFD])|[-_.:]|[0-9]|[\u00B7]|[\u0300-\u036F]|[\u203F-\u2040])*"#, |lex| lex.slice())]
    TagName(&'source str),

    #[regex(r"[ \t\r\n]",logos::skip)]
    TagWhiteSpace,

    #[error]
    Error,
}



#[derive(Logos, Debug, PartialEq,Eq,Clone,Copy)]

enum Script<'source> {

    #[regex("([^<>]*)</script>", |lex| {let s = lex.slice(); &s[..s.len()-9]})]
    ScriptBody(&'source str),
    #[regex("([^<>]*)</>", |lex| lex.slice())]
    ScriptShortBody(&'source str),

    #[error]
    Error,
}


#[derive(Logos, Debug, PartialEq,Eq,Clone,Copy)]

enum Style<'source> {

    #[regex("([^<>]*)</style>", |lex| {let s = lex.slice(); &s[..s.len()-8]})]
    StyleBody(&'source str),
    #[regex("([^<>]*)</>", |lex| lex.slice())]
    StyleShortBody(&'source str),

    #[error]
    Error,
}


#[derive(Logos, Debug, PartialEq,Eq,Clone,Copy)]

enum Attribute<'source> {

    #[regex(r#"[ ]*(("[^<"]*")|('[^<']*')|(([-_./+?=:;#0-9a-zA-Z])+' '?)|(#[0-9a-fA-F]+)|([0-9]+%?))"#, |lex| lex.slice())]
    Attribute(&'source str),
    #[error]
    Error,
}



#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum ModesType {
    Html,
    Tag,
    Script,
    Style,
    Attribute,
    None
}

#[derive(Clone)]
enum Modes<'source> {
    Html(Lexer<'source, Html<'source>>),
    Tag(Lexer<'source, Tag<'source>>),
    Style(Lexer<'source, Style<'source>>),
    Script(Lexer<'source, Script<'source>>),
    Attribute(Lexer<'source, Attribute<'source>>),
    None
}



#[derive(Debug, PartialEq, Eq,Copy,Clone)]
pub enum Tokens<'source> {

    //Html mode
    HtmlComment(&'source str),
    XmlDeclarationOrPi(&'source str),
    CdataDeclaration(&'source str),
    HtmlConditionalComment(&'source str),
    DtdDeclaration(&'source str),
    ScripletDeclaration(&'source str),
    SeaWS(&'source str),
    ScriptOpen(&'source str),
    StyleOpen(&'source str),
    TagOpen(&'source str),
    HtmlText(&'source str),

    //Tag mode
    TagClose(&'source str),
    TagSlashClose(&'source str),
    TagSlash(&'source str),
    TagEquals(&'source str),
    TagName(&'source str),
    TagWhiteSpace,    

    //Script mode
    ScriptBody(&'source str),
    ScriptShortBody(&'source str),

    //Style mode
    StyleBody(&'source str),
    StyleShortBody(&'source str),

    //Attribute mode
    AttributeValue(&'source str),

    Error

}



const ADAPTERS_STACK_SIZE : usize = 10;


pub struct ModeAdapter<'source> {
    current_mode : Modes<'source>,
    lexers_stack: SmallVec<[ModesType ;ADAPTERS_STACK_SIZE]>,

}

impl <'source> ModeAdapter<'source> {

    pub fn new () ->Self {
        ModeAdapter {
            current_mode : Modes::None,
            lexers_stack : smallvec!(ModesType::None),
        }
    }

    pub fn initialize (&mut self,mode : ModesType, s: &'source str) {

        match mode {
            ModesType::Html => {
                self.current_mode = Modes::Html(Html::lexer(s));
            },
            ModesType::Tag => {
                self.current_mode = Modes::Tag(Tag::lexer(s));
            },
            ModesType::Style => {
                self.current_mode = Modes::Style(Style::lexer(s));
            },
            ModesType::Script => {
                self.current_mode = Modes::Script(Script::lexer(s));
            },
            ModesType::Attribute => {
                self.current_mode = Modes::Attribute(Attribute::lexer(s));
            },
            ModesType::None=> panic!("The adapter has not been initializated or the mode is wrong")
        }

        self.lexers_stack.push(mode);   

    }

}

impl <'source> Default for ModeAdapter<'source> {
    fn default() -> Self {
        ModeAdapter {
            current_mode : Modes::None,
            lexers_stack : smallvec!(ModesType::None),
        }
    }
}

// Clones as we switch between modes
impl<'source> Iterator for ModeAdapter<'source> {
    type Item = Span<Tokens<'source>,LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {

        match &mut self.current_mode {
            Modes::Html(ref mut html) => {
                let result = html.next();
                let range = html.span();
        
                match result {
                    Some(Html::TagOpen(_)) => {
        
                        self.current_mode = Modes::Tag(html.to_owned().morph());
                        self.lexers_stack.push(ModesType::Tag);

                    }
                    Some(Html::ScriptOpen(_)) => {
                        self.current_mode = Modes::Script(html.to_owned().morph());
                        self.lexers_stack.push(ModesType::Script);
                    }
                    Some(Html::StyleOpen(_)) => {
                        self.current_mode =Modes::Style(html.to_owned().morph());
                        self.lexers_stack.push(ModesType::Style);
                    }
                    _ => {}
                }


                result.map(|t| html_to_token(t,range.start,range.end))

            }

            Modes::Tag(ref mut tag) => {
                let result = tag.next();
                let range = tag.span();

                match result {
                    Some(Tag::TagClose(_)) => {
                        pop_mode!(self,tag);
                    }
                    Some(Tag::TagSlashClose(_)) => {
                        pop_mode!(self,tag);
                    }
                    Some(Tag::TagEquals(_)) => {
                        self.current_mode = Modes::Attribute(tag.to_owned().morph());
                        self.lexers_stack.push(ModesType::Attribute);
                    }                                        
                    _ => {}
                }

                result.map(|t| tag_to_token(t,range.start,range.end))

            }

            Modes::Style(ref mut style) => {
                let result = style.next();
                let range = style.span();

                match result {
                    Some(Style::StyleBody(_)) => {
                        pop_mode!(self,style);
                    }
                    Some(Style::StyleShortBody(_)) => {
                        pop_mode!(self,style);
                    }
                    _ => {}
                }

                result.map(|t| style_to_token(t,range.start,range.end)) 

            }

            Modes::Script(ref mut script) => {
                let result = script.next();
                let range = script.span();
   
                match result {
                    Some(Script::ScriptBody(_)) => {
                        pop_mode!(self,script);
                    }
                    Some(Script::ScriptShortBody(_)) => {
                        pop_mode!(self,script);
                    }
                    _ => {}
                } 


                result.map(|t| script_to_token(t,range.start,range.end))
  
            }

            Modes::Attribute(ref mut att) => {
                let result = att.next();
                let range = att.span();
        
                match result {
                    Some(Attribute::Attribute(_)) => {
                        pop_mode!(self,att);
                    }
                    _ => {}
                }

                result.map(|t| att_to_token(t,range.start,range.end))

            },
            Modes::None => panic!("The adapter has not been initializated or the mode is wrong")

        }
    }
}

