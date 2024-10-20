// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use sui_indexer_alt::benchmark::benchmark_indexer::BenchmarkIndexer;
use sui_indexer_alt::benchmark::BenchmarkArgs;
use sui_synthetic_ingestion::benchmark::run_benchmark;
use sui_synthetic_ingestion::SyntheticIngestionConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = BenchmarkArgs::parse();
    let first_checkpoint = args
        .indexer_config
        .first_checkpoint
        .expect("first_checkpoint is required for benchmarking");
    let last_checkpoint = args
        .indexer_config
        .last_checkpoint
        .expect("last_checkpoint is required for benchmarking");

    // Enable tracing, configured by environment variables.
    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    let ingestion_dir = tempfile::tempdir()?.into_path();
    let num_checkpoints = last_checkpoint - first_checkpoint + 1;
    let synthetic_ingestion_config = SyntheticIngestionConfig {
        ingestion_dir: ingestion_dir.clone(),
        checkpoint_size: args.checkpoint_size,
        num_checkpoints,
        starting_checkpoint: first_checkpoint,
    };
    let indexer = BenchmarkIndexer::new(args);
    run_benchmark(synthetic_ingestion_config, indexer).await;

    Ok(())
}
