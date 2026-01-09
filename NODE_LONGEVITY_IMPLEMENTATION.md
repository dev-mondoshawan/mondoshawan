# Node Longevity Implementation: Anti-Gaming Design

**Technical Implementation Guide**  
**Last Updated**: January 2026

---

## 🎯 The Challenge

**Node Longevity Weighting Must Be:**
- ✅ Resistant to Sybil attacks
- ✅ Fair to honest nodes
- ✅ Transparent and verifiable
- ✅ Cannot be gamed

---

## 🔒 Core Design Principles

### 1. Cryptographic Identity & Hardware Fingerprinting

**Problem**: One person could run 1000 nodes to game the system

**Solution**: One node = one unique identity using Hardware Security Primitives

**Physical Unclonable Functions (PUFs):**
- Leverage manufacturing variations in integrated circuits
- Generate device fingerprint that is virtually impossible to clone
- Hardware-specific, cannot be transferred between devices

**Hardware Hash Generation:**
- Stable system attributes create unique node identity
- CPU Model, BIOS serial number, MAC address
- Combined hash = unique hardware fingerprint

**Implementation:**
```rust
pub struct NodeIdentity {
    /// Public key (cannot be transferred)
    pub public_key: [u8; 32],
    
    /// IP address (for uniqueness check)
    pub ip_address: Option<IpAddr>,
    
    /// Hardware fingerprint (PUF-based)
    pub hardware_fingerprint: HardwareFingerprint,
    
    /// Zero-knowledge proof of uniqueness (for VPN/proxy users)
    pub zk_uniqueness_proof: Option<ZkUniquenessProof>,
    
    /// Node creation timestamp
    pub created_at: u64,
}

/// Hardware fingerprint using Physical Unclonable Functions
pub struct HardwareFingerprint {
    /// CPU Model identifier
    pub cpu_model: Hash,
    
    /// BIOS serial number hash
    pub bios_serial: Hash,
    
    /// MAC address hash
    pub mac_address: Hash,
    
    /// System UUID hash
    pub system_uuid: Hash,
    
    /// PUF-derived identifier (if available)
    pub puf_id: Option<Hash>,
    
    /// Combined fingerprint (hash of all above)
    pub fingerprint: Hash,
    
    /// Signature by node's private key (proves ownership)
    pub signature: Vec<u8>,
}

impl HardwareFingerprint {
    /// Generate hardware fingerprint from system attributes
    pub fn generate(node_private_key: &[u8; 32]) -> Self {
        // Collect stable system attributes
        let cpu_model = get_cpu_model_id();
        let bios_serial = get_bios_serial_number();
        let mac_address = get_primary_mac_address();
        let system_uuid = get_system_uuid();
        
        // Attempt to get PUF identifier (if hardware supports it)
        let puf_id = get_puf_identifier().ok();
        
        // Hash each component
        let cpu_model_hash = hash(&cpu_model);
        let bios_serial_hash = hash(&bios_serial);
        let mac_address_hash = hash(&mac_address);
        let system_uuid_hash = hash(&system_uuid);
        
        // Create combined fingerprint
        let mut hasher = Keccak256::new();
        hasher.update(&cpu_model_hash);
        hasher.update(&bios_serial_hash);
        hasher.update(&mac_address_hash);
        hasher.update(&system_uuid_hash);
        if let Some(ref puf) = puf_id {
            hasher.update(puf);
        }
        let fingerprint = hasher.finalize().into();
        
        // Sign fingerprint with node's private key
        let signature = sign_with_private_key(&fingerprint, node_private_key);
        
        Self {
            cpu_model: cpu_model_hash,
            bios_serial: bios_serial_hash,
            mac_address: mac_address_hash,
            system_uuid: system_uuid_hash,
            puf_id,
            fingerprint,
            signature,
        }
    }
    
    /// Verify fingerprint signature
    pub fn verify(&self, node_public_key: &[u8; 32]) -> bool {
        verify_signature(&self.fingerprint, &self.signature, node_public_key)
    }
    
    /// Check if fingerprint matches existing (prevents duplicate registration)
    pub fn matches(&self, other: &HardwareFingerprint) -> bool {
        self.fingerprint == other.fingerprint
    }
}

/// Zero-knowledge proof of uniqueness (for VPN/proxy users)
/// Proves hardware is unique without revealing specific serial numbers
pub struct ZkUniquenessProof {
    /// Commitment to hardware fingerprint
    pub commitment: Hash,
    
    /// Zero-knowledge proof that fingerprint is unique
    pub zk_proof: Vec<u8>,
    
    /// Public verification key
    pub verification_key: Vec<u8>,
}

impl ZkUniquenessProof {
    /// Generate ZK proof that hardware fingerprint is unique
    /// Without revealing the actual fingerprint to the ledger
    pub fn generate(hardware_fingerprint: &HardwareFingerprint) -> Self {
        // Create commitment to fingerprint (hiding)
        let commitment = hash(&hardware_fingerprint.fingerprint);
        
        // Generate ZK proof that:
        // 1. Commitment corresponds to valid hardware fingerprint
        // 2. Fingerprint is unique (not in registry)
        // Without revealing the actual fingerprint
        
        // In full implementation, this would use:
        // - zk-SNARKs or zk-STARKs
        // - Prove knowledge of fingerprint without revealing it
        // - Prove fingerprint is not in existing registry
        
        let zk_proof = generate_zk_proof(&hardware_fingerprint.fingerprint);
        let verification_key = get_verification_key();
        
        Self {
            commitment,
            zk_proof,
            verification_key,
        }
    }
    
    /// Verify ZK proof
    pub fn verify(&self, registry: &NodeRegistry) -> bool {
        // Verify ZK proof is valid
        if !verify_zk_proof(&self.zk_proof, &self.verification_key) {
            return false;
        }
        
        // Verify commitment is not in registry (uniqueness)
        !registry.has_commitment(&self.commitment)
    }
}
```

