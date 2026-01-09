import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import logoHero from "./assets/logo-hero.png?url";

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
        padding: "2rem",
        fontFamily: "'Segoe UI', system-ui, -apple-system, BlinkMacSystemFont, sans-serif",
        background: "linear-gradient(135deg, #020617 0%, #0f172a 50%, #1e293b 100%)",
        color: "#f8fafc",
      }}
    >
      <div style={{ maxWidth: "1400px", margin: "0 auto" }}>
        <div style={{ marginBottom: "2rem", textAlign: "center" }}>
          {/* Logo */}
          <img 
            src={logoHero} 
            alt="Mondoshawan Logo" 
            style={{ 
              width: "200px", 
              height: "200px",
              objectFit: "contain",
              marginBottom: "1rem",
              filter: "drop-shadow(0 0 30px rgba(99, 102, 241, 0.5))",
              animation: "pulse 3s ease-in-out infinite",
              display: "block",
              margin: "0 auto 1rem auto"
            }} 
          />
          <h1 style={{ 
            fontSize: "2.5rem", 
            marginBottom: "0.5rem",
            background: "linear-gradient(135deg, #6366f1, #ec4899, #06b6d4)",
            WebkitBackgroundClip: "text",
            WebkitTextFillColor: "transparent",
            backgroundClip: "text",
            fontWeight: "700",
            letterSpacing: "-0.02em"
          }}>
            Mondoshawan Desktop
          </h1>
          <p style={{ 
            opacity: 0.8, 
            marginBottom: 0,
            fontSize: "1.1rem",
            color: "#94a3b8"
          }}>
            All-in-One Blockchain Experience
          </p>
        </div>

      <div style={{ 
        marginBottom: "2rem", 
        display: "flex", 
        gap: "0.75rem",
        justifyContent: "center",
        flexWrap: "wrap"
      }}>
        <button
          onClick={() => setActiveTab("dashboard")}
          style={{
            padding: "0.75rem 1.5rem",
            borderRadius: 8,
            border: "none",
            cursor: "pointer",
            background: activeTab === "dashboard" 
              ? "linear-gradient(135deg, #6366f1, #4f46e5)" 
              : "rgba(30, 41, 59, 0.7)",
            color: "#f8fafc",
            fontWeight: "600",
            fontSize: "0.95rem",
            boxShadow: activeTab === "dashboard" 
              ? "0 4px 12px rgba(99, 102, 241, 0.3)" 
              : "none",
            transition: "all 0.3s ease",
            backdropFilter: "blur(12px)"
          }}
        >
          Dashboard
        </button>
        <button
          onClick={() => setActiveTab("wallet")}
          style={{
            padding: "0.75rem 1.5rem",
            borderRadius: 8,
            border: "none",
            cursor: "pointer",
            background: activeTab === "wallet" 
              ? "linear-gradient(135deg, #6366f1, #4f46e5)" 
              : "rgba(30, 41, 59, 0.7)",
            color: "#f8fafc",
            fontWeight: "600",
            fontSize: "0.95rem",
            boxShadow: activeTab === "wallet" 
              ? "0 4px 12px rgba(99, 102, 241, 0.3)" 
              : "none",
            transition: "all 0.3s ease",
            backdropFilter: "blur(12px)"
          }}
        >
          Wallet
        </button>
        <button
          onClick={() => setActiveTab("send")}
          style={{
            padding: "0.75rem 1.5rem",
            borderRadius: 8,
            border: "none",
            cursor: "pointer",
            background: activeTab === "send" 
              ? "linear-gradient(135deg, #6366f1, #4f46e5)" 
              : "rgba(30, 41, 59, 0.7)",
            color: "#f8fafc",
            fontWeight: "600",
            fontSize: "0.95rem",
            boxShadow: activeTab === "send" 
              ? "0 4px 12px rgba(99, 102, 241, 0.3)" 
              : "none",
            transition: "all 0.3s ease",
            backdropFilter: "blur(12px)"
          }}
        >
          Send
        </button>
        <button
          onClick={() => setActiveTab("explorer")}
          style={{
            padding: "0.75rem 1.5rem",
            borderRadius: 8,
            border: "none",
            cursor: "pointer",
            background: activeTab === "explorer" 
              ? "linear-gradient(135deg, #6366f1, #4f46e5)" 
              : "rgba(30, 41, 59, 0.7)",
            color: "#f8fafc",
            fontWeight: "600",
            fontSize: "0.95rem",
            boxShadow: activeTab === "explorer" 
              ? "0 4px 12px rgba(99, 102, 241, 0.3)" 
              : "none",
            transition: "all 0.3s ease",
            backdropFilter: "blur(12px)"
          }}
        >
          Explorer
        </button>
        <button
          onClick={() => setActiveTab("metrics")}
          style={{
            padding: "0.75rem 1.5rem",
            borderRadius: 8,
            border: "none",
            cursor: "pointer",
            background: activeTab === "metrics" 
              ? "linear-gradient(135deg, #6366f1, #4f46e5)" 
              : "rgba(30, 41, 59, 0.7)",
            color: "#f8fafc",
            fontWeight: "600",
            fontSize: "0.95rem",
            boxShadow: activeTab === "metrics" 
              ? "0 4px 12px rgba(99, 102, 241, 0.3)" 
              : "none",
            transition: "all 0.3s ease",
            backdropFilter: "blur(12px)"
          }}
        >
          Metrics
        </button>
      </div>

      {error && (
        <div
          style={{
            background: "rgba(239, 68, 68, 0.1)",
            border: "1px solid rgba(239, 68, 68, 0.3)",
            padding: "1rem 1.25rem",
            borderRadius: 12,
            marginBottom: "1.5rem",
            backdropFilter: "blur(12px)",
            boxShadow: "0 4px 12px rgba(239, 68, 68, 0.1)"
          }}
        >
          <strong style={{ color: "#fca5a5" }}>Error:</strong>{" "}
          <span style={{ color: "#fecaca" }}>{error}</span>
        </div>
      )}

      {activeTab === "dashboard" && (
        <>
          <section
            style={{
              marginBottom: "1.5rem",
              padding: "1.5rem",
              borderRadius: 16,
              background: "rgba(30, 41, 59, 0.7)",
              backdropFilter: "blur(12px)",
              border: "1px solid rgba(99, 102, 241, 0.2)",
              boxShadow: "0 8px 32px rgba(0, 0, 0, 0.3)",
            }}
          >
            <h2 style={{ fontSize: "1.4rem", marginBottom: "1rem", fontWeight: "600", color: "#f8fafc" }}>
              🎛️ Node Status
            </h2>
            {nodeStatus ? (
              <ul style={{ listStyle: "none", paddingLeft: 0, marginBottom: "1rem" }}>
                <li style={{ marginBottom: "0.75rem", display: "flex", alignItems: "center", gap: "0.5rem" }}>
                  <strong style={{ color: "#94a3b8", minWidth: "140px" }}>Height</strong>
                  <span style={{ color: "#06b6d4", fontWeight: "600", fontSize: "1.05rem" }}>{nodeStatus.height}</span>
                </li>
                <li style={{ marginBottom: "0.75rem", display: "flex", alignItems: "center", gap: "0.5rem" }}>
                  <strong style={{ color: "#94a3b8", minWidth: "140px" }}>Total Transactions</strong>
                  <span style={{ color: "#06b6d4", fontWeight: "600", fontSize: "1.05rem" }}>{nodeStatus.tx_count}</span>
                </li>
                <li style={{ marginBottom: "0.75rem", display: "flex", alignItems: "center", gap: "0.5rem" }}>
                  <strong style={{ color: "#94a3b8", minWidth: "140px" }}>Connected Peers</strong>
                  <span style={{ color: "#06b6d4", fontWeight: "600", fontSize: "1.05rem" }}>{nodeStatus.peer_count}</span>
                </li>
                <li style={{ marginBottom: "0.75rem", display: "flex", alignItems: "center", gap: "0.5rem" }}>
                  <strong style={{ color: "#94a3b8", minWidth: "140px" }}>Mining</strong>
                  <span style={{ 
                    color: miningOn ? "#10b981" : "#64748b", 
                    fontWeight: "700", 
                    fontSize: "1.05rem",
                    textShadow: miningOn ? "0 0 10px rgba(16, 185, 129, 0.5)" : "none"
                  }}>
                    {miningOn ? "🟢 ON" : "⚫ OFF"}
                  </span>
                </li>
              </ul>
            ) : (
              <p style={{ color: "#94a3b8", fontStyle: "italic" }}>Connecting to local node...</p>
            )}
            <button
              onClick={refresh}
              disabled={loading}
              style={{
                marginTop: "0.25rem",
                padding: "0.65rem 1.5rem",
                borderRadius: 8,
                border: "none",
                background: loading ? "rgba(99, 102, 241, 0.5)" : "linear-gradient(135deg, #6366f1, #4f46e5)",
                color: "white",
                cursor: loading ? "not-allowed" : "pointer",
                fontWeight: "600",
                fontSize: "0.95rem",
                boxShadow: loading ? "none" : "0 4px 12px rgba(99, 102, 241, 0.3)",
                transition: "all 0.3s ease",
                opacity: loading ? 0.6 : 1,
              }}
            >
              {loading ? "⏳ Refreshing..." : "🔄 Refresh"}
            </button>
          </section>

          <section
            style={{
              padding: "1.5rem",
              borderRadius: 16,
              background: "rgba(30, 41, 59, 0.7)",
              backdropFilter: "blur(12px)",
              border: "1px solid rgba(16, 185, 129, 0.2)",
              boxShadow: "0 8px 32px rgba(0, 0, 0, 0.3)",
            }}
          >
            <h2 style={{ fontSize: "1.4rem", marginBottom: "1rem", fontWeight: "600", color: "#f8fafc" }}>
              ⛏️ TriStream Mining
            </h2>
            <div style={{ marginBottom: "1rem", display: "flex", gap: "0.75rem" }}>
              <button
                onClick={startMining}
                disabled={loading || miningOn}
                style={{
                  padding: "0.65rem 1.5rem",
                  borderRadius: 8,
                  border: "none",
                  background: miningOn ? "rgba(75, 85, 99, 0.5)" : "linear-gradient(135deg, #10b981, #059669)",
                  color: "white",
                  cursor: miningOn || loading ? "not-allowed" : "pointer",
                  fontWeight: "600",
                  fontSize: "0.95rem",
                  boxShadow: miningOn || loading ? "none" : "0 4px 12px rgba(16, 185, 129, 0.3)",
                  transition: "all 0.3s ease",
                  opacity: miningOn || loading ? 0.5 : 1,
                }}
              >
                {loading ? "⏳" : "▶️"} Start Mining
              </button>
              <button
                onClick={stopMining}
                disabled={loading || !miningOn}
                style={{
                  padding: "0.65rem 1.5rem",
                  borderRadius: 8,
                  border: "none",
                  background: !miningOn ? "rgba(75, 85, 99, 0.5)" : "linear-gradient(135deg, #ef4444, #b91c1c)",
                  color: "white",
                  cursor: !miningOn || loading ? "not-allowed" : "pointer",
                  fontWeight: "600",
                  fontSize: "0.95rem",
                  boxShadow: !miningOn || loading ? "none" : "0 4px 12px rgba(239, 68, 68, 0.3)",
                  transition: "all 0.3s ease",
                  opacity: !miningOn || loading ? 0.5 : 1,
                }}
              >
                {loading ? "⏳" : "⏹️"} Stop Mining
              </button>
            </div>

            {miningStatus && (
              <>
                <div style={{ 
                  marginBottom: "1rem", 
                  padding: "0.75rem 1rem",
                  background: "rgba(99, 102, 241, 0.1)",
                  borderRadius: 8,
                  border: "1px solid rgba(99, 102, 241, 0.2)"
                }}>
                  <strong style={{ color: "#94a3b8" }}>Pending Transactions</strong>:{" "}
                  <span style={{ color: "#06b6d4", fontWeight: "700", fontSize: "1.1rem" }}>
                    {miningStatus.pending_txs}
                  </span>
                </div>
                <h3 style={{ marginTop: "0.5rem", marginBottom: "0.75rem", fontSize: "1.1rem", color: "#e2e8f0" }}>
                  Stream Configuration
                </h3>
                <ul style={{ listStyle: "none", paddingLeft: 0, fontSize: "0.95rem" }}>
                  <li style={{ 
                    marginBottom: "0.5rem",
                    padding: "0.75rem",
                    background: "rgba(16, 185, 129, 0.1)",
                    borderRadius: 8,
                    border: "1px solid rgba(16, 185, 129, 0.2)"
                  }}>
                    <strong style={{ color: "#10b981" }}>Stream A</strong>: 
                    <span style={{ color: "#94a3b8" }}> {miningStatus.streams.streamA.max_txs} tx / {miningStatus.streams.streamA.block_time_ms} ms</span>
                    <span style={{ color: "#fbbf24", marginLeft: "0.5rem" }}>💰 {miningStatus.streams.streamA.reward}</span>
                  </li>
                  <li style={{ 
                    marginBottom: "0.5rem",
                    padding: "0.75rem",
                    background: "rgba(139, 92, 246, 0.1)",
                    borderRadius: 8,
                    border: "1px solid rgba(139, 92, 246, 0.2)"
                  }}>
                    <strong style={{ color: "#8b5cf6" }}>Stream B</strong>: 
                    <span style={{ color: "#94a3b8" }}> {miningStatus.streams.streamB.max_txs} tx / {miningStatus.streams.streamB.block_time_ms} ms</span>
                    <span style={{ color: "#fbbf24", marginLeft: "0.5rem" }}>💰 {miningStatus.streams.streamB.reward}</span>
                  </li>
                  <li style={{ 
                    marginBottom: "0.5rem",
                    padding: "0.75rem",
                    background: "rgba(236, 72, 153, 0.1)",
                    borderRadius: 8,
                    border: "1px solid rgba(236, 72, 153, 0.2)"
                  }}>
                    <strong style={{ color: "#ec4899" }}>Stream C</strong>: 
                    <span style={{ color: "#94a3b8" }}> {miningStatus.streams.streamC.max_txs} tx / {miningStatus.streams.streamC.block_time_ms} ms</span>
                    <span style={{ color: "#fbbf24", marginLeft: "0.5rem" }}>💰 {miningStatus.streams.streamC.reward}</span>
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
            padding: "1.5rem",
            borderRadius: 16,
            background: "rgba(30, 41, 59, 0.7)",
            backdropFilter: "blur(12px)",
            border: "1px solid rgba(99, 102, 241, 0.2)",
            boxShadow: "0 8px 32px rgba(0, 0, 0, 0.3)",
          }}
        >
          <h2 style={{ fontSize: "1.4rem", marginBottom: "1rem", fontWeight: "600", color: "#f8fafc" }}>
            👛 Wallet Inspector
          </h2>
          <div style={{ marginBottom: "1rem" }}>
            <label style={{ display: "block", marginBottom: "0.5rem", color: "#94a3b8", fontWeight: "500" }}>
              Address (0x…)
            </label>
            <input
              type="text"
              value={walletAddress}
              onChange={(e) => setWalletAddress(e.target.value)}
              placeholder="0x..."
              style={{
                width: "100%",
                padding: "0.75rem",
                borderRadius: 8,
                border: "1px solid rgba(99, 102, 241, 0.3)",
                background: "rgba(2, 6, 23, 0.6)",
                color: "#e5e7eb",
                fontSize: "0.95rem",
                fontFamily: "'JetBrains Mono', 'Courier New', monospace",
                transition: "all 0.3s ease",
              }}
            />
          </div>
          <button
            onClick={loadWallet}
            disabled={loading || !walletAddress}
            style={{
              padding: "0.65rem 1.5rem",
              borderRadius: 8,
              border: "none",
              background: (!walletAddress || loading) ? "rgba(99, 102, 241, 0.5)" : "linear-gradient(135deg, #6366f1, #4f46e5)",
              color: "white",
              cursor: (!walletAddress || loading) ? "not-allowed" : "pointer",
              marginBottom: "1.25rem",
              fontWeight: "600",
              fontSize: "0.95rem",
              boxShadow: (!walletAddress || loading) ? "none" : "0 4px 12px rgba(99, 102, 241, 0.3)",
              transition: "all 0.3s ease",
              opacity: (!walletAddress || loading) ? 0.6 : 1,
            }}
          >
            {loading ? "⏳ Loading..." : "🔍 Load Wallet"}
          </button>

          {walletBalanceHex && walletNonceHex && (
            <div
              style={{
                marginTop: "0.5rem",
                padding: "1.25rem",
                borderRadius: 12,
                background: "rgba(6, 182, 212, 0.1)",
                border: "1px solid rgba(6, 182, 212, 0.3)",
                backdropFilter: "blur(8px)",
              }}
            >
              {(() => {
                const { raw, mshw } = formatBalance(walletBalanceHex);
                return (
                  <>
                    <div style={{ marginBottom: "0.75rem" }}>
                      <strong style={{ color: "#94a3b8", fontSize: "0.9rem" }}>Balance (raw)</strong>
                      <p style={{ 
                        color: "#06b6d4", 
                        fontFamily: "'JetBrains Mono', 'Courier New', monospace",
                        fontSize: "0.95rem",
                        marginTop: "0.25rem",
                        wordBreak: "break-all"
                      }}>{raw}</p>
                    </div>
                    <div style={{ marginBottom: "0.75rem" }}>
                      <strong style={{ color: "#94a3b8", fontSize: "0.9rem" }}>Balance (MSHW)</strong>
                      <p style={{ 
                        color: "#10b981", 
                        fontSize: "1.5rem",
                        fontWeight: "700",
                        marginTop: "0.25rem"
                      }}>💰 {mshw}</p>
                    </div>
                  </>
                );
              })()}
              <div>
                <strong style={{ color: "#94a3b8", fontSize: "0.9rem" }}>Nonce</strong>
                <p style={{ 
                  color: "#8b5cf6", 
                  fontFamily: "'JetBrains Mono', 'Courier New', monospace",
                  fontSize: "1.1rem",
                  fontWeight: "600",
                  marginTop: "0.25rem"
                }}>{walletNonceHex}</p>
              </div>
            </div>
          )}
        </section>
      )}

      {activeTab === "send" && (
        <section
          style={{
            padding: "1.5rem",
            borderRadius: 16,
            background: "rgba(30, 41, 59, 0.7)",
            backdropFilter: "blur(12px)",
            border: "1px solid rgba(16, 185, 129, 0.2)",
            boxShadow: "0 8px 32px rgba(0, 0, 0, 0.3)",
          }}
        >
          <h2 style={{ fontSize: "1.4rem", marginBottom: "1rem", fontWeight: "600", color: "#f8fafc" }}>
            💸 Send Transaction
          </h2>

          {!walletAddr && (
            <div style={{ 
              marginBottom: "1.5rem",
              padding: "1.25rem",
              background: "rgba(251, 191, 36, 0.1)",
              border: "1px solid rgba(251, 191, 36, 0.3)",
              borderRadius: 12,
              textAlign: "center"
            }}>
              <p style={{ color: "#fbbf24", marginBottom: "1rem", fontSize: "1.05rem" }}>⚠️ No wallet loaded.</p>
              <div style={{ display: "flex", gap: "0.75rem", justifyContent: "center" }}>
                <button
                  onClick={createNewKey}
                  disabled={loading}
                  style={{
                    padding: "0.65rem 1.5rem",
                    borderRadius: 8,
                    border: "none",
                    background: loading ? "rgba(16, 185, 129, 0.5)" : "linear-gradient(135deg, #10b981, #059669)",
                    color: "white",
                    cursor: loading ? "not-allowed" : "pointer",
                    fontWeight: "600",
                    fontSize: "0.95rem",
                    boxShadow: loading ? "none" : "0 4px 12px rgba(16, 185, 129, 0.3)",
                    transition: "all 0.3s ease",
                    opacity: loading ? 0.6 : 1,
                  }}
                >
                  {loading ? "⏳ Creating..." : "✨ Create New Wallet"}
                </button>
                <button
                  onClick={loadWalletAddress}
                  disabled={loading}
                  style={{
                    padding: "0.65rem 1.5rem",
                    borderRadius: 8,
                    border: "none",
                    background: loading ? "rgba(99, 102, 241, 0.5)" : "linear-gradient(135deg, #6366f1, #4f46e5)",
                    color: "white",
                    cursor: loading ? "not-allowed" : "pointer",
                    fontWeight: "600",
                    fontSize: "0.95rem",
                    boxShadow: loading ? "none" : "0 4px 12px rgba(99, 102, 241, 0.3)",
                    transition: "all 0.3s ease",
                    opacity: loading ? 0.6 : 1,
                  }}
                >
                  {loading ? "⏳" : "🔓"} Load Existing
                </button>
              </div>
            </div>
          )}

          {walletAddr && (
            <>
              <div style={{
                padding: "1rem",
                background: "rgba(99, 102, 241, 0.1)",
                border: "1px solid rgba(99, 102, 241, 0.2)",
                borderRadius: 10,
                marginBottom: "1.5rem",
              }}>
                <strong style={{ color: "#94a3b8", fontSize: "0.9rem" }}>Your Address</strong>
                <p style={{ 
                  color: "#6366f1",
                  fontFamily: "'JetBrains Mono', 'Courier New', monospace",
                  fontSize: "0.95rem",
                  marginTop: "0.5rem",
                  wordBreak: "break-all",
                  fontWeight: "600"
                }}>{walletAddr}</p>
              </div>
              <div style={{ marginBottom: "1rem" }}>
                <label style={{ display: "block", marginBottom: "0.5rem", color: "#94a3b8", fontWeight: "500" }}>
                  To Address (0x...)
                </label>
                <input
                  type="text"
                  value={sendTo}
                  onChange={(e) => setSendTo(e.target.value)}
                  placeholder="0x..."
                  style={{
                    width: "100%",
                    padding: "0.75rem",
                    borderRadius: 8,
                    border: "1px solid rgba(99, 102, 241, 0.3)",
                    background: "rgba(2, 6, 23, 0.6)",
                    color: "#e5e7eb",
                    fontSize: "0.95rem",
                    fontFamily: "'JetBrains Mono', 'Courier New', monospace",
                    transition: "all 0.3s ease",
                  }}
                />
              </div>
              <div style={{ marginBottom: "1rem" }}>
                <label style={{ display: "block", marginBottom: "0.5rem", color: "#94a3b8", fontWeight: "500" }}>
                  Value (MSHW)
                </label>
                <input
                  type="text"
                  value={sendValue}
                  onChange={(e) => setSendValue(e.target.value)}
                  placeholder="0.1"
                  style={{
                    width: "100%",
                    padding: "0.75rem",
                    borderRadius: 8,
                    border: "1px solid rgba(16, 185, 129, 0.3)",
                    background: "rgba(2, 6, 23, 0.6)",
                    color: "#e5e7eb",
                    fontSize: "0.95rem",
                    transition: "all 0.3s ease",
                  }}
                />
              </div>
              <div style={{ marginBottom: "1.25rem" }}>
                <label style={{ display: "block", marginBottom: "0.5rem", color: "#94a3b8", fontWeight: "500" }}>
                  Fee (MSHW)
                </label>
                <input
                  type="text"
                  value={sendFee}
                  onChange={(e) => setSendFee(e.target.value)}
                  placeholder="0.001"
                  style={{
                    width: "100%",
                    padding: "0.75rem",
                    borderRadius: 8,
                    border: "1px solid rgba(139, 92, 246, 0.3)",
                    background: "rgba(2, 6, 23, 0.6)",
                    color: "#e5e7eb",
                    fontSize: "0.95rem",
                    transition: "all 0.3s ease",
                  }}
                />
              </div>
              <button
                onClick={sendTx}
                disabled={loading}
                style={{
                  padding: "0.75rem 2rem",
                  borderRadius: 8,
                  border: "none",
                  background: loading ? "rgba(16, 185, 129, 0.5)" : "linear-gradient(135deg, #10b981, #059669)",
                  color: "white",
                  cursor: loading ? "not-allowed" : "pointer",
                  fontWeight: "600",
                  fontSize: "1rem",
                  boxShadow: loading ? "none" : "0 4px 12px rgba(16, 185, 129, 0.4)",
                  transition: "all 0.3s ease",
                  opacity: loading ? 0.6 : 1,
                  width: "100%",
                }}
              >
                {loading ? "⏳ Sending..." : "🚀 Send Transaction"}
              </button>

              {txHash && (
                <div
                  style={{
                    marginTop: "1.5rem",
                    padding: "1.25rem",
                    borderRadius: 12,
                    background: "rgba(16, 185, 129, 0.1)",
                    border: "1px solid rgba(16, 185, 129, 0.3)",
                  }}
                >
                  <strong style={{ color: "#10b981", fontSize: "1.1rem" }}>✅ Transaction Sent!</strong>
                  <p style={{ color: "#94a3b8", fontSize: "0.9rem", marginTop: "0.5rem", marginBottom: "0.5rem" }}>Transaction Hash</p>
                  <p style={{
                    color: "#06b6d4",
                    fontFamily: "'JetBrains Mono', 'Courier New', monospace",
                    fontSize: "0.9rem",
                    wordBreak: "break-all",
                    background: "rgba(2, 6, 23, 0.8)",
                    padding: "0.75rem",
                    borderRadius: 8,
                  }}>{txHash}</p>
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
              padding: "1.5rem",
              borderRadius: 16,
              background: "rgba(30, 41, 59, 0.7)",
              backdropFilter: "blur(12px)",
              border: "1px solid rgba(6, 182, 212, 0.2)",
              boxShadow: "0 8px 32px rgba(0, 0, 0, 0.3)",
            }}
          >
            <h2 style={{ fontSize: "1.4rem", marginBottom: "1rem", fontWeight: "600", color: "#f8fafc" }}>
              📊 DAG Statistics
            </h2>
            {dagStats ? (
              <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(180px, 1fr))", gap: "1rem" }}>
                <div style={{ 
                  padding: "1rem",
                  borderRadius: 10,
                  background: "rgba(99, 102, 241, 0.1)",
                  border: "1px solid rgba(99, 102, 241, 0.2)"
                }}>
                  <div style={{ fontSize: "2rem", fontWeight: "700", color: "#6366f1" }}>
                    {dagStats.total_blocks}
                  </div>
                  <div style={{ color: "#94a3b8", fontSize: "0.9rem", marginTop: "0.25rem" }}>Total Blocks</div>
                </div>
                <div style={{ 
                  padding: "1rem",
                  borderRadius: 10,
                  background: "rgba(6, 182, 212, 0.1)",
                  border: "1px solid rgba(6, 182, 212, 0.2)"
                }}>
                  <div style={{ fontSize: "2rem", fontWeight: "700", color: "#06b6d4" }}>
                    {dagStats.blue_blocks}
                  </div>
                  <div style={{ color: "#94a3b8", fontSize: "0.9rem", marginTop: "0.25rem" }}>Blue Blocks</div>
                </div>
                <div style={{ 
                  padding: "1rem",
                  borderRadius: 10,
                  background: "rgba(239, 68, 68, 0.1)",
                  border: "1px solid rgba(239, 68, 68, 0.2)"
                }}>
                  <div style={{ fontSize: "2rem", fontWeight: "700", color: "#ef4444" }}>
                    {dagStats.red_blocks}
                  </div>
                  <div style={{ color: "#94a3b8", fontSize: "0.9rem", marginTop: "0.25rem" }}>Red Blocks</div>
                </div>
                <div style={{ 
                  padding: "1rem",
                  borderRadius: 10,
                  background: "rgba(16, 185, 129, 0.1)",
                  border: "1px solid rgba(16, 185, 129, 0.2)"
                }}>
                  <div style={{ fontSize: "2rem", fontWeight: "700", color: "#10b981" }}>
                    {dagStats.total_transactions}
                  </div>
                  <div style={{ color: "#94a3b8", fontSize: "0.9rem", marginTop: "0.25rem" }}>Total Transactions</div>
                </div>
                <div style={{ 
                  padding: "1rem",
                  borderRadius: 10,
                  background: "rgba(139, 92, 246, 0.1)",
                  border: "1px solid rgba(139, 92, 246, 0.2)",
                  gridColumn: "span 1"
                }}>
                  <div style={{ fontSize: "2rem", fontWeight: "700", color: "#8b5cf6" }}>
                    {dagStats.avg_txs_per_block.toFixed(2)}
                  </div>
                  <div style={{ color: "#94a3b8", fontSize: "0.9rem", marginTop: "0.25rem" }}>Avg Txs/Block</div>
                </div>
              </div>
            ) : (
              <p style={{ color: "#94a3b8", fontStyle: "italic" }}>Loading DAG stats...</p>
            )}
            <button
              onClick={refreshExplorer}
              disabled={loading}
              style={{
                marginTop: "1rem",
                padding: "0.65rem 1.5rem",
                borderRadius: 8,
                border: "none",
                background: loading ? "rgba(99, 102, 241, 0.5)" : "linear-gradient(135deg, #6366f1, #4f46e5)",
                color: "white",
                cursor: loading ? "not-allowed" : "pointer",
                fontWeight: "600",
                fontSize: "0.95rem",
                boxShadow: loading ? "none" : "0 4px 12px rgba(99, 102, 241, 0.3)",
                transition: "all 0.3s ease",
                opacity: loading ? 0.6 : 1,
              }}
            >
              {loading ? "⏳ Refreshing..." : "🔄 Refresh"}
            </button>
          </section>

          <section
            style={{
              padding: "1.5rem",
              borderRadius: 16,
              background: "rgba(30, 41, 59, 0.7)",
              backdropFilter: "blur(12px)",
              border: "1px solid rgba(99, 102, 241, 0.2)",
              boxShadow: "0 8px 32px rgba(0, 0, 0, 0.3)",
            }}
          >
            <h2 style={{ fontSize: "1.4rem", marginBottom: "1rem", fontWeight: "600", color: "#f8fafc" }}>
              📦 Recent Blocks
            </h2>
            {blocks.length > 0 ? (
              <div style={{ overflowX: "auto", overflowY: "auto", maxHeight: "600px" }}>
                {blocks.map((block) => {
                  const streamColors: Record<string, { bg: string; border: string; text: string }> = {
                    A: { bg: "rgba(16, 185, 129, 0.1)", border: "rgba(16, 185, 129, 0.3)", text: "#10b981" },
                    B: { bg: "rgba(139, 92, 246, 0.1)", border: "rgba(139, 92, 246, 0.3)", text: "#8b5cf6" },
                    C: { bg: "rgba(236, 72, 153, 0.1)", border: "rgba(236, 72, 153, 0.3)", text: "#ec4899" },
                  };
                  const stream = (block.stream_type || 'A') as string;
                  const colors = streamColors[stream] ?? streamColors['A'];
                  
                  return (
                    <div
                      key={block.hash}
                      style={{
                        padding: "1rem",
                        marginBottom: "0.75rem",
                        borderRadius: 10,
                        background: colors.bg,
                        border: `1px solid ${colors.border}`,
                        fontSize: "0.95rem",
                        transition: "all 0.3s ease",
                      }}
                    >
                      <div style={{ marginBottom: "0.5rem" }}>
                        <strong style={{ fontSize: "1.1rem", color: "#f8fafc" }}>
                          Block #{parseInt(block.number, 16)}
                        </strong>
                        {block.stream_type && (
                          <span style={{ 
                            marginLeft: "0.75rem", 
                            padding: "0.25rem 0.75rem",
                            background: colors.border,
                            borderRadius: 6,
                            fontSize: "0.85rem",
                            fontWeight: "600",
                            color: colors.text
                          }}>
                            Stream {block.stream_type}
                          </span>
                        )}
                      </div>
                      <div style={{ color: "#94a3b8", fontSize: "0.85rem", marginBottom: "0.25rem" }}>
                        <strong>Hash:</strong> <span style={{ fontFamily: "'JetBrains Mono', monospace", color: "#06b6d4" }}>{block.hash.substring(0, 32)}...</span>
                      </div>
                      <div style={{ color: "#94a3b8", fontSize: "0.85rem", marginBottom: "0.25rem" }}>
                        <strong>Transactions:</strong> <span style={{ color: "#10b981", fontWeight: "600" }}>{block.transactions.length}</span>
                      </div>
                      <div style={{ color: "#94a3b8", fontSize: "0.85rem" }}>
                        <strong>Time:</strong> <span style={{ color: "#8b5cf6" }}>{new Date(parseInt(block.timestamp, 16) * 1000).toLocaleString()}</span>
                      </div>
                    </div>
                  );
                })}
              </div>
            ) : (
              <p style={{ color: "#94a3b8", fontStyle: "italic" }}>No blocks found.</p>
            )}
          </section>
        </>
      )}

      {activeTab === "metrics" && (
        <>
          <section
            style={{
              marginBottom: "1.5rem",
              padding: "1.5rem",
              borderRadius: 16,
              background: "rgba(30, 41, 59, 0.7)",
              backdropFilter: "blur(12px)",
              border: "1px solid rgba(16, 185, 129, 0.2)",
              boxShadow: "0 8px 32px rgba(0, 0, 0, 0.3)",
            }}
          >
            <h2 style={{ fontSize: "1.4rem", marginBottom: "1rem", fontWeight: "600", color: "#f8fafc" }}>
              ⚡ Network Performance
            </h2>
            <div style={{ 
              marginBottom: "1.5rem",
              padding: "1.5rem",
              background: "rgba(16, 185, 129, 0.15)",
              border: "2px solid rgba(16, 185, 129, 0.3)",
              borderRadius: 12,
              textAlign: "center"
            }}>
              <div style={{ 
                fontSize: "3.5rem", 
                fontWeight: "800", 
                color: "#10b981",
                textShadow: "0 0 20px rgba(16, 185, 129, 0.5)",
                marginBottom: "0.5rem"
              }}>
                {tps ? `${tps}` : "--"}
              </div>
              <div style={{ 
                color: "#94a3b8", 
                fontSize: "1rem",
                fontWeight: "500",
                letterSpacing: "0.05em"
              }}>
                🚀 TRANSACTIONS PER SECOND
              </div>
              <div style={{ color: "#64748b", fontSize: "0.85rem", marginTop: "0.25rem" }}>
                (60s rolling window)
              </div>
            </div>
            {dagStats && (
              <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(160px, 1fr))", gap: "1rem", marginTop: "1rem" }}>
                <div style={{ 
                  padding: "1rem",
                  borderRadius: 10,
                  background: "rgba(99, 102, 241, 0.1)",
                  border: "1px solid rgba(99, 102, 241, 0.2)",
                  textAlign: "center"
                }}>
                  <div style={{ fontSize: "2rem", fontWeight: "700", color: "#6366f1" }}>
                    {dagStats.total_blocks}
                  </div>
                  <div style={{ color: "#94a3b8", fontSize: "0.85rem", marginTop: "0.25rem" }}>Total Blocks</div>
                </div>
                <div style={{ 
                  padding: "1rem",
                  borderRadius: 10,
                  background: "rgba(6, 182, 212, 0.1)",
                  border: "1px solid rgba(6, 182, 212, 0.2)",
                  textAlign: "center"
                }}>
                  <div style={{ fontSize: "2rem", fontWeight: "700", color: "#06b6d4" }}>
                    {dagStats.blue_blocks}
                  </div>
                  <div style={{ color: "#94a3b8", fontSize: "0.85rem", marginTop: "0.25rem" }}>Blue Blocks</div>
                </div>
                <div style={{ 
                  padding: "1rem",
                  borderRadius: 10,
                  background: "rgba(239, 68, 68, 0.1)",
                  border: "1px solid rgba(239, 68, 68, 0.2)",
                  textAlign: "center"
                }}>
                  <div style={{ fontSize: "2rem", fontWeight: "700", color: "#ef4444" }}>
                    {dagStats.red_blocks}
                  </div>
                  <div style={{ color: "#94a3b8", fontSize: "0.85rem", marginTop: "0.25rem" }}>Red Blocks</div>
                </div>
                <div style={{ 
                  padding: "1rem",
                  borderRadius: 10,
                  background: "rgba(139, 92, 246, 0.1)",
                  border: "1px solid rgba(139, 92, 246, 0.2)",
                  textAlign: "center"
                }}>
                  <div style={{ fontSize: "2rem", fontWeight: "700", color: "#8b5cf6" }}>
                    {dagStats.avg_txs_per_block.toFixed(1)}
                  </div>
                  <div style={{ color: "#94a3b8", fontSize: "0.85rem", marginTop: "0.25rem" }}>Avg Txs/Block</div>
                </div>
              </div>
            )}
            <button
              onClick={refreshMetrics}
              disabled={loading}
              style={{
                marginTop: "1rem",
                padding: "0.65rem 1.5rem",
                borderRadius: 8,
                border: "none",
                background: loading ? "rgba(99, 102, 241, 0.5)" : "linear-gradient(135deg, #6366f1, #4f46e5)",
                color: "white",
                cursor: loading ? "not-allowed" : "pointer",
                fontWeight: "600",
                fontSize: "0.95rem",
                boxShadow: loading ? "none" : "0 4px 12px rgba(99, 102, 241, 0.3)",
                transition: "all 0.3s ease",
                opacity: loading ? 0.6 : 1,
              }}
            >
              {loading ? "⏳ Refreshing..." : "🔄 Refresh"}
            </button>
          </section>

          <section
            style={{
              padding: "1.5rem",
              borderRadius: 16,
              background: "rgba(30, 41, 59, 0.7)",
              backdropFilter: "blur(12px)",
              border: "1px solid rgba(139, 92, 246, 0.2)",
              boxShadow: "0 8px 32px rgba(0, 0, 0, 0.3)",
            }}
          >
            <h2 style={{ fontSize: "1.4rem", marginBottom: "1rem", fontWeight: "600", color: "#f8fafc" }}>
              🧩 Shard Statistics
            </h2>
            {shardStats && shardStats.shard_count > 0 ? (
              <>
                <div style={{ 
                  marginBottom: "1.25rem",
                  padding: "1rem",
                  background: "rgba(139, 92, 246, 0.1)",
                  border: "1px solid rgba(139, 92, 246, 0.2)",
                  borderRadius: 10,
                  textAlign: "center"
                }}>
                  <strong style={{ color: "#8b5cf6", fontSize: "1.2rem" }}>Active Shards</strong>
                  <span style={{ color: "#10b981", fontSize: "1.8rem", fontWeight: "700", marginLeft: "0.75rem" }}>
                    {shardStats.shard_count}
                  </span>
                </div>
                <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fill, minmax(260px, 1fr))", gap: "1rem" }}>
                  {shardStats.shards.map((shard, idx) => {
                    const shardColors = [
                      { bg: "rgba(16, 185, 129, 0.1)", border: "rgba(16, 185, 129, 0.3)", text: "#10b981" },
                      { bg: "rgba(139, 92, 246, 0.1)", border: "rgba(139, 92, 246, 0.3)", text: "#8b5cf6" },
                      { bg: "rgba(236, 72, 153, 0.1)", border: "rgba(236, 72, 153, 0.3)", text: "#ec4899" },
                      { bg: "rgba(6, 182, 212, 0.1)", border: "rgba(6, 182, 212, 0.3)", text: "#06b6d4" },
                    ];
                    const color = shardColors[idx % shardColors.length];
                    return (
                      <div
                        key={shard.shard_id}
                        style={{
                          padding: "1rem",
                          borderRadius: 10,
                          background: color.bg,
                          border: `1px solid ${color.border}`,
                          fontSize: "0.95rem",
                        }}
                      >
                        <div style={{ fontWeight: "700", marginBottom: "0.75rem", fontSize: "1.1rem", color: color.text }}>
                          📦 Shard #{shard.shard_id}
                        </div>
                        <div style={{ display: "flex", flexDirection: "column", gap: "0.5rem" }}>
                          <div style={{ display: "flex", justifyContent: "space-between" }}>
                            <span style={{ color: "#94a3b8" }}>Blocks:</span>
                            <strong style={{ color: "#f8fafc" }}>{shard.block_count}</strong>
                          </div>
                          <div style={{ display: "flex", justifyContent: "space-between" }}>
                            <span style={{ color: "#94a3b8" }}>Pending Txs:</span>
                            <strong style={{ color: "#fbbf24" }}>{shard.transaction_pool_size}</strong>
                          </div>
                          <div style={{ display: "flex", justifyContent: "space-between" }}>
                            <span style={{ color: "#94a3b8" }}>Cross-Shard Out:</span>
                            <strong style={{ color: "#ec4899" }}>{shard.cross_shard_outgoing}</strong>
                          </div>
                          <div style={{ display: "flex", justifyContent: "space-between" }}>
                            <span style={{ color: "#94a3b8" }}>Cross-Shard In:</span>
                            <strong style={{ color: "#06b6d4" }}>{shard.cross_shard_incoming}</strong>
                          </div>
                        </div>
                      </div>
                    );
                  })}
                </div>
              </>
            ) : (
              <div style={{
                padding: "2rem",
                textAlign: "center",
                background: "rgba(251, 191, 36, 0.1)",
                border: "1px solid rgba(251, 191, 36, 0.2)",
                borderRadius: 12
              }}>
                <p style={{ color: "#fbbf24", fontSize: "1.1rem", fontStyle: "italic" }}>
                  ⚠️ Sharding not enabled or no shard data available.
                </p>
                <p style={{ color: "#94a3b8", fontSize: "0.9rem", marginTop: "0.5rem" }}>
                  Start the node with <code style={{ background: "rgba(2, 6, 23, 0.8)", padding: "0.25rem 0.5rem", borderRadius: 4 }}>--shards N</code> to enable sharding.
                </p>
              </div>
            )}
          </section>
        </>
      )}
      </div>
    </div>
  );
}

export default App;
