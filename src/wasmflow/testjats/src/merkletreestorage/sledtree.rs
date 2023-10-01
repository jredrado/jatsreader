
use std::marker::PhantomData;
use std::path::Path;
use std::path::PathBuf;
use std::collections::HashMap;
use std::fmt;

use starling::Array;
use starling::traits::Exception;

use starling::merkle_bit::{BinaryMerkleTreeResult, MerkleBIT, MerkleTree};
use starling::traits::{Decode, Encode};
use starling::tree::tree_branch::TreeBranch;
use starling::tree::tree_data::TreeData;
use starling::tree::tree_leaf::TreeLeaf;
use starling::tree::tree_node::TreeNode;
use starling::tree_hasher::TreeHasher;

use starling::traits::Database;
use crate::merkletreestorage::sleddb::SledDB;

use serde::Serialize;
use serde::Deserialize;
use serde::Serializer;
use serde::Deserializer;
use serde::ser::SerializeStruct;

use serde::de;
use serde::de::Visitor;
use serde::de::MapAccess;
use serde::de::SeqAccess;

/// Internal type alias for the underlying tree.
type Tree<const N: usize, Value = Vec<u8>> = MerkleBIT<SledTree<N, Value>, N>;

/// A `MerkleBIT` implemented with a `HashMap`.  Can be used for quickly storing items in memory, though
/// larger sets of items should be stored on disk or over the network in a real database.

pub struct SledTree<const N: usize = 32, Value: Encode + Decode = Vec<u8>> {
    /// The underlying tree.  The type requirements have already been implemented for easy use.
    tree: Option<Tree<N>>,
    root: Option<Array<N>>,
    path: PathBuf,
    depth: usize,
    /// Marker for `Value`
    _value: PhantomData<Value>,
}

impl<const N: usize, Value: Encode + Decode> MerkleTree<N> for SledTree<N, Value> {
    type Database = SledDB<N>;
    type Branch = TreeBranch<N>;
    type Leaf = TreeLeaf<N>;
    type Data = TreeData;
    type Node = TreeNode<N>;
    type Hasher = TreeHasher;
    type Value = Value;
}

impl<const N: usize> SledTree<N> {
    /// Creates a new `SledTree`.  `depth` indicates the maximum depth of the tree.
    /// # Errors
    /// None.
    #[inline]
    pub fn new(path: &Path,depth: usize) -> BinaryMerkleTreeResult<Self> {
        let db = SledDB::<N>::open(path)?;
        let tree = MerkleBIT::from_db(db, depth)?;
        Ok(Self {
            tree: Some(tree),
            _value: PhantomData::default(),
            root: None,
            depth,
            path : path.to_owned()
        })
    }

    #[inline]
    pub fn new_with_root(path: &Path,root: Array<N>,depth: usize) -> BinaryMerkleTreeResult<Self> {
        let db = SledDB::<N>::open(path)?;
        let tree = MerkleBIT::from_db(db, depth)?;
        Ok(Self {
            tree: Some(tree),
            _value: PhantomData::default(),
            root: Some(root),
            depth,
            path : path.to_owned()
        })
    }

    /// Creates a new `SledTree`.  This method exists for conforming with the general API for the `MerkleBIT`
    /// and does not need to be used (except for compatibility).  Prefer `new` when possible.
    /// # Errors
    /// None.
    #[inline]
    pub fn open(path: &Path, depth: usize) -> BinaryMerkleTreeResult<Self> {
        let tree = MerkleBIT::new(path, depth)?;
        Ok(Self {
            tree: Some(tree),
            _value: PhantomData::default(),
            root: None,
            depth,
            path : path.to_owned()
        })
    }

    /// Gets the values associated with `keys` from the tree.
    /// # Errors
    /// `Exception` generated if the `get` encounters an invalid state during tree traversal.
    #[inline]
    pub fn get(
        &self,
        keys: &mut [Array<N>],
    ) -> BinaryMerkleTreeResult<HashMap<Array<N>, Option<<Self as MerkleTree<N>>::Value>>> {
        self.tree.as_ref().expect("A tree must be initialized").get(self.root.as_ref().expect("Unable to get the root"), keys)
    }

