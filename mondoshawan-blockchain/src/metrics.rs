//! Metrics collection for production monitoring
//! 
//! Provides Prometheus metrics for monitoring blockchain operations,
//! including blocks, transactions, network, mining, and sharding metrics.

use prometheus::{
    Counter, Gauge, Histogram, HistogramOpts, Opts, Registry,
    Encoder, TextEncoder,
};
use std::sync::Arc;
use std::sync::Mutex;

/// Metrics collector for the blockchain
pub struct Metrics {
    // Block metrics
    pub blocks_mined: Counter,
    pub blocks_received: Counter,
    pub block_size: Histogram,
    
    // Transaction metrics
    pub transactions_processed: Counter,
    pub transaction_pool_size: Gauge,
    pub transactions_per_second: Gauge,
    
    // Network metrics
    pub peers_connected: Gauge,
    pub messages_sent: Counter,
    pub messages_received: Counter,
    
    // Mining metrics
    pub blocks_mined_stream_a: Counter,
    pub blocks_mined_stream_b: Counter,
    pub blocks_mined_stream_c: Counter,
    pub mining_rewards: Counter,
    
    // Sharding metrics
    pub shard_transaction_count: Vec<Gauge>,
    pub cross_shard_transactions: Counter,
    
    // Registry
    registry: Registry,
}

impl Metrics {
    /// Create a new metrics collector
    pub fn new(shard_count: usize) -> Result<Self, prometheus::Error> {
        let registry = Registry::new();
        
        // Block metrics
        let blocks_mined = Counter::with_opts(
            Opts::new("mondoshawan_blocks_mined_total", "Total number of blocks mined")
                .namespace("mondoshawan")
        )?;
        
        let blocks_received = Counter::with_opts(
            Opts::new("mondoshawan_blocks_received_total", "Total number of blocks received from network")
                .namespace("mondoshawan")
        )?;
        
        let block_size = Histogram::with_opts(
            HistogramOpts::new("mondoshawan_block_size_bytes", "Block size in bytes")
                .namespace("mondoshawan")
                .buckets(vec![1024.0, 10240.0, 102400.0, 1024000.0, 10240000.0])
        )?;
        
        // Transaction metrics
        let transactions_processed = Counter::with_opts(
            Opts::new("mondoshawan_transactions_processed_total", "Total number of transactions processed")
                .namespace("mondoshawan")
        )?;
        
        let transaction_pool_size = Gauge::with_opts(
            Opts::new("mondoshawan_transaction_pool_size", "Current transaction pool size")
                .namespace("mondoshawan")
        )?;
        
        let transactions_per_second = Gauge::with_opts(
            Opts::new("mondoshawan_transactions_per_second", "Current transactions per second")
                .namespace("mondoshawan")
        )?;
        
        // Network metrics
        let peers_connected = Gauge::with_opts(
            Opts::new("mondoshawan_peers_connected", "Number of connected peers")
                .namespace("mondoshawan")
        )?;
        
        let messages_sent = Counter::with_opts(
            Opts::new("mondoshawan_messages_sent_total", "Total number of messages sent")
                .namespace("mondoshawan")
        )?;
        
        let messages_received = Counter::with_opts(
            Opts::new("mondoshawan_messages_received_total", "Total number of messages received")
                .namespace("mondoshawan")
        )?;
        
        // Mining metrics
        let blocks_mined_stream_a = Counter::with_opts(
            Opts::new("mondoshawan_blocks_mined_stream_a_total", "Total blocks mined in Stream A")
                .namespace("mondoshawan")
        )?;
        
        let blocks_mined_stream_b = Counter::with_opts(
            Opts::new("mondoshawan_blocks_mined_stream_b_total", "Total blocks mined in Stream B")
                .namespace("mondoshawan")
        )?;
        
        let blocks_mined_stream_c = Counter::with_opts(
            Opts::new("mondoshawan_blocks_mined_stream_c_total", "Total blocks mined in Stream C")
                .namespace("mondoshawan")
        )?;
        
        let mining_rewards = Counter::with_opts(
            Opts::new("mondoshawan_mining_rewards_total", "Total mining rewards earned (in smallest unit)")
                .namespace("mondoshawan")
        )?;
        
        // Sharding metrics
        let mut shard_transaction_count = Vec::new();
        for i in 0..shard_count {
            let gauge = Gauge::with_opts(
                Opts::new("mondoshawan_shard_transaction_count", "Transaction count in shard")
                    .namespace("mondoshawan")
                    .const_label("shard_id", &i.to_string())
            )?;
            shard_transaction_count.push(gauge);
        }
        
        let cross_shard_transactions = Counter::with_opts(
            Opts::new("mondoshawan_cross_shard_transactions_total", "Total cross-shard transactions")
                .namespace("mondoshawan")
        )?;
        
        // Register all metrics
        registry.register(Box::new(blocks_mined.clone()))?;
        registry.register(Box::new(blocks_received.clone()))?;
        registry.register(Box::new(block_size.clone()))?;
        registry.register(Box::new(transactions_processed.clone()))?;
        registry.register(Box::new(transaction_pool_size.clone()))?;
        registry.register(Box::new(transactions_per_second.clone()))?;
        registry.register(Box::new(peers_connected.clone()))?;
        registry.register(Box::new(messages_sent.clone()))?;
        registry.register(Box::new(messages_received.clone()))?;
        registry.register(Box::new(blocks_mined_stream_a.clone()))?;
        registry.register(Box::new(blocks_mined_stream_b.clone()))?;
        registry.register(Box::new(blocks_mined_stream_c.clone()))?;
        registry.register(Box::new(mining_rewards.clone()))?;
        
        for gauge in &shard_transaction_count {
            registry.register(Box::new(gauge.clone()))?;
        }
        
        registry.register(Box::new(cross_shard_transactions.clone()))?;
        
        Ok(Self {
            blocks_mined,
            blocks_received,
            block_size,
            transactions_processed,
            transaction_pool_size,
            transactions_per_second,
            peers_connected,
            messages_sent,
            messages_received,
            blocks_mined_stream_a,
            blocks_mined_stream_b,
            blocks_mined_stream_c,
            mining_rewards,
            shard_transaction_count,
            cross_shard_transactions,
            registry,
        })
    }
    
