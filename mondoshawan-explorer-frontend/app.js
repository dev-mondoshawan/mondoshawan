// Mondoshawan Block Explorer - Frontend Application

// RPC Configuration - Can be set via URL parameter or localStorage
const urlParams = new URLSearchParams(window.location.search);
const RPC_BASE = urlParams.get('rpc') || 
                 localStorage.getItem('rpc_endpoint') || 
                 'http://localhost:8545';

// Store RPC endpoint for future use
if (!localStorage.getItem('rpc_endpoint')) {
    localStorage.setItem('rpc_endpoint', RPC_BASE);
}

// Generic JSON-RPC call helper
async function rpcCall(method, params = []) {
    const response = await fetch(RPC_BASE, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            jsonrpc: '2.0',
            method: method,
            params: params,
            id: 1
        })
    });
    
    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }
    
    const data = await response.json();
    if (data.error) {
        throw new Error(data.error.message || 'RPC error');
    }
    
    return data.result;
}

// Initialize app
document.addEventListener('DOMContentLoaded', () => {
    loadDashboard();
    loadRecentBlocks();
    loadRecentTransactions();
    loadShardingStats();
    loadCrossShardTransactions();
    loadOrderingPolicy();
    loadMevMetrics();
    setupSearch();
    setupFairnessControls();
    setupForensicsControls();
    setupAccountAbstraction();
});

// Load dashboard statistics
async function loadDashboard() {
    try {
        // Get all stats in parallel using JSON-RPC
        const [blockNumber, dagStats, tps, peerCount, nodeStatus] = await Promise.all([
            rpcCall('eth_blockNumber').catch(() => '0x0'),
            rpcCall('mds_getDagStats').catch(() => ({ total_blocks: 0, total_transactions: 0 })),
            rpcCall('mds_getTps', [60]).catch(() => 0),  // 60-second window
            rpcCall('net_peerCount').catch(() => '0x0'),
            rpcCall('mds_getNodeStatus').catch(() => null)
        ]);

        // Parse hex values
        const latestBlock = parseInt(blockNumber, 16) || 0;
        const peers = parseInt(peerCount, 16) || 0;
        const tpsValue = parseFloat(tps) || 0;
        const totalTransactions = dagStats.total_transactions || 0;

        // Update UI
        document.getElementById('total-blocks').textContent = latestBlock;
        document.getElementById('total-transactions').textContent = totalTransactions;
        document.getElementById('peers-connected').textContent = peers;
        document.getElementById('tps').textContent = tpsValue.toFixed(2);
        
        // Update last updated timestamp
        const now = new Date();
        const timeStr = now.toLocaleTimeString();
        document.getElementById('update-time').textContent = `Updated: ${timeStr}`;
        document.getElementById('update-indicator').style.background = '#10b981';
    } catch (error) {
        console.error('Error loading dashboard:', error);
        document.getElementById('total-blocks').textContent = 'Error';
        document.getElementById('update-time').textContent = 'Update failed';
        document.getElementById('update-indicator').style.background = '#ef4444';
    }
}

// Load recent blocks
async function loadRecentBlocks() {
    try {
        // Get latest block number
        const blockNumberHex = await rpcCall('eth_blockNumber');
        const latestBlock = parseInt(blockNumberHex, 16);
        
        if (latestBlock === 0) {
            document.getElementById('blocks-list').innerHTML = '<p class="loading">No blocks found</p>';
            return;
        }
        
        // Get last 10 blocks
        const blockPromises = [];
        const limit = 10;
        for (let i = 0; i < limit && (latestBlock - i) >= 0; i++) {
            const blockNum = latestBlock - i;
            blockPromises.push(
                rpcCall('eth_getBlockByNumber', [`0x${blockNum.toString(16)}`, true])
                    .catch(() => null)
            );
        }
        
        const blocks = await Promise.all(blockPromises);
        
        // Filter out null blocks
        const validBlocks = blocks.filter(b => b !== null);
        
        const blocksList = document.getElementById('blocks-list');
        
        if (validBlocks.length === 0) {
            blocksList.innerHTML = '<p class="loading">No blocks found</p>';
            return;
        }
        
        blocksList.innerHTML = validBlocks.map(block => {
            const blockNum = parseInt(block.number, 16);
            const timestamp = parseInt(block.timestamp, 16);
            const txCount = block.transactions ? block.transactions.length : 0;
            
            return `
                <div class="block-item">
                    <h3>Block #${blockNum}</h3>
                    <p><strong>Hash:</strong> <code>${block.hash}</code></p>
                    <p><strong>Timestamp:</strong> ${new Date(timestamp * 1000).toLocaleString()}</p>
                    <p><strong>Transactions:</strong> ${txCount}</p>
                </div>
            `;
        }).join('');
    } catch (error) {
        console.error('Error loading blocks:', error);
        document.getElementById('blocks-list').innerHTML = '<p class="error">Error loading blocks</p>';
    }
}

// Load recent transactions
async function loadRecentTransactions() {
    try {
        // Get latest block number
        const blockNumberHex = await rpcCall('eth_blockNumber');
        const latestBlock = parseInt(blockNumberHex, 16);
        
        if (latestBlock === 0) {
            document.getElementById('transactions-list').innerHTML = '<p class="loading">No transactions found</p>';
            return;
        }
        
        // Get transactions from recent blocks
        const transactions = [];
        const maxBlocks = 5;  // Check last 5 blocks
        const limit = 10;
        
        for (let i = 0; i < maxBlocks && transactions.length < limit && (latestBlock - i) >= 0; i++) {
            const blockNum = latestBlock - i;
            try {
                const block = await rpcCall('eth_getBlockByNumber', [`0x${blockNum.toString(16)}`, true]);
                
                if (block && block.transactions) {
                    for (const tx of block.transactions) {
                        if (transactions.length >= limit) break;
                        transactions.push({
                            ...tx,
                            block_number: blockNum,
                            block_hash: block.hash
                        });
                    }
                }
            } catch (error) {
                console.error(`Error loading block ${blockNum}:`, error);
            }
        }
        
        const transactionsList = document.getElementById('transactions-list');
        
        if (transactions.length === 0) {
            transactionsList.innerHTML = '<p class="loading">No transactions found</p>';
            return;
        }
        
        // Render transactions
        transactionsList.innerHTML = transactions.map(tx => {
            const value = parseInt(tx.value || '0x0', 16) / 1e18;
            return `
                <div class="transaction-item" data-tx-hash="${tx.hash}">
                    <h3>Transaction</h3>
                    <p><strong>Hash:</strong> <code>${tx.hash}</code></p>
                    <p><strong>From:</strong> <code>${tx.from}</code></p>
                    <p><strong>To:</strong> <code>${tx.to || 'Contract Creation'}</code></p>
                    <p><strong>Value:</strong> ${value.toFixed(6)} MSHW</p>
                    <p><strong>Block:</strong> #${tx.block_number}</p>
                    <div class="risk-indicator" data-tx="${tx.hash}">
                        <span class="risk-loading">Loading risk analysis...</span>
                    </div>
                </div>
            `;
        }).join('');
        
        // Load risk scores asynchronously
        transactions.forEach(async (tx) => {
            try {
                const risk = await getTransactionRisk(tx.hash);
                const indicator = document.querySelector(`.risk-indicator[data-tx="${tx.hash}"]`);
                if (indicator) {
                    const riskPercent = (risk.score * 100).toFixed(1);
                    const riskClass = getRiskClass(risk.score);
                    indicator.innerHTML = `
                        <span class="risk-badge ${riskClass}">Risk: ${riskPercent}%</span>
                        ${risk.labels && risk.labels.length > 0 ? `
                            <span class="risk-label-preview">${risk.labels[0]}</span>
                        ` : ''}
                    `;
                }
            } catch (error) {
                const indicator = document.querySelector(`.risk-indicator[data-tx="${tx.hash}"]`);
                if (indicator) {
                    indicator.innerHTML = '<span class="risk-unavailable">Risk analysis unavailable</span>';
                }
            }
        });
    } catch (error) {
        console.error('Error loading transactions:', error);
        document.getElementById('transactions-list').innerHTML = '<p class="error">Error loading transactions</p>';
    }
}

