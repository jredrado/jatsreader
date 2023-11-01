use anyhow::Result;

use authselect::*;


fn main() -> Result<()> {

    let p = FragmentParser::new();
    let r = p.parse("epubcfi( /6/4[chapter01]:5/10[1]:0 )");

    println!("Result {:?}",&r);

    Ok(())
}
