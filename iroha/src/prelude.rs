//! Re-exports important traits and types. Meant to be glob imported when using `Iroha`.

#[doc(inline)]
pub use crate::{
    account::{Account, Id as AccountId},
    asset::{Asset, AssetDefinition, AssetDefinitionId, AssetId},
    block::{CommittedBlock, PendingBlock, ValidBlock},
    domain::Domain,
    isi::{Add, Demint, Instruction, Mint, Register, Remove, Transfer},
    peer::{Peer, PeerId},
    query::{IrohaQuery, Query, QueryRequest, QueryResult, SignedQueryRequest},
    tx::{AcceptedTransaction, RequestedTransaction, SignedTransaction, ValidTransaction},
    wsv::WorldStateView,
    CommittedBlockReceiver, CommittedBlockSender, Identifiable, Iroha, TransactionReceiver,
    TransactionSender, ValidBlockReceiver, ValidBlockSender,
};

#[doc(inline)]
#[cfg(feature = "bridge")]
pub use crate::bridge::{Bridge, BridgeDefinition, BridgeDefinitionId, BridgeId, BridgeKind};

#[doc(inline)]
pub use iroha_crypto::{Hash, KeyPair, PrivateKey, PublicKey, Signature};

use std::sync::{ Arc };
use async_std::sync::{ RwLock, RwLockReadGuard, RwLockWriteGuard };

/// `Shared` value zero cost abstraction primitive, to shorten the code
#[derive(Debug)]
pub struct Shared<T>(pub Arc<RwLock<T>>);

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Shared<T> {
        Shared(self.0.clone())
    }
}

impl<T> Shared<T> {

    /// Just wrapping aroung 'Arc` and `RwLock` construction
    pub fn new(a: T) -> Shared<T> {
        Shared(Arc::new(RwLock::new(a)))
    }

    /// Just wrapping aroung 'Arc` and `RwLock` construction
    #[inline(always)]
    pub async fn read(&self) -> RwLockReadGuard<'_, T> {
        self.0.read().await
    }

    /// Just wrapping aroung 'Arc` and `RwLock` construction
    #[inline(always)]
    pub async fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.0.write().await
    }

}

