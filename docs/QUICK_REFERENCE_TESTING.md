# Quick Reference: Self-Signed Certificate Testing

## ⚡ Quick Status Check

Run this to verify the workflow is ready:

```bash
# Check for Azure references (should return "No azure references found")
grep -r "azure" .github/workflows/ -i

# Check for self-signed implementation (should find it)
grep -n "Sign Windows executable (Self-Signed)" .github/workflows/release.yml

# Check YAML syntax
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))" && echo "✅ Valid YAML"
```

---

## 🔑 Required GitHub Secrets

Go to: https://github.com/Gameaday/ia-get-cli/settings/secrets/actions

### Secret #1: `CODE_SIGNING_CERT`
- **Type**: Repository secret
- **Value**: Base64-encoded .pfx certificate file
- **How to get**: See `docs/SELF_SIGNED_SETUP.md` Step 2

### Secret #2: `CODE_SIGNING_PASSWORD`
- **Type**: Repository secret  
- **Value**: Password used when creating the certificate
- **Example**: `ia-get-SecurePass2025!` (use your own!)

---

## 🧪 How to Test

### 1. Create Test Tag
```bash
git tag v1.0.0-signing-test
git push origin v1.0.0-signing-test
```

### 2. Watch Workflow
- Go to: https://github.com/Gameaday/ia-get-cli/actions
- Click on the newest "Build 🏗️ and Publish 📦️" workflow
- Expand "Sign Windows executable (Self-Signed)"
- Look for: ✅ Successfully signed executable

### 3. Expected Output
```
✅ Certificate imported: CN=ia-get Project, O=Open Source
Signing: target/x86_64-pc-windows-msvc/release/ia-get.exe
✅ Successfully signed executable
   Subject: CN=ia-get Project, O=Open Source
   Thumbprint: ABC123...
```

### 4. Verify Signature
Download the Windows release and check:
```powershell
# On Windows
Get-AuthenticodeSignature .\ia-get.exe | Format-List *

# Should show:
# Status: Valid
# SignerCertificate: CN=ia-get Project, O=Open Source
```

Or right-click `ia-get.exe` → Properties → Digital Signatures tab

---

## 🐛 Troubleshooting

### Error: "Cannot import PFX file"
- ❌ **Cause**: Wrong password or corrupted certificate
- ✅ **Fix**: Verify `CODE_SIGNING_PASSWORD` secret matches certificate password

### Error: "Certificate not found"
- ❌ **Cause**: `CODE_SIGNING_CERT` secret is missing or invalid
- ✅ **Fix**: Re-encode certificate to base64 and update secret

### Warning: VS Code shows "Context access might be invalid"
- ⚠️ **Status**: Normal and expected
- ✅ **Action**: Can be safely ignored (secrets work in GitHub Actions)

### Signature shows as "UnknownError" or "NotTrusted"
- ℹ️ **Status**: Normal for self-signed certificates
- ✅ **Action**: This is expected! Self-signed certs aren't trusted by Windows by default, but the signature is still cryptographically valid.

---

## 📄 Documentation Links

| Document | Purpose |
|----------|---------|
| `docs/SELF_SIGNED_SETUP.md` | Complete setup guide with certificate generation |
| `docs/VERIFICATION_SELF_SIGNED_COMPLETE.md` | Verification that implementation is complete |
| `docs/CLEANUP_COMPLETE.md` | What Azure references were removed |
| `docs/SELF_SIGNED_IMPLEMENTATION_SUMMARY.md` | Implementation details |

---

## ✅ Checklist

Before testing, ensure:
- [ ] Certificate generated (or reuse existing)
- [ ] Certificate base64-encoded
- [ ] `CODE_SIGNING_CERT` secret added to GitHub
- [ ] `CODE_SIGNING_PASSWORD` secret added to GitHub
- [ ] Secrets are in "Actions" section (not "Dependabot" or "Codespaces")

After testing:
- [ ] Workflow completed successfully
- [ ] "Sign Windows executable" step shows ✅
- [ ] "Verify Windows code signature" step shows ✅
- [ ] Downloaded .exe has Digital Signatures tab populated
- [ ] Signature shows your certificate details

---

## 🎯 Current Status

**Implementation**: ✅ Complete  
**Azure References**: ✅ Removed  
**Workflow Ready**: ✅ Yes  
**Secrets Required**: ⏳ User action needed  
**Testing Required**: ⏳ User action needed

---

**Ready to test!** Just add the secrets and push a tag! 🚀