    /// Inserts elements into the tree.  Using `previous_root` specifies that the insert depends on
    /// the state from the previous root, and will update references accordingly.
    /// # Errors
    /// `Exception` generated if the `insert` encounters an invalid state during tree traversal.
    #[inline]
    pub fn insert(
        &mut self,
        keys: &mut [Array<N>],
        values: &[<Self as MerkleTree<N>>::Value],
    ) -> BinaryMerkleTreeResult<Array<N>> {
        let result = self.tree.as_mut().expect("A tree must be initialized").insert(self.root.as_ref(), keys, values);

        self.root = result.as_ref().ok().copied();
        result
    }

    /// Removes a root from the tree.  This will remove all elements with less than two references
    /// under the given root.
    /// # Errors
    /// `Exception` generated if the `remove` encounters an invalid state during tree traversal.
    #[inline]
    pub fn remove(&mut self) -> BinaryMerkleTreeResult<()> {
        self.tree.as_mut().expect("A tree must be initialized").remove(self.root.as_ref().expect("Unable to get the root"))
    }

    /// Generates an inclusion proof for the given key at the specified root.
    /// # Errors
    /// `Exception` generated if an invalid state is encountered during tree traversal
    #[inline]
    pub fn generate_inclusion_proof(
        &self,
        key: Array<N>,
    ) -> BinaryMerkleTreeResult<Vec<(Array<N>, bool)>> {
        self.tree.as_ref().expect("A tree must be initialized").generate_inclusion_proof(self.root.as_ref().expect("Unable to get the root"), key)
    }

    /// Verifies an inclusion proof with the given root, key, and value.
    /// # Errors
    /// `Exception` generated if the given proof is invalid.
    #[inline]
    pub fn verify_inclusion_proof(
        &self,
        key: Array<N>,
        value: &<Self as MerkleTree<N>>::Value,
        proof: &[(Array<N>, bool)],
    ) -> BinaryMerkleTreeResult<()> {
        Tree::verify_inclusion_proof(self.root.as_ref().expect("Unable to get the root"), key, value, proof)
    }

    /// Gets a single item out of the tree.
    /// # Errors
    /// `Exception` generated if the `get_one` encounters an invalid state during tree traversal.
    #[inline]
    pub fn get_one(
        &self,
        key: &Array<N>,
    ) -> BinaryMerkleTreeResult<Option<<Self as MerkleTree<N>>::Value>> {
        self.tree.as_ref().expect("A tree must be initialized").get_one(self.root.as_ref().expect("Unable to get the root"), key)
    }


    #[inline]
    pub fn insert_one(
        &mut self,
        key: &Array<N>,
        value: &<Self as MerkleTree<N>>::Value,
    ) -> BinaryMerkleTreeResult<Array<N>> {
        let result = self.tree.as_mut().expect("A tree must be initialized").insert_one(self.root.as_ref(), key, value);

        self.root = result.as_ref().ok().copied();

        result
    }

    #[inline]
    #[must_use]
    /// Decomposes the tree into the its DB and size
    pub fn decompose<'b>(self) -> (SledDB<N>, usize) {
        self.tree.expect("A tree must be initialized").decompose()
    }

}


//The shallow projection of Tree is the root of the merkle tree 

impl<const N: usize> Serialize for SledTree<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {

        let mut state = serializer.serialize_struct("SledTree", 3)?;
        state.serialize_field("root", &self.root)?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("depth", &self.depth)?;        
        state.end()

    }
}

