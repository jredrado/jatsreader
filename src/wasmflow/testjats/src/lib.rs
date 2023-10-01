
extern crate alloc;

//mod alter_table;
//mod index;
//mod transaction;
//mod authstorage;
//mod authmemstorage;
mod merkletreestorage;
//mod sleddb;
//mod sledtree;

use authcomp::NoProofs;
use authcomp::Prover;
use authcomp::AuthT;
use authcomp::AuthType;
use authcomp::Computation;
use authcomp::Error;
use authcomp::Encode;
use authcomp::Decode;
use authcomp::DeserializeOwned;
use authcomp::ProofType;
use authcomp::Serialize;
use authcomp::Projection;

use authparser::AuthDocument;
use authparser::read_auth_jats;
use authparser::Filter;

use authdoc::QualName;

use gluesql_core::prelude::*;

struct MyFilter  {

}

impl Filter<QualName> for MyFilter {
    fn filter(&self, name: &QualName) -> bool{
        name.local == "ref-list"
    }
    
}

struct Node<C> 
    where C:AuthType<authdoc::Node>
{

    signature: Vec<u8>,
    data: AuthT<authdoc::Node,C>,
    children : Vec<Vec<u8>>,
}

#[derive(Debug,Default,Clone)]
pub struct JSONProjection {

}

impl Projection for JSONProjection{

    fn shallow<A> (value: &A) -> ProofType 
         where A: Encode + Serialize
    {
        serde_json_lenient::to_vec(value).unwrap()
    }

    fn unshallow<'a,A: DeserializeOwned+Decode<'a>> (value: &'a[u8]) -> Result<A,Error>{
        serde_json_lenient::from_slice(value).map_err(|e| Error::Shallow(String::from(format!("{:?}",e))))
    }

}

#[no_mangle]
pub extern "C" fn main() {
    /*
    let source = include_bytes!("../../../../pubmed/data/PMC9546530/pnas.202202536.nxml");
    let mut auth_document = AuthDocument::<NoProofs<(),()>>::default();

    let my_filter = MyFilter{};
    let root = read_auth_jats::<NoProofs<(),()>>(source,&mut auth_document,&my_filter);

    println!("{:?}",root);
    */
    //println!("{:?}",auth_document);

    let comp = Prover::<(),()>::run( || {
        //let storage = crate::authmemstorage::MemoryStorage::<Prover::<(),(),JSONProjection>>::default();

        //let storage2 = crate::authmemstorage::MemoryStorage::<Prover::<(),(),JSONProjection>>::default();
        //let auth_storage = Prover::<(),(),JSONProjection>::auth(storage2);

        let storage2 = crate::merkletreestorage::merkletreestorage::MerkleTreeStorage::<Prover::<(),(),JSONProjection>>::default();

        let mut glue = Glue::new(storage2);
        let sqls = vec![
            r#"DROP TABLE IF EXISTS Glue;"#,
            r#"CREATE TABLE Glue (id INTEGER, data TEXT,uuid_value UUID, float_value FLOAT, int_value INT, bool_value BOOLEAN,tree MAP, items LIST);"#,
            r#"INSERT INTO Glue VALUES (100,'DD',GENERATE_UUID(), 1.0, 1, true,'{"a": {"foo": "ok", "b": "steak"}, "b": 30}','[1, 2, 3]');"#,
            r#"INSERT INTO Glue VALUES (200,'XX',GENERATE_UUID(), 2.0, 13, true,'{"a": {"foo": "ok", "b": "steak"}, "b": 30}','["hello", "world", 30, true, [9,8]]');"#,
            r#"INSERT INTO Glue VALUES (300,'OO',GENERATE_UUID(), 3.0, 12, false,'{"a": {"foo": "ok", "b": "steak"}, "b": 30}','[{ "foo": 100, "bar": [true, 0, [10.5, false] ] }, 10, 20]');"#,
            r#"SELECT items,UNWRAP(tree, 'a.foo') || '.yeah' AS foo FROM Glue WHERE id > 100;"#,
        ];

        for sql in sqls {
            let output = glue.execute(sql).unwrap();
            println!("{:?}", output)
        }

        //println!("Root: {:?}",glue.storage.expect("No storage").root_hex());

        Ok(())
    });
    
    println!("{:?}",comp)
}