// Setup search functionality
function setupSearch() {
    const searchInput = document.getElementById('search-input');
    const searchBtn = document.getElementById('search-btn');
    
    const performSearch = async () => {
        const query = searchInput.value.trim();
        if (!query) return;
        
        try {
            // Try to determine if it's a block, transaction, or address
            if (query.startsWith('0x') && query.length === 66) {
                // Likely a block hash or transaction hash
                // Try block first
                try {
                    const block = await rpcCall('eth_getBlockByHash', [query, true]);
                    if (block) {
                        await displayBlockFromRpc(block);
                        return;
                    }
                } catch (e) {
                    console.log('Not a block hash, trying transaction...');
                }
                
                // Try transaction
                try {
                    const tx = await rpcCall('eth_getTransactionByHash', [query]);
                    if (tx) {
                        await displayTransaction(tx);
                        await explainTransaction(tx.hash);
                        return;
                    }
                } catch (e) {
                    console.log('Not a transaction hash');
                }
            } else if (query.startsWith('0x') && query.length === 42) {
                // Likely an address - use RPC to get balance and info
                await displayAddress(query);
                await explainAddress(query);
                return;
            } else if (!isNaN(query)) {
                // Block number
                try {
                    const block = await rpcCall('eth_getBlockByNumber', [`0x${parseInt(query).toString(16)}`, true]);
                    if (block) {
                        await displayBlockFromRpc(block);
                        return;
                    }
                } catch (e) {
                    console.error('Error loading block:', e);
                }
            }
            
            // If we get here, search failed
            alert('No results found. Please check your search query.');
        } catch (error) {
            console.error('Search error:', error);
            alert('Search failed. Please try again.');
        }
    };
    
    searchBtn.addEventListener('click', performSearch);
    searchInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            performSearch();
        }
    });
}

// Display block details
async function displayBlock(block) {
    const addressInfo = document.getElementById('address-info');
    addressInfo.innerHTML = `
        <h3>Block Details</h3>
        <p><strong>Number:</strong> ${block.number}</p>
        <p><strong>Hash:</strong> <code>${block.hash}</code></p>
        <p><strong>Timestamp:</strong> ${new Date(block.timestamp * 1000).toLocaleString()}</p>
        <p><strong>Transactions:</strong> ${block.transaction_count}</p>
        <p><strong>Size:</strong> ${block.size} bytes</p>
    `;
    
    // Load fairness metrics for this block
    await loadBlockFairness(block.hash);
    
    // Scroll to address section
    document.getElementById('addresses').scrollIntoView({ behavior: 'smooth' });
}

// Display transaction details
async function displayTransaction(tx) {
    const addressInfo = document.getElementById('address-info');
    const txHash = tx.hash || tx;
    
    // Load transaction risk
    let riskScore = null;
    try {
        riskScore = await getTransactionRisk(txHash);
    } catch (error) {
        console.error('Error loading transaction risk:', error);
    }
    
    // Load cross-shard transaction info
    let crossShardInfo = null;
    try {
        const response = await fetch(`${RPC_BASE}`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                jsonrpc: '2.0',
                method: 'mds_getCrossShardTransaction',
                params: [txHash],
                id: 1
            })
        });
        const data = await response.json();
        if (data.result && data.result.is_cross_shard) {
            crossShardInfo = data.result;
        }
    } catch (error) {
        console.error('Error loading cross-shard info:', error);
    }
    
    let riskHtml = '';
    if (riskScore) {
        const riskPercent = (riskScore.score * 100).toFixed(1);
        const riskClass = getRiskClass(riskScore.score);
        riskHtml = `
            <div class="risk-section">
                <h4>🔒 Security Analysis</h4>
                <div class="risk-score ${riskClass}">
                    <span class="risk-label">Risk Score:</span>
                    <span class="risk-value">${riskPercent}%</span>
                    <span class="risk-confidence">(Confidence: ${(riskScore.confidence * 100).toFixed(0)}%)</span>
                </div>
                ${riskScore.labels.length > 0 ? `
                    <div class="risk-labels">
                        <strong>Risk Labels:</strong>
                        ${riskScore.labels.map(label => `<span class="risk-label-badge">${formatRiskLabel(label)}</span>`).join('')}
                    </div>
                ` : ''}
            </div>
        `;
    }
    
    let shardHtml = '';
    if (crossShardInfo) {
        shardHtml = `
            <div class="shard-section">
                <h4>🔷 Sharding Information</h4>
                <p><strong>Cross-Shard Transaction:</strong> Yes</p>
                <p><strong>Source Shard:</strong> ${crossShardInfo.source_shard}</p>
                <p><strong>Target Shard:</strong> ${crossShardInfo.target_shard}</p>
                <p><strong>Status:</strong> <span class="status-${crossShardInfo.status.toLowerCase()}">${crossShardInfo.status}</span></p>
            </div>
        `;
    } else if (tx.fromShard !== undefined) {
        shardHtml = `
            <div class="shard-section">
                <h4>🔷 Sharding Information</h4>
                <p><strong>From Shard:</strong> ${tx.fromShard}</p>
                <p><strong>To Shard:</strong> ${tx.toShard}</p>
                <p><strong>Cross-Shard:</strong> ${tx.isCrossShard ? 'Yes' : 'No'}</p>
            </div>
        `;
    }
    
    // Check for time-lock and sponsor info
    let timeLockHtml = '';
    if (tx.executeAtBlock || tx.executeAtTimestamp) {
        const currentBlock = parseInt(await rpcCall('eth_blockNumber').catch(() => '0x0'), 16) || 0;
        const currentTimestamp = Math.floor(Date.now() / 1000);
        const executeAtBlock = tx.executeAtBlock ? parseInt(tx.executeAtBlock, 16) : null;
        const executeAtTimestamp = tx.executeAtTimestamp ? parseInt(tx.executeAtTimestamp, 16) : null;
        const isReady = (!executeAtBlock || currentBlock >= executeAtBlock) && 
                       (!executeAtTimestamp || currentTimestamp >= executeAtTimestamp);
        
        timeLockHtml = `
            <div class="time-lock-section" style="margin-top: 1rem; padding: 1rem; background: #1e293b; border-radius: 8px;">
                <h4>⏰ Time-Locked Transaction</h4>
                ${executeAtBlock ? `<p><strong>Execute At Block:</strong> ${executeAtBlock} (Current: ${currentBlock})</p>` : ''}
                ${executeAtTimestamp ? `<p><strong>Execute At:</strong> ${new Date(executeAtTimestamp * 1000).toLocaleString()}</p>` : ''}
                <p><strong>Status:</strong> <span class="status-${isReady ? 'ready' : 'pending'}">${isReady ? '✅ Ready' : '⏳ Pending'}</span></p>
            </div>
        `;
    }
    
    let sponsorHtml = '';
    if (tx.sponsor || tx.isGasless) {
        const sponsor = tx.sponsor || 'N/A';
        sponsorHtml = `
            <div class="sponsor-section" style="margin-top: 1rem; padding: 1rem; background: #1e293b; border-radius: 8px;">
                <h4>💳 Gasless Transaction</h4>
                <p><strong>Sponsor:</strong> <code>${sponsor}</code></p>
                <p><strong>Fee Paid By:</strong> Sponsor (${sponsor.substring(0, 10)}...)</p>
                <p style="color: #10b981; font-size: 0.9rem;">✨ User doesn't need MSHW for gas</p>
            </div>
        `;
    }
    
    addressInfo.innerHTML = `
        <h3>Transaction Details</h3>
        <p><strong>Hash:</strong> <code>${txHash}</code></p>
        <p><strong>From:</strong> <code>${tx.from || 'N/A'}</code></p>
        <p><strong>To:</strong> <code>${tx.to || 'N/A'}</code></p>
        <p><strong>Value:</strong> ${tx.value || 'N/A'}</p>
        <p><strong>Fee:</strong> ${tx.fee || 'N/A'}</p>
        <p><strong>Status:</strong> ${tx.status || 'N/A'}</p>
        <p><strong>Block:</strong> ${tx.block_number || 'Pending'}</p>
        ${timeLockHtml}
        ${sponsorHtml}
        ${shardHtml}
        ${riskHtml}
    `;
    
    // Scroll to address section
    document.getElementById('addresses').scrollIntoView({ behavior: 'smooth' });
}

