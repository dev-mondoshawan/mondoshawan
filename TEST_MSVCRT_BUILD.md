# Testing msvcrt.lib Build

## Quick Test

The easiest way to test the build is to use the **Developer Command Prompt** directly.

### Method 1: Developer Command Prompt (Recommended)

1. **Open Developer Command Prompt for VS 2026**
   - Press `Win + S`
   - Type: "Developer Command Prompt"
   - Select: "Developer Command Prompt for VS 2026" (or VS 2022)

2. **Navigate and build**:
   ```cmd
   cd D:\Mondoshawan\msvcrt.lib-generator
   build-msvcrt-simple.cmd
   ```

3. **Wait for completion**
   - The script will:
     - Compile GetFileVersionInfo.exe
     - Extract exports from msvcrt.dll
     - Process exports into .def files
     - Generate x64\msvcrt.lib and x86\msvcrt.lib

### Method 2: Test Script

Run the test script from a regular Command Prompt:

```cmd
cd D:\Mondoshawan
test-msvcrt-direct.cmd
```

### Method 3: PowerShell Test

```powershell
cd D:\Mondoshawan
.\test-msvcrt-build.ps1
```

## Expected Output

If successful, you should see:
```
=== SUCCESS ===
msvcrt.lib files created:
  - x64\msvcrt.lib
  - x86\msvcrt.lib
```

## After Build Success

Once `msvcrt.lib` is created, use it with Rust:

```powershell
# Set LIB environment variable
$env:LIB = "D:\Mondoshawan\msvcrt.lib-generator\x64;$env:LIB"

# Build Rust project
cd D:\Mondoshawan\Mondoshawan-blockchain
cargo build
```

## Troubleshooting

### "cl.exe not recognized"
- Make sure you're using **Developer Command Prompt**, not regular Command Prompt
- Or run `test-msvcrt-direct.cmd` which sets up the environment

### "dumpbin.exe not found"
- Same as above - use Developer Command Prompt

### Build fails at export processing
- The PowerShell export processing might need adjustment
- Check the generated `x64\msvcrt.exports` file to see if exports were extracted

## Files Created

After successful build:
- `D:\Mondoshawan\msvcrt.lib-generator\x64\msvcrt.lib` ← **Use this for Rust (64-bit)**
- `D:\Mondoshawan\msvcrt.lib-generator\x86\msvcrt.lib` ← For 32-bit builds

