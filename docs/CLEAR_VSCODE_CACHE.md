# Clear VS Code Cached Problems - Fix Script

## Current Situation

### ✅ What's Actually Fixed
- **release.yml**: Contains self-signed PowerShell code (NO Azure references)
- **Gradle/Java**: Java 21 configured correctly
- **Git status**: All changes committed, working tree clean
- **Actual errors**: Only 2 cached Gradle errors remain (already fixed)

### ❌ What VS Code is Showing (Incorrectly)
- Cached error: "Unable to resolve action `azure/trusted-signing@v3`"
- This error references a **chat-editing-snapshot** that no longer exists
- The Problems panel is holding onto stale diagnostics

## Solution Steps

### Step 1: Clear VS Code Cache (Recommended)

Run these commands in VS Code:

1. **Close all editor tabs**:
   - Press `Ctrl+K W` (closes all editors)

2. **Reload VS Code Window**:
   - Press `Ctrl+Shift+P`
   - Type: `Developer: Reload Window`
   - Press Enter

3. **Clear All Editor History** (if reload doesn't work):
   - Press `Ctrl+Shift+P`
   - Type: `Clear Editor History`
   - Press Enter

### Step 2: Restart VS Code Language Server

If the error persists after reload:

1. Press `Ctrl+Shift+P`
2. Type: `Developer: Restart Extension Host`
3. Press Enter

### Step 3: Close and Reopen VS Code (Nuclear Option)

If Steps 1-2 don't work:

1. **Save all work** (though everything is committed)
2. Close VS Code completely
3. Wait 5 seconds
4. Reopen VS Code
5. Open the project folder

### Step 4: Verify Problems Panel

After reloading, check the Problems panel (`Ctrl+Shift+M`):

**Expected errors (only 2, both cached/fixed):**
- `mobile/flutter/android/app/build.gradle` - Java version (fixed, cached)
- `mobile/flutter/android/build.gradle` - Java version (fixed, cached)

**Should NOT see:**
- ❌ Any `release.yml` errors
- ❌ Any `azure/trusted-signing` errors
- ❌ Any GitHub Actions errors

### Step 5: Clear Gradle Cache (Optional)

To clear the Gradle errors from Problems panel:

```powershell
# Run the Java 21 environment setup
. .\scripts\setup-flutter-env.ps1

# Then reload VS Code window
```

## Technical Details

### Why This Happens

1. VS Code's language servers cache diagnostics
2. Chat editing creates temporary snapshot files
3. Problems panel references these snapshots
4. Even after closing/deleting snapshots, cache remains
5. Only a reload/restart clears the cache

### What Files Are Actually Correct

```powershell
# Verify no Azure references exist
git grep "azure/trusted-signing" .github/workflows/

# Should return: (nothing found)
```

```powershell
# Check actual file content
Get-Content .github\workflows\release.yml | Select-String -Pattern "Sign Windows" -Context 2,15

# Should show: "Sign Windows executable (Self-Signed)"
```

## Quick Fix Commands

### Option A: Quick Reload (Try First)
```
Ctrl+Shift+P → "Developer: Reload Window"
```

### Option B: Clear Everything
```
Ctrl+K W           (close all editors)
Ctrl+Shift+P       (command palette)
Type: "reload"     (reload window)
```

### Option C: Restart VS Code
Close and reopen VS Code completely.

## Expected Final State

After clearing cache, you should see:

**Problems Panel:**
- 0 errors related to release.yml
- 0 errors related to Azure
- 2 errors for Gradle (cached, actually fixed)

**Files:**
- `.github/workflows/release.yml` - Shows self-signed code
- All tabs closed/clean
- No chat-editing-snapshot files

## Verification Commands

Run these to confirm everything is correct:

```powershell
# 1. Check git status
git status
# Expected: working tree clean

# 2. Search for Azure references
git grep -i azure .github/workflows/
# Expected: no matches

# 3. Verify Java environment
. .\scripts\setup-flutter-env.ps1
# Expected: Java 21.0.8

# 4. Check actual file
Get-Content .github\workflows\release.yml -TotalCount 350 | Select-Object -Skip 290
# Expected: Shows "Sign Windows executable (Self-Signed)"
```

---

**Current Status**: Everything is correctly fixed and committed. The Problems panel just needs VS Code to be reloaded to clear cached diagnostics.

**Recommended Action**: Press `Ctrl+Shift+P` → Type "reload" → Press Enter
