
pub use self::computation::{HashType,Computation,AuthType,AuthContainer,AuthTypeReq,AuthT,AuthTContainer,ProofStream,ProofType,UnAuth,UnAuthMut,AuthUpdate};
pub use self::computation::DecodeOwned;
pub use self::computation::{shallow,unshallow,hash,to_vec,from_bytes};
pub use self::computation::{CorePackProjection,Projection};

pub use minicbor::{Encode, Decode};

pub use serde::{Serialize,Deserialize};
pub use serde::de::DeserializeOwned;

pub use self::prover::{Prover,AuthTProver};
pub use self::verifier::{Verifier,AuthTVerifier};
pub use self::no_proofs::{NoProofs,AuthTNoProofs};

pub use self::error::Error;

//pub use miniserde::Serialize as ToJSON;
pub use nanoserde::SerJson as ToJSON;
pub use nanoserde::SerJsonState as JSONState;

mod computation;
mod prover;
mod verifier;
mod no_proofs;
mod hashvisitor;
mod error;
mod globals;