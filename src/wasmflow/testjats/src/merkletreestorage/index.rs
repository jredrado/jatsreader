use {
    crate::merkletreestorage::merkletreestorage::MerkleTreeStorage,
    async_trait::async_trait,
    gluesql_core::{
        ast::{IndexOperator, OrderByExpr},
        data::Value,
        result::{Error, MutResult, Result},
        store::{Index, IndexMut, RowIter},
    },
};

use authcomp::Computation;
use authcomp::AuthType;

#[async_trait(?Send)]
impl<C> Index for MerkleTreeStorage<C>
    where C:Computation
{
    async fn scan_indexed_data(
        &self,
        _table_name: &str,
        _index_name: &str,
        _asc: Option<bool>,
        _cmp_value: Option<(&IndexOperator, Value)>,
    ) -> Result<RowIter> {
        Err(Error::StorageMsg(
            "[MemoryStorage] index is not supported".to_owned(),
        ))
    }
}

#[async_trait(?Send)]
impl<C> IndexMut for MerkleTreeStorage<C>
    where C:Computation
{
    async fn create_index(
        self,
        _table_name: &str,
        _index_name: &str,
        _column: &OrderByExpr,
    ) -> MutResult<Self, ()> {
        Err((
            self,
            Error::StorageMsg("[MemoryStorage] index is not supported".to_owned()),
        ))
    }

    async fn drop_index(self, _table_name: &str, _index_name: &str) -> MutResult<Self, ()> {
        Err((
            self,
            Error::StorageMsg("[MemoryStorage] index is not supported".to_owned()),
        ))
    }
}