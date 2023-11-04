use anyhow::Result;
use logos::Logos;

use authselect::*;


fn main() -> Result<()> {

    //let input = "/6/4[chapter01]:5/10[1]:0";
    let input = "epubcfi(/6/4[chap01ref]!/4[body01]/10[para05],/2/1:1,/3:4)";
    let mut lexer = authselect::Token::lexer(input);

    let p = FragmentParser::new();
    let r = p.parse(input,lexer);

    println!("Result {:?}",&r);

    //let input = "/6/4[chap01ref]!/4[body01]/10[para05]/2/1:3[,y]";
    /*
    let input = "/6/4[chap01ref]!/4[body01]/10[para05]/2/1:3[yyy;s=b dadfd]";

    let mut lexer = authselect::Token::lexer(input);

    for token in lexer{
        println!("Token {:?}",&token);
    }
    
    */

    Ok(())
}
