# Visual Studio Setup for Rust

## Current Status

Visual Studio is installed on D drive, but the C++ build tools (linker) are not currently accessible to Rust.

## Issue

Rust's MSVC toolchain requires:
- Visual Studio C++ build tools
- MSVC linker (link.exe)
- Windows SDK

## Solutions

### Option 1: Install C++ Build Tools Component

If Visual Studio is installed but C++ tools are missing:

1. Open Visual Studio Installer
2. Click "Modify" on your Visual Studio installation
3. Select "Desktop development with C++" workload
4. Ensure these components are selected:
   - MSVC v143 - VS 2022 C++ x64/x86 build tools
   - Windows 10/11 SDK (latest version)
   - C++ CMake tools for Windows
5. Click "Modify" to install

### Option 2: Use Developer Command Prompt

1. Open "Developer Command Prompt for VS 2022" (or VS 2026)
2. Navigate to project: `cd D:\Mondoshawan\Mondoshawan-blockchain`
3. Run: `cargo build`

The Developer Command Prompt automatically sets up the environment.

### Option 3: Manual Environment Setup

Create a PowerShell script to set up the environment:

```powershell
# Find vcvars64.bat
$vcvars = Get-ChildItem "D:\Program Files\Microsoft Visual Studio" -Recurse -Filter "vcvars64.bat" -ErrorAction SilentlyContinue | Select-Object -First 1

if ($vcvars) {
    # Run vcvars64.bat and capture environment
    cmd /c "`"$($vcvars.FullName)`" && set" | ForEach-Object {
        if ($_ -match "^([^=]+)=(.*)$") {
            [System.Environment]::SetEnvironmentVariable($matches[1], $matches[2], "Process")
        }
    }
}

# Set Rust paths
$env:CARGO_HOME = "D:\Rust\.cargo"
$env:RUSTUP_HOME = "D:\Rust\.rustup"
$env:PATH = "D:\Rust\.cargo\bin;$env:PATH"
```

### Option 4: Switch to GNU Toolchain

If you prefer not to use MSVC:

```powershell
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

This requires MinGW-w64 to be installed.

## Verification

After setup, verify:

```powershell
# Check if link.exe is available
Get-Command link.exe

# Test Rust compilation
cd D:\Mondoshawan\Mondoshawan-blockchain
cargo check
```

## Next Steps

1. Verify C++ build tools are installed in Visual Studio
2. Use Developer Command Prompt for immediate access
3. Or set up environment variables as shown above