    /// Get metrics in Prometheus format
    pub fn gather(&self) -> Result<String, prometheus::Error> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }
    
    /// Record a block being mined
    pub fn record_block_mined(&self, stream: &str, size: usize, reward: u128) {
        self.blocks_mined.inc();
        self.block_size.observe(size as f64);
        self.mining_rewards.inc_by(reward as f64);
        
        match stream {
            "A" => self.blocks_mined_stream_a.inc(),
            "B" => self.blocks_mined_stream_b.inc(),
            "C" => self.blocks_mined_stream_c.inc(),
            _ => {}
        }
    }
    
    /// Record a block being received
    pub fn record_block_received(&self, size: usize) {
        self.blocks_received.inc();
        self.block_size.observe(size as f64);
    }
    
    /// Record transactions processed
    pub fn record_transactions_processed(&self, count: usize) {
        self.transactions_processed.inc_by(count as f64);
    }
    
    /// Update transaction pool size
    pub fn update_transaction_pool_size(&self, size: usize) {
        self.transaction_pool_size.set(size as f64);
    }
    
    /// Update transactions per second
    pub fn update_transactions_per_second(&self, tps: f64) {
        self.transactions_per_second.set(tps);
    }
    
    /// Update peers connected
    pub fn update_peers_connected(&self, count: usize) {
        self.peers_connected.set(count as f64);
    }
    
    /// Record message sent
    pub fn record_message_sent(&self) {
        self.messages_sent.inc();
    }
    
    /// Record message received
    pub fn record_message_received(&self) {
        self.messages_received.inc();
    }
    
    /// Update shard transaction count
    pub fn update_shard_transaction_count(&self, shard_id: usize, count: usize) {
        if let Some(gauge) = self.shard_transaction_count.get(shard_id) {
            gauge.set(count as f64);
        }
    }
    
    /// Record cross-shard transaction
    pub fn record_cross_shard_transaction(&self) {
        self.cross_shard_transactions.inc();
    }
}

/// Thread-safe metrics wrapper
pub type MetricsHandle = Arc<Mutex<Metrics>>;

/// Create a metrics handle
pub fn create_metrics(shard_count: usize) -> Result<MetricsHandle, prometheus::Error> {
    Ok(Arc::new(Mutex::new(Metrics::new(shard_count)?)))
}