// Display address details
async function displayAddress(address) {
    const addressInfo = document.getElementById('address-info');
    const addressStr = address.address || address;
    
    // Load shard information
    let shardId = null;
    try {
        const response = await fetch(`${RPC_BASE}`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                jsonrpc: '2.0',
                method: 'mds_getShardForAddress',
                params: [addressStr],
                id: 1
            })
        });
        const data = await response.json();
        if (data.result) {
            shardId = parseInt(data.result, 16);
        }
    } catch (error) {
        console.error('Error loading shard info:', error);
    }
    
    // Load risk score
    let riskScore = null;
    try {
        riskScore = await getRiskScore(addressStr);
    } catch (error) {
        console.error('Error loading risk score:', error);
    }
    
    // Load reputation score
    let reputation = null;
    let reputationFactors = null;
    try {
        reputation = await rpcCall('mds_getReputation', [addressStr]).catch(() => null);
        if (reputation) {
            reputationFactors = await rpcCall('mds_getReputationFactors', [addressStr]).catch(() => null);
        }
    } catch (error) {
        console.error('Error loading reputation:', error);
    }
    
    let riskHtml = '';
    if (riskScore) {
        const riskPercent = (riskScore.score * 100).toFixed(1);
        const riskClass = getRiskClass(riskScore.score);
        riskHtml = `
            <div class="risk-section">
                <h4>🔒 Security Analysis</h4>
                <div class="risk-score ${riskClass}">
                    <span class="risk-label">Risk Score:</span>
                    <span class="risk-value">${riskPercent}%</span>
                    <span class="risk-confidence">(Confidence: ${(riskScore.confidence * 100).toFixed(0)}%)</span>
                </div>
                ${riskScore.labels.length > 0 ? `
                    <div class="risk-labels">
                        <strong>Risk Labels:</strong>
                        ${riskScore.labels.map(label => `<span class="risk-label-badge">${formatRiskLabel(label)}</span>`).join('')}
                    </div>
                ` : ''}
            </div>
        `;
    }
    
    let reputationHtml = '';
    if (reputation) {
        const repValue = reputation.reputation || 0;
        const repClass = repValue >= 70 ? 'high' : repValue >= 40 ? 'medium' : 'low';
        const repLabel = repValue >= 70 ? '⭐ High' : repValue >= 40 ? '✓ Medium' : '⚠ Low';
        
        reputationHtml = `
            <div class="reputation-section" style="margin-top: 1rem; padding: 1rem; background: #1e293b; border-radius: 8px;">
                <h4>⭐ Reputation Score</h4>
                <div class="reputation-score ${repClass}" style="font-size: 1.5rem; font-weight: bold; margin: 0.5rem 0;">
                    ${repValue.toFixed(1)}/100 <span style="font-size: 1rem; color: #64748b;">(${repLabel})</span>
                </div>
                ${reputationFactors && reputationFactors.factors ? `
                    <div style="margin-top: 1rem; font-size: 0.9rem; color: #94a3b8;">
                        <p><strong>Successful Txs:</strong> ${reputationFactors.factors.successfulTxs || 0}</p>
                        <p><strong>Failed Txs:</strong> ${reputationFactors.factors.failedTxs || 0}</p>
                        ${reputationFactors.factors.blocksMined > 0 ? `<p><strong>Blocks Mined:</strong> ${reputationFactors.factors.blocksMined}</p>` : ''}
                        ${reputationFactors.factors.nodeLongevity ? `<p><strong>Node Longevity:</strong> ${(reputationFactors.factors.nodeLongevity * 100).toFixed(2)}%</p>` : ''}
                        ${reputationFactors.factors.suspiciousActivities > 0 ? `<p style="color: #ef4444;"><strong>Suspicious Activities:</strong> ${reputationFactors.factors.suspiciousActivities}</p>` : ''}
                    </div>
                ` : ''}
            </div>
        `;
    }
    
    let shardHtml = '';
    if (shardId !== null) {
        shardHtml = `
            <div class="shard-section">
                <h4>🔷 Sharding Information</h4>
                <p><strong>Shard ID:</strong> ${shardId}</p>
            </div>
        `;
    }
    
    // Check if address is a contract wallet
    let walletInfo = null;
    try {
        const isWallet = await rpcCall('mds_isContractWallet', [addressStr]).catch(() => null);
        if (isWallet && isWallet.isContractWallet) {
            walletInfo = await rpcCall('mds_getWallet', [addressStr]).catch(() => null);
        }
    } catch (error) {
        console.error('Error checking wallet:', error);
    }
    
    let walletHtml = '';
    if (walletInfo) {
        const walletType = walletInfo.walletType || 'basic';
        walletHtml = `
            <div class="wallet-section" style="margin-top: 1rem; padding: 1rem; background: #1e293b; border-radius: 8px; border-left: 4px solid #8b5cf6;">
                <h4>🔐 Smart Contract Wallet</h4>
                <p><strong>Wallet Type:</strong> <span style="text-transform: capitalize;">${walletType}</span></p>
                <p><strong>Owner:</strong> <code>${walletInfo.owner || 'N/A'}</code></p>
                ${walletType === 'multisig' && walletInfo.signers ? `
                    <p><strong>Signers:</strong> ${walletInfo.signers.length}</p>
                    <p><strong>Threshold:</strong> ${walletInfo.threshold || 'N/A'}</p>
                ` : ''}
                ${walletType === 'socialRecovery' && walletInfo.guardians ? `
                    <p><strong>Guardians:</strong> ${walletInfo.guardians.length}</p>
                    <p><strong>Recovery Threshold:</strong> ${walletInfo.recoveryThreshold || 'N/A'}</p>
                ` : ''}
            </div>
        `;
    }
    
    addressInfo.innerHTML = `
        <h3>Address Details</h3>
        <p><strong>Address:</strong> <code>${addressStr}</code></p>
        <p><strong>Balance:</strong> ${address.balance || 'N/A'}</p>
        <p><strong>Transactions:</strong> ${address.transaction_count || 'N/A'}</p>
        <p><strong>First Seen:</strong> ${address.first_seen ? new Date(address.first_seen * 1000).toLocaleString() : 'N/A'}</p>
        <p><strong>Last Seen:</strong> ${address.last_seen ? new Date(address.last_seen * 1000).toLocaleString() : 'N/A'}</p>
        ${walletHtml}
        ${reputationHtml}
        ${shardHtml}
        ${riskHtml}
    `;
    
    // Scroll to address section
    document.getElementById('addresses').scrollIntoView({ behavior: 'smooth' });
}

