# Building Mondoshawan - Step by Step

## The Problem
You're getting `cannot open file 'msvcrt.lib'` because the LIB environment variable isn't set.

## Solution: Use Developer Command Prompt (EASIEST)

**This is the recommended way - it sets everything up automatically!**

1. **Close your current PowerShell window**
2. **Open Start Menu** (Windows key)
3. **Search for**: "Developer Command Prompt for VS 2022"
4. **Click** to open it
5. **Run**:
   ```cmd
   cd D:\Mondoshawan\Mondoshawan-blockchain
   cargo build
   ```

That's it! The Developer Command Prompt automatically sets up all the paths.

## Alternative: Setup Script in PowerShell

If you want to stay in PowerShell, run this first:

```powershell
cd D:\Mondoshawan\Mondoshawan-blockchain
..\setup-msvc-env.ps1
cargo build
```

The `setup-msvc-env.ps1` script will configure the environment for you.

## Why Developer Command Prompt is Better

The Developer Command Prompt runs `vcvars64.bat` which:
- ✓ Sets up LIB path (finds msvcrt.lib, kernel32.lib, etc.)
- ✓ Sets up PATH for linker
- ✓ Configures Windows SDK paths
- ✓ Sets up all MSVC environment variables correctly

## Quick Test

After running the setup script or using Developer Command Prompt:

```cmd
echo %LIB%
```

You should see paths like:
```
C:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\14.50.35717\lib\x64;...
```

## Summary

**Best solution**: Use Developer Command Prompt for VS 2022
**Alternative**: Run `setup-msvc-env.ps1` before `cargo build`

