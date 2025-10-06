# Self-Signed Code Signing Setup Guide

**Date**: October 6, 2025  
**Status**: ✅ **ACTIVE** - Self-signed certificate implementation  
**Cost**: FREE (no recurring charges)

## 📋 Overview

This project uses **self-signed certificates** to digitally sign Windows executables. While self-signed certificates still trigger SmartScreen warnings (same as unsigned), they provide:

- ✅ **Consistent identity** across releases
- ✅ **Tamper detection** - signatures break if files are modified
- ✅ **Zero cost** - completely free
- ✅ **Full control** - you own the certificate

## ⚠️ Important Notes

### What Self-Signing Does:
- ✅ Proves the file hasn't been tampered with
- ✅ Provides consistent publisher identity
- ✅ Shows your name in signature properties
- ✅ Free forever

### What Self-Signing Does NOT Do:
- ❌ Does NOT eliminate SmartScreen warnings
- ❌ Does NOT build Windows reputation automatically
- ❌ Does NOT work like commercial certificates

**Users will still see**: "Windows protected your PC" warning and must click "More info" → "Run anyway"

## 🚀 Quick Setup (5 minutes)

### Step 1: Generate Self-Signed Certificate

Run this in PowerShell **as Administrator**:

```powershell
# Create a self-signed code signing certificate
$cert = New-SelfSignedCertificate `
    -Type CodeSigningCert `
    -Subject "CN=ia-get Project, O=Open Source" `
    -CertStoreLocation "Cert:\CurrentUser\My" `
    -NotAfter (Get-Date).AddYears(10) `
    -KeyLength 2048

Write-Host "✅ Certificate created:"
Write-Host "   Subject: $($cert.Subject)"
Write-Host "   Thumbprint: $($cert.Thumbprint)"
Write-Host "   Expires: $($cert.NotAfter)"

# Export the certificate with a password
$password = ConvertTo-SecureString -String "YourSecurePassword123!" -Force -AsPlainText
$certPath = "$HOME\Desktop\ia-get-codesign.pfx"
Export-PfxCertificate -Cert $cert -FilePath $certPath -Password $password

Write-Host ""
Write-Host "✅ Certificate exported to: $certPath"
Write-Host ""
Write-Host "⚠️  IMPORTANT: Save this certificate and password securely!"
Write-Host "   You'll need them to sign future releases."
```

**Important**: 
- Choose a strong password and **save it securely**
- Keep the `.pfx` file in a safe location
- You'll need both for GitHub Secrets

### Step 2: Encode Certificate for GitHub

```powershell
# Base64 encode the certificate
$certBytes = [IO.File]::ReadAllBytes("$HOME\Desktop\ia-get-codesign.pfx")
$certBase64 = [Convert]::ToBase64String($certBytes)

# Copy to clipboard
$certBase64 | Set-Clipboard

Write-Host "✅ Certificate encoded and copied to clipboard"
Write-Host ""
Write-Host "Next steps:"
Write-Host "1. Go to GitHub Repository → Settings → Secrets and variables → Actions"
Write-Host "2. Click 'New repository secret'"
Write-Host "3. Name: CODE_SIGNING_CERT"
Write-Host "4. Value: Paste from clipboard (Ctrl+V)"
Write-Host "5. Click 'Add secret'"
```

### Step 3: Add GitHub Secrets

Go to your repository on GitHub:

1. **Navigate to**: `https://github.com/Gameaday/ia-get-cli/settings/secrets/actions`

2. **Add first secret**:
   - Click "New repository secret"
   - Name: `CODE_SIGNING_CERT`
   - Value: Paste the base64-encoded certificate (from clipboard)
   - Click "Add secret"

3. **Add second secret**:
   - Click "New repository secret"
   - Name: `CODE_SIGNING_PASSWORD`
   - Value: The password you used (e.g., `YourSecurePassword123!`)
   - Click "Add secret"

### Step 4: Test the Setup

Create a test release to verify signing works:

```bash
# Create and push a test tag
git tag v1.0.0-signing-test
git push origin v1.0.0-signing-test
```

Then:
1. Go to Actions tab on GitHub
2. Wait for the workflow to complete
3. Check the "Sign Windows executable" step - should show ✅
4. Download the release artifact
5. Right-click the `.exe` → Properties → Digital Signatures tab
6. You should see your certificate!

## 🔍 Verifying Signatures

### On Windows:

**Method 1: File Properties**
1. Right-click `ia-get.exe` → Properties
2. Go to "Digital Signatures" tab
3. Select signature → Click "Details"
4. Should show your certificate subject

**Method 2: PowerShell**
```powershell
Get-AuthenticodeSignature ia-get.exe | Format-List *
```

**Method 3: signtool (Windows SDK)**
```cmd
signtool verify /pa ia-get.exe
```

### Expected Output:
```
Status: Valid
Subject: CN=ia-get Project, O=Open Source
Issuer: CN=ia-get Project, O=Open Source (self-issued)
```

## 🔐 Security Best Practices

### 1. Secure Certificate Storage

**DO**:
- ✅ Store the `.pfx` file in encrypted storage
- ✅ Use a password manager for the password
- ✅ Keep backups in multiple secure locations
- ✅ Limit access to the certificate

**DON'T**:
- ❌ Commit the `.pfx` file to Git
- ❌ Share the certificate password via email/chat
- ❌ Store password in plain text files
- ❌ Use weak passwords

### 2. Certificate Lifecycle

**Validity**: Certificate is valid for 10 years

**When to renew**:
- Before expiration (check with PowerShell: `Get-ChildItem Cert:\CurrentUser\My`)
- If certificate is compromised
- If you want to change identity information

