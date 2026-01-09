# Where C++ Runtime Libraries Are Installed

## Primary Locations

### 1. MSVC Library Directory (Main Location)
**Path**: `C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\<version>\lib\x64\`

**Key Files Needed**:
- `msvcrt.lib` - C runtime library (the one we're missing!)
- `vcruntime.lib` - Visual C++ runtime
- `libcmt.lib` - C runtime (static)
- `libvcruntime.lib` - Visual C++ runtime (static)

**Example Full Path**:
```
C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\14.50.35717\lib\x64\msvcrt.lib
```

### 2. Windows SDK UCRT Directory
**Path**: `C:\Program Files (x86)\Windows Kits\10\Lib\<version>\ucrt\x64\`

**Key Files**:
- `ucrt.lib` - Universal C Runtime
- Sometimes contains `msvcrt.lib` as well

**Example Full Path**:
```
C:\Program Files (x86)\Windows Kits\10\Lib\10.0.19041.0\ucrt\x64\ucrt.lib
```

### 3. Alternative Locations to Check

#### Visual Studio 2022 (if installed)
```
C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\<version>\lib\x64\
```

#### Visual Studio 2019 (if installed)
```
C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Tools\MSVC\<version>\lib\x64\
```

#### Program Files (x86) location (older installs)
```
C:\Program Files (x86)\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\<version>\lib\x64\
```

## What Files You Need to Copy

### Essential Files (Minimum):
1. **msvcrt.lib** - The main C runtime library
2. **vcruntime.lib** - Visual C++ runtime (often needed alongside msvcrt.lib)

### Recommended (Complete Set):
1. `msvcrt.lib`
2. `vcruntime.lib`
3. `libcmt.lib` (static C runtime)
4. `libvcruntime.lib` (static Visual C++ runtime)
5. `msvcprt.lib` (C++ standard library)
6. `libcpmt.lib` (static C++ standard library)

## How to Copy from Another Installation

### Step 1: Find the Source
1. Check if you have Visual Studio 2022 or 2019 installed
2. Or check another computer with a working VS installation
3. Or check if there's a backup/previous installation

### Step 2: Locate the Files
Run this PowerShell to find all msvcrt.lib files:
```powershell
Get-ChildItem "C:\Program Files*" -Recurse -Filter "msvcrt.lib" -ErrorAction SilentlyContinue | Select-Object FullName
```

### Step 3: Copy to Your Installation
```powershell
# Source (example - adjust to your actual path)
$source = "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.40.33807\lib\x64\msvcrt.lib"

# Destination
$dest = "C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\14.50.35717\lib\x64\msvcrt.lib"

# Copy (may need admin rights)
Copy-Item $source $dest -Force
```

## Quick Search Script

Save this as `find-runtime-libs.ps1`:

```powershell
Write-Host "=== Searching for C++ Runtime Libraries ===" -ForegroundColor Cyan

# Search all Program Files locations
$searchPaths = @(
    "C:\Program Files\Microsoft Visual Studio",
    "C:\Program Files (x86)\Microsoft Visual Studio",
    "D:\Program Files\Microsoft Visual Studio"
)

$libs = @("msvcrt.lib", "vcruntime.lib", "libcmt.lib")

foreach ($path in $searchPaths) {
    if (Test-Path $path) {
        Write-Host "`nSearching: $path" -ForegroundColor Yellow
        foreach ($lib in $libs) {
            $found = Get-ChildItem $path -Recurse -Filter $lib -ErrorAction SilentlyContinue | Select-Object -First 1
            if ($found) {
                Write-Host "  ✓ $lib found at:" -ForegroundColor Green
                Write-Host "    $($found.FullName)" -ForegroundColor Cyan
            }
        }
    }
}
```

## After Copying

1. **Verify the file exists**:
   ```powershell
   Test-Path "C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\14.50.35717\lib\x64\msvcrt.lib"
   ```

2. **Try building Rust again**:
   ```powershell
   cd D:\Mondoshawan\Mondoshawan-blockchain
   cargo build
   ```

## Alternative: Download from Official Source

If you can't find files from another installation, you might be able to:
1. Download Visual Studio Build Tools separately
2. Extract the runtime libraries from the installer
3. Or use a pre-built msvcrt.lib from a trusted source

## Notes

- **File sizes**: A proper `msvcrt.lib` is typically **50-200 KB** (much larger than our generated one)
- **Version compatibility**: Files from VS 2022 (v143) should work with VS 2026 (v145)
- **Admin rights**: You'll likely need admin rights to copy to Program Files
- **Backup first**: Consider backing up the generated msvcrt.lib before replacing it
