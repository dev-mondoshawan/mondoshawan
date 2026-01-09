# Fix: msvcrt.lib Error

## Error Message
```
LINK : fatal error LNK1104: cannot open file 'msvcrt.lib'
```

## Root Cause
The **C++ runtime libraries** component is not installed. Your Visual Studio installation has:
- ✅ MSVC compiler
- ✅ Linker (link.exe)
- ✅ Clang libraries
- ❌ **Missing**: MSVC runtime libraries (msvcrt.lib, libcmt.lib, etc.)

## Solutions (Choose One)

### Solution 1: Use Developer Command Prompt ⭐ RECOMMENDED

This is the **easiest and most reliable** solution.

**Steps:**
1. Close your current PowerShell/terminal
2. Open **Start Menu**
3. Search for: **"Developer Command Prompt for VS 2022"**
4. Click to open (it will open a CMD window)
5. Navigate to your project:
   ```cmd
   cd D:\Mondoshawan\Mondoshawan-blockchain
   ```
6. Build:
   ```cmd
   cargo build
   ```

**Why This Works**: The Developer Command Prompt automatically runs `vcvars64.bat` which sets up the complete environment, including finding `msvcrt.lib` even if it's in a non-standard location or if the component is partially installed.

### Solution 2: Install Missing C++ Component

Install the C++ runtime libraries component.

**Steps:**
1. Open **Visual Studio Installer**
   - Search "Visual Studio Installer" in Start Menu
   - Or: `C:\Program Files (x86)\Microsoft Visual Studio\Installer\vs_installer.exe`

2. Find **Visual Studio 2026** (or VS 18)
   - Click **Modify**

3. Under **Workloads**, ensure:
   - ✅ **Desktop development with C++** is checked

4. Under **Individual components**, search for and check:
   - ✅ **C++ runtime libraries** ← **This is the missing piece**
   - ✅ **MSVC v143 - VS 2022 C++ x64/x86 build tools (Latest)**
   - ✅ **Windows 10 SDK (10.0.19041.0)** or later

5. Click **Modify** to install (this may take 10-30 minutes)

6. **Restart your terminal/PowerShell**

7. Test:
   ```powershell
   cd D:\Mondoshawan\Mondoshawan-blockchain
   cargo build
   ```

### Solution 3: Run Fix Script

I've created a script that attempts to fix the environment:

```powershell
cd D:\Mondoshawan
.\fix-msvcrt-lib.ps1
cd Mondoshawan-blockchain
cargo build
```

**Note**: This may not work if `msvcrt.lib` is truly missing, but it will try to set up the environment correctly.

### Solution 4: Switch to GNU Toolchain

If MSVC continues to be problematic, switch to the GNU toolchain:

```powershell
# Install GNU toolchain
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu

# Install MinGW-w64 (required)
# Download from: https://www.mingw-w64.org/downloads/
# Or use: winget install mingw-w64

# Then build
cd D:\Mondoshawan\Mondoshawan-blockchain
cargo build
```

## Quick Test

After applying a solution:

```powershell
# Verify linker can find msvcrt.lib
link.exe /?

# If that works, try building
cd D:\Mondoshawan\Mondoshawan-blockchain
cargo build
```

## Why This Happened

Your Visual Studio installation has:
- ✅ **Clang tools** (29 Clang libraries found)
- ❌ **MSVC runtime libraries** (msvcrt.lib missing)

This suggests the installation focused on Clang/LLVM tools rather than the full MSVC runtime. The Developer Command Prompt can work around this by using alternative paths or configurations.

## Recommendation

**Use Solution 1 (Developer Command Prompt)** - It's the quickest solution and doesn't require reinstalling anything. It should work even with your current installation.

If that doesn't work, then use **Solution 2** to install the missing component.

## Files Created

- `fix-msvcrt-lib.ps1` - Script to attempt automatic fix
- `FIX_MSVCRT_ERROR.md` - This guide

