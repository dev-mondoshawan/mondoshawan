# Fix: WSL ERROR_PATH_NOT_FOUND

## The Problem

You got this error:
```
The system cannot find the path specified.
Error code: Wsl/ERROR_PATH_NOT_FOUND
```

## The Cause

**The `D:\WSL` directory didn't exist!**

When you try to export or import WSL distributions, WSL needs the target directory to exist first.

## The Fix

I've already created the directory for you. But if you need to do it manually:

```powershell
# Create the directory
New-Item -ItemType Directory -Path "D:\WSL" -Force
```

## Now You Can Run

### Option 1: Use the Fixed Script
```powershell
.\move-wsl-to-d.ps1
```

The script now:
- ✅ Creates `D:\WSL` directory automatically
- ✅ Creates `D:\WSL\Ubuntu` directory automatically
- ✅ Verifies paths exist before running WSL commands
- ✅ Better error handling

### Option 2: Run Commands Manually

Now that `D:\WSL` exists, you can run:

```powershell
# 1. Stop WSL
wsl --shutdown

# 2. Export (this will work now)
wsl --export Ubuntu-24.04 D:\WSL\Ubuntu_backup.tar

# 3. Unregister
wsl --unregister Ubuntu-24.04

# 4. Import (create Ubuntu directory first)
New-Item -ItemType Directory -Path "D:\WSL\Ubuntu" -Force
wsl --import Ubuntu-24.04 D:\WSL\Ubuntu D:\WSL\Ubuntu_backup.tar
```

## Diagnostic Script

If you get path errors again, run:

```powershell
.\fix-wsl-path-error.ps1
```

This will:
- Check WSL status
- Create missing directories
- Test write permissions
- Check disk space
- Verify everything works

## Common Path Errors

### Error: "The system cannot find the path specified"

**Causes:**
1. ✅ **Directory doesn't exist** (FIXED - D:\WSL now exists)
2. Invalid characters in path
3. Insufficient permissions
4. Typo in distribution name

**Solutions:**
- Run `.\fix-wsl-path-error.ps1` to diagnose
- Run PowerShell as Administrator if needed
- Check distribution name: `wsl --list`
- Verify path exists: `Test-Path D:\WSL`

### Error: "Access Denied"

**Solution:**
- Run PowerShell as Administrator
- Check file permissions on D drive

### Error: "Not enough space"

**Solution:**
- Need at least 30 GB free on D drive
- Check space: `Get-PSDrive D`

## Quick Test

Test if WSL can now access the path:

```powershell
# This should work now
wsl --list --verbose

# Test export (small test)
wsl --export Ubuntu-24.04 D:\WSL\test.tar
# (Then delete test.tar if it works)
```

## Summary

✅ **Fixed**: `D:\WSL` directory created  
✅ **Updated**: `move-wsl-to-d.ps1` now creates directories automatically  
✅ **Created**: `fix-wsl-path-error.ps1` for diagnostics  

**You can now run:**
```powershell
.\move-wsl-to-d.ps1
```

The path error should be resolved!

