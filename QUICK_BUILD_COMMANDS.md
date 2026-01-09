# Quick Build Commands for msvcrt.lib

## In Developer Command Prompt

### Step 1: Navigate to the build directory
```cmd
cd /d D:\Mondoshawan\msvcrt.lib-generator
```

**Important**: Use `/d` flag to change drives (C: to D:)

### Step 2: Run the build script
```cmd
build-msvcrt-simple.cmd
```

## Alternative: One-Line Command

You can also do it in one line:
```cmd
cd /d D:\Mondoshawan\msvcrt.lib-generator && build-msvcrt-simple.cmd
```

## What to Expect

The build will:
1. Set up MSVC environment (if needed)
2. Compile GetFileVersionInfo.exe
3. Extract exports from msvcrt.dll (x64 and x86)
4. Process exports into .def files
5. Generate x64\msvcrt.lib and x86\msvcrt.lib

**Time**: Takes about 1-2 minutes

## After Build Success

Once `x64\msvcrt.lib` is created:

```powershell
# In PowerShell
$env:LIB = "D:\Mondoshawan\msvcrt.lib-generator\x64;$env:LIB"
cd D:\Mondoshawan\Mondoshawan-blockchain
cargo build
```

Or in Developer Command Prompt:
```cmd
set LIB=D:\Mondoshawan\msvcrt.lib-generator\x64;%LIB%
cd /d D:\Mondoshawan\Mondoshawan-blockchain
cargo build
```

## Troubleshooting

### "The system cannot find the path specified"
- Make sure you use `cd /d` to change drives
- Or first type `D:` to switch to D: drive, then `cd \Mondoshawan\msvcrt.lib-generator`

### "cl.exe not recognized"
- Make sure you're in Developer Command Prompt
- The script should auto-detect and set up, but if it fails, the Developer Command Prompt should have it

### Build fails at any step
- Check the error message
- Make sure Visual Studio C++ tools are installed
- Try running from Developer Command Prompt (which you're already doing!)