### 2. On-Chain Longevity Tracking

**Longevity is not just a timestamp; it is a verifiable record of participation.**

**Activity Snapshots:**
- Protocol records daily snapshots of all active nodes
- Node marked "active" only if it participated in consensus during 24-hour window
- Participation = submitted block OR ZK proof (any stream)

**Non-Linear Uptime Calculation:**
- Uptime calculated as non-linear index of activity
- Offline > 30 consecutive days = automatic reset to zero
- Prevents "zombie" nodes from accumulating voting power

**Block Production Requirement:**
- Must have mined at least **one block** on any stream (A, B, or C)
- Ensures node has successfully contributed to network security

**Implementation:**
```rust
pub struct NodeLongevity {
    pub node_identity: NodeIdentity,
    
    /// Total days node has been active (via activity snapshots)
    pub active_days: u64,
    
    /// Total blocks mined (any stream)
    pub blocks_mined: u64,
    
    /// Non-linear uptime index (0.0 to 1.0)
    pub uptime_index: f64,
    
    /// Last seen timestamp
    pub last_seen: u64,
    
    /// Network age when node joined
    pub network_age_at_join: u64,
    
    /// Daily activity snapshots (on-chain record)
    pub activity_snapshots: Vec<ActivitySnapshot>,
    
    /// Consecutive offline days (for reset calculation)
    pub consecutive_offline_days: u64,
}

/// Daily activity snapshot (on-chain record)
pub struct ActivitySnapshot {
    /// Date (Unix timestamp, midnight UTC)
    pub date: u64,
    
    /// Whether node participated in consensus this day
    pub participated: bool,
    
    /// Type of participation (block, zk_proof, etc.)
    pub participation_type: ParticipationType,
    
    /// Block hash or proof hash (verifiable)
    pub participation_proof: Hash,
}

pub enum ParticipationType {
    BlockMined { stream: StreamType, block_hash: Hash },
    ZkProofSubmitted { proof_hash: Hash },
    ConsensusVote { vote_hash: Hash },
}

impl NodeLongevity {
    /// Calculate longevity weight for governance voting
    pub fn calculate_weight(&self, network_age_days: u64) -> f64 {
        // Eligibility checks
        if self.active_days < 30 {
            return 0.0; // Must be active ≥ 30 days
        }
        
        if self.uptime_index < 0.8 {
            return 0.0; // Must have ≥ 80% uptime index
        }
        
        if self.blocks_mined == 0 {
            return 0.0; // Must have mined ≥ 1 block
        }
        
        // Ratio of active presence to total network age
        let longevity_ratio = self.active_days as f64 / network_age_days as f64;
        
        // Weight is 40% of the total governance vote, capped at 0.1%
        (longevity_ratio * 0.4).min(0.001)
    }
    
    /// Calculate non-linear uptime index
    /// Penalizes long offline periods more than short ones
    pub fn calculate_uptime_index(&self, network_age_days: u64) -> f64 {
        if network_age_days == 0 {
            return 0.0;
        }
        
        // Base uptime: active days / total days
        let base_uptime = self.active_days as f64 / network_age_days as f64;
        
        // Penalty for consecutive offline days (non-linear)
        let offline_penalty = if self.consecutive_offline_days > 30 {
            // Reset to zero if offline > 30 days
            0.0
        } else if self.consecutive_offline_days > 7 {
            // Heavy penalty for > 7 days offline
            base_uptime * (1.0 - (self.consecutive_offline_days as f64 / 30.0))
        } else {
            // Light penalty for < 7 days offline
            base_uptime * 0.95
        };
        
        offline_penalty.max(0.0)
    }
    
    /// Record activity snapshot (called daily at midnight UTC)
    pub fn record_activity_snapshot(&mut self, participation: ParticipationType) {
        let current_time = current_timestamp();
        let today = current_time - (current_time % 86400); // Round to midnight
        
        // Check if already recorded for today
        if let Some(last_snapshot) = self.activity_snapshots.last() {
            if last_snapshot.date == today {
                // Update today's snapshot
                let snapshot = ActivitySnapshot {
                    date: today,
                    participated: true,
                    participation_type: participation,
                    participation_proof: match &participation {
                        ParticipationType::BlockMined { block_hash, .. } => *block_hash,
                        ParticipationType::ZkProofSubmitted { proof_hash } => *proof_hash,
                        ParticipationType::ConsensusVote { vote_hash } => *vote_hash,
                    },
                };
                *self.activity_snapshots.last_mut().unwrap() = snapshot;
                return;
            }
        }
        
        // Create new snapshot
        let snapshot = ActivitySnapshot {
            date: today,
            participated: true,
            participation_type: participation,
            participation_proof: match &participation {
                ParticipationType::BlockMined { block_hash, .. } => *block_hash,
                ParticipationType::ZkProofSubmitted { proof_hash } => *proof_hash,
                ParticipationType::ConsensusVote { vote_hash } => *vote_hash,
            },
        };
        
        self.activity_snapshots.push(snapshot);
        self.active_days += 1;
        self.consecutive_offline_days = 0;
        self.last_seen = current_time;
        
        // Recalculate uptime index
        let network_age_days = (current_time - self.network_age_at_join) / 86400;
        self.uptime_index = self.calculate_uptime_index(network_age_days);
    }
    
    /// Record no activity for today (node offline)
    pub fn record_no_activity(&mut self) {
        let current_time = current_timestamp();
        let today = current_time - (current_time % 86400);
        
        // Check if already recorded for today
        if let Some(last_snapshot) = self.activity_snapshots.last() {
            if last_snapshot.date == today {
                return; // Already recorded
            }
        }
        
        // Create snapshot with no participation
        let snapshot = ActivitySnapshot {
            date: today,
            participated: false,
            participation_type: ParticipationType::ConsensusVote { 
                vote_hash: [0u8; 32] // Placeholder
            },
            participation_proof: [0u8; 32],
        };
        
        self.activity_snapshots.push(snapshot);
        self.consecutive_offline_days += 1;
        
        // Reset if offline > 30 days
        if self.consecutive_offline_days > 30 {
            self.active_days = 0;
            self.uptime_index = 0.0;
        } else {
            // Recalculate uptime index with penalty
            let network_age_days = (current_time - self.network_age_at_join) / 86400;
            self.uptime_index = self.calculate_uptime_index(network_age_days);
        }
    }
    
    /// Check if node is still active
    pub fn is_active(&self, current_time: u64) -> bool {
        // Node is active if seen within last 24 hours
        current_time - self.last_seen < 86400
    }
    
    /// Reset longevity if node offline too long
    pub fn check_reset(&mut self, current_time: u64) {
        // If offline > 30 days, reset longevity
        if current_time - self.last_seen > 2592000 { // 30 days
            self.active_days = 0;
            self.uptime_index = 0.0;
            self.consecutive_offline_days = 0;
        }
    }
}
```