// Display search results
function displaySearchResults(results) {
    const addressInfo = document.getElementById('address-info');
    
    if (results.results.length === 0) {
        addressInfo.innerHTML = '<p class="error">No results found</p>';
        return;
    }
    
    addressInfo.innerHTML = `
        <h3>Search Results for "${results.query}"</h3>
        ${results.results.map(item => `
            <div class="block-item">
                <h3>${item.type}</h3>
                <p>${item.summary}</p>
            </div>
        `).join('')}
    `;
    
    // Scroll to address section
    document.getElementById('addresses').scrollIntoView({ behavior: 'smooth' });
}

// Get risk score for an address
async function getRiskScore(address) {
    const response = await fetch(RPC_BASE, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            jsonrpc: '2.0',
            method: 'mds_getRiskScore',
            params: [address],
            id: 1
        })
    });
    
    const data = await response.json();
    if (data.error) {
        throw new Error(data.error.message);
    }
    
    return data.result;
}

// Get risk labels for an address
async function getRiskLabels(address) {
    const response = await fetch(RPC_BASE, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            jsonrpc: '2.0',
            method: 'mds_getRiskLabels',
            params: [address],
            id: 1
        })
    });
    
    const data = await response.json();
    if (data.error) {
        throw new Error(data.error.message);
    }
    
    return data.result;
}

// Get transaction risk
async function getTransactionRisk(txHash) {
    const response = await fetch(RPC_BASE, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            jsonrpc: '2.0',
            method: 'mds_getTransactionRisk',
            params: [txHash],
            id: 1
        })
    });
    
    const data = await response.json();
    if (data.error) {
        throw new Error(data.error.message);
    }
    
    return data.result;
}

// Get risk class for styling
function getRiskClass(score) {
    if (score >= 0.7) return 'risk-high';
    if (score >= 0.4) return 'risk-medium';
    return 'risk-low';
}

// Format risk label for display
function formatRiskLabel(label) {
    return label
        .split('_')
        .map(word => word.charAt(0).toUpperCase() + word.slice(1))
        .join(' ');
}

// Setup forensics controls
function setupForensicsControls() {
    const traceBtn = document.getElementById('trace-funds-btn');
    const summaryBtn = document.getElementById('get-summary-btn');
    const anomalyBtn = document.getElementById('detect-anomalies-btn');
    
    traceBtn.addEventListener('click', async () => {
        const address = document.getElementById('trace-address-input').value.trim();
        const maxHops = parseInt(document.getElementById('trace-hops-input').value) || 5;
        
        if (!address) {
            alert('Please enter an address');
            return;
        }
        
        await traceFunds(address, maxHops);
    });
    
    summaryBtn.addEventListener('click', async () => {
        const address = document.getElementById('summary-address-input').value.trim();
        
        if (!address) {
            alert('Please enter an address');
            return;
        }
        
        await getAddressSummary(address);
    });
    
    anomalyBtn.addEventListener('click', async () => {
        const address = document.getElementById('anomaly-address-input').value.trim();
        
        if (!address) {
            alert('Please enter an address');
            return;
        }
        
        await detectAnomalies(address);
    });
}

// Trace funds from an address
async function traceFunds(address, maxHops) {
    try {
        const response = await fetch(`${RPC_BASE}`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                jsonrpc: '2.0',
                method: 'mds_traceFunds',
                params: [address, maxHops, 10],
                id: 1
            })
        });
        
        const data = await response.json();
        const result = data.result;
        
        const flowsDisplay = document.getElementById('fund-flows-display');
        
        if (result.flows_found === 0) {
            flowsDisplay.innerHTML = '<p class="info">No fund flows found for this address</p>';
            return;
        }
        
        let html = `<div class="flows-summary">
            <p><strong>Source:</strong> ${result.source}</p>
            <p><strong>Flows Found:</strong> ${result.flows_found}</p>
            <p><strong>Max Hops:</strong> ${result.max_hops}</p>
        </div>`;
        
        html += '<div class="flows-list">';
        result.flows.forEach((flow, idx) => {
            const value = parseInt(flow.total_value, 16) / 1e18;
            html += `
                <div class="flow-item">
                    <h4>Flow ${idx + 1} (${flow.hop_count} hops, ${value.toFixed(4)} tokens)</h4>
                    <div class="flow-path">
                        ${flow.path.map((addr, i) => 
                            `<span class="path-address" data-address="${addr}">${addr.substring(0, 10)}...${addr.substring(34)}</span>${i < flow.path.length - 1 ? ' → ' : ''}`
                        ).join('')}
                    </div>
                    <div class="flow-transactions">
                        <strong>Transactions:</strong>
                        ${flow.transactions.map(tx => `<span class="tx-link" data-tx="${tx}">${tx.substring(0, 10)}...</span>`).join(', ')}
                    </div>
                </div>
            `;
        });
        html += '</div>';
        
        flowsDisplay.innerHTML = html;
        
        // Add click handlers for addresses and transactions
        flowsDisplay.querySelectorAll('.path-address').forEach(el => {
            el.addEventListener('click', () => {
                displayAddress({ address: el.dataset.address });
            });
        });
        
        flowsDisplay.querySelectorAll('.tx-link').forEach(el => {
            el.addEventListener('click', () => {
                displayTransaction({ hash: el.dataset.tx });
            });
        });
    } catch (error) {
        console.error('Error tracing funds:', error);
        document.getElementById('fund-flows-display').innerHTML = '<p class="error">Error tracing funds</p>';
    }
}

// Get address summary
async function getAddressSummary(address) {
    try {
        const response = await fetch(`${RPC_BASE}`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                jsonrpc: '2.0',
                method: 'mds_getAddressSummary',
                params: [address],
                id: 1
            })
        });
        
        const data = await response.json();
        const summary = data.result;
        
        const summaryDisplay = document.getElementById('summary-display');
        
        const totalReceived = parseInt(summary.total_received, 16) / 1e18;
        const totalSent = parseInt(summary.total_sent, 16) / 1e18;
        const netBalance = parseInt(summary.net_balance, 16) / 1e18;
        
        let html = `
            <div class="summary-details">
                <div class="summary-stat">
                    <span class="stat-label">Total Received:</span>
                    <span class="stat-value">${totalReceived.toFixed(4)} tokens</span>
                </div>
                <div class="summary-stat">
                    <span class="stat-label">Total Sent:</span>
                    <span class="stat-value">${totalSent.toFixed(4)} tokens</span>
                </div>
                <div class="summary-stat">
                    <span class="stat-label">Net Balance:</span>
                    <span class="stat-value">${netBalance.toFixed(4)} tokens</span>
                </div>
                <div class="summary-stat">
                    <span class="stat-label">Incoming Transactions:</span>
                    <span class="stat-value">${summary.incoming_tx_count}</span>
                </div>
                <div class="summary-stat">
                    <span class="stat-label">Outgoing Transactions:</span>
                    <span class="stat-value">${summary.outgoing_tx_count}</span>
                </div>
                <div class="summary-stat">
                    <span class="stat-label">Unique Contacts:</span>
                    <span class="stat-value">${summary.unique_contacts}</span>
                </div>
            </div>
        `;
        
        if (summary.suspicious_patterns.length > 0) {
            html += `
                <div class="suspicious-patterns">
                    <h4>⚠️ Suspicious Patterns Detected:</h4>
                    <ul>
                        ${summary.suspicious_patterns.map(pattern => `<li>${pattern.replace(/_/g, ' ')}</li>`).join('')}
                    </ul>
                </div>
            `;
        }
        
        if (summary.risk_indicators.length > 0) {
            html += `
                <div class="risk-indicators">
                    <h4>🔒 Risk Indicators:</h4>
                    <ul>
                        ${summary.risk_indicators.map(indicator => `<li>${indicator}</li>`).join('')}
                    </ul>
                </div>
            `;
        }
        
        summaryDisplay.innerHTML = html;
    } catch (error) {
        console.error('Error getting address summary:', error);
        document.getElementById('summary-display').innerHTML = '<p class="error">Error getting address summary</p>';
    }
}

