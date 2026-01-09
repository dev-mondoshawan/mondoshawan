# Fix: cl.exe Not Found in Developer Command Prompt

## Problem
Even in Developer Command Prompt for VS 2026, `cl.exe` is not found.

## Diagnosis

This usually means:
1. **C++ build tools are not installed** - Most common cause
2. **Developer Command Prompt isn't loading correctly**
3. **Visual Studio installation is incomplete**

## Solution 1: Install C++ Build Tools (RECOMMENDED)

### Step 1: Open Visual Studio Installer
1. Press `Win + S`
2. Search: "Visual Studio Installer"
3. Open it

### Step 2: Modify Installation
1. Find **Visual Studio 2026** (or VS 18)
2. Click **Modify**

### Step 3: Select C++ Workload
1. Under **Workloads**, check:
   - ✅ **Desktop development with C++**

### Step 4: Verify Individual Components
Under **Individual components**, ensure these are checked:
- ✅ **MSVC v143 - VS 2022 C++ x64/x86 build tools (Latest)**
- ✅ **Windows 10/11 SDK** (latest version)
- ✅ **C++ core features**
- ✅ **C++ CMake tools for Windows**

### Step 5: Install
1. Click **Modify** button
2. Wait for installation to complete
3. **Restart your computer** (important!)

### Step 6: Test
1. Open **Developer Command Prompt for VS 2026** again
2. Run: `where cl.exe`
3. Should show a path like: `C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\...\cl.exe`

## Solution 2: Manual PATH Setup (Temporary Fix)

If C++ tools are installed but PATH isn't set:

### Find cl.exe Location
```powershell
Get-ChildItem "C:\Program Files\Microsoft Visual Studio\18" -Recurse -Filter "cl.exe" | Select-Object -First 1
```

### Add to PATH Temporarily
In Developer Command Prompt:
```cmd
set PATH=C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\14.50.35717\bin\Hostx64\x64;%PATH%
```

Replace `14.50.35717` with your actual MSVC version if different.

Then verify:
```cmd
where cl.exe
```

## Solution 3: Use vcvars64.bat Directly

If Developer Command Prompt isn't working, try calling vcvars64.bat directly:

### Find vcvars64.bat
```powershell
Get-ChildItem "C:\Program Files\Microsoft Visual Studio\18" -Recurse -Filter "vcvars64.bat" | Select-Object -First 1
```

### In Regular Command Prompt
```cmd
call "C:\Program Files\Microsoft Visual Studio\18\Community\VC\Auxiliary\Build\vcvars64.bat"
where cl.exe
```

If this works, then the Developer Command Prompt should work too after installing C++ tools.

## Solution 4: Verify Installation

Check if C++ tools are actually installed:

```powershell
# Check for MSVC compiler
Test-Path "C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC"

# List MSVC versions
Get-ChildItem "C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC" -Directory
```

If these paths don't exist, C++ tools are **not installed**.

## Quick Check Commands

Run these in PowerShell to diagnose:

```powershell
# Check VS installation
Test-Path "C:\Program Files\Microsoft Visual Studio\18\Community"

# Find cl.exe
Get-ChildItem "C:\Program Files\Microsoft Visual Studio\18" -Recurse -Filter "cl.exe" -ErrorAction SilentlyContinue | Select-Object -First 1

# Check for vcvars64.bat
Get-ChildItem "C:\Program Files\Microsoft Visual Studio\18" -Recurse -Filter "vcvars64.bat" -ErrorAction SilentlyContinue | Select-Object -First 1
```

## Most Likely Fix

**99% of the time**, this is because "Desktop development with C++" workload is not installed.

1. Open Visual Studio Installer
2. Modify VS 2026
3. Check "Desktop development with C++"
4. Install
5. Restart computer
6. Try again

## After Fix

Once `cl.exe` is available, you can build msvcrt.lib:

```cmd
cd /d D:\Mondoshawan\msvcrt.lib-generator
build-msvcrt-simple.cmd
```
