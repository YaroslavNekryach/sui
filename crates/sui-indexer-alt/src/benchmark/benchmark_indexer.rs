// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::benchmark::BenchmarkArgs;
use crate::Indexer;
use sui_synthetic_ingestion::benchmark::BenchmarkableIndexer;
use sui_synthetic_ingestion::IndexerProgress;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;

pub struct BenchmarkIndexer {
    inner: Option<BenchmarkIndexerInner>,
    committed_checkpoints_rx: watch::Receiver<Option<IndexerProgress>>,
}

struct BenchmarkIndexerInner {
    args: BenchmarkArgs,
    committed_checkpoints_tx: watch::Sender<Option<IndexerProgress>>,
}

impl BenchmarkIndexer {
    pub fn new(args: BenchmarkArgs) -> Self {
        let (committed_checkpoints_tx, committed_checkpoints_rx) = watch::channel(None);
        Self {
            inner: Some(BenchmarkIndexerInner {
                args,
                committed_checkpoints_tx,
            }),
            committed_checkpoints_rx,
        }
    }
}

#[async_trait::async_trait]
impl BenchmarkableIndexer for BenchmarkIndexer {
    fn subscribe_to_committed_checkpoints(&self) -> watch::Receiver<Option<IndexerProgress>> {
        self.committed_checkpoints_rx.clone()
    }

    async fn start(&mut self) {
        let BenchmarkIndexerInner {
            args,
            committed_checkpoints_tx,
        } = self.inner.take().unwrap();
        let first_checkpoint = args.indexer_config.first_checkpoint.unwrap();
        let last_checkpoint = args.indexer_config.last_checkpoint.unwrap();
        let expected_total_transactions =
            args.checkpoint_size * (last_checkpoint - first_checkpoint + 1);
        committed_checkpoints_tx
            .send(Some(IndexerProgress {
                checkpoint: first_checkpoint - 1,
                network_total_transactions: 0,
            }))
            .unwrap();
        let cancel = CancellationToken::new();
        let mut indexer = Indexer::new(args.indexer_config, cancel.clone())
            .await
            .unwrap();
        indexer.register_pipelines().await.unwrap();
        let h_indexer = indexer.run().await.unwrap();
        tokio::task::spawn(async move {
            cancel.cancelled().await;
            let _ = h_indexer.await;
            committed_checkpoints_tx
                .send(Some(IndexerProgress {
                    checkpoint: last_checkpoint,
                    network_total_transactions: expected_total_transactions,
                }))
                .unwrap();
        });
    }

    async fn stop(self) {}
}
