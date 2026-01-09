# Navigation Help

## Problem: Can't Change Directory

If you're having trouble with `cd D:\Mondoshawan\Mondoshawan-blockchain`, here are solutions:

## Solution 1: Use PowerShell Navigation

In PowerShell, use:
```powershell
Set-Location D:\Mondoshawan\Mondoshawan-blockchain
```

Or the short version:
```powershell
cd D:\Mondoshawan\Mondoshawan-blockchain
```

## Solution 2: Use the Helper Script

I created a helper script for you:

```powershell
cd D:\Mondoshawan
.\go-to-rust.ps1
```

This will automatically navigate to the Rust project directory.

## Solution 3: Check Your Current Location

First, see where you are:
```powershell
Get-Location
# or
pwd
```

Then navigate step by step:
```powershell
cd D:\
cd Mondoshawan
cd Mondoshawan-blockchain
```

## Solution 4: Use Full Path in One Command

```powershell
Set-Location -Path "D:\Mondoshawan\Mondoshawan-blockchain"
```

## Solution 5: If Using CMD (Command Prompt)

In Command Prompt (not PowerShell), use:
```cmd
cd /d D:\Mondoshawan\Mondoshawan-blockchain
```

The `/d` flag is needed in CMD to change drives.

## Solution 6: Verify the Path Exists

Check if the directory exists:
```powershell
Test-Path "D:\Mondoshawan\Mondoshawan-blockchain"
```

If it returns `False`, the directory might not exist or have a different name.

## Common Issues

### Issue: "Cannot find path"
- Check if you're in the right drive (D:)
- Verify the directory name is exactly `Mondoshawan-blockchain` (with hyphen, not underscore)

### Issue: "Access Denied"
- Make sure you have permissions
- Try running as Administrator

### Issue: Different Shell
- If you're in CMD, use: `cd /d D:\Mondoshawan\Mondoshawan-blockchain`
- If you're in PowerShell, use: `Set-Location D:\Mondoshawan\Mondoshawan-blockchain`

## Quick Test

Run this to verify everything:
```powershell
cd D:\Mondoshawan
Get-ChildItem | Select-Object Name
```

You should see `Mondoshawan-blockchain` in the list.

## Once You're In the Directory

After successfully navigating, you can build:
```powershell
cargo build
```

Or if using Developer Command Prompt:
```cmd
cargo build
```

