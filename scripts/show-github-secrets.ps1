# GitHub Secrets Helper
# This script displays the values needed for GitHub repository secrets
# Usage: powershell -ExecutionPolicy Bypass -File .\scripts\show-github-secrets.ps1

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "GitHub Repository Secrets Configuration" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

$secretsDir = ".secrets"

if (-not (Test-Path $secretsDir)) {
    Write-Error "Secrets directory not found. Please run setup first."
    exit 1
}

# Read certificate
$certFile = Join-Path $secretsDir "CODE_SIGNING_CERT.txt"
if (Test-Path $certFile) {
    Write-Host "SECRET NAME: CODE_SIGNING_CERT" -ForegroundColor Yellow
    Write-Host "SECRET VALUE (copy everything below):" -ForegroundColor Green
    Write-Host "----------------------------------------" -ForegroundColor DarkGray
    Get-Content $certFile
    Write-Host "----------------------------------------" -ForegroundColor DarkGray
    Write-Host ""
} else {
    Write-Warning "Certificate file not found: $certFile"
}

# Read password
$passwordFile = Join-Path $secretsDir "CODE_SIGNING_PASSWORD.txt"
if (Test-Path $passwordFile) {
    Write-Host "SECRET NAME: CODE_SIGNING_PASSWORD" -ForegroundColor Yellow
    Write-Host "SECRET VALUE:" -ForegroundColor Green
    Write-Host "----------------------------------------" -ForegroundColor DarkGray
    Get-Content $passwordFile
    Write-Host "----------------------------------------" -ForegroundColor DarkGray
    Write-Host ""
} else {
    Write-Warning "Password file not found: $passwordFile"
}

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Instructions:" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "1. Go to: https://github.com/Gameaday/ia-get-cli/settings/secrets/actions"
Write-Host "2. Click 'New repository secret'"
Write-Host "3. Add the two secrets shown above"
Write-Host "4. Push a test tag to trigger the release workflow"
Write-Host ""
Write-Host "Example tag commands:"
Write-Host "  git tag v2.0.0-test"
Write-Host "  git push origin v2.0.0-test"
Write-Host ""