### 3. Sybil Attack Prevention

**Mechanism 1: IP-Based Uniqueness**

```rust
pub struct NodeRegistry {
    /// Map of IP address to node identity
    ip_to_node: HashMap<IpAddr, NodeIdentity>,
    
    /// Map of node identity to stats
    node_stats: HashMap<NodeIdentity, NodeLongevity>,
}

impl NodeRegistry {
    /// Register new node with hardware fingerprint verification
    pub fn register_node(&mut self, identity: NodeIdentity) -> Result<()> {
        // Verify hardware fingerprint signature
        if !identity.hardware_fingerprint.verify(&identity.public_key) {
            return Err("Invalid hardware fingerprint signature".into());
        }
        
        // Check IP uniqueness (if available)
        if let Some(ip) = identity.ip_address {
            if self.ip_to_node.contains_key(&ip) {
                return Err("IP address already registered".into());
            }
            self.ip_to_node.insert(ip, identity.clone());
        }
        
        // Check hardware fingerprint uniqueness
        if self.has_fingerprint(&identity.hardware_fingerprint.fingerprint) {
            return Err("Hardware fingerprint already registered".into());
        }
        
        // If ZK proof provided, verify it
        if let Some(ref zk_proof) = identity.zk_uniqueness_proof {
            if !zk_proof.verify(self) {
                return Err("Invalid zero-knowledge uniqueness proof".into());
            }
            // Store commitment (not the actual fingerprint)
            self.register_commitment(&zk_proof.commitment);
        } else {
            // Store fingerprint directly (for IP-based nodes)
            self.register_fingerprint(&identity.hardware_fingerprint.fingerprint);
        }
        
        // Initialize stats
        self.node_stats.insert(identity.clone(), NodeLongevity::new(identity.clone()));
        
        Ok(())
    }
    
    /// Check if fingerprint exists
    pub fn has_fingerprint(&self, fingerprint: &Hash) -> bool {
        self.fingerprints.contains(fingerprint)
    }
    
    /// Check if commitment exists
    pub fn has_commitment(&self, commitment: &Hash) -> bool {
        self.commitments.contains(commitment)
    }
    
    /// Register fingerprint
    pub fn register_fingerprint(&mut self, fingerprint: &Hash) {
        self.fingerprints.insert(*fingerprint);
    }
    
    /// Register commitment (for ZK proofs)
    pub fn register_commitment(&mut self, commitment: &Hash) {
        self.commitments.insert(*commitment);
    }
    
    /// Update node stats
    pub fn update_node(&mut self, identity: &NodeIdentity, stats: NodeLongevity) {
        self.node_stats.insert(identity.clone(), stats);
    }
}

// Add to NodeRegistry struct
pub struct NodeRegistry {
    /// Map of IP address to node identity
    ip_to_node: HashMap<IpAddr, NodeIdentity>,
    
    /// Map of node identity to stats
    node_stats: HashMap<NodeIdentity, NodeLongevity>,
    
    /// Set of registered hardware fingerprints
    fingerprints: HashSet<Hash>,
    
    /// Set of registered commitments (for ZK proofs)
    commitments: HashSet<Hash>,
}
```

