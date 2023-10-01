
use std::path::Path;

use starling::Array;
use starling::traits::{Database, Exception};
use starling::tree::tree_node::TreeNode;
use starling::traits::Encode;
use starling::traits::Decode;

use rumdb_wasi;
use rumdb_wasi::storage::Storage;

pub struct BitCaskDB<const N: usize> {
   
    db: rumdb_wasi::RumDb ,
    path: Option<String>
    
}

impl<const N: usize> BitCaskDB<N> {
    // Creates a new `BitCaskDB`.
    // 
    
    #[inline]
    #[must_use]
    pub const fn new(db: rumdb_wasi::RumDb, path : Option<String>) -> Self {
        Self { db, path }
    }
    

    #[inline]
    #[must_use]
    /// Decomposes the `BitCaskDB` into its underlying `HashMap`.
    pub fn decompose<'b> (&'b self) -> (&'b rumdb_wasi::RumDb,&'b Option<String>) {
        (&self.db,&self.path)
    }
    
}

impl<const N: usize> Database<N, TreeNode<N>> for BitCaskDB<N> {
    type EntryType = (Array<N>, Vec<u8>);

    #[inline]
    fn open(path: &Path) -> Result<Self, Exception> {
        let db = rumdb_wasi::RumDb::open_default(path).map_err( |e| Exception::new(&e.to_string()))?;
        Ok(BitCaskDB::<N> {
            db,
            path : path.to_str().map ( |s| String::from(s)) 
        })

    }

    #[inline]
    fn get_node(&self, key: Array<N>) -> Result<Option<TreeNode<N>>, Exception> {

        println!("get_node: {:?}",&key);

        let persisted_value = self.db.get(key.as_ref()).map_err(|e| Exception::new(&e.to_string()))?;
        
        println!("persisted_value: {:?}",&persisted_value);

        match persisted_value {
            Some(p) => {
                let node = TreeNode::<N>::decode(&p).map_err( |e| Exception::new(&e.to_string()))?;
                Ok(Some(node))
            }
            None => Err(Exception::new("No key"))
        }

    }

    #[inline]
    fn insert(&mut self, key: Array<N>, value: TreeNode<N>) -> Result<(), Exception> {
        println!("insert: {:?}",&key);

        let persisted_value = value.encode().map_err( |e| Exception::new(&e.to_string()))?;
        self.db.put(key.to_vec(), persisted_value).map_err( |e| Exception::new(&e.to_string()))?;
        Ok(())
    }

    #[inline]
    fn remove(&mut self, key: &Array<N>) -> Result<(), Exception> {
        self.db.remove(key.as_ref()).map_err( |e| Exception::new(&e.to_string()))?;
        Ok(())
    }

    #[inline]
    fn batch_write(&mut self) -> Result<(), Exception> {
        Ok(())
    }
}