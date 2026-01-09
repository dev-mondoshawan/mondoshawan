# Using Generated msvcrt.lib

## Problem
Your Visual Studio installation is missing `msvcrt.lib`, which prevents Rust from compiling.

## Solution
Generate `msvcrt.lib` from the system's `msvcrt.dll` using the neosmart generator.

## Quick Start

### Option 1: Automated Build (Recommended)

Run this in PowerShell:
```powershell
cd D:\Mondoshawan
.\build-msvcrt-lib.ps1
```

This will:
1. Find Developer Command Prompt
2. Build msvcrt.lib automatically
3. Show you how to use it

### Option 2: Manual Build

1. **Open Developer Command Prompt for VS 2026**
   - Start Menu → Search: "Developer Command Prompt" (for VS 2026)

2. **Navigate and build**:
   ```cmd
   cd D:\Mondoshawan\msvcrt.lib-generator
   build-msvcrt-simple.cmd
   ```

3. **Wait for build to complete**
   - Should create `x64\msvcrt.lib` and `x86\msvcrt.lib`

## Using the Generated Library

### Method 1: Add to LIB Environment Variable (Temporary)

In PowerShell:
```powershell
$env:LIB = "D:\Mondoshawan\msvcrt.lib-generator\x64;$env:LIB"
cd D:\Mondoshawan\Mondoshawan-blockchain
cargo build
```

### Method 2: Copy to MSVC Lib Directory (Permanent)

```powershell
# Find your MSVC lib directory
$msvcLib = "C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\14.50.35717\lib\x64"

# Copy the file
Copy-Item "D:\Mondoshawan\msvcrt.lib-generator\x64\msvcrt.lib" $msvcLib
```

Then build normally:
```powershell
cd D:\Mondoshawan\Mondoshawan-blockchain
cargo build
```

### Method 3: Use in Developer Command Prompt

1. Open Developer Command Prompt
2. Set LIB:
   ```cmd
   set LIB=D:\Mondoshawan\msvcrt.lib-generator\x64;%LIB%
   ```
3. Build:
   ```cmd
   cd D:\Mondoshawan\Mondoshawan-blockchain
   cargo build
   ```

## Verification

After building msvcrt.lib, verify it exists:
```powershell
Test-Path "D:\Mondoshawan\msvcrt.lib-generator\x64\msvcrt.lib"
```

Should return: `True`

## Important Notes

⚠️ **Warning**: `msvcrt.dll` is the unversioned C runtime. Microsoft doesn't guarantee API stability. This is a workaround for development.

✅ **For Production**: Consider installing the proper C++ runtime libraries component via Visual Studio Installer.

## Troubleshooting

### Build fails with "cl.exe not found"
- Make sure you're using Developer Command Prompt
- Or run `build-msvcrt-lib.ps1` which sets up the environment automatically

### Build fails with "dumpbin.exe not found"
- Same as above - use Developer Command Prompt

### Rust still can't find msvcrt.lib
- Make sure LIB environment variable includes the path
- Or copy the file to the MSVC lib directory
- Restart your terminal after setting environment variables

## Alternative: Download Pre-built

If building fails, you can download a pre-built version from:
https://github.com/neosmart/msvcrt.lib/releases

Look for a release matching your Windows version.


