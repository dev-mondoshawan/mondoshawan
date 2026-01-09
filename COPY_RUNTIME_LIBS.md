# How to Copy C++ Runtime Libraries from Another Installation

## Quick Guide

### Step 1: Find the Source Files
Run the search script:
```powershell
cd D:\Mondoshawan
.\find-runtime-libs.ps1
```

This will search all Visual Studio installations and show you where `msvcrt.lib` and other runtime libraries are located.

### Step 2: Copy the Files

Once you find a source, copy `msvcrt.lib` (and optionally `vcruntime.lib`) to your installation:

**Source locations to check:**
- Another computer with Visual Studio installed
- Visual Studio 2022 installation (if you have one)
- Visual Studio 2019 installation (if you have one)
- Backup/previous installation

**Destination:**
```
C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\14.50.35717\lib\x64\msvcrt.lib
```

### Step 3: Copy Command (Run as Administrator)

```powershell
# Example - adjust source path to match what you find
$source = "C:\Path\To\Source\msvcrt.lib"
$dest = "C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\14.50.35717\lib\x64\msvcrt.lib"

# Copy (requires admin rights)
Copy-Item $source $dest -Force

# Verify
Test-Path $dest
```

## What Files You Need

### Minimum (Essential):
- **msvcrt.lib** - The main C runtime library (required!)

### Recommended (Complete):
- msvcrt.lib
- vcruntime.lib
- libcmt.lib (static C runtime)
- libvcruntime.lib (static Visual C++ runtime)

## File Size Reference

A proper `msvcrt.lib` should be:
- **50-200 KB** typically
- Much larger than our generated one (which was only a few KB)

If you find a file that's only a few KB, it's likely incomplete or just a stub.

## Alternative: Download Locations

If you can't find files from another installation:

1. **Visual Studio Build Tools**: Download and install just the build tools
2. **Pre-built libraries**: Some repositories provide pre-built msvcrt.lib files
3. **Windows SDK**: Sometimes includes runtime libraries

## After Copying

1. Verify the file exists:
   ```powershell
   Test-Path "C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\14.50.35717\lib\x64\msvcrt.lib"
   ```

2. Check the file size (should be 50-200 KB):
   ```powershell
   (Get-Item "C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\14.50.35717\lib\x64\msvcrt.lib").Length
   ```

3. Try building Rust:
   ```powershell
   cd D:\Mondoshawan\Mondoshawan-blockchain
   cargo build
   ```

## Notes

- **Version compatibility**: Files from VS 2022 (v143) should work fine with VS 2026 (v145)
- **Admin rights required**: You'll need to run PowerShell as Administrator to copy to Program Files
- **Backup**: The generated msvcrt.lib is in `D:\Mondoshawan\msvcrt.lib-generator\x64\` if you need to reference it
