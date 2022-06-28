use {
    super::chains::chain::blocks::GetBlocksQuery,
    super::chains::chain::PatchChainRequest,
    crate::error::Error,
    crate::models::bootstrapped_status::BootstrappedStatus,
    crate::models::checkpoint::Checkpoint,
    crate::models::invalid_block::InvalidBlock,
    async_trait::async_trait,
    std::result::Result,
    tezos_core::types::encoded::{BlockHash, ChainID, OperationHash},
};

/// Tezos protocol-independent RPCs.
///
/// See [RPCs - Reference](https://tezos.gitlab.io/shell/rpc.html) for more details.
#[async_trait]
pub trait ShellRPC {
    /// Forcefully set the bootstrapped flag of the node.
    ///
    /// [`PATCH /chains/<chain_id>`](https://tezos.gitlab.io/shell/rpc.html#patch-chains-chain-id)
    async fn patch_chain(&self, body: &PatchChainRequest) -> Result<(), Error>;

    /// Get the chain unique identifier.
    ///
    /// [`GET /chains/<chain_id>/chain_id`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-chain-id)
    async fn get_chain_id(&self) -> Result<ChainID, Error>;

    /// Get a list of block hashes from `<chain>`, up to the last checkpoint, sorted with
    /// decreasing fitness. Without arguments it returns the head of the chain.
    ///
    /// Optional arguments allow to return the list of predecessors of a given block or of a set of blocks.
    ///
    /// [`GET /chains/<chain_id>/blocks`](https://tezos.gitlab.io/shell/rpc.html#get_chains__chain_id__blocks)
    async fn get_blocks(&self, query: &GetBlocksQuery) -> Result<Vec<Vec<BlockHash>>, Error>;

    /// Get blocks that have been declared invalid along with the errors that led to them being declared invalid.
    ///
    /// [`GET /chains/<chain_id>/invalid_blocks`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-invalid-blocks)
    async fn get_invalid_blocks(&self) -> Result<Vec<InvalidBlock>, Error>;

    /// Get the errors that appeared during the block (in)validation.
    ///
    /// [`GET /chains/<chain_id>/invalid_blocks/<block_hash>`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-invalid-blocks-block-hash)
    async fn get_invalid_block(&self, block_hash: &BlockHash) -> Result<InvalidBlock, Error>;

    /// Remove an invalid block for the tezos storage.
    ///
    /// [`DELETE /chains/<chain_id>/invalid_blocks/<block_hash>`](https://tezos.gitlab.io/shell/rpc.html#delete-chains-chain-id-invalid-blocks-block-hash)
    async fn remove_invalid_block(&self, block_hash: &BlockHash) -> Result<(), Error>;

    /// Get the bootstrap status of a chain.
    ///
    /// [`DELETE /chains/<chain_id>/is_bootstrapped`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-is-bootstrapped)
    async fn is_bootstrapped(&self) -> Result<BootstrappedStatus, Error>;

    /// Get the current caboose for this chain.
    ///
    /// [`GET /chains/<chain_id>/levels/caboose`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-levels-caboose)
    async fn get_caboose(&self) -> Result<Checkpoint, Error>;

    /// Get the current checkpoint for this chain.
    ///
    /// [`GET /chains/<chain_id>/levels/checkpoint`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-levels-checkpoint)
    async fn get_checkpoint(&self) -> Result<Checkpoint, Error>;

    /// Get the current savepoint for this chain.
    ///
    /// [`GET /chains/<chain_id>/levels/savepoint`](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-levels-savepoint)
    async fn get_savepoint(&self) -> Result<Checkpoint, Error>;

    /// Inject an operation in node and broadcast it.
    ///
    /// The `signed_operation_contents` should be constructed using contextual RPCs
    /// from the latest block and signed by the client.
    ///
    /// The injection of the operation will apply it on the current mempool context.
    /// This context may change at each operation injection or operation reception from peers.
    ///
    /// By default, the RPC will wait for the operation to be (pre-)validated before returning.
    /// However, if `?async` is true, the function returns immediately.
    /// The optional `?chain` parameter can be used to specify whether to inject on the test chain or the main chain.
    ///
    /// Returns the ID of the operation.
    ///
    /// [`POST /injection/operation?[async]&[chain=<chain_id>]`](https://tezos.gitlab.io/shell/rpc.html#post-injection-operation)
    async fn inject_operation(
        &self,
        signed_operation_contents: &String,
        do_async: &bool,
    ) -> Result<OperationHash, Error>;
}
