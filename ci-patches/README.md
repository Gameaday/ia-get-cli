# Release pipeline fixes (pending application)

These patches fix the release pipeline. They are carried as patch files rather
than as direct commits because the credentials available to the automated
session that produced them cannot push changes under `.github/workflows/`
(`refusing to allow a GitHub App to create or update workflow ... without
'workflows' permission`).

Apply them on `main`, in order, then cut the release:

```bash
git am ci-patches/0001-*.patch ci-patches/0002-*.patch ci-patches/0003-*.patch
git push origin main
git tag v2.1.1
git push origin v2.1.1
```

| Patch | Touches | Summary |
| --- | --- | --- |
| `0001` | `release.yml` | Attach the built archives to the release. The packaging step writes them to `artifacts/`, but the upload pattern was only matched from the repository root, so v2.1.0 was published with no binaries. Also generates one combined `RELEASE_HASHES.txt`, expands the release-notes heredoc and collects checksums with `gh release download`. |
| `0002` | `release.yml`, deletes `before_deploy.sh` | Drops the duplicate checkout, the unused binutils install, `MACOSX_DEPLOYMENT_TARGET: 10.7`, the dead `.deb` upload pattern and the stale `--features gui` comments; declares `permissions: contents: write`. |
| `0003` | `rust-ci.yml` | Removes the duplicate release job, which could never run (a release created with `GITHUB_TOKEN` does not trigger further workflows) and whose upload glob `artifacts/*.{tar.gz,zip}` is not valid `@actions/glob` syntax; declares `permissions: contents: read`. |

The series applies cleanly on top of the last release commit and reproduces a
tree that was reviewed locally; the shell bodies were executed against the real
v2.1.0 release assets where possible. The workflow YAML has not been executed
end to end, so the first tagged run should be watched.

This directory can be deleted once the patches are applied.