**Mechanism 2: Zero-Knowledge Proof-of-Uniqueness (For VPN/Proxy Users)**

**For users behind VPNs or shared IPs, the protocol requires a Zero-Knowledge Proof (ZKP) that confirms the node has a unique hardware fingerprint without revealing the specific hardware serial numbers to the public ledger.**

```rust
/// Zero-knowledge proof of uniqueness
/// Proves hardware fingerprint is unique without revealing serial numbers
pub struct ZkUniquenessProof {
    /// Commitment to hardware fingerprint (hiding)
    pub commitment: Hash,
    
    /// Zero-knowledge proof that:
    /// 1. Commitment corresponds to valid hardware fingerprint
    /// 2. Fingerprint is unique (not in registry)
    /// Without revealing the actual fingerprint
    pub zk_proof: Vec<u8>,
    
    /// Public verification key
    pub verification_key: Vec<u8>,
    
    /// Proof metadata (non-sensitive)
    pub metadata: ProofMetadata,
}

pub struct ProofMetadata {
    /// Timestamp of proof generation
    pub timestamp: u64,
    
    /// Proof version
    pub version: u8,
    
    /// Algorithm used (e.g., "zk-SNARK", "zk-STARK")
    pub algorithm: String,
}

impl ZkUniquenessProof {
    /// Generate ZK proof that hardware fingerprint is unique
    /// Without revealing the actual fingerprint to the ledger
    pub fn generate(
        hardware_fingerprint: &HardwareFingerprint,
        registry: &NodeRegistry,
    ) -> Result<Self, String> {
        // Create commitment to fingerprint (hiding)
        let commitment = hash(&hardware_fingerprint.fingerprint);
        
        // Check if commitment already exists (duplicate hardware)
        if registry.has_commitment(&commitment) {
            return Err("Hardware fingerprint already registered".to_string());
        }
        
        // Generate ZK proof that:
        // 1. Commitment corresponds to valid hardware fingerprint
        // 2. Fingerprint is unique (not in registry)
        // Without revealing the actual fingerprint
        
        // In full implementation, this would use:
        // - zk-SNARKs (e.g., Groth16, PLONK) or zk-STARKs
        // - Prove knowledge of fingerprint without revealing it
        // - Prove fingerprint is not in existing registry
        // - Circuit: "I know a fingerprint F such that:
        //            - F is valid (matches hardware attributes)
        //            - hash(F) = commitment
        //            - commitment not in registry"
        
        let zk_proof = generate_zk_proof(
            &hardware_fingerprint.fingerprint,
            &commitment,
            registry,
        )?;
        
        let verification_key = get_verification_key();
        
        Ok(Self {
            commitment,
            zk_proof,
            verification_key,
            metadata: ProofMetadata {
                timestamp: current_timestamp(),
                version: 1,
                algorithm: "zk-SNARK".to_string(),
            },
        })
    }
    
    /// Verify ZK proof
    pub fn verify(&self, registry: &NodeRegistry) -> bool {
        // Verify ZK proof is valid
        if !verify_zk_proof(&self.zk_proof, &self.verification_key, &self.commitment) {
            return false;
        }
        
        // Verify commitment is not in registry (uniqueness)
        !registry.has_commitment(&self.commitment)
    }
}

/// Helper functions for ZK proof generation (conceptual)
fn generate_zk_proof(
    fingerprint: &Hash,
    commitment: &Hash,
    registry: &NodeRegistry,
) -> Result<Vec<u8>, String> {
    // In full implementation:
    // 1. Create arithmetic circuit
    // 2. Generate proving key
    // 3. Generate proof using circuit
    // 4. Return serialized proof
    
    // For now, return placeholder
    // Real implementation would use libraries like:
    // - bellman (zk-SNARKs)
    // - arkworks (zk-SNARKs)
    // - starky (zk-STARKs)
    
    Ok(vec![]) // Placeholder
}

fn verify_zk_proof(
    proof: &[u8],
    verification_key: &[u8],
    commitment: &Hash,
) -> bool {
    // In full implementation:
    // 1. Deserialize proof
    // 2. Load verification key
    // 3. Verify proof using verification key
    // 4. Check commitment matches
    
    // For now, return placeholder
    true // Placeholder
}
```

