# How to Move WSL Ubuntu to D Drive

## Where to Run WSL Commands

### Option 1: PowerShell (Recommended)
1. **Open PowerShell** (regular or as Administrator)
2. **Run the commands** directly in PowerShell
3. **Location**: Any directory works, but I recommend:
   ```powershell
   cd D:\Mondoshawan
   ```

### Option 2: Command Prompt (CMD)
1. **Open Command Prompt** (regular or as Administrator)
2. **Run the same commands** (they work in CMD too)
3. **Location**: Any directory

### Option 3: Windows Terminal
1. **Open Windows Terminal**
2. **Run commands** in PowerShell or CMD tab
3. **Location**: Any directory

## Step-by-Step: Move WSL Ubuntu to D Drive

### Step 1: Check Current WSL Installation
```powershell
# Run in PowerShell or CMD
wsl --list --verbose
```

This shows:
- Installed distributions (Ubuntu, etc.)
- Which one is running
- Version (WSL 1 or 2)

### Step 2: Stop WSL (if running)
```powershell
# Stop all WSL distributions
wsl --shutdown
```

### Step 3: Export Ubuntu to D Drive
```powershell
# Export Ubuntu to a tar file on D drive
wsl --export Ubuntu D:\WSL\Ubuntu_backup.tar
```

**Note**: 
- Replace `Ubuntu` with your actual distribution name if different
- This creates a backup file (~20-25 GB)
- Takes a few minutes

### Step 4: Unregister from C Drive
```powershell
# Remove Ubuntu from C drive
wsl --unregister Ubuntu
```

**Warning**: This deletes the Ubuntu installation from C drive!
Make sure Step 3 completed successfully first.

### Step 5: Import to D Drive
```powershell
# Create D drive directory first
New-Item -ItemType Directory -Path "D:\WSL\Ubuntu" -Force

# Import Ubuntu to D drive
wsl --import Ubuntu D:\WSL\Ubuntu D:\WSL\Ubuntu_backup.tar
```

### Step 6: Verify and Clean Up
```powershell
# Verify it's on D drive
wsl --list --verbose

# Test it works
wsl -d Ubuntu

# Once verified, delete the backup tar file
Remove-Item D:\WSL\Ubuntu_backup.tar
```

## Quick Script (All in One)

I'll create a script that does this automatically:

```powershell
# Save as: move-wsl-to-d.ps1
Write-Host "=== Moving WSL Ubuntu to D Drive ===" -ForegroundColor Cyan

# Check if Ubuntu exists
$distros = wsl --list --quiet
if ($distros -notcontains "Ubuntu") {
    Write-Host "Ubuntu not found. Available distributions:" -ForegroundColor Yellow
    wsl --list
    exit 1
}

# Stop WSL
Write-Host "Stopping WSL..." -ForegroundColor Yellow
wsl --shutdown
Start-Sleep -Seconds 2

# Create D drive directory
Write-Host "Creating D:\WSL directory..." -ForegroundColor Yellow
New-Item -ItemType Directory -Path "D:\WSL" -Force | Out-Null

# Export
Write-Host "Exporting Ubuntu (this may take 10-20 minutes)..." -ForegroundColor Yellow
wsl --export Ubuntu D:\WSL\Ubuntu_backup.tar

# Unregister
Write-Host "Unregistering from C drive..." -ForegroundColor Yellow
wsl --unregister Ubuntu

# Import to D drive
Write-Host "Importing to D drive..." -ForegroundColor Yellow
wsl --import Ubuntu D:\WSL\Ubuntu D:\WSL\Ubuntu_backup.tar

# Verify
Write-Host "Verifying installation..." -ForegroundColor Yellow
wsl --list --verbose

Write-Host "`n✓ Ubuntu moved to D:\WSL\Ubuntu" -ForegroundColor Green
Write-Host "You can now delete: D:\WSL\Ubuntu_backup.tar" -ForegroundColor Yellow
```

## Where to Run

### Recommended: PowerShell in D:\Mondoshawan
```powershell
# 1. Open PowerShell
# 2. Navigate to project
cd D:\Mondoshawan

# 3. Run commands one by one, or use the script
```

### Or: Any PowerShell/CMD Window
- You can run WSL commands from anywhere
- They don't need to be in a specific directory
- Just open PowerShell or CMD and run the commands

## Important Notes

1. **Backup First**: The export creates a backup, but make sure it completes before unregistering
2. **Time**: Export/Import can take 10-30 minutes for 27 GB
3. **Space**: Need ~27 GB free on D drive temporarily (for backup + new location)
4. **WSL Must Be Stopped**: Use `wsl --shutdown` first
5. **Distribution Name**: Replace `Ubuntu` with your actual distribution name if different

## Check Your Distribution Name

If you're not sure of the exact name:
```powershell
wsl --list
```

Common names:
- `Ubuntu`
- `Ubuntu-24.04`
- `Ubuntu-22.04`
- `Ubuntu-20.04`

## After Moving

Once moved:
- Ubuntu will be on D drive: `D:\WSL\Ubuntu`
- C drive space freed: ~27 GB
- Everything works the same: `wsl` or `wsl -d Ubuntu`

## Troubleshooting

### "Distribution not found"
- Check exact name with: `wsl --list`
- Use the exact name shown

### "Access denied"
- Run PowerShell as Administrator
- Or ensure you have permissions to D drive

### "Not enough space on D drive"
- Need at least 30 GB free on D drive
- Clean D drive first if needed

## Quick Start

**Simplest way**:
1. Open PowerShell (anywhere)
2. Run: `wsl --list` (to see your distribution name)
3. Follow steps 1-6 above
4. Or use the script I'll create

