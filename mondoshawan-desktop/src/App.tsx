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
  const [activeTab, setActiveTab] = useState<"dashboard" | "wallet" | "send" | "history" | "explorer" | "metrics" | "account-abstraction">("dashboard");
  
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
  const [miningDashboard, setMiningDashboard] = useState<any | null>(null);
  
  // Transaction History state
  const [txHistory, setTxHistory] = useState<any[]>([]);
  const [txHistoryLimit, _setTxHistoryLimit] = useState<number>(50);
  
  // Address Book state
  const [contacts, setContacts] = useState<any[]>([]);
  const [showAddContact, setShowAddContact] = useState(false);
  const [contactName, setContactName] = useState("");
  const [contactAddress, setContactAddress] = useState("");
  const [contactNotes, setContactNotes] = useState("");
  
  // Multi-Account state
  const [accounts, setAccounts] = useState<any[]>([]);
  const [showAddAccount, setShowAddAccount] = useState(false);
  const [accountName, setAccountName] = useState("");
  const [accountAddress, setAccountAddress] = useState("");
  const [selectedAccount, setSelectedAccount] = useState<string | null>(null);
  
  // Account Abstraction state
  const [wallets, setWallets] = useState<any[]>([]);
  const [selectedWallet, setSelectedWallet] = useState<string | null>(null);
  const [walletType, setWalletType] = useState<"basic" | "multisig" | "social" | "spending" | "combined">("basic");
  const [walletOwner, setWalletOwner] = useState<string>("");
  const [multisigSigners, setMultisigSigners] = useState<string[]>([]);
  const [multisigThreshold, setMultisigThreshold] = useState<number>(2);
  const [guardians, setGuardians] = useState<string[]>([]);
  const [recoveryThreshold, setRecoveryThreshold] = useState<number>(2);
  const [spendingLimit, setSpendingLimit] = useState<string>("");
  // Note: pendingMultisigTxs, recoveryStatus, batchOperations reserved for future UI expansion
  // const [pendingMultisigTxs, setPendingMultisigTxs] = useState<any[]>([]);
  // const [recoveryStatus, setRecoveryStatus] = useState<any | null>(null);
  // const [batchOperations, setBatchOperations] = useState<any[]>([]);
  
  // Parallel EVM state
  const [parallelEVMEnabled, setParallelEVMEnabled] = useState<boolean>(false);
  const [parallelEVMStats, setParallelEVMStats] = useState<any | null>(null);
  
  // Reputation state
  const [reputation, setReputation] = useState<any | null>(null);
  const [reputationFactors, setReputationFactors] = useState<any | null>(null);
  
  // Time-locked & Gasless state
  const [isTimeLocked, setIsTimeLocked] = useState<boolean>(false);
  const [executeAtBlock, setExecuteAtBlock] = useState<string>("");
  const [executeAtTimestamp, setExecuteAtTimestamp] = useState<string>("");
  const [isGasless, setIsGasless] = useState<boolean>(false);
  const [sponsorAddress, setSponsorAddress] = useState<string>("");
  
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
      
      // Load reputation
      try {
        const rep = await invoke<any>("get_reputation", { address: walletAddress });
        const factors = await invoke<any>("get_reputation_factors", { address: walletAddress });
        if (rep) setReputation(rep);
        if (factors) setReputationFactors(factors);
      } catch (e) {
        // Reputation might not be available, ignore
      }
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
    if (!sendTo || !sendValue) {
      setError("Fill in recipient and value");
      return;
    }
    if (!isGasless && !sendFee) {
      setError("Fill in fee (or enable gasless transaction)");
      return;
    }
    if (isGasless && !sponsorAddress) {
      setError("Enter sponsor address for gasless transaction");
      return;
    }
    setLoading(true);
    setError(null);
    setTxHash(null);
    try {
      // Convert MSHW to base units (1 MSHW = 10^18)
      const valueBigInt = BigInt(Math.floor(parseFloat(sendValue) * 1e18));
      const feeBigInt = isGasless ? BigInt(0) : BigInt(Math.floor(parseFloat(sendFee) * 1e18));

      // Handle time-locked or gasless transactions
      if (isTimeLocked && (executeAtBlock || executeAtTimestamp)) {
        const executeAtBlockNum = executeAtBlock ? parseInt(executeAtBlock) : undefined;
        const executeAtTimestampNum = executeAtTimestamp ? parseInt(executeAtTimestamp) : undefined;
        
        const hash = await invoke<string>("create_time_locked_transaction", {
          from: walletAddr,
          to: sendTo,
          value: `0x${valueBigInt.toString(16)}`,
          fee: `0x${feeBigInt.toString(16)}`,
          executeAtBlock: executeAtBlockNum,
          executeAtTimestamp: executeAtTimestampNum,
        });
        setTxHash(hash);
      } else if (isGasless) {
        const hash = await invoke<string>("create_gasless_transaction", {
          from: walletAddr,
          to: sendTo,
          value: `0x${valueBigInt.toString(16)}`,
          fee: `0x${feeBigInt.toString(16)}`,
          sponsor: sponsorAddress,
        });
        setTxHash(hash);
      } else {
        // Regular transaction
        const hash = await invoke<string>("send_transaction", {
          toAddress: sendTo,
          valueHex: `0x${valueBigInt.toString(16)}`,
          feeHex: `0x${feeBigInt.toString(16)}`,
        });
        setTxHash(hash);
      }
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
      const [tpsData, dagData, shardData, miningData] = await Promise.all([
        invoke<any>("get_tps"),
        invoke<DagStats>("get_dag_stats"),
        invoke<ShardStats>("get_shard_stats"),
        invoke<any>("get_mining_dashboard"),
      ]);
      setTps(tpsData);
      setDagStats(dagData);
      setShardStats(shardData);
      setMiningDashboard(miningData);
      await loadParallelEVMStats();
    } catch (e: any) {
      setError(e?.toString?.() ?? "Failed to fetch metrics");
    } finally {
      setLoading(false);
    }
  }

  async function loadParallelEVMStats() {
    try {
      const stats = await invoke<any>("get_parallel_evm_stats");
      setParallelEVMStats(stats);
      setParallelEVMEnabled(stats?.enabled || false);
    } catch (e: any) {
      // Ignore errors, stats might not be available
    }
  }

  async function createWallet() {
    if (!walletOwner) {
      setError("Enter owner address");
      return;
    }
    setLoading(true);
    setError(null);
    try {
      const config: any = {};
      if (walletType === "multisig" || walletType === "combined") {
        config.multisig = {
          signers: multisigSigners,
          threshold: multisigThreshold,
        };
      }
      if (walletType === "social" || walletType === "combined") {
        config.recovery = {
          guardians: guardians,
          threshold: recoveryThreshold,
          security_delay_seconds: 86400 * 2, // 2 days
        };
      }
      if (walletType === "spending" || walletType === "combined") {
        const limitBigInt = BigInt(Math.floor(parseFloat(spendingLimit || "0") * 1e18));
        config.spending_limits = {
          daily_limit: limitBigInt.toString(),
          reset_period_seconds: 86400, // 24 hours
        };
      }

      await invoke<any>("create_wallet", {
        walletType: walletType,
        owner: walletOwner,
        config: config,
      });
      
      setError(null);
      await loadWallets();
    } catch (e: any) {
      setError(e?.toString?.() ?? "Failed to create wallet");
    } finally {
      setLoading(false);
    }
  }

  async function loadWallets() {
    if (!walletOwner) {
      setWallets([]);
      return;
    }
    setLoading(true);
    try {
      const walletList = await invoke<any>("get_owner_wallets", { owner: walletOwner });
      setWallets(Array.isArray(walletList) ? walletList : []);
    } catch (e: any) {
      setWallets([]);
    } finally {
      setLoading(false);
    }
  }

  async function viewWalletDetails(address: string) {
    setLoading(true);
    setError(null);
    try {
      const wallet = await invoke<any>("get_wallet", { address: address });
      alert(`Wallet Details:

Type: ${wallet.wallet_type}
Address: ${wallet.address}
Owner: ${wallet.owner}
Nonce: ${wallet.nonce || 0}`);
    } catch (e: any) {
      setError(e?.toString?.() ?? "Failed to load wallet details");
    } finally {
      setLoading(false);
    }
  }

  // Transaction History
  async function loadTxHistory() {
    if (!walletAddr) {
      setError("No wallet loaded");
      return;
    }
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<any>("get_address_transactions", {
        address: walletAddr,
        limit: txHistoryLimit,
      });
      setTxHistory(result.transactions || []);
    } catch (e: any) {
      setError(e?.toString?.() ?? "Failed to load transaction history");
    } finally {
      setLoading(false);
    }
  }

  // Address Book Functions
  async function loadContacts() {
    try {
      const result = await invoke<any[]>("get_contacts");
      setContacts(result);
    } catch (e: any) {
      setError(e?.toString?.() ?? "Failed to load contacts");
    }
  }

  async function addContact() {
    if (!contactName || !contactAddress) {
      setError("Name and address required");
      return;
    }
    setLoading(true);
    setError(null);
    try {
      await invoke("add_contact", {
        name: contactName,
        address: contactAddress,
        notes: contactNotes || null,
      });
      setContactName("");
      setContactAddress("");
      setContactNotes("");
      setShowAddContact(false);
      await loadContacts();
    } catch (e: any) {
      setError(e?.toString?.() ?? "Failed to add contact");
    } finally {
      setLoading(false);
    }
  }

  async function removeContact(address: string) {
    setLoading(true);
    setError(null);
    try {
      await invoke("remove_contact", { address });
      await loadContacts();
    } catch (e: any) {
      setError(e?.toString?.() ?? "Failed to remove contact");
    } finally {
      setLoading(false);
    }
  }

  // Multi-Account Functions
  async function loadAccounts() {
    try {
      const result = await invoke<any[]>("get_accounts");
      setAccounts(result);
      if (result.length > 0 && !selectedAccount) {
        setSelectedAccount(result[0].address);
      }
    } catch (e: any) {
      setError(e?.toString?.() ?? "Failed to load accounts");
    }
  }

  async function addAccount() {
    if (!accountName || !accountAddress) {
      setError("Name and address required");
      return;
    }
    setLoading(true);
    setError(null);
    try {
      await invoke("add_account", {
        name: accountName,
        address: accountAddress,
      });
      setAccountName("");
      setAccountAddress("");
      setShowAddAccount(false);
      await loadAccounts();
    } catch (e: any) {
      setError(e?.toString?.() ?? "Failed to add account");
    } finally {
      setLoading(false);
    }
  }

  async function removeAccount(address: string) {
    setLoading(true);
    setError(null);
    try {
      await invoke("remove_account", { address });
      await loadAccounts();
      if (selectedAccount === address && accounts.length > 0) {
        setSelectedAccount(accounts[0].address);
      }
    } catch (e: any) {
      setError(e?.toString?.() ?? "Failed to remove account");
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
    if (activeTab === "history") {
      loadTxHistory();
    }
  }, [activeTab, walletAddr]);

  useEffect(() => {
    loadContacts();
    loadAccounts();
  }, []);

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
          onClick={() => setActiveTab("history")}
          style={{
            padding: "0.75rem 1.5rem",
            borderRadius: 8,
            border: "none",
            cursor: "pointer",
            background: activeTab === "history" 
              ? "linear-gradient(135deg, #6366f1, #4f46e5)" 
              : "rgba(30, 41, 59, 0.7)",
            color: "#f8fafc",
            fontWeight: "600",
            fontSize: "0.95rem",
            boxShadow: activeTab === "history" 
              ? "0 4px 12px rgba(99, 102, 241, 0.3)" 
              : "none",
            transition: "all 0.3s ease",
            backdropFilter: "blur(12px)"
          }}
        >
          History
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
        <button
          onClick={() => setActiveTab("account-abstraction")}
          style={{
            padding: "0.75rem 1.5rem",
            borderRadius: 8,
            border: "none",
            cursor: "pointer",
            background: activeTab === "account-abstraction" 
              ? "linear-gradient(135deg, #8b5cf6, #7c3aed)" 
              : "rgba(30, 41, 59, 0.7)",
            color: "#f8fafc",
            fontWeight: "600",
            fontSize: "0.95rem",
            boxShadow: activeTab === "account-abstraction" 
              ? "0 4px 12px rgba(139, 92, 246, 0.3)" 
              : "none",
            transition: "all 0.3s ease",
            backdropFilter: "blur(12px)"
          }}
        >
          Account Abstraction
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

          {/* Reputation Display */}
          {walletAddress && reputation && (
            <div
              style={{
                marginTop: "1.5rem",
                padding: "1.25rem",
                borderRadius: 12,
                background: "rgba(16, 185, 129, 0.1)",
                border: "1px solid rgba(16, 185, 129, 0.3)",
                backdropFilter: "blur(8px)",
              }}
            >
              <h3 style={{ fontSize: "1.1rem", marginBottom: "1rem", fontWeight: "600", color: "#f8fafc" }}>
                ⭐ Reputation
              </h3>
              <div style={{ marginBottom: "0.75rem" }}>
                <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "0.5rem" }}>
                  <strong style={{ color: "#94a3b8", fontSize: "0.9rem" }}>Score</strong>
                  <span style={{ 
                    color: reputation.score >= 80 ? "#10b981" : reputation.score >= 40 ? "#fbbf24" : "#ef4444",
                    fontSize: "1.5rem",
                    fontWeight: "700"
                  }}>
                    {reputation.score}/100
                  </span>
                </div>
                <div style={{ 
                  color: reputation.level === "High" ? "#10b981" : reputation.level === "Medium" ? "#fbbf24" : "#ef4444",
                  fontSize: "0.95rem",
                  fontWeight: "600"
                }}>
                  Level: {reputation.level}
                </div>
              </div>
              {reputationFactors && (
                <div style={{ 
                  marginTop: "1rem", 
                  padding: "1rem", 
                  background: "rgba(2, 6, 23, 0.6)", 
                  borderRadius: 8,
                  fontSize: "0.85rem"
                }}>
                  <div style={{ marginBottom: "0.5rem", color: "#94a3b8" }}>Factors:</div>
                  <div style={{ display: "grid", gridTemplateColumns: "repeat(2, 1fr)", gap: "0.5rem" }}>
                    <div>✅ Successful: {reputationFactors.successful_txs || 0}</div>
                    <div>❌ Failed: {reputationFactors.failed_txs || 0}</div>
                    <div>⛏️ Blocks: {reputationFactors.blocks_mined || 0}</div>
                    <div>📅 Age: {reputationFactors.account_age_days || 0} days</div>
                    <div>💰 Value: {((reputationFactors.total_value_transacted || 0) / 1e18).toFixed(2)} MSHW</div>
                    <div>👥 Contacts: {reputationFactors.unique_contacts || 0}</div>
                  </div>
                </div>
              )}
            </div>
          )}
        </section>
      )}

      {activeTab === "wallet" && (
        <>
          {/* Address Book Section */}
          <section
            style={{
              marginTop: "1.5rem",
              padding: "1.5rem",
              borderRadius: 16,
              background: "rgba(30, 41, 59, 0.7)",
              backdropFilter: "blur(12px)",
              border: "1px solid rgba(236, 72, 153, 0.2)",
              boxShadow: "0 8px 32px rgba(0, 0, 0, 0.3)",
            }}
          >
            <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "1rem" }}>
              <h2 style={{ fontSize: "1.4rem", fontWeight: "600", color: "#f8fafc" }}>
                📖 Address Book
              </h2>
              <button
                onClick={() => setShowAddContact(!showAddContact)}
                style={{
                  padding: "0.5rem 1rem",
                  borderRadius: 8,
                  border: "none",
                  background: "linear-gradient(135deg, #ec4899, #db2777)",
                  color: "white",
                  cursor: "pointer",
                  fontWeight: "600",
                  fontSize: "0.9rem",
                  boxShadow: "0 4px 12px rgba(236, 72, 153, 0.3)",
                }}
              >
                {showAddContact ? "❌ Cancel" : "➕ Add Contact"}
              </button>
            </div>

            {showAddContact && (
              <div style={{
                marginBottom: "1.5rem",
                padding: "1rem",
                background: "rgba(236, 72, 153, 0.1)",
                border: "1px solid rgba(236, 72, 153, 0.3)",
                borderRadius: 10,
              }}>
                <div style={{ marginBottom: "0.75rem" }}>
                  <label style={{ display: "block", marginBottom: "0.5rem", color: "#94a3b8", fontWeight: "500", fontSize: "0.9rem" }}>
                    Name
                  </label>
                  <input
                    type="text"
                    value={contactName}
                    onChange={(e) => setContactName(e.target.value)}
                    placeholder="Alice"
                    style={{
                      width: "100%",
                      padding: "0.65rem",
                      borderRadius: 8,
                      border: "1px solid rgba(236, 72, 153, 0.3)",
                      background: "rgba(2, 6, 23, 0.6)",
                      color: "#e5e7eb",
                      fontSize: "0.9rem",
                    }}
                  />
                </div>
                <div style={{ marginBottom: "0.75rem" }}>
                  <label style={{ display: "block", marginBottom: "0.5rem", color: "#94a3b8", fontWeight: "500", fontSize: "0.9rem" }}>
                    Address
                  </label>
                  <input
                    type="text"
                    value={contactAddress}
                    onChange={(e) => setContactAddress(e.target.value)}
                    placeholder="0x..."
                    style={{
                      width: "100%",
                      padding: "0.65rem",
                      borderRadius: 8,
                      border: "1px solid rgba(236, 72, 153, 0.3)",
                      background: "rgba(2, 6, 23, 0.6)",
                      color: "#e5e7eb",
                      fontSize: "0.9rem",
                      fontFamily: "'JetBrains Mono', monospace",
                    }}
                  />
                </div>
                <div style={{ marginBottom: "1rem" }}>
                  <label style={{ display: "block", marginBottom: "0.5rem", color: "#94a3b8", fontWeight: "500", fontSize: "0.9rem" }}>
                    Notes (optional)
                  </label>
                  <input
                    type="text"
                    value={contactNotes}
                    onChange={(e) => setContactNotes(e.target.value)}
                    placeholder="Friend, exchange, etc."
                    style={{
                      width: "100%",
                      padding: "0.65rem",
                      borderRadius: 8,
                      border: "1px solid rgba(236, 72, 153, 0.3)",
                      background: "rgba(2, 6, 23, 0.6)",
                      color: "#e5e7eb",
                      fontSize: "0.9rem",
                    }}
                  />
                </div>
                <button
                  onClick={addContact}
                  disabled={loading}
                  style={{
                    padding: "0.65rem 1.5rem",
                    borderRadius: 8,
                    border: "none",
                    background: loading ? "rgba(236, 72, 153, 0.5)" : "linear-gradient(135deg, #ec4899, #db2777)",
                    color: "white",
                    cursor: loading ? "not-allowed" : "pointer",
                    fontWeight: "600",
                    fontSize: "0.95rem",
                    boxShadow: loading ? "none" : "0 4px 12px rgba(236, 72, 153, 0.3)",
                    opacity: loading ? 0.6 : 1,
                    width: "100%",
                  }}
                >
                  {loading ? "⏳ Saving..." : "💾 Save Contact"}
                </button>
              </div>
            )}

            {contacts.length === 0 ? (
              <div style={{ padding: "1.5rem", textAlign: "center", color: "#94a3b8", fontStyle: "italic" }}>
                No contacts yet. Add your first contact!
              </div>
            ) : (
              <div style={{ display: "grid", gap: "0.75rem" }}>
                {contacts.map((contact: any) => (
                  <div
                    key={contact.address}
                    style={{
                      padding: "1rem",
                      background: "rgba(236, 72, 153, 0.1)",
                      border: "1px solid rgba(236, 72, 153, 0.2)",
                      borderRadius: 10,
                      display: "flex",
                      justifyContent: "space-between",
                      alignItems: "center",
                    }}
                  >
                    <div style={{ flex: 1 }}>
                      <div style={{ fontSize: "1.1rem", fontWeight: "600", color: "#ec4899", marginBottom: "0.25rem" }}>
                        {contact.name}
                      </div>
                      <div style={{ 
                        fontSize: "0.85rem", 
                        color: "#06b6d4",
                        fontFamily: "'JetBrains Mono', monospace",
                        wordBreak: "break-all",
                        marginBottom: "0.25rem"
                      }}>
                        {contact.address}
                      </div>
                      {contact.notes && (
                        <div style={{ fontSize: "0.8rem", color: "#94a3b8", fontStyle: "italic" }}>
                          {contact.notes}
                        </div>
                      )}
                    </div>
                    <button
                      onClick={() => removeContact(contact.address)}
                      disabled={loading}
                      style={{
                        padding: "0.5rem 1rem",
                        borderRadius: 8,
                        border: "none",
                        background: "linear-gradient(135deg, #ef4444, #dc2626)",
                        color: "white",
                        cursor: loading ? "not-allowed" : "pointer",
                        fontWeight: "600",
                        fontSize: "0.85rem",
                        opacity: loading ? 0.6 : 1,
                      }}
                    >
                      🗑️
                    </button>
                  </div>
                ))}
              </div>
            )}
          </section>

          {/* Multi-Account Section */}
          <section
            style={{
              marginTop: "1.5rem",
              padding: "1.5rem",
              borderRadius: 16,
              background: "rgba(30, 41, 59, 0.7)",
              backdropFilter: "blur(12px)",
              border: "1px solid rgba(6, 182, 212, 0.2)",
              boxShadow: "0 8px 32px rgba(0, 0, 0, 0.3)",
            }}
          >
            <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "1rem" }}>
              <h2 style={{ fontSize: "1.4rem", fontWeight: "600", color: "#f8fafc" }}>
                💼 My Accounts
              </h2>
              <button
                onClick={() => setShowAddAccount(!showAddAccount)}
                style={{
                  padding: "0.5rem 1rem",
                  borderRadius: 8,
                  border: "none",
                  background: "linear-gradient(135deg, #06b6d4, #0891b2)",
                  color: "white",
                  cursor: "pointer",
                  fontWeight: "600",
                  fontSize: "0.9rem",
                  boxShadow: "0 4px 12px rgba(6, 182, 212, 0.3)",
                }}
              >
                {showAddAccount ? "❌ Cancel" : "➕ Add Account"}
              </button>
            </div>

            {showAddAccount && (
              <div style={{
                marginBottom: "1.5rem",
                padding: "1rem",
                background: "rgba(6, 182, 212, 0.1)",
                border: "1px solid rgba(6, 182, 212, 0.3)",
                borderRadius: 10,
              }}>
                <div style={{ marginBottom: "0.75rem" }}>
                  <label style={{ display: "block", marginBottom: "0.5rem", color: "#94a3b8", fontWeight: "500", fontSize: "0.9rem" }}>
                    Account Name
                  </label>
                  <input
                    type="text"
                    value={accountName}
                    onChange={(e) => setAccountName(e.target.value)}
                    placeholder="Main Account"
                    style={{
                      width: "100%",
                      padding: "0.65rem",
                      borderRadius: 8,
                      border: "1px solid rgba(6, 182, 212, 0.3)",
                      background: "rgba(2, 6, 23, 0.6)",
                      color: "#e5e7eb",
                      fontSize: "0.9rem",
                    }}
                  />
                </div>
                <div style={{ marginBottom: "1rem" }}>
                  <label style={{ display: "block", marginBottom: "0.5rem", color: "#94a3b8", fontWeight: "500", fontSize: "0.9rem" }}>
                    Address
                  </label>
                  <input
                    type="text"
                    value={accountAddress}
                    onChange={(e) => setAccountAddress(e.target.value)}
                    placeholder="0x..."
                    style={{
                      width: "100%",
                      padding: "0.65rem",
                      borderRadius: 8,
                      border: "1px solid rgba(6, 182, 212, 0.3)",
                      background: "rgba(2, 6, 23, 0.6)",
                      color: "#e5e7eb",
                      fontSize: "0.9rem",
                      fontFamily: "'JetBrains Mono', monospace",
                    }}
                  />
                </div>
                <button
                  onClick={addAccount}
                  disabled={loading}
                  style={{
                    padding: "0.65rem 1.5rem",
                    borderRadius: 8,
                    border: "none",
                    background: loading ? "rgba(6, 182, 212, 0.5)" : "linear-gradient(135deg, #06b6d4, #0891b2)",
                    color: "white",
                    cursor: loading ? "not-allowed" : "pointer",
                    fontWeight: "600",
                    fontSize: "0.95rem",
                    boxShadow: loading ? "none" : "0 4px 12px rgba(6, 182, 212, 0.3)",
                    opacity: loading ? 0.6 : 1,
                    width: "100%",
                  }}
                >
                  {loading ? "⏳ Saving..." : "💾 Save Account"}
                </button>
              </div>
            )}

            {accounts.length === 0 ? (
              <div style={{ padding: "1.5rem", textAlign: "center", color: "#94a3b8", fontStyle: "italic" }}>
                No accounts added. Create or add your first account!
              </div>
            ) : (
              <div style={{ display: "grid", gap: "0.75rem" }}>
                {accounts.map((account: any) => (
                  <div
                    key={account.address}
                    style={{
                      padding: "1rem",
                      background: selectedAccount === account.address 
                        ? "rgba(6, 182, 212, 0.15)" 
                        : "rgba(6, 182, 212, 0.05)",
                      border: selectedAccount === account.address
                        ? "2px solid rgba(6, 182, 212, 0.4)"
                        : "1px solid rgba(6, 182, 212, 0.2)",
                      borderRadius: 10,
                      display: "flex",
                      justifyContent: "space-between",
                      alignItems: "center",
                      cursor: "pointer",
                      transition: "all 0.3s ease",
                    }}
                    onClick={() => setSelectedAccount(account.address)}
                  >
                    <div style={{ flex: 1 }}>
                      <div style={{ 
                        fontSize: "1.1rem", 
                        fontWeight: "600", 
                        color: selectedAccount === account.address ? "#06b6d4" : "#64748b",
                        marginBottom: "0.25rem"
                      }}>
                        {selectedAccount === account.address && "✔️ "}{account.name}
                      </div>
                      <div style={{ 
                        fontSize: "0.85rem", 
                        color: "#06b6d4",
                        fontFamily: "'JetBrains Mono', monospace",
                        wordBreak: "break-all"
                      }}>
                        {account.address}
                      </div>
                    </div>
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        removeAccount(account.address);
                      }}
                      disabled={loading}
                      style={{
                        padding: "0.5rem 1rem",
                        borderRadius: 8,
                        border: "none",
                        background: "linear-gradient(135deg, #ef4444, #dc2626)",
                        color: "white",
                        cursor: loading ? "not-allowed" : "pointer",
                        fontWeight: "600",
                        fontSize: "0.85rem",
                        opacity: loading ? 0.6 : 1,
                      }}
                    >
                      🗑️
                    </button>
                  </div>
                ))}
              </div>
            )}
          </section>
        </>
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
                  disabled={isGasless}
                  style={{
                    width: "100%",
                    padding: "0.75rem",
                    borderRadius: 8,
                    border: "1px solid rgba(139, 92, 246, 0.3)",
                    background: isGasless ? "rgba(2, 6, 23, 0.3)" : "rgba(2, 6, 23, 0.6)",
                    color: isGasless ? "#64748b" : "#e5e7eb",
                    fontSize: "0.95rem",
                    transition: "all 0.3s ease",
                    opacity: isGasless ? 0.5 : 1,
                  }}
                />
              </div>

              {/* Time-Locked Transaction Options */}
              <div style={{ marginBottom: "1.25rem", padding: "1rem", background: "rgba(6, 182, 212, 0.1)", border: "1px solid rgba(6, 182, 212, 0.3)", borderRadius: 10 }}>
                <label style={{ display: "flex", alignItems: "center", cursor: "pointer", color: "#94a3b8", fontWeight: "500" }}>
                  <input
                    type="checkbox"
                    checked={isTimeLocked}
                    onChange={(e) => setIsTimeLocked(e.target.checked)}
                    style={{ marginRight: "0.5rem", width: "18px", height: "18px", cursor: "pointer" }}
                  />
                  <span>⏰ Time-Locked Transaction</span>
                </label>
                {isTimeLocked && (
                  <div style={{ marginTop: "1rem", display: "flex", flexDirection: "column", gap: "0.75rem" }}>
                    <div>
                      <label style={{ display: "block", marginBottom: "0.5rem", color: "#94a3b8", fontSize: "0.9rem" }}>
                        Execute at Block Number (optional)
                      </label>
                      <input
                        type="number"
                        value={executeAtBlock}
                        onChange={(e) => setExecuteAtBlock(e.target.value)}
                        placeholder="e.g., 1000"
                        style={{
                          width: "100%",
                          padding: "0.65rem",
                          borderRadius: 8,
                          border: "1px solid rgba(6, 182, 212, 0.3)",
                          background: "rgba(2, 6, 23, 0.6)",
                          color: "#e5e7eb",
                          fontSize: "0.9rem",
                        }}
                      />
                    </div>
                    <div>
                      <label style={{ display: "block", marginBottom: "0.5rem", color: "#94a3b8", fontSize: "0.9rem" }}>
                        OR Execute at Timestamp (Unix timestamp, optional)
                      </label>
                      <input
                        type="number"
                        value={executeAtTimestamp}
                        onChange={(e) => setExecuteAtTimestamp(e.target.value)}
                        placeholder="e.g., 1704067200"
                        style={{
                          width: "100%",
                          padding: "0.65rem",
                          borderRadius: 8,
                          border: "1px solid rgba(6, 182, 212, 0.3)",
                          background: "rgba(2, 6, 23, 0.6)",
                          color: "#e5e7eb",
                          fontSize: "0.9rem",
                        }}
                      />
                    </div>
                  </div>
                )}
              </div>

              {/* Gasless Transaction Options */}
              <div style={{ marginBottom: "1.25rem", padding: "1rem", background: "rgba(16, 185, 129, 0.1)", border: "1px solid rgba(16, 185, 129, 0.3)", borderRadius: 10 }}>
                <label style={{ display: "flex", alignItems: "center", cursor: "pointer", color: "#94a3b8", fontWeight: "500" }}>
                  <input
                    type="checkbox"
                    checked={isGasless}
                    onChange={(e) => setIsGasless(e.target.checked)}
                    style={{ marginRight: "0.5rem", width: "18px", height: "18px", cursor: "pointer" }}
                  />
                  <span>🎁 Gasless Transaction (Sponsored)</span>
                </label>
                {isGasless && (
                  <div style={{ marginTop: "1rem" }}>
                    <label style={{ display: "block", marginBottom: "0.5rem", color: "#94a3b8", fontSize: "0.9rem" }}>
                      Sponsor Address (who pays the fee)
                    </label>
                    <input
                      type="text"
                      value={sponsorAddress}
                      onChange={(e) => setSponsorAddress(e.target.value)}
                      placeholder="0x..."
                      style={{
                        width: "100%",
                        padding: "0.65rem",
                        borderRadius: 8,
                        border: "1px solid rgba(16, 185, 129, 0.3)",
                        background: "rgba(2, 6, 23, 0.6)",
                        color: "#e5e7eb",
                        fontSize: "0.9rem",
                        fontFamily: "'JetBrains Mono', monospace",
                      }}
                    />
                  </div>
                )}
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

      {activeTab === "history" && (
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
            📜 Transaction History
          </h2>
          
          {!walletAddr ? (
            <div style={{
              padding: "2rem",
              textAlign: "center",
              color: "#94a3b8",
              fontStyle: "italic"
            }}>
              No wallet loaded. Go to Wallet or Send tab to create/load a wallet.
            </div>
          ) : (
            <>
              <div style={{ marginBottom: "1rem", display: "flex", gap: "1rem", alignItems: "center" }}>
                <div style={{ flex: 1 }}>
                  <strong style={{ color: "#8b5cf6" }}>Current Wallet:</strong>{" "}
                  <span style={{ 
                    fontFamily: "'JetBrains Mono', monospace", 
                    fontSize: "0.85rem",
                    color: "#06b6d4"
                  }}>
                    {walletAddr}
                  </span>
                </div>
                <button
                  onClick={loadTxHistory}
                  disabled={loading}
                  style={{
                    padding: "0.65rem 1.5rem",
                    borderRadius: 8,
                    border: "none",
                    background: loading ? "rgba(139, 92, 246, 0.5)" : "linear-gradient(135deg, #8b5cf6, #7c3aed)",
                    color: "white",
                    cursor: loading ? "not-allowed" : "pointer",
                    fontWeight: "600",
                    fontSize: "0.95rem",
                    boxShadow: loading ? "none" : "0 4px 12px rgba(139, 92, 246, 0.3)",
                    transition: "all 0.3s ease",
                    opacity: loading ? 0.6 : 1,
                  }}
                >
                  {loading ? "⏳ Loading..." : "🔄 Refresh"}
                </button>
              </div>

              {txHistory.length === 0 ? (
                <div style={{
                  padding: "2rem",
                  textAlign: "center",
                  color: "#94a3b8",
                  fontStyle: "italic"
                }}>
                  No transactions found for this address.
                </div>
              ) : (
                <div style={{ overflowX: "auto", overflowY: "auto", maxHeight: "600px" }}>
                  {txHistory.map((tx: any) => {
                    const isIncoming = tx.direction === "incoming";
                    const timestamp = parseInt(tx.timestamp, 16);
                    const date = new Date(timestamp * 1000);
                    const value = BigInt(tx.value);
                    const valueMSHW = (Number(value) / 1e18).toFixed(6);
                    
                    return (
                      <div
                        key={tx.hash}
                        style={{
                          padding: "1rem",
                          marginBottom: "0.75rem",
                          borderRadius: 10,
                          background: isIncoming 
                            ? "rgba(16, 185, 129, 0.1)" 
                            : "rgba(239, 68, 68, 0.1)",
                          border: isIncoming 
                            ? "1px solid rgba(16, 185, 129, 0.3)" 
                            : "1px solid rgba(239, 68, 68, 0.3)",
                        }}
                      >
                        <div style={{ 
                          display: "flex", 
                          justifyContent: "space-between",
                          alignItems: "center",
                          marginBottom: "0.75rem"
                        }}>
                          <div style={{ 
                            fontSize: "1.1rem", 
                            fontWeight: "600",
                            color: isIncoming ? "#10b981" : "#ef4444"
                          }}>
                            {isIncoming ? "⬇️ Received" : "⬆️ Sent"}
                          </div>
                          <div style={{ 
                            fontSize: "1.2rem", 
                            fontWeight: "700",
                            color: isIncoming ? "#10b981" : "#ef4444"
                          }}>
                            {isIncoming ? "+" : "-"}{valueMSHW} MSHW
                          </div>
                        </div>
                        
                        <div style={{ fontSize: "0.85rem", color: "#94a3b8", marginBottom: "0.5rem" }}>
                          <strong>From:</strong>{" "}
                          <span style={{ fontFamily: "'JetBrains Mono', monospace", color: "#06b6d4" }}>
                            {tx.from}
                          </span>
                        </div>
                        
                        <div style={{ fontSize: "0.85rem", color: "#94a3b8", marginBottom: "0.5rem" }}>
                          <strong>To:</strong>{" "}
                          <span style={{ fontFamily: "'JetBrains Mono', monospace", color: "#06b6d4" }}>
                            {tx.to}
                          </span>
                        </div>
                        
                        <div style={{ 
                          display: "flex", 
                          justifyContent: "space-between",
                          fontSize: "0.85rem",
                          color: "#94a3b8",
                          marginTop: "0.75rem",
                          paddingTop: "0.75rem",
                          borderTop: "1px solid rgba(148, 163, 184, 0.1)"
                        }}>
                          <div>
                            <strong>Block:</strong> {parseInt(tx.block_number, 16)}
                          </div>
                          <div>
                            {date.toLocaleString()}
                          </div>
                        </div>
                        
                        <div style={{ 
                          fontSize: "0.75rem", 
                          color: "#64748b",
                          marginTop: "0.5rem",
                          fontFamily: "'JetBrains Mono', monospace",
                          wordBreak: "break-all"
                        }}>
                          {tx.hash}
                        </div>
                      </div>
                    );
                  })}
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
              <>
                {/* Consensus Health Indicator */}
                <div style={{
                  marginTop: "1rem",
                  marginBottom: "1rem",
                  padding: "1rem",
                  borderRadius: 10,
                  background: dagStats.blue_blocks / dagStats.total_blocks > 0.9 
                    ? "rgba(16, 185, 129, 0.1)" 
                    : "rgba(251, 191, 36, 0.1)",
                  border: dagStats.blue_blocks / dagStats.total_blocks > 0.9
                    ? "1px solid rgba(16, 185, 129, 0.3)"
                    : "1px solid rgba(251, 191, 36, 0.3)",
                }}>
                  <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between" }}>
                    <div>
                      <div style={{ fontSize: "0.9rem", color: "#94a3b8", marginBottom: "0.25rem" }}>
                        💚 Consensus Health
                      </div>
                      <div style={{ 
                        fontSize: "1.8rem", 
                        fontWeight: "700",
                        color: dagStats.blue_blocks / dagStats.total_blocks > 0.9 ? "#10b981" : "#fbbf24"
                      }}>
                        {((dagStats.blue_blocks / dagStats.total_blocks) * 100).toFixed(1)}%
                      </div>
                    </div>
                    <div style={{ 
                      width: "120px", 
                      height: "120px",
                      borderRadius: "50%",
                      background: `conic-gradient(
                        ${dagStats.blue_blocks / dagStats.total_blocks > 0.9 ? "#10b981" : "#fbbf24"} ${(dagStats.blue_blocks / dagStats.total_blocks) * 360}deg,
                        rgba(148, 163, 184, 0.2) 0deg
                      )`,
                      display: "flex",
                      alignItems: "center",
                      justifyContent: "center",
                      boxShadow: "0 4px 12px rgba(0, 0, 0, 0.3)"
                    }}>
                      <div style={{
                        width: "90px",
                        height: "90px",
                        borderRadius: "50%",
                        background: "rgba(30, 41, 59, 0.95)",
                        display: "flex",
                        alignItems: "center",
                        justifyContent: "center",
                        fontSize: "0.85rem",
                        fontWeight: "600",
                        color: "#f8fafc"
                      }}>
                        Blue Ratio
                      </div>
                    </div>
                  </div>
                </div>
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
              </>
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

          {/* Mining Dashboard */}
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
              ⛏️ Mining Dashboard
            </h2>
            {miningDashboard ? (
              <>
                <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(220px, 1fr))", gap: "1rem", marginBottom: "1.5rem" }}>
                  {/* Stream A Stats */}
                  <div style={{
                    padding: "1.25rem",
                    borderRadius: 12,
                    background: "rgba(16, 185, 129, 0.1)",
                    border: "1px solid rgba(16, 185, 129, 0.3)",
                  }}>
                    <div style={{ fontSize: "1.1rem", fontWeight: "600", color: "#10b981", marginBottom: "1rem" }}>
                      🟢 Stream A (ASIC)
                    </div>
                    <div style={{ display: "flex", flexDirection: "column", gap: "0.75rem" }}>
                      <div>
                        <div style={{ color: "#94a3b8", fontSize: "0.85rem", marginBottom: "0.25rem" }}>Blocks Mined</div>
                        <div style={{ fontSize: "1.8rem", fontWeight: "700", color: "#10b981" }}>
                          {miningDashboard.streams.stream_a.blocks_mined}
                        </div>
                      </div>
                      <div>
                        <div style={{ color: "#94a3b8", fontSize: "0.85rem", marginBottom: "0.25rem" }}>Hashrate (blocks/hr)</div>
                        <div style={{ fontSize: "1.3rem", fontWeight: "600", color: "#f8fafc" }}>
                          {miningDashboard.streams.stream_a.hashrate_estimate_blocks_per_hour.toFixed(1)}
                        </div>
                      </div>
                      <div>
                        <div style={{ color: "#94a3b8", fontSize: "0.85rem", marginBottom: "0.25rem" }}>Earnings (MSHW)</div>
                        <div style={{ fontSize: "1.1rem", fontWeight: "600", color: "#fbbf24" }}>
                          💰 {(parseInt(miningDashboard.streams.stream_a.earnings, 16) / 1e18).toFixed(2)}
                        </div>
                      </div>
                    </div>
                  </div>

                  {/* Stream B Stats */}
                  <div style={{
                    padding: "1.25rem",
                    borderRadius: 12,
                    background: "rgba(139, 92, 246, 0.1)",
                    border: "1px solid rgba(139, 92, 246, 0.3)",
                  }}>
                    <div style={{ fontSize: "1.1rem", fontWeight: "600", color: "#8b5cf6", marginBottom: "1rem" }}>
                      🔵 Stream B (CPU/GPU)
                    </div>
                    <div style={{ display: "flex", flexDirection: "column", gap: "0.75rem" }}>
                      <div>
                        <div style={{ color: "#94a3b8", fontSize: "0.85rem", marginBottom: "0.25rem" }}>Blocks Mined</div>
                        <div style={{ fontSize: "1.8rem", fontWeight: "700", color: "#8b5cf6" }}>
                          {miningDashboard.streams.stream_b.blocks_mined}
                        </div>
                      </div>
                      <div>
                        <div style={{ color: "#94a3b8", fontSize: "0.85rem", marginBottom: "0.25rem" }}>Hashrate (blocks/hr)</div>
                        <div style={{ fontSize: "1.3rem", fontWeight: "600", color: "#f8fafc" }}>
                          {miningDashboard.streams.stream_b.hashrate_estimate_blocks_per_hour.toFixed(1)}
                        </div>
                      </div>
                      <div>
                        <div style={{ color: "#94a3b8", fontSize: "0.85rem", marginBottom: "0.25rem" }}>Earnings (MSHW)</div>
                        <div style={{ fontSize: "1.1rem", fontWeight: "600", color: "#fbbf24" }}>
                          💰 {(parseInt(miningDashboard.streams.stream_b.earnings, 16) / 1e18).toFixed(2)}
                        </div>
                      </div>
                    </div>
                  </div>

                  {/* Stream C Stats */}
                  <div style={{
                    padding: "1.25rem",
                    borderRadius: 12,
                    background: "rgba(236, 72, 153, 0.1)",
                    border: "1px solid rgba(236, 72, 153, 0.3)",
                  }}>
                    <div style={{ fontSize: "1.1rem", fontWeight: "600", color: "#ec4899", marginBottom: "1rem" }}>
                      🟣 Stream C (ZK)
                    </div>
                    <div style={{ display: "flex", flexDirection: "column", gap: "0.75rem" }}>
                      <div>
                        <div style={{ color: "#94a3b8", fontSize: "0.85rem", marginBottom: "0.25rem" }}>Blocks Mined</div>
                        <div style={{ fontSize: "1.8rem", fontWeight: "700", color: "#ec4899" }}>
                          {miningDashboard.streams.stream_c.blocks_mined}
                        </div>
                      </div>
                      <div>
                        <div style={{ color: "#94a3b8", fontSize: "0.85rem", marginBottom: "0.25rem" }}>Hashrate (blocks/hr)</div>
                        <div style={{ fontSize: "1.3rem", fontWeight: "600", color: "#f8fafc" }}>
                          {miningDashboard.streams.stream_c.hashrate_estimate_blocks_per_hour.toFixed(1)}
                        </div>
                      </div>
                      <div>
                        <div style={{ color: "#94a3b8", fontSize: "0.85rem", marginBottom: "0.25rem" }}>Fees Collected</div>
                        <div style={{ fontSize: "1.1rem", fontWeight: "600", color: "#fbbf24" }}>
                          💰 {(parseInt(miningDashboard.streams.stream_c.fees_collected, 16) / 1e18).toFixed(6)}
                        </div>
                      </div>
                    </div>
                  </div>
                </div>

                {/* Total Earnings Summary */}
                <div style={{
                  padding: "1.5rem",
                  borderRadius: 12,
                  background: "rgba(251, 191, 36, 0.1)",
                  border: "1px solid rgba(251, 191, 36, 0.3)",
                  textAlign: "center"
                }}>
                  <div style={{ color: "#94a3b8", fontSize: "0.9rem", marginBottom: "0.5rem" }}>Total Earnings (Last 100 Blocks)</div>
                  <div style={{ fontSize: "2.5rem", fontWeight: "700", color: "#fbbf24" }}>
                    💰 {(parseInt(miningDashboard.total_earnings_recent, 16) / 1e18).toFixed(2)} MSHW
                  </div>
                  <div style={{ color: "#94a3b8", fontSize: "0.85rem", marginTop: "0.5rem" }}>
                    Sample: {miningDashboard.recent_sample_size} blocks | Total Blocks: {miningDashboard.total_blocks}
                  </div>
                </div>

                {/* Visual Distribution Bar */}
                <div style={{
                  marginTop: "1.5rem",
                  padding: "1.25rem",
                  borderRadius: 12,
                  background: "rgba(30, 41, 59, 0.5)",
                  border: "1px solid rgba(148, 163, 184, 0.2)"
                }}>
                  <div style={{ fontSize: "1rem", fontWeight: "600", color: "#f8fafc", marginBottom: "1rem" }}>
                    📊 Stream Distribution (Last 100 Blocks)
                  </div>
                  <div style={{ 
                    display: "flex", 
                    height: "40px", 
                    borderRadius: 8, 
                    overflow: "hidden",
                    boxShadow: "0 4px 12px rgba(0, 0, 0, 0.3)"
                  }}>
                    {/* Stream A portion */}
                    <div style={{
                      flex: miningDashboard.streams.stream_a.blocks_mined,
                      background: "linear-gradient(135deg, #10b981, #059669)",
                      display: "flex",
                      alignItems: "center",
                      justifyContent: "center",
                      color: "white",
                      fontWeight: "600",
                      fontSize: "0.9rem",
                      transition: "all 0.3s ease"
                    }}>
                      {miningDashboard.streams.stream_a.blocks_mined > 0 && (
                        <span>{miningDashboard.streams.stream_a.blocks_mined}</span>
                      )}
                    </div>
                    {/* Stream B portion */}
                    <div style={{
                      flex: miningDashboard.streams.stream_b.blocks_mined,
                      background: "linear-gradient(135deg, #8b5cf6, #7c3aed)",
                      display: "flex",
                      alignItems: "center",
                      justifyContent: "center",
                      color: "white",
                      fontWeight: "600",
                      fontSize: "0.9rem",
                      transition: "all 0.3s ease"
                    }}>
                      {miningDashboard.streams.stream_b.blocks_mined > 0 && (
                        <span>{miningDashboard.streams.stream_b.blocks_mined}</span>
                      )}
                    </div>
                    {/* Stream C portion */}
                    <div style={{
                      flex: miningDashboard.streams.stream_c.blocks_mined,
                      background: "linear-gradient(135deg, #ec4899, #db2777)",
                      display: "flex",
                      alignItems: "center",
                      justifyContent: "center",
                      color: "white",
                      fontWeight: "600",
                      fontSize: "0.9rem",
                      transition: "all 0.3s ease"
                    }}>
                      {miningDashboard.streams.stream_c.blocks_mined > 0 && (
                        <span>{miningDashboard.streams.stream_c.blocks_mined}</span>
                      )}
                    </div>
                  </div>
                  <div style={{ 
                    display: "flex", 
                    justifyContent: "space-around", 
                    marginTop: "1rem",
                    fontSize: "0.85rem"
                  }}>
                    <div style={{ textAlign: "center" }}>
                      <div style={{ color: "#10b981", fontWeight: "600" }}>🟢 A</div>
                      <div style={{ color: "#94a3b8" }}>
                        {miningDashboard.streams.stream_a.blocks_mined > 0 
                          ? ((miningDashboard.streams.stream_a.blocks_mined / 100) * 100).toFixed(1)
                          : "0"}%
                      </div>
                    </div>
                    <div style={{ textAlign: "center" }}>
                      <div style={{ color: "#8b5cf6", fontWeight: "600" }}>🔵 B</div>
                      <div style={{ color: "#94a3b8" }}>
                        {miningDashboard.streams.stream_b.blocks_mined > 0
                          ? ((miningDashboard.streams.stream_b.blocks_mined / 100) * 100).toFixed(1)
                          : "0"}%
                      </div>
                    </div>
                    <div style={{ textAlign: "center" }}>
                      <div style={{ color: "#ec4899", fontWeight: "600" }}>🟣 C</div>
                      <div style={{ color: "#94a3b8" }}>
                        {miningDashboard.streams.stream_c.blocks_mined > 0
                          ? ((miningDashboard.streams.stream_c.blocks_mined / 100) * 100).toFixed(1)
                          : "0"}%
                      </div>
                    </div>
                  </div>
                </div>
              </>
            ) : (
              <p style={{ color: "#94a3b8", fontStyle: "italic" }}>Loading mining dashboard...</p>
            )}
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

          {/* Parallel EVM Section */}
          <section
            style={{
              marginTop: "1.5rem",
              padding: "1.5rem",
              borderRadius: 16,
              background: "rgba(30, 41, 59, 0.7)",
              backdropFilter: "blur(12px)",
              border: "1px solid rgba(16, 185, 129, 0.2)",
              boxShadow: "0 8px 32px rgba(0, 0, 0, 0.3)",
            }}
          >
            <h2 style={{ fontSize: "1.4rem", marginBottom: "1rem", fontWeight: "600", color: "#f8fafc" }}>
              ⚡ Parallel EVM
            </h2>
            <div style={{ marginBottom: "1.5rem" }}>
              <label style={{ display: "flex", alignItems: "center", cursor: "pointer", color: "#94a3b8", fontWeight: "500", marginBottom: "1rem" }}>
                <input
                  type="checkbox"
                  checked={parallelEVMEnabled}
                  onChange={async (e) => {
                    setParallelEVMEnabled(e.target.checked);
                    try {
                      await invoke("enable_parallel_evm", { enabled: e.target.checked });
                      await loadParallelEVMStats();
                    } catch (err: any) {
                      setError(err?.toString?.() ?? "Failed to update Parallel EVM");
                    }
                  }}
                  style={{ marginRight: "0.5rem", width: "18px", height: "18px", cursor: "pointer" }}
                />
                <span>Enable Parallel EVM Execution</span>
              </label>
              <button
                onClick={loadParallelEVMStats}
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
                  opacity: loading ? 0.6 : 1,
                }}
              >
                {loading ? "⏳ Loading..." : "🔄 Refresh Stats"}
              </button>
            </div>
            {parallelEVMStats && (
              <div style={{
                padding: "1.25rem",
                background: "rgba(16, 185, 129, 0.1)",
                border: "1px solid rgba(16, 185, 129, 0.3)",
                borderRadius: 12,
              }}>
                <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(200px, 1fr))", gap: "1rem" }}>
                  <div>
                    <div style={{ color: "#94a3b8", fontSize: "0.85rem", marginBottom: "0.25rem" }}>Status</div>
                    <div style={{ color: parallelEVMStats.enabled ? "#10b981" : "#ef4444", fontSize: "1.1rem", fontWeight: "600" }}>
                      {parallelEVMStats.enabled ? "✅ Enabled" : "❌ Disabled"}
                    </div>
                  </div>
                  <div>
                    <div style={{ color: "#94a3b8", fontSize: "0.85rem", marginBottom: "0.25rem" }}>Max Parallel</div>
                    <div style={{ color: "#f8fafc", fontSize: "1.1rem", fontWeight: "600" }}>
                      {parallelEVMStats.maxParallel || 100}
                    </div>
                  </div>
                  {parallelEVMStats.stats && (
                    <>
                      <div>
                        <div style={{ color: "#94a3b8", fontSize: "0.85rem", marginBottom: "0.25rem" }}>Avg Speedup</div>
                        <div style={{ color: "#10b981", fontSize: "1.1rem", fontWeight: "600" }}>
                          {parallelEVMStats.stats.avgSpeedup?.toFixed(2) || "N/A"}x
                        </div>
                      </div>
                      <div>
                        <div style={{ color: "#94a3b8", fontSize: "0.85rem", marginBottom: "0.25rem" }}>Parallel Rate</div>
                        <div style={{ color: "#06b6d4", fontSize: "1.1rem", fontWeight: "600" }}>
                          {((parallelEVMStats.stats.parallelRate || 0) * 100).toFixed(1)}%
                        </div>
                      </div>
                    </>
                  )}
                </div>
              </div>
            )}
          </section>
        </>
      )}

      {/* Account Abstraction Tab */}
      {activeTab === "account-abstraction" && (
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
          <h2 style={{ fontSize: "1.4rem", marginBottom: "1.5rem", fontWeight: "600", color: "#f8fafc" }}>
            🔐 Account Abstraction
          </h2>

          {/* Wallet Creation */}
          <div style={{ marginBottom: "2rem", padding: "1.5rem", background: "rgba(139, 92, 246, 0.1)", border: "1px solid rgba(139, 92, 246, 0.3)", borderRadius: 12 }}>
            <h3 style={{ fontSize: "1.2rem", marginBottom: "1rem", fontWeight: "600", color: "#f8fafc" }}>
              Create Smart Contract Wallet
            </h3>
            <div style={{ marginBottom: "1rem" }}>
              <label style={{ display: "block", marginBottom: "0.5rem", color: "#94a3b8", fontWeight: "500" }}>
                Wallet Type
              </label>
              <select
                value={walletType}
                onChange={(e) => setWalletType(e.target.value as any)}
                style={{
                  width: "100%",
                  padding: "0.75rem",
                  borderRadius: 8,
                  border: "1px solid rgba(139, 92, 246, 0.3)",
                  background: "rgba(2, 6, 23, 0.6)",
                  color: "#e5e7eb",
                  fontSize: "0.95rem",
                }}
              >
                <option value="basic">Basic Wallet</option>
                <option value="multisig">Multi-Signature Wallet</option>
                <option value="social">Social Recovery Wallet</option>
                <option value="spending">Spending Limit Wallet</option>
                <option value="combined">Combined (Multi-Sig + Recovery + Limits)</option>
              </select>
            </div>
            <div style={{ marginBottom: "1rem" }}>
              <label style={{ display: "block", marginBottom: "0.5rem", color: "#94a3b8", fontWeight: "500" }}>
                Owner Address
              </label>
              <input
                type="text"
                value={walletOwner}
                onChange={(e) => setWalletOwner(e.target.value)}
                placeholder="0x..."
                style={{
                  width: "100%",
                  padding: "0.75rem",
                  borderRadius: 8,
                  border: "1px solid rgba(139, 92, 246, 0.3)",
                  background: "rgba(2, 6, 23, 0.6)",
                  color: "#e5e7eb",
                  fontSize: "0.95rem",
                  fontFamily: "'JetBrains Mono', monospace",
                }}
              />
            </div>
            {(walletType === "multisig" || walletType === "combined") && (
              <div style={{ marginBottom: "1rem" }}>
                <label style={{ display: "block", marginBottom: "0.5rem", color: "#94a3b8", fontWeight: "500" }}>
                  Signers (comma-separated addresses)
                </label>
                <input
                  type="text"
                  value={multisigSigners.join(", ")}
                  onChange={(e) => setMultisigSigners(e.target.value.split(",").map(s => s.trim()).filter(s => s))}
                  placeholder="0x..., 0x..., 0x..."
                  style={{
                    width: "100%",
                    padding: "0.75rem",
                    borderRadius: 8,
                    border: "1px solid rgba(139, 92, 246, 0.3)",
                    background: "rgba(2, 6, 23, 0.6)",
                    color: "#e5e7eb",
                    fontSize: "0.95rem",
                    fontFamily: "'JetBrains Mono', monospace",
                  }}
                />
                <div style={{ marginTop: "0.5rem" }}>
                  <label style={{ display: "block", marginBottom: "0.5rem", color: "#94a3b8", fontSize: "0.9rem" }}>
                    Threshold (n-of-m)
                  </label>
                  <input
                    type="number"
                    value={multisigThreshold}
                    onChange={(e) => setMultisigThreshold(parseInt(e.target.value) || 2)}
                    min="1"
                    max={multisigSigners.length || 1}
                    style={{
                      width: "100%",
                      padding: "0.65rem",
                      borderRadius: 8,
                      border: "1px solid rgba(139, 92, 246, 0.3)",
                      background: "rgba(2, 6, 23, 0.6)",
                      color: "#e5e7eb",
                      fontSize: "0.9rem",
                    }}
                  />
                </div>
              </div>
            )}
            {(walletType === "social" || walletType === "combined") && (
              <div style={{ marginBottom: "1rem" }}>
                <label style={{ display: "block", marginBottom: "0.5rem", color: "#94a3b8", fontWeight: "500" }}>
                  Guardians (comma-separated addresses)
                </label>
                <input
                  type="text"
                  value={guardians.join(", ")}
                  onChange={(e) => setGuardians(e.target.value.split(",").map(s => s.trim()).filter(s => s))}
                  placeholder="0x..., 0x..., 0x..."
                  style={{
                    width: "100%",
                    padding: "0.75rem",
                    borderRadius: 8,
                    border: "1px solid rgba(139, 92, 246, 0.3)",
                    background: "rgba(2, 6, 23, 0.6)",
                    color: "#e5e7eb",
                    fontSize: "0.95rem",
                    fontFamily: "'JetBrains Mono', monospace",
                  }}
                />
                <div style={{ marginTop: "0.5rem" }}>
                  <label style={{ display: "block", marginBottom: "0.5rem", color: "#94a3b8", fontSize: "0.9rem" }}>
                    Recovery Threshold
                  </label>
                  <input
                    type="number"
                    value={recoveryThreshold}
                    onChange={(e) => setRecoveryThreshold(parseInt(e.target.value) || 2)}
                    min="1"
                    max={guardians.length || 1}
                    style={{
                      width: "100%",
                      padding: "0.65rem",
                      borderRadius: 8,
                      border: "1px solid rgba(139, 92, 246, 0.3)",
                      background: "rgba(2, 6, 23, 0.6)",
                      color: "#e5e7eb",
                      fontSize: "0.9rem",
                    }}
                  />
                </div>
              </div>
            )}
            {(walletType === "spending" || walletType === "combined") && (
              <div style={{ marginBottom: "1rem" }}>
                <label style={{ display: "block", marginBottom: "0.5rem", color: "#94a3b8", fontWeight: "500" }}>
                  Spending Limit (MSHW)
                </label>
                <input
                  type="text"
                  value={spendingLimit}
                  onChange={(e) => setSpendingLimit(e.target.value)}
                  placeholder="1000"
                  style={{
                    width: "100%",
                    padding: "0.75rem",
                    borderRadius: 8,
                    border: "1px solid rgba(139, 92, 246, 0.3)",
                    background: "rgba(2, 6, 23, 0.6)",
                    color: "#e5e7eb",
                    fontSize: "0.95rem",
                  }}
                />
              </div>
            )}
            <button
              onClick={createWallet}
              disabled={loading || !walletOwner}
              style={{
                padding: "0.75rem 2rem",
                borderRadius: 8,
                border: "none",
                background: (!walletOwner || loading) ? "rgba(139, 92, 246, 0.5)" : "linear-gradient(135deg, #8b5cf6, #7c3aed)",
                color: "white",
                cursor: (!walletOwner || loading) ? "not-allowed" : "pointer",
                fontWeight: "600",
                fontSize: "1rem",
                boxShadow: (!walletOwner || loading) ? "none" : "0 4px 12px rgba(139, 92, 246, 0.4)",
                transition: "all 0.3s ease",
                opacity: (!walletOwner || loading) ? 0.6 : 1,
                width: "100%",
              }}
            >
              {loading ? "⏳ Creating..." : "✨ Create Wallet"}
            </button>
          </div>

          {/* Wallet List */}
          <div style={{ marginBottom: "2rem" }}>
            <h3 style={{ fontSize: "1.2rem", marginBottom: "1rem", fontWeight: "600", color: "#f8fafc" }}>
              My Smart Contract Wallets
            </h3>
            <button
              onClick={loadWallets}
              disabled={loading}
              style={{
                padding: "0.65rem 1.5rem",
                borderRadius: 8,
                border: "none",
                background: loading ? "rgba(139, 92, 246, 0.5)" : "linear-gradient(135deg, #8b5cf6, #7c3aed)",
                color: "white",
                cursor: loading ? "not-allowed" : "pointer",
                fontWeight: "600",
                fontSize: "0.95rem",
                marginBottom: "1rem",
                opacity: loading ? 0.6 : 1,
              }}
            >
              {loading ? "⏳ Loading..." : "🔄 Refresh Wallets"}
            </button>
            {wallets.length === 0 ? (
              <div style={{ padding: "1.5rem", textAlign: "center", color: "#94a3b8", fontStyle: "italic" }}>
                No wallets found. Create your first smart contract wallet!
              </div>
            ) : (
              <div style={{ display: "grid", gap: "0.75rem" }}>
                {wallets.map((wallet: any) => (
                  <div
                    key={wallet.address}
                    style={{
                      padding: "1rem",
                      background: selectedWallet === wallet.address ? "rgba(139, 92, 246, 0.15)" : "rgba(139, 92, 246, 0.05)",
                      border: selectedWallet === wallet.address ? "2px solid rgba(139, 92, 246, 0.4)" : "1px solid rgba(139, 92, 246, 0.2)",
                      borderRadius: 10,
                      cursor: "pointer",
                      transition: "all 0.3s ease",
                    }}
                    onClick={() => setSelectedWallet(wallet.address)}
                  >
                    <div style={{ display: "flex", justifyContent: "space-between", alignItems: "start" }}>
                      <div style={{ flex: 1 }}>
                        <div style={{ fontSize: "0.9rem", color: "#94a3b8", marginBottom: "0.25rem" }}>
                          {wallet.wallet_type || "Unknown"}
                        </div>
                        <div style={{ 
                          fontSize: "0.85rem", 
                          color: "#8b5cf6",
                          fontFamily: "'JetBrains Mono', monospace",
                          wordBreak: "break-all"
                        }}>
                          {wallet.address}
                        </div>
                      </div>
                      <button
                        onClick={(e) => {
                          e.stopPropagation();
                          viewWalletDetails(wallet.address);
                        }}
                        style={{
                          padding: "0.5rem 1rem",
                          borderRadius: 8,
                          border: "none",
                          background: "linear-gradient(135deg, #06b6d4, #0891b2)",
                          color: "white",
                          cursor: "pointer",
                          fontWeight: "600",
                          fontSize: "0.85rem",
                        }}
                      >
                        View
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        </section>
      )}
      </div>
    </div>
  );
}

export default App;