**Renewal process**:
1. Generate new certificate (same steps as above)
2. Update GitHub Secrets
3. Old releases will still show old certificate (this is normal)

### 3. Private Key Protection

The `.pfx` file contains your **private key**. If someone gets:
- ✅ Just the `.pfx` file: They still need the password
- ✅ Just the password: They still need the `.pfx` file
- ❌ Both `.pfx` + password: They can sign files as you!

**If compromised**:
1. Generate a new certificate immediately
2. Update GitHub Secrets
3. Consider informing users about the old certificate

## 📝 Certificate Information

Use this information when creating your certificate:

| Field | Recommended Value | Example |
|-------|------------------|---------|
| Subject (CN) | Project name | `ia-get Project` |
| Organization (O) | Organization type | `Open Source` |
| Validity | 5-10 years | 10 years |
| Key Length | 2048 or 4096 bits | 2048 bits |
| Type | Code Signing | CodeSigningCert |

## 🎯 What Users Will See

### First Download:
1. SmartScreen warning: "Windows protected your PC"
2. Must click "More info" → "Run anyway"
3. File properties show your certificate

### After Running Once:
- Windows remembers the executable
- May still show warning for new versions
- Signature remains verifiable

### In Windows Explorer:
- Right-click → Properties → Digital Signatures
- Shows your certificate information
- Can verify signature validity

## 🔧 Troubleshooting

### Issue: "Certificate not found" in GitHub Actions

**Cause**: Secrets not set correctly

**Solution**:
1. Verify both secrets exist: `CODE_SIGNING_CERT` and `CODE_SIGNING_PASSWORD`
2. Check base64 encoding was done correctly
3. Ensure no extra spaces/newlines in secret values

### Issue: "Cannot import PFX file"

**Cause**: Wrong password or corrupted file

**Solution**:
1. Test locally first: `Import-PfxCertificate -FilePath cert.pfx -Password $password`
2. Verify password matches exactly
3. Re-export certificate if needed

### Issue: "Signature shows as invalid"

**Cause**: Self-signed certificates appear as "unknown publisher"

**Solution**:
- This is **normal** for self-signed certificates
- Signature is still cryptographically valid
- Users must manually trust on first run

### Issue: "Timestamping failed"

**Cause**: Timestamp server unreachable

**Solution**:
- Workflow will retry automatically
- DigiCert timestamp server is reliable
- Can try alternative: `http://timestamp.sectigo.com`

## 📊 Workflow Integration

The signing process is automated in `.github/workflows/release.yml`:

```yaml
- name: Sign Windows executable (Self-Signed)
  if: matrix.job.target == 'x86_64-pc-windows-msvc' && matrix.job.os == 'windows-latest'
  shell: powershell
  run: |
    # Decode certificate, sign executable, verify signature
    # See release.yml for full implementation
```

**What it does**:
1. Decodes certificate from GitHub Secret
2. Imports certificate to Windows certificate store
3. Signs `ia-get.exe` with timestamp
4. Verifies signature
5. Cleans up certificate from store
6. Shows signing status

## 🆚 Comparison with Other Methods

| Feature | Self-Signed | SignPath.io | Azure | No Signing |
|---------|-------------|-------------|-------|------------|
| Cost | Free | Free (OSS) | $10/mo | Free |
| Setup Time | 5 min | 1-2 days | 1 hour | 0 min |
| SmartScreen | ⚠️ Warning | ✅ Reduced | ✅ Reduced | ⚠️ Warning |
| Reputation | Low | Builds | Builds | None |
| Control | Full | Limited | Limited | N/A |
| Maintenance | Minimal | None | Minimal | None |

## 📚 Additional Resources

- [New-SelfSignedCertificate Documentation](https://learn.microsoft.com/en-us/powershell/module/pki/new-selfsignedcertificate)
- [Set-AuthenticodeSignature Documentation](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.security/set-authenticodesignature)
- [Code Signing Best Practices](https://learn.microsoft.com/en-us/windows-hardware/drivers/dashboard/code-signing-best-practices)
- [About Authenticode](https://learn.microsoft.com/en-us/windows-hardware/drivers/install/authenticode)

## 🎉 Benefits of This Approach

### For the Project:
- ✅ Consistent identity across all releases
- ✅ Files can be verified as coming from you
- ✅ Tamper detection built-in
- ✅ Professional appearance in file properties
- ✅ Zero ongoing costs

### For Users:
- ✅ Can verify file authenticity
- ✅ Signatures don't expire (with timestamp)
- ✅ Clear publisher information
- ✅ Tamper detection
- ✅ Same security as unsigned (SmartScreen still warns)

## 🔄 Maintenance Schedule

| Task | Frequency | Notes |
|------|-----------|-------|
| Verify signature | Each release | Automatic in workflow |
| Check expiration | Yearly | Certificate valid 10 years |
| Backup certificate | Once | Keep in secure location |
| Update password | Never* | Unless compromised |
| Rotate certificate | Every 5-10 years | Or if compromised |

*Password should be strong from the start

## ✅ Setup Checklist

Use this to verify your setup:

- [ ] Certificate generated with PowerShell
- [ ] Certificate exported as `.pfx` file
- [ ] Password chosen and saved securely
- [ ] Certificate backed up to secure location
- [ ] Certificate base64-encoded
- [ ] `CODE_SIGNING_CERT` secret added to GitHub
- [ ] `CODE_SIGNING_PASSWORD` secret added to GitHub
- [ ] Test release created and signed successfully
- [ ] Downloaded `.exe` shows digital signature
- [ ] Signature verifies correctly
- [ ] Workflow completes without errors

---

**Status**: ✅ Self-signing implementation complete and active!
