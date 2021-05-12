//! `Flashbots` namespace

use crate::{
    api::Namespace,
    helpers::{self, CallFuture},
    types::{BlockNumber, Bytes, CallBundleResponse},
    Transport,
};

/// `Flashbots` namespace: *must be used with a proper flashbots provider*
#[derive(Debug, Clone)]
pub struct Flashbots<T> {
    transport: T,
}

impl<T: Transport> Namespace<T> for Flashbots<T> {
    fn new(transport: T) -> Self
    where
        Self: Sized,
    {
        Flashbots { transport }
    }

    fn transport(&self) -> &T {
        &self.transport
    }
}

impl<T: Transport> Flashbots<T> {
    /// Call bundle
    pub fn call_bundle(
        &self,
        signed_txs: &[Bytes],
        target_block: BlockNumber,
        state_block: BlockNumber,
        timestamp: Option<u64>,
    ) -> CallFuture<CallBundleResponse, T::Out> {
        let signed_txs = helpers::serialize(&signed_txs);
        let target_block = helpers::serialize(&target_block);
        let state_block = helpers::serialize(&state_block);
        let timestamp = helpers::serialize(&timestamp);
        CallFuture::new(
            self.transport
                .execute("eth_callBundle", vec![signed_txs, target_block, state_block, timestamp]),
        )
    }

    /// Send bundle
    pub fn send_bundle(
        &self,
        signed_txs: &[Bytes],
        block: BlockNumber,
        min_timestamp: Option<u64>,
        max_timestamp: Option<u64>,
    ) -> CallFuture<(), T::Out> {
        let signed_txs = helpers::serialize(&signed_txs);
        let block = helpers::serialize(&block);
        let min_timestamp = helpers::serialize(&min_timestamp);
        let max_timestamp = helpers::serialize(&max_timestamp);
        CallFuture::new(
            self.transport
                .execute("eth_sendBundle", vec![signed_txs, block, min_timestamp, max_timestamp]),
        )
    }
}
