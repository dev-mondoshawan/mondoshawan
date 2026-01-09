# Quick Guide: Move WSL to D Drive

## Where to Run

**Run from PowerShell** - You can run from:
- Current location: `D:\Mondoshawan` ✅ (recommended)
- Any PowerShell window
- Any Command Prompt window
- Windows Terminal

## Your WSL Distributions

You have:
1. **Ubuntu-24.04** (default, 27.19 GB on C drive)
2. **Ubuntu** (also on C drive)
3. **docker-desktop** (for Docker)

## Easiest Method: Use the Script

### Step 1: Open PowerShell
- You're already in `D:\Mondoshawan` - that's perfect!
- Or open any PowerShell window

### Step 2: Run the Script
```powershell
.\move-wsl-to-d.ps1
```

The script will:
- Show your distributions
- Ask which one to move
- Export it to D drive
- Remove from C drive
- Import to D drive
- Verify it works

### Step 3: Follow Prompts
- The script will ask which Ubuntu to move
- Choose 1 for Ubuntu-24.04 (the 27 GB one)
- Or choose 2 for Ubuntu
- Confirm when prompted

## Manual Method (If You Prefer)

### In PowerShell or CMD:

```powershell
# 1. Stop WSL
wsl --shutdown

# 2. Export Ubuntu-24.04 (the big one)
wsl --export Ubuntu-24.04 D:\WSL\Ubuntu_backup.tar

# 3. Unregister from C drive
wsl --unregister Ubuntu-24.04

# 4. Import to D drive
wsl --import Ubuntu-24.04 D:\WSL\Ubuntu D:\WSL\Ubuntu_backup.tar

# 5. Verify
wsl --list --verbose

# 6. Delete backup (after verifying it works)
Remove-Item D:\WSL\Ubuntu_backup.tar
```

## Time Estimate

- Export: 10-20 minutes (for 27 GB)
- Import: 10-20 minutes
- **Total: 20-40 minutes**

## Space Requirements

- Need ~30 GB free on D drive temporarily
- After move: ~27 GB freed on C drive
- Backup file can be deleted after verification

## After Moving

Ubuntu will work exactly the same:
```powershell
wsl
# or
wsl -d Ubuntu-24.04
```

But it will be stored on D drive instead of C drive!

## Quick Start

**Right now, in your current PowerShell window:**

```powershell
# Just run this:
.\move-wsl-to-d.ps1
```

That's it! The script handles everything.