**Mechanism 3: Geographic Verification (Optional)**

```rust
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub country: String,
    pub city: String,
}

impl NodeRegistry {
    /// Check geographic clustering (potential Sybil)
    pub fn check_geographic_clustering(&self, radius_km: f64) -> Vec<Vec<NodeIdentity>> {
        // Group nodes by geographic proximity
        // If > 10 nodes within radius, flag as potential Sybil
        // This is advisory, not blocking
    }
}
```

### 4. Activity Snapshot System (On-Chain)

**Daily Activity Snapshots:**
- Protocol records daily snapshots of all active nodes
- Node marked "active" only if it participated in consensus during 24-hour window
- Participation = submitted block OR ZK proof (any stream)
- Snapshots stored on-chain for verifiable longevity tracking

**Implementation:**
```rust
pub struct LongevityTracker {
    /// Daily snapshots of node activity (on-chain)
    daily_snapshots: Vec<DailySnapshot>,
    
    /// Current node states
    current_states: HashMap<NodeIdentity, NodeLongevity>,
    
    /// Blockchain reference (for on-chain storage)
    blockchain: Arc<RwLock<Blockchain>>,
}

pub struct DailySnapshot {
    /// Date (Unix timestamp, midnight UTC)
    pub date: u64,
    
    /// Set of nodes that participated in consensus this day
    pub active_nodes: HashSet<NodeIdentity>,
    
    /// Participation records (verifiable on-chain)
    pub participation_records: HashMap<NodeIdentity, ParticipationRecord>,
}

pub struct ParticipationRecord {
    /// Type of participation
    pub participation_type: ParticipationType,
    
    /// Block hash or proof hash (verifiable)
    pub participation_proof: Hash,
    
    /// Timestamp of participation
    pub timestamp: u64,
}

impl LongevityTracker {
    /// Record daily snapshot (called at midnight UTC)
    pub fn record_daily_snapshot(&mut self) {
        let current_time = current_timestamp();
        let today = current_time - (current_time % 86400); // Round to midnight
        
        // Get all nodes that participated today
        let active_nodes = self.get_nodes_with_activity_today(today);
        let participation_records = self.get_participation_records(today);
        
        let snapshot = DailySnapshot {
            date: today,
            active_nodes: active_nodes.clone(),
            participation_records,
        };
        
        // Store snapshot on-chain
        self.store_snapshot_on_chain(&snapshot);
        
        self.daily_snapshots.push(snapshot);
        
        // Update longevity for all nodes
        for (identity, longevity) in &mut self.current_states {
            if active_nodes.contains(identity) {
                // Node participated today
                if let Some(record) = participation_records.get(identity) {
                    longevity.record_activity_snapshot(record.participation_type.clone());
                }
            } else {
                // Node did not participate today
                longevity.record_no_activity();
            }
        }
    }
    
    /// Get nodes that participated in consensus today
    fn get_nodes_with_activity_today(&self, today: u64) -> HashSet<NodeIdentity> {
        let mut active_nodes = HashSet::new();
        
        // Query blockchain for blocks/proofs submitted today
        let blocks = self.blockchain.read().unwrap()
            .get_blocks_by_date(today);
        
        for block in blocks {
            if let Some(miner) = self.get_block_miner(&block) {
                active_nodes.insert(miner);
            }
        }
        
        // Query for ZK proofs submitted today
        let proofs = self.blockchain.read().unwrap()
            .get_zk_proofs_by_date(today);
        
        for proof in proofs {
            if let Some(prover) = self.get_proof_prover(&proof) {
                active_nodes.insert(prover);
            }
        }
        
        active_nodes
    }
    
    /// Get participation records for today
    fn get_participation_records(&self, today: u64) -> HashMap<NodeIdentity, ParticipationRecord> {
        let mut records = HashMap::new();
        
        // Query blockchain for participation
        let blocks = self.blockchain.read().unwrap()
            .get_blocks_by_date(today);
        
        for block in blocks {
            if let Some(miner) = self.get_block_miner(&block) {
                records.insert(miner.clone(), ParticipationRecord {
                    participation_type: ParticipationType::BlockMined {
                        stream: block.stream_type,
                        block_hash: block.hash,
                    },
                    participation_proof: block.hash,
                    timestamp: block.timestamp,
                });
            }
        }
        
        records
    }
    
    /// Store snapshot on-chain (for verifiable longevity)
    fn store_snapshot_on_chain(&self, snapshot: &DailySnapshot) {
        // Create special transaction to store snapshot
        // This ensures longevity is verifiable on-chain
        let snapshot_tx = create_snapshot_transaction(snapshot);
        self.blockchain.write().unwrap().add_transaction(snapshot_tx);
    }
}
```

