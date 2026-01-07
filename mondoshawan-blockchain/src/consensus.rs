//! GhostDAG Consensus Implementation
//! 
//! Full GhostDAG (BlockDAG) consensus algorithm based on Kaspa's protocol.
//! Orders blocks in a DAG structure using blue score calculation.

use crate::blockchain::Block;
use crate::types::Hash;
use std::collections::{HashMap, HashSet, VecDeque};

/// GhostDAG consensus engine
pub struct GhostDAG {
    blocks: HashMap<Hash, Block>,
    children: HashMap<Hash, Vec<Hash>>,
    blue_set: HashSet<Hash>,      // Blue blocks (selected for consensus)
    red_set: HashSet<Hash>,        // Red blocks (not selected)
    blue_score: HashMap<Hash, u64>, // Blue score for each block
    ordering: Vec<Hash>,           // Final block ordering
}

impl GhostDAG {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
            children: HashMap::new(),
            blue_set: HashSet::new(),
            red_set: HashSet::new(),
            blue_score: HashMap::new(),
            ordering: Vec::new(),
        }
    }

    /// Add a block to the DAG and recalculate consensus
    pub fn add_block(&mut self, block: Block) {
        let hash = block.hash;
        self.blocks.insert(hash, block.clone());
        
        // Build parent-child relationships
        for parent_hash in &block.header.parent_hashes {
            self.children.entry(*parent_hash)
                .or_insert_with(Vec::new)
                .push(hash);
        }
        
        // Recalculate blue set and ordering
        self.update_blue_set();
    }

    /// Update blue set using GhostDAG algorithm
    fn update_blue_set(&mut self) {
        if self.blocks.is_empty() {
            return;
        }
        
        // Find genesis blocks (blocks with no parents)
        let genesis_blocks: Vec<Hash> = self.blocks.iter()
            .filter(|(_, block)| block.header.parent_hashes.is_empty())
            .map(|(hash, _)| *hash)
            .collect();
        
        if genesis_blocks.is_empty() {
            return;
        }
        
        // Reset blue set and scores
        self.blue_score.clear();
        self.blue_set.clear();
        self.red_set.clear();
        
        // Initialize genesis blocks
        for genesis_hash in &genesis_blocks {
            self.blue_score.insert(*genesis_hash, 1);
            self.blue_set.insert(*genesis_hash);
        }
        
        // BFS traversal to calculate blue scores
        let mut queue = VecDeque::from(genesis_blocks);
        let mut visited = HashSet::new();
        
        for genesis in &queue {
            visited.insert(*genesis);
        }
        
        while let Some(current) = queue.pop_front() {
            // Process children of current block
            if let Some(children) = self.children.get(&current) {
                for child_hash in children {
                    if visited.contains(child_hash) {
                        continue;
                    }
                    visited.insert(*child_hash);
                    queue.push_back(*child_hash);
                    
                    // Calculate blue score for child
                    // Blue score = max(blue scores of blue parents) + 1
                    let parent_scores: Vec<u64> = self.blocks.get(child_hash)
                        .map(|block| &block.header.parent_hashes)
                        .unwrap_or(&vec![])
                        .iter()
                        .filter(|parent_hash| {
                            // Check if parent is in blue set
                            self.blue_set.iter().any(|blue_hash| blue_hash == *parent_hash)
                        })
                        .filter_map(|parent_hash| self.blue_score.get(parent_hash).copied())
                        .collect();
                    
                    if !parent_scores.is_empty() {
                        let max_parent_score = parent_scores.iter().max().copied().unwrap_or(0);
                        let child_blue_score = max_parent_score + 1;
                        self.blue_score.insert(*child_hash, child_blue_score);
                        self.blue_set.insert(*child_hash);
                    } else {
                        // No blue parents - mark as red
                        self.red_set.insert(*child_hash);
                    }
                }
            }
        }
        
        // Order blocks by blue score (descending) and timestamp (ascending)
        let mut ordered: Vec<(Hash, u64, u64)> = self.blue_set.iter()
            .filter_map(|hash| {
                self.blocks.get(hash).map(|block| {
                    let score = self.blue_score.get(hash).copied().unwrap_or(0);
                    (*hash, score, block.header.timestamp)
                })
            })
            .collect();
        
        // Sort: first by blue score (descending), then by timestamp (ascending)
        ordered.sort_by(|a, b| {
            match b.1.cmp(&a.1) {
                std::cmp::Ordering::Equal => a.2.cmp(&b.2),
                other => other,
            }
        });
        
        self.ordering = ordered.into_iter().map(|(hash, _, _)| hash).collect();
    }

    /// Get blocks in final consensus order
    pub fn get_ordered_blocks(&self) -> Vec<&Block> {
        self.ordering.iter()
            .filter_map(|hash| self.blocks.get(hash))
            .collect()
    }

    /// Get blue set (selected blocks for consensus)
    pub fn get_blue_set(&self) -> &HashSet<Hash> {
        &self.blue_set
    }

    /// Get red set (blocks not selected)
    pub fn get_red_set(&self) -> &HashSet<Hash> {
        &self.red_set
    }

    /// Get blue score for a block
    pub fn get_blue_score(&self, hash: &Hash) -> Option<u64> {
        self.blue_score.get(hash).copied()
    }

    /// Get total number of blocks in DAG
    pub fn get_block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Get number of blue blocks
    pub fn get_blue_block_count(&self) -> usize {
        self.blue_set.len()
    }

    /// Get number of red blocks
    pub fn get_red_block_count(&self) -> usize {
        self.red_set.len()
    }

    /// Calculate transactions per second from ordered blocks
    pub fn get_tps(&self, duration_seconds: u64) -> f64 {
        if self.ordering.is_empty() {
            return 0.0;
        }
        
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Get blocks from recent duration
        let recent_blocks: Vec<&Block> = self.ordering.iter()
            .filter_map(|hash| self.blocks.get(hash))
            .filter(|block| {
                let age = current_time.saturating_sub(block.header.timestamp);
                age <= duration_seconds
            })
            .collect();
        
        if recent_blocks.is_empty() {
            return 0.0;
        }
        
        let total_txs: usize = recent_blocks.iter()
            .map(|b| b.transactions.len())
            .sum();
        
        let timestamps: Vec<u64> = recent_blocks.iter()
            .map(|b| b.header.timestamp)
            .collect();
        
        let time_span = timestamps.iter().max()
            .and_then(|max| timestamps.iter().min().map(|min| max - min))
            .unwrap_or(1);
        
        if time_span == 0 {
            return 0.0;
        }
        
        total_txs as f64 / time_span as f64
    }

    /// Get DAG statistics
    pub fn get_stats(&self) -> DAGStats {
        let total_txs: usize = self.blocks.values()
            .map(|b| b.transactions.len())
            .sum();
        
        let total_size: usize = self.blocks.values()
            .map(|b| {
                // Approximate block size
                std::mem::size_of::<Block>() + 
                b.transactions.len() * std::mem::size_of::<crate::blockchain::Transaction>()
            })
            .sum();
        
        DAGStats {
            total_blocks: self.blocks.len(),
            blue_blocks: self.blue_set.len(),
            red_blocks: self.red_set.len(),
            total_transactions: total_txs,
            total_size_bytes: total_size,
            avg_block_size: if self.blocks.is_empty() { 0 } else { total_size / self.blocks.len() },
            avg_txs_per_block: if self.blocks.is_empty() { 0.0 } else { total_txs as f64 / self.blocks.len() as f64 },
        }
    }

    /// Get block by hash
    pub fn get_block(&self, hash: &Hash) -> Option<&Block> {
        self.blocks.get(hash)
    }

    /// Check if block is in blue set
    pub fn is_blue(&self, hash: &Hash) -> bool {
        self.blue_set.contains(hash)
    }

    /// Check if block is in red set
    pub fn is_red(&self, hash: &Hash) -> bool {
        self.red_set.contains(hash)
    }
}

/// DAG statistics
#[derive(Debug, Clone)]
pub struct DAGStats {
    pub total_blocks: usize,
    pub blue_blocks: usize,
    pub red_blocks: usize,
    pub total_transactions: usize,
    pub total_size_bytes: usize,
    pub avg_block_size: usize,
    pub avg_txs_per_block: f64,
}

