use authcomp::{Computation,Prover,Verifier,AuthTProver,AuthTVerifier,AuthT,Error};
use authcomp::AuthType;


use epubcontract::EPubParser;


fn main() {
    let source = include_bytes!("../../../epubcontract/assets/sagrada-biblia-universidad-de-navarra_processed.epub");

    let publication = EPubParser::<Prover<(),()>>::parse (source);
    
    println!("Publication: {:?}",publication);
}