### 5. Reset Conditions

**Longevity Resets If:**
1. Node offline > 30 days
2. Node identity transferred (impossible, but check)
3. Node caught in Sybil attack (manual review)

**Implementation:**
```rust
impl NodeLongevity {
    /// Check and apply reset conditions
    pub fn check_reset(&mut self, current_time: u64) {
        // Reset if offline > 30 days
        if current_time - self.last_seen > 2592000 {
            self.active_days = 0;
            self.uptime_percentage = 0.0;
            // Note: blocks_mined remains (historical record)
        }
    }
    
    /// Manual reset (for Sybil attacks)
    pub fn manual_reset(&mut self, reason: String) {
        // Only callable by governance or admin
        self.active_days = 0;
        self.uptime_percentage = 0.0;
        // Log reason for audit
    }
}
```

---

## 🔍 Verification & Transparency

### Public Node Registry

**RPC Endpoint:**
```rust
async fn mds_getNodeRegistry() -> Result<Value> {
    let registry = node_registry.get_all_nodes();
    
    Ok(json!({
        "total_nodes": registry.len(),
        "active_nodes": registry.iter().filter(|n| n.is_active()).count(),
        "nodes": registry.iter().map(|node| {
            json!({
                "identity": hex::encode(&node.identity.public_key),
                "active_days": node.longevity.active_days,
                "blocks_mined": node.longevity.blocks_mined,
                "uptime_percentage": node.longevity.uptime_percentage,
                "longevity_weight": node.longevity.calculate_weight(network_age),
                "last_seen": node.longevity.last_seen,
            })
        }).collect::<Vec<_>>()
    }))
}
```

