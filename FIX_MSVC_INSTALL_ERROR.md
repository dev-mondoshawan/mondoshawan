# Fix: MSVC v143 Installation Error

## Problem
Visual Studio Installer errors when trying to install MSVC v143 and references a file that doesn't exist.

## Common Causes

1. **Corrupted installation cache**
2. **Missing prerequisites**
3. **File path issues**
4. **Version mismatch** (VS 2026 might use a different MSVC version)

## Solution 1: Clear Installation Cache

### Step 1: Close Visual Studio Installer
Make sure it's completely closed.

### Step 2: Clear Cache
1. Press `Win + R`
2. Type: `%ProgramData%\Microsoft\VisualStudio\Packages`
3. Press Enter
4. **Delete all contents** (or rename the folder as backup)
5. Restart Visual Studio Installer

### Step 3: Try Installation Again
1. Open Visual Studio Installer
2. Modify VS 2026
3. Try installing "Desktop development with C++" again

## Solution 2: Use Available MSVC Version

VS 2026 might use a **different MSVC version** (not v143). Check what's available:

### Check Installed Versions
```powershell
Get-ChildItem "C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC" -Directory
```

### Install Without Specific Version
1. In Visual Studio Installer
2. Under **Individual components**
3. Look for **MSVC** options (may be listed differently)
4. Install the **latest available** MSVC version
5. Don't specifically select "v143" - let it install what's compatible

## Solution 3: Install Build Tools Separately

If VS Installer keeps failing:

### Option A: Download Build Tools Standalone
1. Go to: https://visualstudio.microsoft.com/downloads/
2. Download: **Build Tools for Visual Studio 2022** (or 2026 if available)
3. Run installer
4. Select: **C++ build tools** workload
5. Install

### Option B: Use vswhere to Find What's Installed
```powershell
& "C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
```

## Solution 4: Manual Component Installation

### Check What's Actually Needed
VS 2026 might need:
- **MSVC v143** (VS 2022 toolset) - OR
- **MSVC v144** or newer (VS 2026 toolset)

### Try Installing These Instead:
1. **MSVC - latest available** (not specifically v143)
2. **Windows SDK** (latest)
3. **C++ core features**
4. **CMake tools**

## Solution 5: Check Error Logs

### Find Installation Logs
1. Press `Win + R`
2. Type: `%TEMP%`
3. Look for files like: `dd_*.log` or `vs_*.log`
4. Open the most recent log
5. Search for the error message
6. Look for the file path it's trying to access

### Common Log Locations
- `%TEMP%\dd_*.log`
- `%TEMP%\vs_*.log`
- `%ProgramData%\Microsoft\VisualStudio\Packages\*.log`

## Solution 6: Repair Installation

1. Open Visual Studio Installer
2. Find VS 2026
3. Click **More** → **Repair**
4. Wait for repair to complete
5. Try installing C++ tools again

## Solution 7: Use What's Already There

If VS 2026 is installed but MSVC v143 won't install, check if a different version is available:

```powershell
# Check for any MSVC installation
Get-ChildItem "C:\Program Files\Microsoft Visual Studio\18" -Recurse -Filter "cl.exe" -ErrorAction SilentlyContinue | Select-Object -First 1
```

If this finds `cl.exe`, you can use it even if it's not v143!

## Quick Diagnostic Commands

Run these to understand what's installed:

```powershell
# Check VS installation
Test-Path "C:\Program Files\Microsoft Visual Studio\18\Community"

# Check for any MSVC
Get-ChildItem "C:\Program Files\Microsoft Visual Studio\18" -Recurse -Directory -Filter "MSVC" -ErrorAction SilentlyContinue

# Check for cl.exe anywhere
Get-ChildItem "C:\Program Files\Microsoft Visual Studio\18" -Recurse -Filter "cl.exe" -ErrorAction SilentlyContinue | Select-Object -First 1

# Check for vcvars64.bat
Get-ChildItem "C:\Program Files\Microsoft Visual Studio\18" -Recurse -Filter "vcvars64.bat" -ErrorAction SilentlyContinue | Select-Object -First 1
```

## Alternative: Skip MSVC v143

If v143 won't install, try:
1. Install **Desktop development with C++** workload (without selecting specific v143)
2. Let it install whatever MSVC version it wants
3. Any recent MSVC version should work for building msvcrt.lib

## Next Steps

After resolving the installation:
1. Restart computer
2. Open Developer Command Prompt for VS 2026
3. Run: `where cl.exe`
4. If it works, proceed with building msvcrt.lib
