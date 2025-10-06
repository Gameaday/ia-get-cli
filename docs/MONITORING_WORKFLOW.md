# GitHub Actions Monitoring - v2.0.1-test

## Status
✅ Test tag `v2.0.1-test` created and pushed to GitHub
✅ GitHub Actions workflow should be running now

## Monitor the Workflow

### 1. Open GitHub Actions
**Direct link**: https://github.com/Gameaday/ia-get-cli/actions

### 2. Find the Workflow Run
- Look for: "Build 🏗️ and Publish 📦️"
- Tag: v2.0.1-test
- Should show "In progress" 🟡 or "Success" 🟢

### 3. Check Key Steps

Click on the workflow run, then find these jobs:

#### Job 1: Flutter Mobile Release
- Build Android APK and App Bundle (Pure Dart)
- Should complete in ~5-10 minutes

#### Job 2: Publishing for windows-latest (x86_64-pc-windows-msvc)
**This is the critical signing job!**

**Steps to verify:**
1. Click on "Publishing for windows-latest" job
2. Expand "Sign Windows executable (Self-Signed)"
3. Look for these messages:
   ```
   ✅ Certificate imported: CN=ia-get Project, O=Open Source
   Signing: target/x86_64-pc-windows-msvc/release/ia-get.exe
   ✅ Successfully signed executable
      Subject: CN=ia-get Project, O=Open Source
      Thumbprint: D59AC40C8B48E56E677241EB7A7E226AE6A9EC37
   ```

4. Expand "Verify Windows code signature"
5. Look for:
   ```
   Signature verification:
     Status: UnknownError (or Valid)
     Subject: CN=ia-get Project, O=Open Source
     Issuer: CN=ia-get Project, O=Open Source
   Note: Self-signed certificates show warnings but are properly signed
   ```

### 4. What to Expect

**Sign Step - Expected Output:**
- Certificate decoded from secret ✅
- Certificate imported to store ✅
- Executable signed with certificate ✅
- Timestamp added from DigiCert ✅
- Certificate removed from store (cleanup) ✅

**Verify Step - Expected Output:**
- Status: "UnknownError" or "NotSigned" is ACCEPTABLE for self-signed certs
- Subject matches: CN=ia-get Project, O=Open Source
- Thumbprint displayed
- Timestamp certificate from DigiCert shown

### 5. Download Release Artifact

After workflow completes:
1. Go to: https://github.com/Gameaday/ia-get-cli/releases/tag/v2.0.1-test
2. Download: `ia-get-v2.0.1-test-x86_64-pc-windows-msvc.zip`
3. Extract and verify signature locally:

```powershell
Expand-Archive ia-get-v2.0.1-test-x86_64-pc-windows-msvc.zip -DestinationPath test-release
Get-AuthenticodeSignature test-release\ia-get.exe | Format-List *
```

**Expected local verification:**
```
SignerCertificate      : CN=ia-get Project, O=Open Source
TimeStamperCertificate : DigiCert SHA256 RSA4096 Timestamp Responder 2025 1
Status                 : UnknownError
StatusMessage          : A certificate chain processed, but terminated in a root 
                         certificate which is not trusted by the trust provider.
```

This is CORRECT - the signature is valid, Windows just doesn't trust self-signed certificates by default.

---

## Troubleshooting

### If workflow doesn't start
- Wait 1-2 minutes (GitHub Actions has a small delay)
- Refresh the Actions page
- Check: https://github.com/Gameaday/ia-get-cli/actions

### If "Sign Windows executable" fails
**Common issues:**

1. **Secret not found**
   - Error: "Input required and not supplied: CODE_SIGNING_CERT"
   - Solution: Verify secrets are named exactly (case-sensitive)

2. **Certificate decode failed**
   - Error: "Invalid length for a Base-64 char array"
   - Solution: Re-copy certificate value, ensure no truncation

3. **Import failed**
   - Error: "The specified network password is not correct"
   - Solution: Verify CODE_SIGNING_PASSWORD is correct

4. **Executable not found**
   - Error: "❌ Executable not found"
   - Solution: Build step may have failed - check cargo build output

### If signature shows "NotSigned"
- Check that the sign step actually completed
- Look for "✅ Successfully signed executable" message
- If not present, review sign step errors

---

## Success Criteria ✅

- [ ] Workflow starts within 1-2 minutes of push
- [ ] All jobs show green checkmarks
- [ ] "Sign Windows executable" step shows certificate imported
- [ ] "Sign Windows executable" step shows signing successful
- [ ] "Verify Windows code signature" shows signature details
- [ ] Release created at: https://github.com/Gameaday/ia-get-cli/releases/tag/v2.0.1-test
- [ ] Windows artifact downloadable
- [ ] Downloaded executable has signature (Status: UnknownError is OK)

---

## Next Steps After Success

1. **Review the workflow run** - Note any warnings or improvements needed

2. **Test the signed executable** - Download and run on Windows

3. **Document user experience** - What warnings do users see?

4. **Clean up test tag** (optional):
   ```bash
   git tag -d v2.0.1-test
   git push origin :refs/tags/v2.0.1-test
   gh release delete v2.0.1-test --yes  # If using GitHub CLI
   ```

5. **Create production release**:
   ```bash
   git tag v2.0.1
   git push origin v2.0.1
   ```

---

**Current Status**: Monitoring workflow at https://github.com/Gameaday/ia-get-cli/actions
**Expected Completion**: 15-20 minutes (all platforms)
**Windows Job**: ~5-10 minutes
