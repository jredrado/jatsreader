#![deny(clippy::str_to_string)]


use authcomp::ProofType;
use authcomp::Computation;
use authcomp::AuthType;
use authcomp::AuthT;
use authcomp::UnAuth;

use core::hash::{Hash,Hasher};
use core::fmt;
use std::path::Path;

use crate::merkletreestorage::bitcasktree::BitCaskTree;

use {
    async_trait::async_trait,
    gluesql_core::{
        data::{Key, Row, Schema},
        result::{MutResult, Result},
        store::{RowIter, Store, StoreMut},
    },
    indexmap::IndexMap,
    serde::{Deserialize, Serialize},
    std::{collections::HashMap, collections::LinkedList,iter::empty},
};

use starling::hash_tree::HashTree;


#[derive(Debug,Clone)]
pub struct Item {
    pub schema: Schema,
    pub row_counter: i64,
    pub rows: BitCaskTree,
    pub keys: LinkedList<starling::Array<32>>
}

#[derive(Default, Clone)]
pub struct MerkleTreeStorage<C> 
    where C:Computation
{
    pub items: HashMap<String, Item>,
    pub schema_root_merkle_tree: BitCaskTree,
    phantom: core::marker::PhantomData<C>
}

#[async_trait(?Send)]
impl<C> Store for MerkleTreeStorage<C> 
    where C:Computation,
        C:AuthType<Vec<u8>>
{
    async fn fetch_all_schemas(&self) -> Result<Vec<Schema>> {
        let mut schemas = self
            .items
            .iter()
            .map(|(_, item)| item.schema.clone())
            .collect::<Vec<_>>();
        schemas.sort_by(|a, b| a.table_name.cmp(&b.table_name));

        Ok(schemas)
    }
    async fn fetch_schema(&self, table_name: &str) -> Result<Option<Schema>> {
        self.items
            .get(table_name)
            .map(|item| Ok(item.schema.clone()))
            .transpose()
    }

    async fn fetch_data(&self, table_name: &str, key: &Key) -> Result<Option<Row>> {
        
        match key {
            Key::Bytea(ref b_key) => {

                let k : [u8;32] = <Vec<u8> as AsRef<[u8]>>::as_ref(b_key).try_into().expect("Unable to get key");
                let row = self
                .items
                .get(table_name)
                .and_then(|item| item.rows.get_one(&k.into()).ok().flatten().map( 
                                        |r| Row(serde_json_lenient::from_slice::<Row>( &*authcomp::from_bytes::<AuthT<Vec<u8>,C>>(&r).unwrap().unauth().borrow() ).unwrap().0 ))
                                    );

                Ok(row)
            },
            _ => Err(gluesql_core::result::Error::StorageMsg(String::from("Unable to get key from storage")))

        }
 
        
    }

    async fn scan_data(&self, table_name: &str) -> Result<RowIter> {
          
        if let Some(item) = self.items.get(table_name) {
            let rows = Box::new(item.keys.iter().map( |k| {
                let row = item.rows.get_one(k).ok().flatten().map( 
                    |r|  Row(serde_json_lenient::from_slice::<Row>( &*authcomp::from_bytes::<AuthT<Vec<u8>,C>>(&r).unwrap().unauth().borrow() ).unwrap().0 ));

                Ok((Key::Bytea(k.to_vec()),row.unwrap()))
            } ) );

            return Ok(rows)
        }

        Err( gluesql_core::result::Error::StorageMsg("scan_data".to_string()) )
        
    }


}

impl<C> MerkleTreeStorage<C> 
    where 
        C:Computation,
        C:AuthType<alloc::vec::Vec<u8>>
    
