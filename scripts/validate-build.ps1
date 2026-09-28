# Build validation script for Windows
# Catches issues early in the development process

Write-Host "🔍 Build Validation Script" -ForegroundColor Cyan
Write-Host "===========================" -ForegroundColor Cyan

# Function to print status
function Write-Status {
    param([string]$Status, [string]$Message)
    switch ($Status) {
        "success" { Write-Host "✅ $Message" -ForegroundColor Green }
        "warning" { Write-Host "⚠️  $Message" -ForegroundColor Yellow }
        "error" { Write-Host "❌ $Message" -ForegroundColor Red }
    }
}

Write-Host "📋 Checking prerequisites..." -ForegroundColor Blue

try {
    $null = Get-Command cargo -ErrorAction Stop
    Write-Status "success" "Cargo found"
} catch {
    Write-Status "error" "Cargo not found. Please install Rust."
    exit 1
}

try {
    $null = Get-Command rustc -ErrorAction Stop
    Write-Status "success" "Rustc found"
} catch {
    Write-Status "error" "Rustc not found. Please install Rust."
    exit 1
}

Write-Host ""
Write-Host "📋 Step 1: Checking code formatting..." -ForegroundColor Blue
try {
    & cargo fmt --check --all 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Status "success" "Code formatting is correct"
    } else {
        Write-Status "error" "Code formatting issues found. Run 'cargo fmt' to fix."
        Write-Host "💡 Tip: Run 'cargo fmt' to automatically fix formatting issues" -ForegroundColor Cyan
        exit 1
    }
} catch {
    Write-Status "error" "Failed to check formatting: $_"
    exit 1
}

Write-Host ""
Write-Host "📋 Step 2: Running clippy..." -ForegroundColor Blue
try {
    & cargo clippy --all-targets -- -D warnings 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Status "success" "Clippy passed with no warnings"
    } else {
        Write-Status "error" "Clippy found issues"
        exit 1
    }
} catch {
    Write-Status "error" "Failed to run clippy: $_"
    exit 1
}

Write-Host ""
Write-Host "📋 Step 3: Checking compilation..." -ForegroundColor Blue
try {
    & cargo check --all-targets 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Status "success" "Compilation successful"
    } else {
        Write-Status "error" "Compilation failed"
        exit 1
    }
} catch {
    Write-Status "error" "Failed to check compilation: $_"
    exit 1
}

Write-Host ""
Write-Host "📋 Step 4: Running tests..." -ForegroundColor Blue
try {
    & cargo test --quiet 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Status "success" "Tests passed"
    } else {
        Write-Status "error" "Tests failed"
        exit 1
    }
} catch {
    Write-Status "error" "Failed to run tests: $_"
    exit 1
}

Write-Host ""
Write-Host "📋 Step 5: Checking for security vulnerabilities..." -ForegroundColor Blue
try {
    $null = Get-Command cargo-audit -ErrorAction Stop
    & cargo audit --quiet 2>$null | Out-Null
    if ($LASTEXITCODE -eq 0) {
        Write-Status "success" "Security audit passed"
    } else {
        Write-Status "warning" "Security vulnerabilities found (non-blocking)"
    }
} catch {
    Write-Status "warning" "cargo-audit not installed. Install with: cargo install cargo-audit"
}

Write-Host ""
Write-Host "📋 Step 6: Checking for outdated dependencies..." -ForegroundColor Blue
try {
    $null = Get-Command cargo-outdated -ErrorAction Stop
    $result = & cargo outdated --quiet 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Status "success" "No major dependency updates available"
    } else {
        Write-Status "warning" "Outdated dependencies found (non-blocking)"
    }
} catch {
    Write-Status "warning" "cargo-outdated not installed. Install with: cargo install cargo-outdated"
}

Write-Host ""
Write-Host "🎉 Build validation completed successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "📊 Summary:" -ForegroundColor Cyan
Write-Host "   - Code formatting: ✅" -ForegroundColor Green
Write-Host "   - Clippy: ✅" -ForegroundColor Green
Write-Host "   - Compilation: ✅" -ForegroundColor Green
Write-Host "   - Tests: ✅" -ForegroundColor Green
Write-Host ""
Write-Host "💡 Your code is ready for commit and CI/CD pipeline!" -ForegroundColor Cyan