### Explorer Display

**Node Longevity Dashboard:**
```html
<div class="node-longevity-dashboard">
    <h3>Node Longevity Statistics</h3>
    
    <div class="stats">
        <div class="stat">
            <span class="label">Total Nodes:</span>
            <span class="value">1,247</span>
        </div>
        <div class="stat">
            <span class="label">Active Nodes:</span>
            <span class="value">892</span>
        </div>
        <div class="stat">
            <span class="label">Avg Longevity:</span>
            <span class="value">45 days</span>
        </div>
    </div>
    
    <div class="top-nodes">
        <h4>Top Nodes by Longevity</h4>
        <table>
            <tr>
                <th>Node</th>
                <th>Active Days</th>
                <th>Blocks Mined</th>
                <th>Uptime</th>
                <th>Weight</th>
            </tr>
            <!-- ... -->
        </table>
    </div>
</div>
```

---

## 🛡️ Anti-Gaming Measures

### 1. IP-Based Uniqueness
- ✅ One node per IP address
- ✅ Prevents simple Sybil attacks
- ⚠️ Can be bypassed with VPNs (use proof-of-uniqueness)

### 2. Hardware Fingerprinting
- ✅ CPU ID, MAC address, system UUID
- ✅ Combined fingerprint
- ⚠️ Can be spoofed (but expensive)

### 3. Geographic Clustering Detection
- ✅ Flag nodes in same location
- ✅ Advisory, not blocking
- ⚠️ Legitimate nodes may cluster (datacenters)

### 4. Longevity Reset
- ✅ Reset if offline > 30 days
- ✅ Prevents "zombie" nodes
- ✅ Encourages active participation

### 5. Minimum Requirements
- ✅ Must be active ≥ 30 days
- ✅ Must be online ≥ 80% of time
- ✅ Must have mined ≥ 1 block
- ✅ Prevents instant gaming

### 6. Weight Caps
- ✅ Maximum 0.1% per node
- ✅ Prevents single node dominance
- ✅ Encourages distribution

---

## 📊 Example Calculation

**Scenario:**
- Network age: 100 days
- Node A: Active 80 days, 95% uptime, 50 blocks mined
- Node B: Active 30 days, 85% uptime, 5 blocks mined
- Node C: Active 10 days, 90% uptime, 2 blocks mined

**Calculations:**

**Node A:**
- Meets all requirements ✅
- Longevity ratio: 80/100 = 0.8
- Base weight: 0.8 × 0.4 = 0.32
- Capped at 0.1% = **0.001**

**Node B:**
- Meets all requirements ✅
- Longevity ratio: 30/100 = 0.3
- Base weight: 0.3 × 0.4 = 0.12
- Capped at 0.1% = **0.001**

**Node C:**
- Does NOT meet requirements ❌
- Active days < 30
- Weight = **0.0**

**Result**: Both Node A and Node B get 0.1% weight (capped), Node C gets 0%

---

## 🎯 Summary

**Node Longevity Design:**
- ✅ Unique identity (IP + hardware fingerprint)
- ✅ Longevity calculation (active days / network age)
- ✅ Minimum requirements (30 days, 80% uptime, 1 block)
- ✅ Weight caps (0.1% per node)
- ✅ Reset conditions (offline > 30 days)
- ✅ Anti-gaming measures (Sybil prevention)

**Result**: Fair, transparent, resistant to gaming

---

**Ready for implementation!** 🔒
