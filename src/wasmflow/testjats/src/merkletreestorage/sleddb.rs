
use std::collections::hash_map::HashMap;
use std::path::Path;

use starling::Array;
use starling::traits::{Database, Exception};
use starling::tree::tree_node::TreeNode;
use starling::traits::Encode;
use starling::traits::Decode;

pub struct SledDB<const N: usize> {
   
    db: sled::Db,
    path: Option<String>
    
}

impl<const N: usize> SledDB<N> {
    // Creates a new `SledDB`.
    // 
    
    #[inline]
    #[must_use]
    pub const fn new(db: sled::Db, path : Option<String>) -> Self {
        Self { db, path }
    }
    

    #[inline]
    #[must_use]
    /// Decomposes the `SledDB` into its underlying `HashMap`.
    pub fn decompose<'b> (&'b self) -> (&'b sled::Db,&'b Option<String>) {
        (&self.db,&self.path)
    }
    
}

impl<const N: usize> Database<N, TreeNode<N>> for SledDB<N> {
    type EntryType = (Array<N>, Vec<u8>);

    #[inline]
    fn open(path: &Path) -> Result<Self, Exception> {
        let db = sled::Config::new().path(path).temporary(false).open().map_err( |e| Exception::new(&e.to_string()))?;
        Ok(SledDB::<N> {
            db,
            path : path.to_str().map ( |s| String::from(s)) 
        })

    }

    #[inline]
    fn get_node(&self, key: Array<N>) -> Result<Option<TreeNode<N>>, Exception> {

        self.db.get(&key).map_or(Ok(None), |persisted_value| {
            let node = TreeNode::<N>::decode(&persisted_value.unwrap()).map_err( |e| Exception::new(&e.to_string()))?;
            Ok(Some(node))
        })
    }

    #[inline]
    fn insert(&mut self, key: Array<N>, value: TreeNode<N>) -> Result<(), Exception> {
        let persisted_value = value.encode().map_err( |e| Exception::new(&e.to_string()))?;
        self.db.insert(key, persisted_value).map_err( |e| Exception::new(&e.to_string()))?;
        Ok(())
    }

    #[inline]
    fn remove(&mut self, key: &Array<N>) -> Result<(), Exception> {
        self.db.remove(key).map_err( |e| Exception::new(&e.to_string()))?;
        Ok(())
    }

    #[inline]
    fn batch_write(&mut self) -> Result<(), Exception> {
        Ok(())
    }
}