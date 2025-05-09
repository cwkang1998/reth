//! Command that initializes the node from a genesis file.

use crate::common::{AccessRights, CliNodeTypes, Environment, EnvironmentArgs};
use clap::Parser;
use reth_chainspec::{EthChainSpec, EthereumHardforks};
use reth_cli::chainspec::ChainSpecParser;
use reth_provider::BlockHashReader;
use std::sync::Arc;
use tracing::info;

/// Initializes the database with the genesis block.
#[derive(Debug, Parser)]
pub struct InitCommand<C: ChainSpecParser> {
    #[command(flatten)]
    env: EnvironmentArgs<C>,
}

impl<C: ChainSpecParser<ChainSpec: EthChainSpec + EthereumHardforks>> InitCommand<C> {
    /// Execute the `init` command
    pub async fn execute<N: CliNodeTypes<ChainSpec = C::ChainSpec>>(self) -> eyre::Result<()> {
        info!(target: "reth::cli", "reth init starting");

        let Environment { provider_factory, .. } = self.env.init::<N>(AccessRights::RW)?;

        let hash = provider_factory
            .block_hash(0)?
            .ok_or_else(|| eyre::eyre!("Genesis hash not found."))?;

        info!(target: "reth::cli", hash = ?hash, "Genesis block written");
        Ok(())
    }
}

impl<C: ChainSpecParser> InitCommand<C> {
    /// Returns the underlying chain being used to run this command
    pub fn chain_spec(&self) -> Option<&Arc<C::ChainSpec>> {
        Some(&self.env.chain)
    }
}
