use logos::Logos;
use logos::Lexer;


#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token<'source> {
    // Operadores 
    #[token("/")]
    Slash,
    #[token(":")]
    Colon,
    #[token("@")]
    At,
    #[token("~")]
    Tilde,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token("=")]
    Equals,
    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("^")]
    Circumflex,
    #[token("!")]
    Exclamation,
    #[token("epubcfi")]
    EpubCFI,
    // Valores literales
    #[regex("[1-9][0-9]*", |lex| lex.slice(),priority = 12)]
    Integer(&'source str),
    #[regex("0")]
    Zero,
    //#[regex(r#""(?:[^"\\]|\\.)*""#, |lex| lex.slice())]
    #[regex(r#"[^/\^\[\]\(\),;:=,!@~]+"#, |lex| lex.slice(), priority = 6)]
    ValueSpace(&'source str),    
    #[regex(r#"[^\s/\^\[\]\(\),;:=,!@~]+"#, |lex| lex.slice(), priority = 8)]
    ValueNoSpace(&'source str),
    // Nota: Debes adaptar la regex de StringLiteral para que maneje correctamente los caracteres especiales escapados.

    // Valores con parte decimal
    #[regex("[0-9]+\\.[0-9]+", |lex| lex.slice(), priority = 10)]
    Number(&'source str),

    // Espacios (ignorados)
    #[regex(r"[ \t\n\f]+", logos::skip, priority = 14)]
    Whitespace,

    // El error se manejará en el método de análisis posterior
    #[error]
    Error,
}

