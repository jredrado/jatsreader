use {
    crate::merkletreestorage::merkletreestorage::MerkleTreeStorage,
    async_trait::async_trait,
    gluesql_core::{
        result::{Error, MutResult},
        store::Transaction,
    },
};

use authcomp::Computation;
use authcomp::AuthType;

#[async_trait(?Send)]
impl<C> Transaction for MerkleTreeStorage<C>
    where   
        C:Computation
{
    async fn begin(self, autocommit: bool) -> MutResult<Self, bool> {
        if autocommit {
            return Ok((self, false));
        }

        Err((
            self,
            Error::StorageMsg("[MemoryStorage] transaction is not supported".to_owned()),
        ))
    }

    async fn rollback(self) -> MutResult<Self, ()> {
        Err((
            self,
            Error::StorageMsg("[MemoryStorage] transaction is not supported".to_owned()),
        ))
    }

    async fn commit(self) -> MutResult<Self, ()> {
        Err((
            self,
            Error::StorageMsg("[MemoryStorage] transaction is not supported".to_owned()),
        ))
    }
}