impl<'de,const N: usize> Deserialize<'de> for SledTree<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Root, Path,Depth }

        // This part could also be generated independently by:
        //
        //    #[derive(Deserialize)]
        //    #[serde(field_identifier, rename_all = "lowercase")]
        //    enum Field { Secs, Nanos }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`root` or `path` or `depth`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "root" => Ok(Field::Root),
                            "path" => Ok(Field::Path),
                            "depth" => Ok(Field::Depth),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct SledTreeVisitor<const N: usize>;

        impl<'de,const N: usize> Visitor<'de> for SledTreeVisitor<N> {
            type Value = SledTree::<N>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct SledTree")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<SledTree::<N>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let root = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let path = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let depth = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;

                let tree = SledTree::<N>::new_with_root(path,root,depth);
                tree.map_err( |e| de::Error::custom(e.to_string()) )
            }

            fn visit_map<V>(self, mut map: V) -> Result<SledTree::<N>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut root = None;
                let mut path = None;
                let mut depth = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Root => {
                            if root.is_some() {
                                return Err(de::Error::duplicate_field("root"));
                            }
                            root = Some(map.next_value()?);
                        }
                        Field::Path => {
                            if path.is_some() {
                                return Err(de::Error::duplicate_field("path"));
                            }
                            path = Some(map.next_value()?);
                        }
                        Field::Depth => {
                            if depth.is_some() {
                                return Err(de::Error::duplicate_field("depth"));
                            }
                            depth = Some(map.next_value()?);
                        }                        
                    }
                }

                let root = root.ok_or_else(|| de::Error::missing_field("root"))?;
                let path = path.ok_or_else(|| de::Error::missing_field("path"))?;
                let depth = depth.ok_or_else(|| de::Error::missing_field("depth"))?;

                let tree = SledTree::<N>::new_with_root(path,root,depth);
                tree.map_err( |e| de::Error::custom(e.to_string()) )
            }
        }

        const FIELDS: &'static [&'static str] = &["root", "path","depth"];
        deserializer.deserialize_struct("SledTree", FIELDS,  SledTreeVisitor::<N>)
    }
}


impl<const N: usize> Default for SledTree<N> {
    fn default() -> Self {
        SledTree::<N> {
            tree: None,
            _value: PhantomData::default(),
            root: None,
            depth: 10,
            path : PathBuf::default()
        }
    }
}


impl<const N: usize> core::fmt::Debug for SledTree<N>  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SledTree")
         .field("path", &self.path)
         .finish()
    }
}

impl<const N: usize> Clone for SledTree<N>   {
    fn clone(&self) -> Self {
        let db = SledDB::<N>::open(&self.path).expect("Open database");
        let tree = MerkleBIT::from_db(db, self.depth).expect("Tree");

        SledTree::<N> {
            tree: Some(tree),
            _value: PhantomData::default(),
            root: self.root,
            depth: self.depth,
            path : self.path.to_owned()
        }
    }
}

impl<const N: usize> minicbor::Encode for SledTree<N>  {
    fn encode<W: minicbor::encode::Write>(&self, e: &mut minicbor::Encoder<W>) -> Result<(), minicbor::encode::Error<W::Error>> {
        todo!()
    }
}


impl<'b, const N: usize> minicbor::Decode<'b> for SledTree<N> {
    fn decode(d: &mut minicbor::decode::Decoder<'b>) -> Result<Self, minicbor::decode::Error> {
        todo!()
    }
}

/* 
use sled::IVec;
pub struct SledTreeIterator<'a,const N: usize = 32>{
    tree: Option<&'a Tree<N>>
}

impl<'a,const N: usize> SledTreeIterator<'a,N> {

    pub fn new( tree: Option<&'a Tree<N>>) -> Self {
        Self {
            tree
        }
    }

}

impl<'a,const N: usize> IntoIterator for SledTreeIterator<'a,N> {
    type Item = Result<(IVec, IVec), sled::Error>;
    type IntoIter = sled::Iter;

    fn into_iter(self) -> Self::IntoIter {
        self.tree.and_then( | t | Some(t.decompose().0.decompose().0.iter()) ).expect("Unable to get iterator")
    }
}


impl<const N: usize> Iterator for SledTree<N> {
    type Item = (Array<N>, Vec<u8>);


    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

*/