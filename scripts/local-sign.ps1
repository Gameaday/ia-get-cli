# Local signing script for ia-get executable
# Usage: powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\local-sign.ps1

# Use secrets from project .secrets folder
$secretsDir = Join-Path -Path (Get-Location) -ChildPath '.secrets'
$certPath = Join-Path -Path $secretsDir -ChildPath 'cert.pfx'
$passwordFile = Join-Path -Path $secretsDir -ChildPath 'CODE_SIGNING_PASSWORD.txt'

if (-not (Test-Path $secretsDir)) {
    Write-Error ".secrets folder not found. Please run setup first."
    exit 1
}

if (-not (Test-Path $certPath)) {
    Write-Error "Certificate not found at $certPath"
    exit 2
}

if (-not (Test-Path $passwordFile)) {
    Write-Error "Password file not found at $passwordFile"
    exit 2
}

$passwordPlain = (Get-Content $passwordFile -Raw).Trim()

try {
    $securePwd = ConvertTo-SecureString -String $passwordPlain -AsPlainText -Force
    $cert = Import-PfxCertificate -FilePath $certPath -CertStoreLocation 'Cert:\CurrentUser\My' -Password $securePwd
    Write-Host "Imported cert thumbprint: $($cert.Thumbprint)"
} catch {
    Write-Error "Failed to import PFX: $_"
    exit 3
}

$exePath = Join-Path -Path (Get-Location) -ChildPath 'target\x86_64-pc-windows-msvc\release\ia-get.exe'
if (-not (Test-Path $exePath)) {
    Write-Error "Executable not found: $exePath. Run cargo build --release --target x86_64-pc-windows-msvc first."
    exit 4
}

Write-Host "Signing $exePath with certificate $($cert.Subject)"
$signResult = Set-AuthenticodeSignature -FilePath $exePath -Certificate $cert -TimeStampServer 'http://timestamp.digicert.com'
Write-Host "Sign result status: $($signResult.Status)"

$signature = Get-AuthenticodeSignature -FilePath $exePath
Write-Host "Verification status: $($signature.Status)"
Write-Host "Signer Subject: $($signature.SignerCertificate.Subject)"
Write-Host "Signer Thumbprint: $($signature.SignerCertificate.Thumbprint)"

# Optional cleanup: remove the certificate from store
try {
    $thumb = $cert.Thumbprint
    Write-Host "Removing imported cert from store (thumbprint $thumb)"
    Remove-Item -Path "Cert:\CurrentUser\My\$thumb" -ErrorAction SilentlyContinue
} catch {
    Write-Warning "Failed to remove cert from store: $_"
}

Write-Host "Local signing script finished."