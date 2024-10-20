// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::IndexerConfig;

pub mod benchmark_indexer;

#[derive(clap::Parser, Debug, Clone)]
pub struct BenchmarkArgs {
    #[command(flatten)]
    pub indexer_config: IndexerConfig,
    #[arg(
        long,
        default_value_t = 200,
        help = "Number of transactions in a checkpoint."
    )]
    pub checkpoint_size: u64,
    #[arg(
        long,
        default_value_t = false,
        help = "Whether to reset the database before running."
    )]
    pub reset_db: bool,
}
