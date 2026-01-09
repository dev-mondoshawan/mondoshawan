# Quick Build Test Guide

## The Problem
- `msvcrt.lib` is missing from your Visual Studio installation
- This prevents Rust from compiling with MSVC toolchain
- GNU toolchain has file access issues

## The Solution: Developer Command Prompt

The **Developer Command Prompt** automatically configures all MSVC paths and can find libraries even if they're in non-standard locations.

## How to Test the Build

### Option 1: Use the Test Script (Easiest)

Run this in PowerShell:
```powershell
cd D:\Mondoshawan
.\test-build.ps1
```

This will:
1. Find Developer Command Prompt automatically
2. Set up the environment
3. Build the project
4. Run tests if build succeeds

### Option 2: Manual Steps

1. **Close this PowerShell window**

2. **Open Start Menu** (Windows key)

3. **Search for**: `Developer Command Prompt for VS 2022`
   - Or: `Developer Command Prompt for VS 2026`
   - Or: `Developer Command Prompt`

4. **Click to open** the Developer Command Prompt

5. **In the Developer Command Prompt**, run:
   ```cmd
   cd D:\Mondoshawan\Mondoshawan-blockchain
   cargo build
   ```

6. **If build succeeds**, run tests:
   ```cmd
   cargo test blockchain::tests
   ```

## Why This Works

The Developer Command Prompt runs `vcvars64.bat` (or `VsDevCmd.bat`) which:
- Sets up all MSVC environment variables
- Configures LIB paths to find runtime libraries
- Sets up INCLUDE paths
- Configures PATH correctly

Even if `msvcrt.lib` isn't in the standard location, the Developer Command Prompt can find it or use alternative configurations.

## Expected Results

### Successful Build:
```
   Compiling Mondoshawan-blockchain v0.1.0 (D:\Mondoshawan\Mondoshawan-blockchain)
    Finished dev [unoptimized + debuginfo] target(s) in X.XXs
```

### Successful Tests:
```
running 5 tests
test blockchain::tests::test_genesis_block ... ok
test blockchain::tests::test_add_block_with_transaction ... ok
test blockchain::tests::test_insufficient_balance ... ok
test blockchain::tests::test_invalid_nonce ... ok
test blockchain::tests::test_duplicate_block ... ok

test result: ok. 5 passed; 0 failed
```

## Troubleshooting

### If Developer Command Prompt doesn't exist:
1. Open **Visual Studio Installer**
2. Click **Modify** on Visual Studio 2026
3. Ensure **"Desktop development with C++"** workload is installed
4. Install if missing, then try again

### If build still fails:
The Developer Command Prompt should work even with your current installation. If it doesn't, we may need to:
1. Install the missing C++ runtime libraries component
2. Or find an alternative build approach

## Next Steps After Successful Build

Once the build works:
1. ✅ Verify all tests pass
2. ✅ Continue with storage layer development
3. ✅ Fix EVM integration
4. ✅ Implement GhostDAG consensus

---

**Try the test script first**: `.\test-build.ps1`


