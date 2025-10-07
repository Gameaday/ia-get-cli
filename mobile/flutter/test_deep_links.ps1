# Deep Link Testing Script for Internet Archive Helper
# This script helps test deep link functionality on connected Android devices

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Internet Archive Helper - Deep Link Test" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check if ADB is available
$adbPath = Get-Command adb -ErrorAction SilentlyContinue
if (-not $adbPath) {
    Write-Host "ERROR: ADB not found in PATH" -ForegroundColor Red
    Write-Host "Please install Android SDK Platform Tools" -ForegroundColor Yellow
    exit 1
}

# Get connected devices
$devices = adb devices | Select-Object -Skip 1 | Where-Object { $_ -match "\tdevice$" }
if ($devices.Count -eq 0) {
    Write-Host "ERROR: No Android devices connected" -ForegroundColor Red
    Write-Host "Please connect a device or start an emulator" -ForegroundColor Yellow
    exit 1
}

Write-Host "Connected device found!" -ForegroundColor Green
Write-Host ""

# Package name
$package = "com.gameaday.internet_archive_helper"

# Test URLs
$testUrls = @(
    @{
        Name = "Custom scheme (iaget://)"
        URL = "iaget://commute_test"
        Description = "Tests custom deep link scheme"
    },
    @{
        Name = "Archive.org details page"
        URL = "https://archive.org/details/commute_test"
        Description = "Tests handling of archive.org details URL"
    },
    @{
        Name = "Archive.org download page"
        URL = "https://archive.org/download/commute_test"
        Description = "Tests handling of archive.org download URL"
    }
)

Write-Host "Available test cases:" -ForegroundColor Yellow
for ($i = 0; $i -lt $testUrls.Count; $i++) {
    Write-Host "  [$($i + 1)] $($testUrls[$i].Name)" -ForegroundColor Cyan
    Write-Host "      $($testUrls[$i].Description)" -ForegroundColor Gray
}
Write-Host "  [A] Run all tests" -ForegroundColor Cyan
Write-Host "  [Q] Quit" -ForegroundColor Cyan
Write-Host ""

$choice = Read-Host "Select test to run"

if ($choice -eq "Q" -or $choice -eq "q") {
    exit 0
}

$testsToRun = @()
if ($choice -eq "A" -or $choice -eq "a") {
    $testsToRun = $testUrls
} else {
    $index = [int]$choice - 1
    if ($index -ge 0 -and $index -lt $testUrls.Count) {
        $testsToRun = @($testUrls[$index])
    } else {
        Write-Host "Invalid selection" -ForegroundColor Red
        exit 1
    }
}

Write-Host ""
Write-Host "Running tests..." -ForegroundColor Green
Write-Host ""

foreach ($test in $testsToRun) {
    Write-Host "Testing: $($test.Name)" -ForegroundColor Yellow
    Write-Host "URL: $($test.URL)" -ForegroundColor Gray
    
    # Send deep link via ADB
    $cmd = "adb shell am start -W -a android.intent.action.VIEW -d `"$($test.URL)`""
    Write-Host "Command: $cmd" -ForegroundColor Gray
    
    # Execute command and capture output
    $result = Invoke-Expression $cmd 2>&1
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Command executed successfully" -ForegroundColor Green
        
        # Show any output if verbose
        if ($result) {
            Write-Host "  Output: $result" -ForegroundColor DarkGray
        }
        
        # Wait a moment for app to respond
        Start-Sleep -Seconds 2
        
        # Check if app is in foreground
        $foreground = adb shell "dumpsys activity activities | grep -E 'mResumedActivity|mFocusedActivity'"
        if ($foreground -match $package) {
            Write-Host "✓ App is now in foreground" -ForegroundColor Green
        } else {
            Write-Host "⚠ App may not be in foreground" -ForegroundColor Yellow
            Write-Host "  Check device to verify behavior" -ForegroundColor Gray
        }
    } else {
        Write-Host "✗ Command failed" -ForegroundColor Red
        if ($result) {
            Write-Host "  Error: $result" -ForegroundColor Red
        }
    }
    
    Write-Host ""
    
    if ($testsToRun.Count -gt 1) {
        Write-Host "Press any key to continue to next test..."
        $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
        Write-Host ""
    }
}

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Testing complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Manual verification steps:" -ForegroundColor Yellow
Write-Host "1. Check that the app opened" -ForegroundColor Gray
Write-Host "2. Verify the archive detail screen is shown" -ForegroundColor Gray
Write-Host "3. Confirm the correct archive loaded (commute_test)" -ForegroundColor Gray
Write-Host "========================================" -ForegroundColor Cyan