// Detect anomalies for an address
async function detectAnomalies(address) {
    try {
        const response = await fetch(`${RPC_BASE}`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                jsonrpc: '2.0',
                method: 'mds_detectAnomalies',
                params: [address],
                id: 1
            })
        });
        
        const data = await response.json();
        const detection = data.result;
        
        const anomalyDisplay = document.getElementById('anomaly-display');
        
        const anomalyClass = detection.anomaly_score > 0.7 ? 'anomaly-high' : 
                            detection.anomaly_score > 0.4 ? 'anomaly-medium' : 'anomaly-low';
        
        let html = `
            <div class="anomaly-summary ${anomalyClass}">
                <div class="anomaly-score">
                    <span class="score-label">Anomaly Score:</span>
                    <span class="score-value">${(detection.anomaly_score * 100).toFixed(1)}%</span>
                </div>
                <div class="anomaly-confidence">
                    <span class="confidence-label">Confidence:</span>
                    <span class="confidence-value">${(detection.confidence * 100).toFixed(1)}%</span>
                </div>
                <div class="anomaly-count">
                    <span class="count-label">Anomalies Detected:</span>
                    <span class="count-value">${detection.anomalies.length}</span>
                </div>
            </div>
        `;
        
        if (detection.anomalies.length > 0) {
            html += '<div class="anomalies-list">';
            detection.anomalies.forEach((anomaly, idx) => {
                const severityClass = anomaly.severity > 0.7 ? 'severity-high' : 
                                    anomaly.severity > 0.4 ? 'severity-medium' : 'severity-low';
                html += `
                    <div class="anomaly-item ${severityClass}">
                        <h4>${anomaly.type.replace(/_/g, ' ')}</h4>
                        <p class="anomaly-description">${anomaly.description}</p>
                        <div class="anomaly-meta">
                            <span class="severity">Severity: ${(anomaly.severity * 100).toFixed(1)}%</span>
                            ${anomaly.related_addresses.length > 0 ? 
                                `<span class="related">Related: ${anomaly.related_addresses.length} addresses</span>` : 
                                ''}
                        </div>
                    </div>
                `;
            });
            html += '</div>';
        } else {
            html += '<p class="info">No anomalies detected - address behavior appears normal</p>';
        }
        
        anomalyDisplay.innerHTML = html;
    } catch (error) {
        console.error('Error detecting anomalies:', error);
        document.getElementById('anomaly-display').innerHTML = '<p class="error">Error detecting anomalies</p>';
    }
}

// Explain an address in human-readable terms
async function explainAddress(address) {
    try {
        const explainContent = document.getElementById('explain-content');
        explainContent.innerHTML = '<div class="explain-loading">🔍 Analyzing address...</div>';
        
        // Scroll to explain section
        document.getElementById('explain').scrollIntoView({ behavior: 'smooth' });
        
        // Fetch all relevant data in parallel
        const [riskScore, riskLabels, addressSummary, anomalies] = await Promise.all([
            getRiskScore(address).catch(() => null),
            getRiskLabels(address).catch(() => null),
            getAddressSummaryData(address).catch(() => null),
            detectAnomaliesData(address).catch(() => null)
        ]);
        
        // Generate human-readable explanation
        const explanation = generateAddressExplanation(address, riskScore, riskLabels, addressSummary, anomalies);
        
        explainContent.innerHTML = explanation;
    } catch (error) {
        console.error('Error explaining address:', error);
        document.getElementById('explain-content').innerHTML = 
            '<div class="explain-error">❌ Error generating explanation. Please try again.</div>';
    }
}

// Explain a transaction in human-readable terms
async function explainTransaction(txHash) {
    try {
        const explainContent = document.getElementById('explain-content');
        explainContent.innerHTML = '<div class="explain-loading">🔍 Analyzing transaction...</div>';
        
        // Scroll to explain section
        document.getElementById('explain').scrollIntoView({ behavior: 'smooth' });
        
        // Fetch transaction details and risk analysis
        const [txData, txRisk] = await Promise.all([
            fetchTransactionData(txHash).catch(() => null),
            getTransactionRisk(txHash).catch(() => null)
        ]);
        
        // Generate human-readable explanation
        const explanation = generateTransactionExplanation(txHash, txData, txRisk);
        
        explainContent.innerHTML = explanation;
    } catch (error) {
        console.error('Error explaining transaction:', error);
        document.getElementById('explain-content').innerHTML = 
            '<div class="explain-error">❌ Error generating explanation. Please try again.</div>';
    }
}

