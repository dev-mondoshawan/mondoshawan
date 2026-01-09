# msvcrt.lib Issue - Missing Symbols

## Problem
The generated `msvcrt.lib` from DLL exports is missing critical symbols that Rust needs:
- `mainCRTStartup`
- `__chkstk`
- `_tls_index`
- `_tls_used`
- `_fltused`
- RTTI symbols (`type_info::vftable`)

## Root Cause
The generated `msvcrt.lib` only contains exported functions from `msvcrt.dll`, but Rust's standard library needs internal C runtime symbols that aren't exported from the DLL.

## Solutions

### Solution 1: Install Proper C++ Runtime (RECOMMENDED)
The proper `msvcrt.lib` should come with Visual Studio's C++ runtime libraries. Since MSVC v143 won't install, try:

1. **Install a different MSVC version** (v142, v141, etc.)
2. **Install "C++ runtime libraries" component** separately
3. **Use Visual Studio Installer** → Modify → Individual components → Search for "runtime"

### Solution 2: Switch to GNU Toolchain
Use MinGW instead of MSVC:

```powershell
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

Then build with:
```powershell
cargo build
```

### Solution 3: Download Pre-built msvcrt.lib
Some repositories provide pre-built `msvcrt.lib` files:
- https://github.com/neosmart/msvcrt.lib/releases
- Download the one matching your Windows version

### Solution 4: Use vcruntime.lib Instead
Try linking against `vcruntime.lib` which might have the missing symbols:

```powershell
# Find vcruntime.lib
Get-ChildItem "C:\Program Files\Microsoft Visual Studio\18" -Recurse -Filter "vcruntime.lib"
```

Then add it to LIB path or copy alongside msvcrt.lib.

## Current Status
- ✓ `msvcrt.lib` generated successfully
- ✗ Missing internal C runtime symbols
- ✗ Rust build fails with unresolved externals

## Next Steps
1. Try Solution 2 (GNU toolchain) - easiest workaround
2. Or try to install proper C++ runtime via Visual Studio Installer
3. Or download pre-built msvcrt.lib from releases