{

    pub fn new() -> Self {
        todo!()
    }

    pub fn root(&mut self) -> Option<[u8; 32]> {

        /*
        self.calculate_root();
        self.schema_root_merkle_tree.root()
        */
        todo!()
    }

    pub fn root_hex(&mut self) -> Option<alloc::string::String> {

        /*self.calculate_root();
        self.schema_root_merkle_tree.root_hex()*/
        todo!()
    }

    fn calculate_root(&mut self){
                
        /*
        let mut items_roots = self
            .items
            .iter()
            .map(|(_, item)| item.merkle_tree.head())
            .collect::<Vec<_>>();
        
            self.schema_root_merkle_tree = MerkleTree::<sha2::Sha256>::new();

        for  (_,item) in self.items.iter() {
            let hashed_key = <sha2::Sha256 as digest::Digest>::hash_elem(&item);
            self.schema_root_merkle_tree.insert(hashed_key);
        }
        */
        //self.schema_root_merkle_tree = MerkleTree::<Sha256>::from_leaves(&items_roots);
        
    }

    fn _fetch_data (&self, table_name: &str, key: &Key) -> Result<Option<Row>> {
        match key {
            Key::Bytea(ref b_key) => {

                let k : [u8;32] = <Vec<u8> as AsRef<[u8]>>::as_ref(b_key).try_into().expect("Unable to get key");
                let row = self
                .items
                .get(table_name)
                .and_then(|item| item.rows.get_one(&k.into()).ok().flatten().map( 
                                        |r| Row(serde_json_lenient::from_slice::<Row>( &*authcomp::from_bytes::<AuthT<Vec<u8>,C>>(&r).unwrap().unauth().borrow() ).unwrap().0 ))
                                    );

                Ok(row)
            },
            _ => Err(gluesql_core::result::Error::StorageMsg(String::from("Unable to get key from storage")))

        }
    }

    pub fn insert_schema(&mut self, schema: &Schema) {
        let table_name = schema.table_name.clone();
        let mut path = String::new();
        fmt::write(&mut path, format_args!("/tmp/{}", table_name));

        let item = Item {
            row_counter: 0,
            schema: schema.clone(),
            rows: BitCaskTree::new(Path::new(&path),10).expect("Can't create storage"),
            keys: LinkedList::new()
        };

        self.items.insert(table_name, item);
    }

    pub fn delete_schema(&mut self, table_name: &str) {
        self.items.remove(table_name);
    }

    pub fn append_data(&mut self, table_name: &str, rows: Vec<Row>) {
        println!("append data: {:?}",table_name);
        if let Some(item) = self.items.get_mut(table_name) {
            println!("append data: {:?}",&item);
            for row in rows {
                println!("append data: {:?}",&row);
                item.row_counter += 1;
                //1-Serialize (we need recursive support json-core) to homogenize heterogeneous value types.
                //2-Auth (Projection + digest)
                //3-Persist
                let authenticated_row = authcomp::to_vec(&C::auth( serde_json_lenient::to_vec::<Row>(&row).unwrap()));
                //let hashed_key = <sha2::Sha256 as digest::Digest>::hash_elem(&authenticated_row);
                let hashed_key = authcomp::hash(&authenticated_row);
                //item.merkle_tree.insert(hashed_key);
                //Other option: item.rows.insert(Key::Bytea(hashed_key.to_vec()), authenticated_row);

                println!("Inserting key {:?}",&hashed_key.data.to_vec());
                item.rows.insert_one(&hashed_key.data.into(), &authenticated_row);
                item.keys.push_front(hashed_key.data.into());
            }
            
        }
    }

    pub fn insert_data(&mut self, table_name: &str, rows: Vec<(Key, Row)>) {
        println!("Insert data: {:?}",table_name);

        if let Some(item) = self.items.get_mut(table_name) {
            for (key, row) in rows {
                let authenticated_row = authcomp::to_vec(&C::auth( serde_json_lenient::to_vec::<Row>(&row).unwrap()));

                let hashed_key = authcomp::hash(&authenticated_row);             
                //item.merkle_tree.insert(hashed_key);
                item.rows.insert_one(&hashed_key.data.into(), &authenticated_row);
                item.keys.push_front(hashed_key.data.into());
            }
        }
    }

    pub fn delete_data(&mut self, table_name: &str, keys: Vec<Key>) {
        //TO DO: Evaluate logical removal straegy
        core::panic!("It is not allowed to remove data!");

        /* 
        if let Some(item) = self.items.get_mut(table_name) {
            for key in keys {
                item.rows.remove(&key);
            }
        }
        */
    }
}

#[async_trait(?Send)]
impl<C> StoreMut for MerkleTreeStorage<C> 
    where C:Computation,
          C:AuthType<alloc::vec::Vec<u8>>
{
    async fn insert_schema(self, schema: &Schema) -> MutResult<Self, ()> {
        let mut storage = self;

        MerkleTreeStorage::insert_schema(&mut storage, schema);

        Ok((storage, ()))
    }

    async fn delete_schema(self, table_name: &str) -> MutResult<Self, ()> {
        let mut storage = self;

        MerkleTreeStorage::delete_schema(&mut storage, table_name);

        Ok((storage, ()))
    }

    async fn append_data(self, table_name: &str, rows: Vec<Row>) -> MutResult<Self, ()> {
        let mut storage = self;

        MerkleTreeStorage::append_data(&mut storage, table_name, rows);

        Ok((storage, ()))
    }

    async fn insert_data(self, table_name: &str, rows: Vec<(Key, Row)>) -> MutResult<Self, ()> {
        let mut storage = self;

        MerkleTreeStorage::insert_data(&mut storage, table_name, rows);

        Ok((storage, ()))
    }

    async fn delete_data(self, table_name: &str, keys: Vec<Key>) -> MutResult<Self, ()> {
        let mut storage = self;

        MerkleTreeStorage::delete_data(&mut storage, table_name, keys);

        Ok((storage, ()))
    }
}
