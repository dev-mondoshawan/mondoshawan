// Mondoshawan Block Explorer - Frontend Application

const API_BASE = 'http://localhost:8081/api';
const RPC_BASE = 'http://localhost:8545';

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
});

// Load dashboard statistics
async function loadDashboard() {
    try {
        const [networkStats, chainStats] = await Promise.all([
            fetch(`${API_BASE}/stats/network`).then(r => r.json()),
            fetch(`${API_BASE}/stats/chain`).then(r => r.json())
        ]);

        document.getElementById('total-blocks').textContent = networkStats.total_blocks || 0;
        document.getElementById('total-transactions').textContent = networkStats.total_transactions || 0;
        document.getElementById('peers-connected').textContent = networkStats.peers_connected || 0;
        document.getElementById('tps').textContent = networkStats.transactions_per_second?.toFixed(2) || '0.00';
    } catch (error) {
        console.error('Error loading dashboard:', error);
        document.getElementById('total-blocks').textContent = 'Error';
    }
}

// Load recent blocks
async function loadRecentBlocks() {
    try {
        const response = await fetch(`${API_BASE}/blocks/recent?limit=10`);
        const blocks = await response.json();
        
        const blocksList = document.getElementById('blocks-list');
        
        if (blocks.length === 0) {
            blocksList.innerHTML = '<p class="loading">No blocks found</p>';
            return;
        }
        
        blocksList.innerHTML = blocks.map(block => `
            <div class="block-item">
                <h3>Block #${block.number}</h3>
                <p><strong>Hash:</strong> <code>${block.hash}</code></p>
                <p><strong>Timestamp:</strong> ${new Date(block.timestamp * 1000).toLocaleString()}</p>
                <p><strong>Transactions:</strong> ${block.transaction_count}</p>
            </div>
        `).join('');
    } catch (error) {
        console.error('Error loading blocks:', error);
        document.getElementById('blocks-list').innerHTML = '<p class="error">Error loading blocks</p>';
    }
}

// Load recent transactions
async function loadRecentTransactions() {
    try {
        const response = await fetch(`${API_BASE}/transactions/recent?limit=10`);
        const transactions = await response.json();
        
        const transactionsList = document.getElementById('transactions-list');
        
        if (transactions.length === 0) {
            transactionsList.innerHTML = '<p class="loading">No transactions found</p>';
            return;
        }
        
        // Load risk scores for transactions (async, will update after initial render)
        transactionsList.innerHTML = transactions.map(tx => {
            const shardInfo = tx.fromShard !== undefined ? `
                <p><strong>Shard:</strong> ${tx.fromShard} → ${tx.toShard} ${tx.isCrossShard ? '<span class="cross-shard-badge">Cross-Shard</span>' : ''}</p>
            ` : '';
            return `
            <div class="transaction-item" data-tx-hash="${tx.hash}">
                <h3>Transaction</h3>
                <p><strong>Hash:</strong> <code>${tx.hash}</code></p>
                <p><strong>From:</strong> <code>${tx.from}</code></p>
                <p><strong>To:</strong> <code>${tx.to || 'N/A'}</code></p>
                ${shardInfo}
                <p><strong>Value:</strong> ${tx.value}</p>
                <p><strong>Status:</strong> ${tx.status}</p>
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
                        ${risk.labels.length > 0 ? `
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
                    const response = await fetch(`${API_BASE}/blocks/${query}`);
                    if (response.ok) {
                        const block = await response.json();
                        await displayBlock(block);
                        return;
                    }
                } catch (e) {}
                
                // Try transaction
                try {
                    const response = await fetch(`${API_BASE}/transactions/${query}`);
                    if (response.ok) {
                        const tx = await response.json();
                        await displayTransaction(tx);
                        await explainTransaction(tx.hash);
                        return;
                    }
                } catch (e) {}
            } else if (query.startsWith('0x') && query.length === 42) {
                // Likely an address
                try {
                    const response = await fetch(`${API_BASE}/addresses/${query}`);
                    if (response.ok) {
                        const address = await response.json();
                        await displayAddress(address);
                        await explainAddress(address.address || address);
                        return;
                    }
                } catch (e) {}
                
                // If API doesn't have address info, still show with risk score
                await displayAddress(query);
                await explainAddress(query);
                return;
            } else if (!isNaN(query)) {
                // Block number
                const response = await fetch(`${API_BASE}/blocks/${query}`);
                if (response.ok) {
                    const block = await response.json();
                    await displayBlock(block);
                    return;
                }
            }
            
            // General search
            const response = await fetch(`${API_BASE}/search?q=${encodeURIComponent(query)}`);
            const results = await response.json();
            displaySearchResults(results);
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
    
    addressInfo.innerHTML = `
        <h3>Transaction Details</h3>
        <p><strong>Hash:</strong> <code>${txHash}</code></p>
        <p><strong>From:</strong> <code>${tx.from || 'N/A'}</code></p>
        <p><strong>To:</strong> <code>${tx.to || 'N/A'}</code></p>
        <p><strong>Value:</strong> ${tx.value || 'N/A'}</p>
        <p><strong>Fee:</strong> ${tx.fee || 'N/A'}</p>
        <p><strong>Status:</strong> ${tx.status || 'N/A'}</p>
        <p><strong>Block:</strong> ${tx.block_number || 'Pending'}</p>
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
    if (shardId !== null) {
        shardHtml = `
            <div class="shard-section">
                <h4>🔷 Sharding Information</h4>
                <p><strong>Shard ID:</strong> ${shardId}</p>
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

// Auto-refresh dashboard every 30 seconds
setInterval(() => {
    loadDashboard();
    loadRecentBlocks();
    loadRecentTransactions();
    loadMevMetrics();
}, 30000);