// Generate human-readable address explanation
function generateAddressExplanation(address, riskScore, riskLabels, summary, anomalies) {
    let html = `<div class="explain-card address-explanation">`;
    html += `<h3>📋 Address Explanation: ${address.substring(0, 10)}...${address.substring(34)}</h3>`;
    
    // Overall assessment
    const riskLevel = riskScore ? (riskScore.score >= 0.7 ? 'High Risk' : riskScore.score >= 0.4 ? 'Medium Risk' : 'Low Risk') : 'Unknown';
    const riskColor = riskScore ? (riskScore.score >= 0.7 ? 'risk-high' : riskScore.score >= 0.4 ? 'risk-medium' : 'risk-low') : '';
    
    html += `<div class="explain-section overall-assessment ${riskColor}">`;
    html += `<h4>🎯 Overall Assessment</h4>`;
    if (riskScore) {
        html += `<p class="assessment-text">`;
        html += `This address has a <strong>${riskLevel}</strong> profile `;
        html += `(${(riskScore.score * 100).toFixed(1)}% risk score, ${(riskScore.confidence * 100).toFixed(0)}% confidence). `;
        
        if (riskScore.score < 0.3) {
            html += `The address appears to be <strong>safe and legitimate</strong> based on current analysis.`;
        } else if (riskScore.score < 0.6) {
            html += `The address shows <strong>some concerning patterns</strong> that warrant caution.`;
        } else {
            html += `The address exhibits <strong>high-risk behavior patterns</strong> and should be treated with extreme caution.`;
        }
        html += `</p>`;
    } else {
        html += `<p class="assessment-text">Unable to assess risk - insufficient data available.</p>`;
    }
    html += `</div>`;
    
    // Activity summary
    if (summary) {
        html += `<div class="explain-section activity-summary">`;
        html += `<h4>📊 Activity Summary</h4>`;
        const totalReceived = parseInt(summary.total_received, 16) / 1e18;
        const totalSent = parseInt(summary.total_sent, 16) / 1e18;
        const netBalance = parseInt(summary.net_balance, 16) / 1e18;
        
        html += `<p class="summary-text">`;
        html += `This address has been involved in <strong>${summary.incoming_tx_count + summary.outgoing_tx_count} transactions</strong> `;
        html += `(${summary.incoming_tx_count} incoming, ${summary.outgoing_tx_count} outgoing). `;
        html += `It has received <strong>${totalReceived.toFixed(4)} tokens</strong> and sent <strong>${totalSent.toFixed(4)} tokens</strong>, `;
        html += `with a current net balance of <strong>${netBalance.toFixed(4)} tokens</strong>. `;
        html += `The address has interacted with <strong>${summary.unique_contacts} unique addresses</strong>.`;
        html += `</p>`;
        
        if (summary.incoming_tx_count > 100 && summary.outgoing_tx_count > 100) {
            html += `<p class="insight">💡 <strong>Insight:</strong> High transaction volume suggests this may be an active trading or service address.</p>`;
        } else if (summary.incoming_tx_count === 0 && summary.outgoing_tx_count > 0) {
            html += `<p class="insight">💡 <strong>Insight:</strong> This address only sends funds - it may be a withdrawal address or service.</p>`;
        } else if (summary.incoming_tx_count > 0 && summary.outgoing_tx_count === 0) {
            html += `<p class="insight">💡 <strong>Insight:</strong> This address only receives funds - it may be a deposit address or cold storage.</p>`;
        }
        html += `</div>`;
    }
    
    // Risk factors
    if (riskLabels && riskLabels.labels && riskLabels.labels.length > 0) {
        html += `<div class="explain-section risk-factors">`;
        html += `<h4>⚠️ Risk Factors</h4>`;
        html += `<ul class="risk-factors-list">`;
        riskLabels.labels.forEach(label => {
            const labelText = formatRiskLabel(label);
            html += `<li>${labelText}</li>`;
        });
        html += `</ul>`;
        html += `</div>`;
    }
    
    // Suspicious patterns
    if (summary && summary.suspicious_patterns && summary.suspicious_patterns.length > 0) {
        html += `<div class="explain-section suspicious-patterns">`;
        html += `<h4>🚨 Suspicious Patterns Detected</h4>`;
        html += `<ul class="patterns-list">`;
        summary.suspicious_patterns.forEach(pattern => {
            const patternText = pattern.replace(/_/g, ' ').replace(/\b\w/g, l => l.toUpperCase());
            html += `<li>${patternText}</li>`;
        });
        html += `</ul>`;
        html += `</div>`;
    }
    
    // Anomalies
    if (anomalies && anomalies.anomalies && anomalies.anomalies.length > 0) {
        html += `<div class="explain-section anomalies-detected">`;
        html += `<h4>🔍 Anomalies Detected</h4>`;
        html += `<p class="anomaly-score">Anomaly Score: ${(anomalies.anomaly_score * 100).toFixed(1)}% (Confidence: ${(anomalies.confidence * 100).toFixed(0)}%)</p>`;
        html += `<ul class="anomalies-list">`;
        anomalies.anomalies.slice(0, 5).forEach(anomaly => {
            const severity = anomaly.severity > 0.7 ? 'High' : anomaly.severity > 0.4 ? 'Medium' : 'Low';
            html += `<li><strong>${anomaly.type.replace(/_/g, ' ')}</strong> (${severity} severity): ${anomaly.description}</li>`;
        });
        html += `</ul>`;
        html += `</div>`;
    }
    
    // Recommendations
    html += `<div class="explain-section recommendations">`;
    html += `<h4>💡 Recommendations</h4>`;
    html += `<ul class="recommendations-list">`;
    
    if (riskScore && riskScore.score >= 0.7) {
        html += `<li class="recommendation-high">⚠️ <strong>High Risk:</strong> Exercise extreme caution. Consider avoiding interactions with this address.</li>`;
    } else if (riskScore && riskScore.score >= 0.4) {
        html += `<li class="recommendation-medium">⚡ <strong>Medium Risk:</strong> Proceed with caution. Verify the address through trusted sources before transacting.</li>`;
    } else if (riskScore && riskScore.score < 0.4) {
        html += `<li class="recommendation-low">✅ <strong>Low Risk:</strong> Address appears safe, but always verify through multiple sources.</li>`;
    }
    
    if (summary && summary.unique_contacts > 50) {
        html += `<li>📈 This address has many contacts - verify it's the correct address for your transaction.</li>`;
    }
    
    if (anomalies && anomalies.anomaly_score > 0.5) {
        html += `<li>🔍 Unusual patterns detected - investigate further before proceeding.</li>`;
    }
    
    html += `<li>🔒 Always verify addresses through official channels before sending funds.</li>`;
    html += `</ul>`;
    html += `</div>`;
    
    html += `</div>`;
    return html;
}

// Generate human-readable transaction explanation
function generateTransactionExplanation(txHash, txData, txRisk) {
    let html = `<div class="explain-card transaction-explanation">`;
    html += `<h3>📋 Transaction Explanation: ${txHash.substring(0, 16)}...${txHash.substring(48)}</h3>`;
    
    // Transaction overview
    if (txData) {
        html += `<div class="explain-section transaction-overview">`;
        html += `<h4>📝 Transaction Overview</h4>`;
        
        const value = parseInt(txData.value, 16) / 1e18;
        const fee = parseInt(txData.gasPrice, 16) * parseInt(txData.gas, 16) / 1e18;
        
        html += `<p class="overview-text">`;
        html += `This transaction transfers <strong>${value.toFixed(6)} tokens</strong> `;
        html += `from <code>${txData.from.substring(0, 10)}...${txData.from.substring(34)}</code> `;
        html += `to <code>${txData.to ? txData.to.substring(0, 10) + '...' + txData.to.substring(34) : 'Contract Creation'}</code>. `;
        html += `The transaction fee is <strong>${fee.toFixed(6)} tokens</strong>.`;
        html += `</p>`;
        
        if (txData.to === '0x0000000000000000000000000000000000000000' || !txData.to) {
            html += `<p class="insight">💡 <strong>Insight:</strong> This is a contract creation transaction.</p>`;
        }
        
        if (txData.input && txData.input !== '0x' && txData.input.length > 2) {
            html += `<p class="insight">💡 <strong>Insight:</strong> This transaction includes contract call data (${(txData.input.length - 2) / 2} bytes).</p>`;
        }
        
        html += `</div>`;
    }
    
    // Risk assessment
    if (txRisk) {
        html += `<div class="explain-section risk-assessment">`;
        html += `<h4>⚠️ Risk Assessment</h4>`;
        
        const riskLevel = txRisk.score >= 0.7 ? 'High Risk' : txRisk.score >= 0.4 ? 'Medium Risk' : 'Low Risk';
        const riskColor = txRisk.score >= 0.7 ? 'risk-high' : txRisk.score >= 0.4 ? 'risk-medium' : 'risk-low';
        
        html += `<p class="risk-text ${riskColor}">`;
        html += `This transaction has a <strong>${riskLevel}</strong> rating `;
        html += `(${(txRisk.score * 100).toFixed(1)}% risk score, ${(txRisk.confidence * 100).toFixed(0)}% confidence).`;
        html += `</p>`;
        
        if (txRisk.labels && txRisk.labels.length > 0) {
            html += `<p class="risk-labels">Risk factors: ${txRisk.labels.map(l => formatRiskLabel(l)).join(', ')}</p>`;
        }
        html += `</div>`;
    }
    
    // Transaction type analysis
    if (txData) {
        html += `<div class="explain-section transaction-type">`;
        html += `<h4>🔍 Transaction Type Analysis</h4>`;
        
        if (!txData.to || txData.to === '0x0000000000000000000000000000000000000000') {
            html += `<p class="type-text">This is a <strong>contract deployment</strong> transaction. A new smart contract is being created on the blockchain.</p>`;
        } else if (txData.input && txData.input !== '0x' && txData.input.length > 2) {
            html += `<p class="type-text">This is a <strong>contract interaction</strong> transaction. It's calling a function on a smart contract.</p>`;
        } else {
            html += `<p class="type-text">This is a <strong>simple transfer</strong> transaction. It's moving tokens from one address to another.</p>`;
        }
        html += `</div>`;
    }
    
    // Recommendations
    html += `<div class="explain-section recommendations">`;
    html += `<h4>💡 Recommendations</h4>`;
    html += `<ul class="recommendations-list">`;
    
    if (txRisk && txRisk.score >= 0.7) {
        html += `<li class="recommendation-high">⚠️ <strong>High Risk Transaction:</strong> This transaction shows high-risk patterns. Verify all details carefully before confirming.</li>`;
    } else if (txRisk && txRisk.score >= 0.4) {
        html += `<li class="recommendation-medium">⚡ <strong>Medium Risk:</strong> Review transaction details and verify recipient address.</li>`;
    } else {
        html += `<li class="recommendation-low">✅ <strong>Low Risk:</strong> Transaction appears safe, but always verify recipient address.</li>`;
    }
    
    if (txData && parseInt(txData.value, 16) > 1e20) {
        html += `<li>💰 Large value transaction - double-check all details before confirming.</li>`;
    }
    
    html += `<li>🔒 Always verify the recipient address matches your intended recipient.</li>`;
    html += `<li>📋 Review transaction data carefully, especially for contract interactions.</li>`;
    html += `</ul>`;
    html += `</div>`;
    
    html += `</div>`;
    return html;
}

