# Phase 2 Checklist: GitHub Actions CI Setup

## Quick Start - 3 Steps to CI Signing

### Step 1: Add GitHub Secrets (5 minutes)

1. **Go to repository secrets page**:
   ```
   https://github.com/Gameaday/ia-get-cli/settings/secrets/actions
   ```

2. **Add first secret**:
   - Click "New repository secret"
   - Name: `CODE_SIGNING_CERT`
   - Value: Run this command and copy the entire output:
     ```powershell
     Get-Content .secrets\CODE_SIGNING_CERT.txt
     ```
   - Click "Add secret"

3. **Add second secret**:
   - Click "New repository secret"
   - Name: `CODE_SIGNING_PASSWORD`
   - Value: `ia-get-SecurePass2025!`
   - Click "Add secret"

4. **Verify secrets added**:
   - You should see both secrets listed (values are hidden)
   - `CODE_SIGNING_CERT`
   - `CODE_SIGNING_PASSWORD`

### Step 2: Create and Push Test Tag (2 minutes)

```bash
# Create test tag
git tag v2.0.1-test

# Push to GitHub
git push origin v2.0.1-test
```

**Alternative** - Tag specific commit:
```bash
git tag v2.0.1-test HEAD
git push origin v2.0.1-test
```

### Step 3: Monitor GitHub Actions (5-10 minutes)

1. **Open Actions tab**:
   ```
   https://github.com/Gameaday/ia-get-cli/actions
   ```

2. **Watch for workflow**:
   - Workflow name: "Build 🏗️ and Publish 📦️"
   - Should start automatically when tag is pushed

3. **Check Windows signing steps**:
   - Click on the workflow run
   - Find "Publishing for windows-latest (x86_64-pc-windows-msvc)" job
   - Expand "Sign Windows executable (Self-Signed)" step
   - Look for: ✅ Certificate imported, ✅ Successfully signed executable

4. **Verify signature**:
   - Expand "Verify Windows code signature" step
   - Check: Status, Subject, Thumbprint are displayed

5. **Check artifacts**:
   - Scroll to bottom of workflow page
   - Download: `ia-get-v2.0.1-test-x86_64-pc-windows-msvc.zip`

### Step 4: Verify Downloaded Artifact (optional)

```powershell
# Extract zip and check signature
Expand-Archive ia-get-v2.0.1-test-x86_64-pc-windows-msvc.zip -DestinationPath test-release
Get-AuthenticodeSignature test-release\ia-get.exe | Format-List *
```

**Expected output**:
- SignerCertificate: CN=ia-get Project, O=Open Source
- TimeStamperCertificate: DigiCert
- Status: UnknownError (normal for self-signed)

---

## Troubleshooting

### Secrets not found in workflow
- ✅ Verify exact names: `CODE_SIGNING_CERT` and `CODE_SIGNING_PASSWORD` (case-sensitive)
- ✅ Check secrets are in "Repository secrets" not "Environment secrets"
- ✅ Ensure no extra spaces in secret values

### Workflow doesn't start
- ✅ Verify tag format matches pattern in workflow (v*.*.* or *.*.*)
- ✅ Check tag was pushed to origin: `git ls-remote --tags origin`
- ✅ Verify workflow file is on main branch

### Signing fails
- ✅ Check sign step output for specific error
- ✅ Verify certificate base64 is complete (no truncation)
- ✅ Test password locally first

### Signature shows "NotSigned"
- ✅ Signing step may have failed - check its output
- ✅ Certificate import may have failed
- ✅ Path to executable may be incorrect

---

## Success Checklist

- [ ] Both GitHub secrets added successfully
- [ ] Test tag created and pushed
- [ ] GitHub Actions workflow triggered automatically
- [ ] "Sign Windows executable" step completed successfully
- [ ] Certificate imported message visible in logs
- [ ] Signature success message visible in logs
- [ ] "Verify Windows code signature" step passed
- [ ] Signature details displayed (Subject, Thumbprint, Timestamp)
- [ ] Windows artifact downloadable from release
- [ ] Downloaded executable has valid signature with timestamp

---

## Quick Commands Reference

**View secrets**:
```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\show-github-secrets.ps1
```

**Create test tag**:
```bash
git tag v2.0.1-test
git push origin v2.0.1-test
```

**Check local signature**:
```powershell
Get-AuthenticodeSignature target\x86_64-pc-windows-msvc\release\ia-get.exe
```

**Test local signing**:
```powershell
cargo build --release --target x86_64-pc-windows-msvc
powershell -ExecutionPolicy Bypass -File .\scripts\local-sign.ps1
```

---

## After Success

Once Phase 2 is complete:

1. **Delete test tag** (optional):
   ```bash
   git tag -d v2.0.1-test
   git push origin :refs/tags/v2.0.1-test
   ```

2. **Create production release**:
   ```bash
   git tag v2.0.1
   git push origin v2.0.1
   ```

3. **Update documentation** with Windows SmartScreen bypass instructions

4. **Plan next improvements**:
   - Consider CA-signed certificate if budget allows
   - Document user experience with self-signed certificate
   - Add Windows installer (optional)

---

**Status**: Ready to start Phase 2
**Time estimate**: 15-20 minutes total
