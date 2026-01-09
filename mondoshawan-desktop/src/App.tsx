import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type NodeStatus = {
  height: number;
  tx_count: number;
  peer_count: number;
  is_mining: boolean;
};

type MiningStatus = {
  is_mining: boolean;
  pending_txs: number;
  streams: {
    streamA: { block_time_ms: number; max_txs: number; reward: string };
    streamB: { block_time_ms: number; max_txs: number; reward: string };
    streamC: { block_time_ms: number; max_txs: number; reward: string };
  };
};

type Block = {
  number: string;
  hash: string;
  timestamp: string;
  transactions: any[];
  stream_type?: string;
};

type DagStats = {
  total_blocks: number;
  blue_blocks: number;
  red_blocks: number;
  total_transactions: number;
  avg_txs_per_block: number;
};

type ShardStats = {
  shard_count: number;
  shards: Array<{
    shard_id: number;
    block_count: number;
    transaction_pool_size: number;
    cross_shard_outgoing: number;
    cross_shard_incoming: number;
  }>;
};

function App() {
  const [activeTab, setActiveTab] = useState<"dashboard" | "wallet" | "send" | "explorer" | "metrics">("dashboard");
  
  // Node & mining state
  const [nodeStatus, setNodeStatus] = useState<NodeStatus | null>(null);
  const [miningStatus, setMiningStatus] = useState<MiningStatus | null>(null);
  
  // Wallet state
  const [walletAddress, setWalletAddress] = useState<string>("");
  const [walletBalanceHex, setWalletBalanceHex] = useState<string | null>(null);
  const [walletNonceHex, setWalletNonceHex] = useState<string | null>(null);
  
  // Send state
  const [walletAddr, setWalletAddr] = useState<string | null>(null);
  const [sendTo, setSendTo] = useState<string>("");
  const [sendValue, setSendValue] = useState<string>("");
  const [sendFee, setSendFee] = useState<string>("");
  const [txHash, setTxHash] = useState<string | null>(null);
  
  // Explorer state
  const [blocks, setBlocks] = useState<Block[]>([]);
  const [dagStats, setDagStats] = useState<DagStats | null>(null);
  
  // Metrics state
  const [tps, setTps] = useState<string | null>(null);
  const [shardStats, setShardStats] = useState<ShardStats | null>(null);
  
  // Common state
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function refresh() {
    setLoading(true);
    setError(null);
    try {
      const node = await invoke<NodeStatus>("get_node_status");
      const mining = await invoke<MiningStatus>("get_mining_status");
      setNodeStatus(node);
      setMiningStatus(mining);
    } catch (e: any) {
      setError(e?.toString?.() ?? "Failed to fetch status");
    } finally {
      setLoading(false);
    }
  }

  async function startMining() {
    setLoading(true);
    setError(null);
    try {
      await invoke("start_mining");
      await refresh();
    } catch (e: any) {
      setError(e?.toString?.() ?? "Failed to start mining");
    } finally {
      setLoading(false);
    }
  }

  async function stopMining() {
    setLoading(true);
    setError(null);
    try {
      await invoke("stop_mining");
      await refresh();
    } catch (e: any) {
      setError(e?.toString?.() ?? "Failed to stop mining");
    } finally {
      setLoading(false);
    }
  }

  async function loadWallet() {
    if (!walletAddress) {
      setError("Please enter an address.");
      return;
    }
    setLoading(true);
    setError(null);
    try {
      const balanceHex = await invoke<string>("get_balance", {
        address: walletAddress,
      });
      const nonceHex = await invoke<string>("get_nonce", {
        address: walletAddress,
      });
      setWalletBalanceHex(balanceHex);
      setWalletNonceHex(nonceHex);
    } catch (e: any) {
      setError(e?.toString?.() ?? "Failed to load wallet");
    } finally {
      setLoading(false);
    }
  }

  async function createNewKey() {
    setLoading(true);
    setError(null);
    try {
      const addr = await invoke<string>("create_new_key");
      setWalletAddr(addr);
    } catch (e: any) {
      setError(e?.toString?.() ?? "Failed to create key");
    } finally {
      setLoading(false);
    }
  }

  async function loadWalletAddress() {
    setLoading(true);
    setError(null);
    try {
      const addr = await invoke<string>("get_wallet_address");
      setWalletAddr(addr);
    } catch (e: any) {
      setError(e?.toString?.() ?? "No key loaded");
    } finally {
      setLoading(false);
    }
  }

  async function sendTx() {
    if (!sendTo || !sendValue || !sendFee) {
      setError("Fill in all fields");
      return;
    }
    setLoading(true);
    setError(null);
    setTxHash(null);
    try {
      // Convert MSHW to base units (1 MSHW = 10^18)
      const valueBigInt = BigInt(Math.floor(parseFloat(sendValue) * 1e18));
      const feeBigInt = BigInt(Math.floor(parseFloat(sendFee) * 1e18));

      const hash = await invoke<string>("send_transaction", {
        toAddress: sendTo,
        valueHex: `0x${valueBigInt.toString(16)}`,
        feeHex: `0x${feeBigInt.toString(16)}`,
      });
      setTxHash(hash);
    } catch (e: any) {
      setError(e?.toString?.() ?? "Failed to send transaction");
    } finally {
      setLoading(false);
    }
  }

  function formatBalance(balanceHex: string | null): { raw: string; mshw: string } {
    if (!balanceHex) {
      return { raw: "-", mshw: "-" };
    }
    try {
      const v = BigInt(balanceHex);
      const denom = 10n ** 18n;
      const whole = v / denom;
      const frac = v % denom;
      const fracStr = (frac / (10n ** 12n)).toString().padStart(6, "0");
      return { raw: balanceHex, mshw: `${whole.toString()}.${fracStr}` };
    } catch {
      return { raw: balanceHex, mshw: "?" };
    }
  }

  async function refreshExplorer() {
    setLoading(true);
    setError(null);
    try {
      const [blocksData, statsData] = await Promise.all([
        invoke<any>("get_latest_blocks", { count: 10 }),
        invoke<DagStats>("get_dag_stats"),
      ]);
      setBlocks(blocksData);
      setDagStats(statsData);
    } catch (e: any) {
      setError(e?.toString?.() ?? "Failed to fetch explorer data");
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    if (activeTab === "explorer") {
      refreshExplorer();
      const interval = setInterval(refreshExplorer, 10000);
      return () => clearInterval(interval);
    }
  }, [activeTab]);

  async function refreshMetrics() {
    setLoading(true);
    setError(null);
    try {
      const [tpsData, dagData, shardData] = await Promise.all([
        invoke<any>("get_tps"),
        invoke<DagStats>("get_dag_stats"),
        invoke<ShardStats>("get_shard_stats"),
      ]);
      setTps(tpsData);
      setDagStats(dagData);
      setShardStats(shardData);
    } catch (e: any) {
      setError(e?.toString?.() ?? "Failed to fetch metrics");
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    if (activeTab === "metrics") {
      refreshMetrics();
      const interval = setInterval(refreshMetrics, 5000);
      return () => clearInterval(interval);
    }
  }, [activeTab]);

  useEffect(() => {
    refresh();
    const interval = setInterval(refresh, 5000);
    return () => clearInterval(interval);
  }, []);

  const miningOn = nodeStatus?.is_mining ?? false;

  return (
    <div
      style={{
        minHeight: "100vh",
        padding: "1.5rem",
        fontFamily: "system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI'",
        background: "#020617",
        color: "#e5e7eb",
      }}
    >
      <h1 style={{ fontSize: "1.8rem", marginBottom: "0.5rem" }}>
        Mondoshawan Desktop
      </h1>
      <p style={{ opacity: 0.8, marginBottom: "1.5rem" }}>
        All-in-one node and mining dashboard for the Mondoshawan Protocol.
      </p>

      <div style={{ marginBottom: "1rem", display: "flex", gap: "0.5rem" }}>
        <button
          onClick={() => setActiveTab("dashboard")}
          style={{
            padding: "0.4rem 0.8rem",
            borderRadius: 6,
            border: "none",
            cursor: "pointer",
            background: activeTab === "dashboard" ? "#4f46e5" : "#111827",
            color: "#e5e7eb",
          }}
        >
          Dashboard
        </button>
        <button
          onClick={() => setActiveTab("wallet")}
          style={{
            padding: "0.4rem 0.8rem",
            borderRadius: 6,
            border: "none",
            cursor: "pointer",
            background: activeTab === "wallet" ? "#4f46e5" : "#111827",
            color: "#e5e7eb",
          }}
        >
          Wallet
        </button>
        <button
          onClick={() => setActiveTab("send")}
          style={{
            padding: "0.4rem 0.8rem",
            borderRadius: 6,
            border: "none",
            cursor: "pointer",
            background: activeTab === "send" ? "#4f46e5" : "#111827",
            color: "#e5e7eb",
          }}
        >
          Send
        </button>
        <button
          onClick={() => setActiveTab("explorer")}
          style={{
            padding: "0.4rem 0.8rem",
            borderRadius: 6,
            border: "none",
            cursor: "pointer",
            background: activeTab === "explorer" ? "#4f46e5" : "#111827",
            color: "#e5e7eb",
          }}
        >
          Explorer
        </button>
        <button
          onClick={() => setActiveTab("metrics")}
          style={{
            padding: "0.4rem 0.8rem",
            borderRadius: 6,
            border: "none",
            cursor: "pointer",
            background: activeTab === "metrics" ? "#4f46e5" : "#111827",
            color: "#e5e7eb",
          }}
        >
          Metrics
        </button>
      </div>

      {error && (
        <div
          style={{
            background: "#7f1d1d",
            padding: "0.75rem 1rem",
            borderRadius: 6,
            marginBottom: "1rem",
          }}
        >
          <strong>Error:</strong> {error}
        </div>
      )}

      {activeTab === "dashboard" && (
        <>
          <section
            style={{
              marginBottom: "1.5rem",
              padding: "1rem",
              borderRadius: 8,
              background: "#0f172a",
            }}
          >
            <h2 style={{ fontSize: "1.2rem", marginBottom: "0.5rem" }}>
              Node Status
            </h2>
            {nodeStatus ? (
              <ul style={{ listStyle: "none", paddingLeft: 0 }}>
                <li>
                  <strong>Height</strong>: {nodeStatus.height}
                </li>
                <li>
                  <strong>Total Transactions</strong>: {nodeStatus.tx_count}
                </li>
                <li>
                  <strong>Connected Peers</strong>: {nodeStatus.peer_count}
                </li>
                <li>
                  <strong>Mining</strong>: {miningOn ? "ON" : "OFF"}
                </li>
              </ul>
            ) : (
              <p>Connecting to local node...</p>
            )}
            <button
              onClick={refresh}
              disabled={loading}
              style={{
                marginTop: "0.75rem",
                padding: "0.5rem 1rem",
                borderRadius: 6,
                border: "none",
                background: "#4f46e5",
                color: "white",
                cursor: "pointer",
              }}
            >
              {loading ? "Refreshing..." : "Refresh"}
            </button>
          </section>

          <section
            style={{
              padding: "1rem",
              borderRadius: 8,
              background: "#0f172a",
            }}
          >
            <h2 style={{ fontSize: "1.2rem", marginBottom: "0.5rem" }}>
              TriStream Mining
            </h2>
            <div style={{ marginBottom: "0.75rem" }}>
              <button
                onClick={startMining}
                disabled={loading || miningOn}
                style={{
                  padding: "0.5rem 1rem",
                  borderRadius: 6,
                  border: "none",
                  background: miningOn ? "#4b5563" : "#10b981",
                  color: "white",
                  cursor: miningOn ? "default" : "pointer",
                  marginRight: "0.5rem",
                }}
              >
                Start Mining
              </button>
              <button
                onClick={stopMining}
                disabled={loading || !miningOn}
                style={{
                  padding: "0.5rem 1rem",
                  borderRadius: 6,
                  border: "none",
                  background: !miningOn ? "#4b5563" : "#b91c1c",
                  color: "white",
                  cursor: !miningOn ? "default" : "pointer",
                }}
              >
                Stop Mining
              </button>
            </div>

            {miningStatus && (
              <>
                <p>
                  <strong>Pending Transactions</strong>:{" "}
                  {miningStatus.pending_txs}
                </p>
                <h3 style={{ marginTop: "0.75rem", marginBottom: "0.25rem" }}>
                  Streams
                </h3>
                <ul style={{ listStyle: "none", paddingLeft: 0, fontSize: "0.95rem" }}>
                  <li>
                    <strong>Stream A</strong>: {miningStatus.streams.streamA.max_txs} tx /
                    {miningStatus.streams.streamA.block_time_ms} ms, reward{" "}
                    {miningStatus.streams.streamA.reward}
                  </li>
                  <li>
                    <strong>Stream B</strong>: {miningStatus.streams.streamB.max_txs} tx /
                    {miningStatus.streams.streamB.block_time_ms} ms, reward{" "}
                    {miningStatus.streams.streamB.reward}
                  </li>
                  <li>
                    <strong>Stream C</strong>: {miningStatus.streams.streamC.max_txs} tx /
                    {miningStatus.streams.streamC.block_time_ms} ms, reward{" "}
                    {miningStatus.streams.streamC.reward}
                  </li>
                </ul>
              </>
            )}
          </section>
        </>
      )}

      {activeTab === "wallet" && (
        <section
          style={{
            padding: "1rem",
            borderRadius: 8,
            background: "#0f172a",
          }}
        >
          <h2 style={{ fontSize: "1.2rem", marginBottom: "0.5rem" }}>
            Wallet
          </h2>
          <div style={{ marginBottom: "0.75rem" }}>
            <label style={{ display: "block", marginBottom: "0.25rem" }}>
              Address (0x…)
            </label>
            <input
              type="text"
              value={walletAddress}
              onChange={(e) => setWalletAddress(e.target.value)}
              placeholder="0x..."
              style={{
                width: "100%",
                padding: "0.5rem",
                borderRadius: 6,
                border: "1px solid #4b5563",
                background: "#020617",
                color: "#e5e7eb",
              }}
            />
          </div>
          <button
            onClick={loadWallet}
            disabled={loading || !walletAddress}
            style={{
              padding: "0.5rem 1rem",
              borderRadius: 6,
              border: "none",
              background: "#4f46e5",
              color: "white",
              cursor: walletAddress ? "pointer" : "default",
              marginBottom: "1rem",
            }}
          >
            {loading ? "Loading..." : "Load Wallet"}
          </button>

          {walletBalanceHex && walletNonceHex && (
            <div
              style={{
                marginTop: "0.5rem",
                padding: "0.75rem",
                borderRadius: 6,
                background: "#020617",
              }}
            >
              {(() => {
                const { raw, mshw } = formatBalance(walletBalanceHex);
                return (
                  <>
                    <p>
                      <strong>Balance (raw)</strong>: {raw}
                    </p>
                    <p>
                      <strong>Balance (MSHW)</strong>: {mshw}
                    </p>
                  </>
                );
              })()}
              <p>
                <strong>Nonce</strong>: {walletNonceHex}
              </p>
            </div>
          )}
        </section>
      )}

      {activeTab === "send" && (
        <section
          style={{
            padding: "1rem",
            borderRadius: 8,
            background: "#0f172a",
          }}
        >
          <h2 style={{ fontSize: "1.2rem", marginBottom: "0.5rem" }}>
            Send Transaction
          </h2>

          {!walletAddr && (
            <div style={{ marginBottom: "1rem" }}>
              <p>No wallet loaded.</p>
              <button
                onClick={createNewKey}
                disabled={loading}
                style={{
                  padding: "0.5rem 1rem",
                  borderRadius: 6,
                  border: "none",
                  background: "#10b981",
                  color: "white",
                  cursor: "pointer",
                  marginRight: "0.5rem",
                }}
              >
                {loading ? "Creating..." : "Create New Wallet"}
              </button>
              <button
                onClick={loadWalletAddress}
                disabled={loading}
                style={{
                  padding: "0.5rem 1rem",
                  borderRadius: 6,
                  border: "none",
                  background: "#4f46e5",
                  color: "white",
                  cursor: "pointer",
                }}
              >
                Load Existing
              </button>
            </div>
          )}

          {walletAddr && (
            <>
              <p>
                <strong>Your Address</strong>: {walletAddr}
              </p>
              <div style={{ marginBottom: "0.75rem" }}>
                <label style={{ display: "block", marginBottom: "0.25rem" }}>
                  To Address (0x...)
                </label>
                <input
                  type="text"
                  value={sendTo}
                  onChange={(e) => setSendTo(e.target.value)}
                  placeholder="0x..."
                  style={{
                    width: "100%",
                    padding: "0.5rem",
                    borderRadius: 6,
                    border: "1px solid #4b5563",
                    background: "#020617",
                    color: "#e5e7eb",
                  }}
                />
              </div>
              <div style={{ marginBottom: "0.75rem" }}>
                <label style={{ display: "block", marginBottom: "0.25rem" }}>
                  Value (MSHW)
                </label>
                <input
                  type="text"
                  value={sendValue}
                  onChange={(e) => setSendValue(e.target.value)}
                  placeholder="0.1"
                  style={{
                    width: "100%",
                    padding: "0.5rem",
                    borderRadius: 6,
                    border: "1px solid #4b5563",
                    background: "#020617",
                    color: "#e5e7eb",
                  }}
                />
              </div>
              <div style={{ marginBottom: "0.75rem" }}>
                <label style={{ display: "block", marginBottom: "0.25rem" }}>
                  Fee (MSHW)
                </label>
                <input
                  type="text"
                  value={sendFee}
                  onChange={(e) => setSendFee(e.target.value)}
                  placeholder="0.001"
                  style={{
                    width: "100%",
                    padding: "0.5rem",
                    borderRadius: 6,
                    border: "1px solid #4b5563",
                    background: "#020617",
                    color: "#e5e7eb",
                  }}
                />
              </div>
              <button
                onClick={sendTx}
                disabled={loading}
                style={{
                  padding: "0.5rem 1rem",
                  borderRadius: 6,
                  border: "none",
                  background: "#10b981",
                  color: "white",
                  cursor: "pointer",
                }}
              >
                {loading ? "Sending..." : "Send Transaction"}
              </button>

              {txHash && (
                <div
                  style={{
                    marginTop: "1rem",
                    padding: "0.75rem",
                    borderRadius: 6,
                    background: "#020617",
                  }}
                >
                  <p>
                    <strong>Transaction Hash</strong>: {txHash}
                  </p>
                </div>
              )}
            </>
          )}
        </section>
      )}

      {activeTab === "explorer" && (
        <>
          <section
            style={{
              marginBottom: "1.5rem",
              padding: "1rem",
              borderRadius: 8,
              background: "#0f172a",
            }}
          >
            <h2 style={{ fontSize: "1.2rem", marginBottom: "0.5rem" }}>
              DAG Statistics
            </h2>
            {dagStats ? (
              <div style={{ display: "grid", gridTemplateColumns: "repeat(2, 1fr)", gap: "0.5rem" }}>
                <div>
                  <strong>Total Blocks</strong>: {dagStats.total_blocks}
                </div>
                <div>
                  <strong>Blue Blocks</strong>: {dagStats.blue_blocks}
                </div>
                <div>
                  <strong>Red Blocks</strong>: {dagStats.red_blocks}
                </div>
                <div>
                  <strong>Total Transactions</strong>: {dagStats.total_transactions}
                </div>
                <div style={{ gridColumn: "span 2" }}>
                  <strong>Avg Txs/Block</strong>: {dagStats.avg_txs_per_block.toFixed(2)}
                </div>
              </div>
            ) : (
              <p>Loading DAG stats...</p>
            )}
            <button
              onClick={refreshExplorer}
              disabled={loading}
              style={{
                marginTop: "0.75rem",
                padding: "0.5rem 1rem",
                borderRadius: 6,
                border: "none",
                background: "#4f46e5",
                color: "white",
                cursor: "pointer",
              }}
            >
              {loading ? "Refreshing..." : "Refresh"}
            </button>
          </section>

          <section
            style={{
              padding: "1rem",
              borderRadius: 8,
              background: "#0f172a",
            }}
          >
            <h2 style={{ fontSize: "1.2rem", marginBottom: "0.5rem" }}>
              Recent Blocks
            </h2>
            {blocks.length > 0 ? (
              <div style={{ overflowX: "auto" }}>
                {blocks.map((block) => (
                  <div
                    key={block.hash}
                    style={{
                      padding: "0.75rem",
                      marginBottom: "0.5rem",
                      borderRadius: 6,
                      background: "#020617",
                      fontSize: "0.9rem",
                    }}
                  >
                    <div>
                      <strong>Block #{parseInt(block.number, 16)}</strong>
                      {block.stream_type && (
                        <span style={{ marginLeft: "0.5rem", opacity: 0.7 }}>
                          Stream: {block.stream_type}
                        </span>
                      )}
                    </div>
                    <div style={{ opacity: 0.8, fontSize: "0.85rem", marginTop: "0.25rem" }}>
                      Hash: {block.hash.substring(0, 20)}...
                    </div>
                    <div style={{ opacity: 0.8, fontSize: "0.85rem" }}>
                      Transactions: {block.transactions.length}
                    </div>
                    <div style={{ opacity: 0.8, fontSize: "0.85rem" }}>
                      Time: {new Date(parseInt(block.timestamp, 16) * 1000).toLocaleString()}
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <p>No blocks found.</p>
            )}
          </section>
        </>
      )}

      {activeTab === "metrics" && (
        <>
          <section
            style={{
              marginBottom: "1.5rem",
              padding: "1rem",
              borderRadius: 8,
              background: "#0f172a",
            }}
          >
            <h2 style={{ fontSize: "1.2rem", marginBottom: "0.5rem" }}>
              Network Performance
            </h2>
            <div style={{ marginBottom: "1rem" }}>
              <div style={{ fontSize: "2rem", fontWeight: "bold", color: "#10b981" }}>
                {tps ? `${tps} TPS` : "--"}
              </div>
              <div style={{ opacity: 0.8, fontSize: "0.9rem" }}>
                Transactions Per Second (60s window)
              </div>
            </div>
            {dagStats && (
              <div style={{ display: "grid", gridTemplateColumns: "repeat(2, 1fr)", gap: "0.75rem", marginTop: "1rem" }}>
                <div style={{ padding: "0.75rem", borderRadius: 6, background: "#020617" }}>
                  <div style={{ fontSize: "1.5rem", fontWeight: "bold" }}>
                    {dagStats.total_blocks}
                  </div>
                  <div style={{ opacity: 0.8, fontSize: "0.85rem" }}>Total Blocks</div>
                </div>
                <div style={{ padding: "0.75rem", borderRadius: 6, background: "#020617" }}>
                  <div style={{ fontSize: "1.5rem", fontWeight: "bold" }}>
                    {dagStats.blue_blocks}
                  </div>
                  <div style={{ opacity: 0.8, fontSize: "0.85rem" }}>Blue Blocks</div>
                </div>
                <div style={{ padding: "0.75rem", borderRadius: 6, background: "#020617" }}>
                  <div style={{ fontSize: "1.5rem", fontWeight: "bold" }}>
                    {dagStats.red_blocks}
                  </div>
                  <div style={{ opacity: 0.8, fontSize: "0.85rem" }}>Red Blocks</div>
                </div>
                <div style={{ padding: "0.75rem", borderRadius: 6, background: "#020617" }}>
                  <div style={{ fontSize: "1.5rem", fontWeight: "bold" }}>
                    {dagStats.avg_txs_per_block.toFixed(1)}
                  </div>
                  <div style={{ opacity: 0.8, fontSize: "0.85rem" }}>Avg Txs/Block</div>
                </div>
              </div>
            )}
            <button
              onClick={refreshMetrics}
              disabled={loading}
              style={{
                marginTop: "1rem",
                padding: "0.5rem 1rem",
                borderRadius: 6,
                border: "none",
                background: "#4f46e5",
                color: "white",
                cursor: "pointer",
              }}
            >
              {loading ? "Refreshing..." : "Refresh"}
            </button>
          </section>

          <section
            style={{
              padding: "1rem",
              borderRadius: 8,
              background: "#0f172a",
            }}
          >
            <h2 style={{ fontSize: "1.2rem", marginBottom: "0.5rem" }}>
              Shard Statistics
            </h2>
            {shardStats && shardStats.shard_count > 0 ? (
              <>
                <div style={{ marginBottom: "1rem" }}>
                  <strong>Active Shards</strong>: {shardStats.shard_count}
                </div>
                <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fill, minmax(250px, 1fr))", gap: "0.75rem" }}>
                  {shardStats.shards.map((shard) => (
                    <div
                      key={shard.shard_id}
                      style={{
                        padding: "0.75rem",
                        borderRadius: 6,
                        background: "#020617",
                        fontSize: "0.9rem",
                      }}
                    >
                      <div style={{ fontWeight: "bold", marginBottom: "0.5rem" }}>
                        Shard #{shard.shard_id}
                      </div>
                      <div style={{ opacity: 0.8 }}>
                        <div>Blocks: {shard.block_count}</div>
                        <div>Pending Txs: {shard.transaction_pool_size}</div>
                        <div>Cross-Shard Out: {shard.cross_shard_outgoing}</div>
                        <div>Cross-Shard In: {shard.cross_shard_incoming}</div>
                      </div>
                    </div>
                  ))}
                </div>
              </>
            ) : (
              <p>Sharding not enabled or no shard data available.</p>
            )}
          </section>
        </>
      )}
    </div>
  );
}

export default App;