// Helper function to get address summary data
async function getAddressSummaryData(address) {
    const response = await fetch(`${RPC_BASE}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            jsonrpc: '2.0',
            method: 'mds_getAddressSummary',
            params: [address],
            id: 1
        })
    });
    
    const data = await response.json();
    if (data.error) {
        throw new Error(data.error.message);
    }
    
    return data.result;
}

// Helper function to get anomalies data
async function detectAnomaliesData(address) {
    const response = await fetch(`${RPC_BASE}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            jsonrpc: '2.0',
            method: 'mds_detectAnomalies',
            params: [address],
            id: 1
        })
    });
    
    const data = await response.json();
    if (data.error) {
        throw new Error(data.error.message);
    }
    
    return data.result;
}

// Helper function to fetch transaction data
async function fetchTransactionData(txHash) {
    const response = await fetch(`${RPC_BASE}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            jsonrpc: '2.0',
            method: 'eth_getTransactionByHash',
            params: [txHash],
            id: 1
        })
    });
    
    const data = await response.json();
    if (data.error || !data.result) {
        throw new Error(data.error?.message || 'Transaction not found');
    }
    
    return data.result;
}

// Auto-refresh dashboard every 5 seconds for live updates
setInterval(() => {
    loadDashboard();
    loadRecentBlocks();
    loadRecentTransactions();
    loadMevMetrics();
}, 5000);

// Account Abstraction Setup
function setupAccountAbstraction() {
    // Wallet creation
    document.getElementById('create-wallet-btn').addEventListener('click', createWallet);
    document.getElementById('wallet-type-select').addEventListener('change', updateWalletConfig);
    
    // Wallet lookup
    document.getElementById('lookup-wallet-btn').addEventListener('click', lookupWallet);
    
    // Multi-sig transactions
    document.getElementById('view-multisig-txs-btn').addEventListener('click', viewMultisigTransactions);
    
    // Recovery status
    document.getElementById('view-recovery-btn').addEventListener('click', viewRecoveryStatus);
    
    // Batch transactions
    document.getElementById('view-batch-btn').addEventListener('click', viewBatchStatus);
}

// Update wallet configuration UI based on wallet type
function updateWalletConfig() {
    const walletType = document.getElementById('wallet-type-select').value;
    const configDiv = document.getElementById('wallet-config');
    
    let html = '';
    if (walletType === 'multisig') {
        html = `
            <input type="text" id="multisig-signers-input" placeholder="Signer addresses (comma-separated, 0x...)" class="address-input" style="margin-top: 0.5rem;">
            <input type="number" id="multisig-threshold-input" placeholder="Threshold (e.g., 2)" min="1" class="address-input" style="margin-top: 0.5rem;">
        `;
    } else if (walletType === 'socialRecovery') {
        html = `
            <input type="text" id="recovery-guardians-input" placeholder="Guardian addresses (comma-separated, 0x...)" class="address-input" style="margin-top: 0.5rem;">
            <input type="number" id="recovery-threshold-input" placeholder="Recovery threshold (e.g., 2)" min="1" class="address-input" style="margin-top: 0.5rem;">
        `;
    } else if (walletType === 'spendingLimit') {
        html = `
            <input type="text" id="daily-limit-input" placeholder="Daily limit (in base units)" class="address-input" style="margin-top: 0.5rem;">
            <input type="text" id="weekly-limit-input" placeholder="Weekly limit (in base units)" class="address-input" style="margin-top: 0.5rem;">
            <input type="text" id="monthly-limit-input" placeholder="Monthly limit (in base units)" class="address-input" style="margin-top: 0.5rem;">
        `;
    }
    
    configDiv.innerHTML = html;
}

// Create a new wallet
async function createWallet() {
    const walletType = document.getElementById('wallet-type-select').value;
    const owner = document.getElementById('wallet-owner-input').value.trim();
    const resultDiv = document.getElementById('wallet-result');
    
    if (!owner || !owner.startsWith('0x')) {
        resultDiv.innerHTML = '<p class="error">Please enter a valid owner address</p>';
        return;
    }
    
    resultDiv.innerHTML = '<p>Creating wallet...</p>';
    
    try {
        let config = {};
        
        if (walletType === 'multisig') {
            const signersInput = document.getElementById('multisig-signers-input').value.trim();
            const threshold = parseInt(document.getElementById('multisig-threshold-input').value);
            const signers = signersInput.split(',').map(s => s.trim()).filter(s => s);
            
            if (signers.length === 0 || !threshold) {
                resultDiv.innerHTML = '<p class="error">Please provide signers and threshold</p>';
                return;
            }
            
            config = { signers, threshold };
        } else if (walletType === 'socialRecovery') {
            const guardiansInput = document.getElementById('recovery-guardians-input').value.trim();
            const recoveryThreshold = parseInt(document.getElementById('recovery-threshold-input').value);
            const guardians = guardiansInput.split(',').map(g => g.trim()).filter(g => g);
            
            if (guardians.length === 0 || !recoveryThreshold) {
                resultDiv.innerHTML = '<p class="error">Please provide guardians and recovery threshold</p>';
                return;
            }
            
            config = { guardians, recoveryThreshold };
        } else if (walletType === 'spendingLimit') {
            const dailyLimit = document.getElementById('daily-limit-input').value.trim();
            const weeklyLimit = document.getElementById('weekly-limit-input').value.trim();
            const monthlyLimit = document.getElementById('monthly-limit-input').value.trim();
            
            config = {
                spendingLimits: {
                    dailyLimit: dailyLimit || '0',
                    weeklyLimit: weeklyLimit || '0',
                    monthlyLimit: monthlyLimit || '0'
                }
            };
        }
        
        const result = await rpcCall('mds_createWallet', {
            owner: owner,
            walletType: walletType,
            config: config
        });
        
        resultDiv.innerHTML = `
            <div class="success" style="padding: 1rem; background: #065f46; border-radius: 8px; margin-top: 1rem;">
                <h4>✅ Wallet Created Successfully</h4>
                <p><strong>Wallet Address:</strong> <code>${result.walletAddress}</code></p>
                <p><strong>Owner:</strong> <code>${result.owner}</code></p>
                <p><strong>Type:</strong> ${result.walletType}</p>
            </div>
        `;
    } catch (error) {
        resultDiv.innerHTML = `<p class="error">Error creating wallet: ${error.message}</p>`;
    }
}

// Lookup wallet information
async function lookupWallet() {
    const walletAddress = document.getElementById('wallet-address-input').value.trim();
    const detailsDiv = document.getElementById('wallet-details');
    
    if (!walletAddress || !walletAddress.startsWith('0x')) {
        detailsDiv.innerHTML = '<p class="error">Please enter a valid wallet address</p>';
        return;
    }
    
    detailsDiv.innerHTML = '<p>Loading wallet information...</p>';
    
    try {
        const wallet = await rpcCall('mds_getWallet', [walletAddress]);
        
        let detailsHtml = `
            <div style="padding: 1rem; background: #1e293b; border-radius: 8px; margin-top: 1rem;">
                <h4>Wallet Information</h4>
                <p><strong>Address:</strong> <code>${wallet.walletAddress}</code></p>
                <p><strong>Owner:</strong> <code>${wallet.owner}</code></p>
                <p><strong>Type:</strong> ${wallet.walletType}</p>
        `;
        
        if (wallet.signers) {
            detailsHtml += `
                <p><strong>Signers:</strong> ${wallet.signers.length}</p>
                <p><strong>Threshold:</strong> ${wallet.threshold}</p>
                <div style="margin-top: 0.5rem;">
                    <strong>Signer Addresses:</strong>
                    <ul style="margin-left: 1.5rem; margin-top: 0.5rem;">
                        ${wallet.signers.map(s => `<li><code>${s}</code></li>`).join('')}
                    </ul>
                </div>
            `;
        }
        
        if (wallet.guardians) {
            detailsHtml += `
                <p><strong>Guardians:</strong> ${wallet.guardians.length}</p>
                <p><strong>Recovery Threshold:</strong> ${wallet.recoveryThreshold}</p>
            `;
        }
        
        detailsHtml += `</div>`;
        detailsDiv.innerHTML = detailsHtml;
    } catch (error) {
        detailsDiv.innerHTML = `<p class="error">Error loading wallet: ${error.message}</p>`;
    }
}

// View multi-sig transactions
async function viewMultisigTransactions() {
    const walletAddress = document.getElementById('multisig-wallet-input').value.trim();
    const txsDiv = document.getElementById('multisig-transactions');
    
    if (!walletAddress || !walletAddress.startsWith('0x')) {
        txsDiv.innerHTML = '<p class="error">Please enter a valid wallet address</p>';
        return;
    }
    
    txsDiv.innerHTML = '<p>Loading pending transactions...</p>';
    
    try {
        const pending = await rpcCall('mds_getPendingMultisigTransactions', [walletAddress]);
        
        if (!pending || pending.length === 0) {
            txsDiv.innerHTML = '<p>No pending multi-sig transactions</p>';
            return;
        }
        
        let html = '<div style="margin-top: 1rem;">';
        pending.forEach(tx => {
            html += `
                <div style="padding: 1rem; background: #1e293b; border-radius: 8px; margin-bottom: 1rem;">
                    <p><strong>Transaction Hash:</strong> <code>${tx.txHash}</code></p>
                    <p><strong>To:</strong> <code>${tx.to}</code></p>
                    <p><strong>Value:</strong> ${tx.value}</p>
                    <p><strong>Signatures Collected:</strong> ${tx.signaturesCollected || 0} / ${tx.threshold || 'N/A'}</p>
                    <p><strong>Status:</strong> ${tx.status || 'pending'}</p>
                </div>
            `;
        });
        html += '</div>';
        txsDiv.innerHTML = html;
    } catch (error) {
        txsDiv.innerHTML = `<p class="error">Error loading transactions: ${error.message}</p>`;
    }
}

// View recovery status
async function viewRecoveryStatus() {
    const walletAddress = document.getElementById('recovery-wallet-input').value.trim();
    const statusDiv = document.getElementById('recovery-status');
    
    if (!walletAddress || !walletAddress.startsWith('0x')) {
        statusDiv.innerHTML = '<p class="error">Please enter a valid wallet address</p>';
        return;
    }
    
    statusDiv.innerHTML = '<p>Loading recovery status...</p>';
    
    try {
        const status = await rpcCall('mds_getRecoveryStatus', { walletAddress: walletAddress });
        
        const statusClass = status.status === 'ready' ? 'success' : status.status === 'approved' ? 'warning' : 'info';
        statusDiv.innerHTML = `
            <div style="padding: 1rem; background: #1e293b; border-radius: 8px; margin-top: 1rem;">
                <h4>Recovery Status</h4>
                <p><strong>Status:</strong> <span class="${statusClass}">${status.status}</span></p>
                <p><strong>New Owner:</strong> <code>${status.newOwner}</code></p>
                <p><strong>Guardians:</strong> ${status.guardians.length}</p>
                <p><strong>Recovery Threshold:</strong> ${status.recoveryThreshold}</p>
                <p><strong>Approvals:</strong> ${status.approvalCount} / ${status.recoveryThreshold}</p>
                <p><strong>Threshold Met:</strong> ${status.thresholdMet ? 'Yes' : 'No'}</p>
                <p><strong>Ready to Complete:</strong> ${status.isReady ? 'Yes' : 'No'}</p>
                ${status.approvals && status.approvals.length > 0 ? `
                    <div style="margin-top: 1rem;">
                        <strong>Approvals:</strong>
                        <ul style="margin-left: 1.5rem; margin-top: 0.5rem;">
                            ${status.approvals.map(a => `<li><code>${a.guardian}</code> - ${new Date(a.approvedAt * 1000).toLocaleString()}</li>`).join('')}
                        </ul>
                    </div>
                ` : ''}
            </div>
        `;
    } catch (error) {
        if (error.message.includes('not found')) {
            statusDiv.innerHTML = '<p>No active recovery request for this wallet</p>';
        } else {
            statusDiv.innerHTML = `<p class="error">Error loading recovery status: ${error.message}</p>`;
        }
    }
}

// View batch status
async function viewBatchStatus() {
    const batchId = document.getElementById('batch-id-input').value.trim();
    const statusDiv = document.getElementById('batch-status');
    
    if (!batchId || !batchId.startsWith('0x')) {
        statusDiv.innerHTML = '<p class="error">Please enter a valid batch ID</p>';
        return;
    }
    
    statusDiv.innerHTML = '<p>Loading batch status...</p>';
    
    try {
        const status = await rpcCall('mds_getBatchStatus', { batchId: batchId });
        
        const statusClass = status.status === 'completed' ? 'success' : status.status === 'failed' ? 'error' : 'info';
        statusDiv.innerHTML = `
            <div style="padding: 1rem; background: #1e293b; border-radius: 8px; margin-top: 1rem;">
                <h4>Batch Transaction Status</h4>
                <p><strong>Batch ID:</strong> <code>${status.batchId}</code></p>
                <p><strong>Wallet:</strong> <code>${status.walletAddress}</code></p>
                <p><strong>Status:</strong> <span class="${statusClass}">${status.status}</span></p>
                <p><strong>Operations:</strong> ${status.completedOperations} / ${status.operationCount}</p>
                <p><strong>Gas Used:</strong> ${parseInt(status.gasUsed, 16).toLocaleString()}</p>
                <p><strong>Gas Limit:</strong> ${parseInt(status.gasLimit, 16).toLocaleString()}</p>
                ${status.results && status.results.length > 0 ? `
                    <div style="margin-top: 1rem;">
                        <strong>Operation Results:</strong>
                        <ul style="margin-left: 1.5rem; margin-top: 0.5rem;">
                            ${status.results.map(r => `
                                <li>
                                    Operation ${r.operationIndex}: 
                                    ${r.success ? '✅ Success' : '❌ Failed'}
                                    ${r.error ? ` - ${r.error}` : ''}
                                    (Gas: ${parseInt(r.gasUsed, 16).toLocaleString()})
                                </li>
                            `).join('')}
                        </ul>
                    </div>
                ` : ''}
            </div>
        `;
    } catch (error) {
        statusDiv.innerHTML = `<p class="error">Error loading batch status: ${error.message}</p>`;
    }
}
