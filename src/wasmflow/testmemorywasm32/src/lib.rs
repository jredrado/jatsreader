use authcomp::{Computation,NoProofs,AuthTNoProofs,AuthT,Error};
use authcomp::AuthType;


use epubcontract::EPubParser;


#[no_mangle]
pub extern "C" fn main() {
    let source = include_bytes!("../../../epubcontract/assets/pnas.202202536_jats.epub");

    let publication = EPubParser::<NoProofs<(),()>>::parse (source);
    
    println!("Publication: {:?}",publication);
}

