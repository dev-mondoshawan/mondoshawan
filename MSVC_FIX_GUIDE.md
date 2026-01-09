# MSVC Linker Issue - Complete Fix Guide

## Problem
`msvcrt.lib` cannot be found, causing Rust compilation to fail with:
```
LINK : fatal error LNK1104: cannot open file 'msvcrt.lib'
```

## Root Cause
The C++ runtime library (`msvcrt.lib`) is missing or not in the LIB path. This typically means:
1. C++ build tools component is not fully installed
2. The library is in a different location
3. Environment variables are not set correctly

## Solution 1: Use Developer Command Prompt (RECOMMENDED)

**This is the easiest and most reliable solution:**

1. Open **Start Menu**
2. Search for: **"Developer Command Prompt for VS 2022"**
3. Click to open
4. Run:
   ```cmd
   cd D:\Mondoshawan\Mondoshawan-blockchain
   cargo build
   ```

The Developer Command Prompt automatically runs `vcvars64.bat` which sets up ALL required paths, including `msvcrt.lib`.

## Solution 2: Install Missing C++ Components

If `msvcrt.lib` is truly missing:

1. Open **Visual Studio Installer**
2. Click **Modify** on your VS 2022 installation
3. Ensure these are checked:
   - ✅ **Desktop development with C++**
   - ✅ **MSVC v143 - VS 2022 C++ x64/x86 build tools**
   - ✅ **Windows 10/11 SDK** (latest version)
   - ✅ **C++ CMake tools for Windows**
4. Click **Modify** to install
5. Restart your terminal

## Solution 3: Manual Environment Setup

If you must use PowerShell, try this:

```powershell
# Find and run vcvars64.bat
$vcvars = Get-ChildItem "C:\Program Files\Microsoft Visual Studio\18" -Recurse -Filter "vcvars64.bat" | Select-Object -First 1

if ($vcvars) {
    # Run vcvars64.bat in a new cmd process and capture environment
    $envVars = cmd /c "`"$($vcvars.FullName)`" && set"
    
    foreach ($line in $envVars) {
        if ($line -match "^([^=]+)=(.*)$") {
            [System.Environment]::SetEnvironmentVariable($matches[1], $matches[2], "Process")
        }
    }
}
```

## Solution 4: Switch to GNU Toolchain

If MSVC continues to cause issues, switch to GNU toolchain:

```powershell
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

This requires MinGW-w64 to be installed separately.

## Verification

After setup, verify:

```cmd
# Check if msvcrt.lib is accessible
where msvcrt.lib

# Or check LIB environment variable
echo %LIB%
```

You should see paths including:
- `C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\*\lib\x64`
- `C:\Program Files (x86)\Windows Kits\10\Lib\*\um\x64`

## Why Developer Command Prompt Works

The Developer Command Prompt:
1. Automatically runs `vcvars64.bat`
2. Sets up LIB, PATH, INCLUDE correctly
3. Finds all required libraries
4. Configures everything for MSVC compilation

## Quick Test

In Developer Command Prompt:
```cmd
cd D:\Mondoshawan\Mondoshawan-blockchain
cargo check
```

If this works, the environment is correctly configured!

## Summary

**Best Solution**: Use Developer Command Prompt for VS 2022  
**Alternative**: Install missing C++ components via Visual Studio Installer  
**Last Resort**: Switch to GNU toolchain

The Developer Command Prompt is the most reliable because it uses Microsoft's official environment setup script.

