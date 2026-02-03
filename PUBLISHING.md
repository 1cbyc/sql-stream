# Publishing Guide

This guide explains how to publish sql-stream to crates.io and create GitHub releases.

## Prerequisites

### 1. Crates.io Account
- Create an account at https://crates.io
- Generate an API token: https://crates.io/me
- Store the token securely

### 2. GitHub Token for Releases
- Go to GitHub Settings > Developer settings > Personal access tokens
- Generate a token with `repo` scope
- Add as repository secret: `CARGO_REGISTRY_TOKEN`

### 3. Local Setup
```bash
# Login to crates.io (one-time setup)
cargo login <your-api-token>
```

## Publishing to crates.io

### Step 1: Verify Package
The dry-run has already passed, confirming the package is ready:
```bash
cargo publish --dry-run
```

### Step 2: Publish
```bash
cargo publish
```

This will:
- Upload the package to crates.io
- Make it available for `cargo install sql-stream`
- Publish documentation to docs.rs

### Step 3: Verify
- Check https://crates.io/crates/sql-stream
- Check https://docs.rs/sql-stream
- Test installation: `cargo install sql-stream`

## Creating GitHub Releases

### Automatic Release (Recommended)

The project has an automated release workflow. To trigger a release:

```bash
# 1. Update version in Cargo.toml
# Edit Cargo.toml and change version = "0.1.0" to "0.2.0"

# 2. Commit the version bump
git add Cargo.toml
git commit -m "Bump version to 0.2.0"
git push

# 3. Create and push a version tag
git tag v0.2.0
git push origin v0.2.0
```

This will automatically:
- Create a GitHub release with the tag
- Build binaries for:
  - Linux (x86_64)
  - Linux (x86_64-musl)
  - Windows (x86_64)
  - macOS (Intel x86_64)
  - macOS (Apple Silicon aarch64)
- Upload all binaries as release assets
- Publish to crates.io (if CARGO_REGISTRY_TOKEN is set)

### Manual Release

If you prefer manual control:

```bash
# 1. Create a tag
git tag -a v0.1.0 -m "Release version 0.1.0"

# 2. Push the tag
git push origin v0.1.0

# 3. Go to GitHub > Releases > Draft a new release
# 4. Choose the tag v0.1.0
# 5. Upload pre-built binaries manually
```

## Release Checklist

Before creating a release, ensure:

- [ ] All tests pass: `cargo test`
- [ ] Code is formatted: `cargo fmt --check`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Documentation builds: `cargo doc --no-deps`
- [ ] CHANGELOG.md is updated (if you have one)
- [ ] Version is bumped in Cargo.toml
- [ ] README.md examples work
- [ ] CI passes on main branch

## Version Numbering

Follow Semantic Versioning (SemVer):

- **MAJOR** (0.x.0 -> 1.0.0): Breaking API changes
- **MINOR** (0.1.x -> 0.2.0): New features, backward compatible
- **PATCH** (0.1.0 -> 0.1.1): Bug fixes, backward compatible

Examples:
- `0.1.0` - Initial release
- `0.1.1` - Bug fix release
- `0.2.0` - New features (Parquet support, etc.)
- `1.0.0` - Stable API, production ready

## Post-Release Tasks

After publishing:

1. **Announce the release**
   - Post on social media
   - Update project website (if any)
   - Notify relevant communities

2. **Monitor for issues**
   - Watch GitHub issues
   - Check crates.io download stats
   - Respond to user feedback

3. **Update documentation**
   - Ensure docs.rs is properly generated
   - Update examples if needed

## Troubleshooting

### Publishing Fails

**Error: crate already exists**
- You cannot republish the same version
- Bump the version and try again

**Error: authentication failed**
- Run `cargo login` again
- Verify your API token is valid

**Error: uncommitted changes**
- Commit all changes first
- Or use `cargo publish --allow-dirty` (not recommended)

### Release Workflow Fails

**Build fails on specific platform**
- Check the GitHub Actions logs
- Test locally with: `cargo build --release --target <target>`

**Upload fails**
- Verify GITHUB_TOKEN has correct permissions
- Check if release already exists

## Automated Publishing Setup

To enable automatic publishing to crates.io on releases:

1. Generate a crates.io API token
2. Add it as a GitHub repository secret named `CARGO_REGISTRY_TOKEN`
3. The release workflow will automatically publish when you push a tag

## Current Status

- ✅ Package is ready for publishing (dry-run passed)
- ✅ All metadata is configured in Cargo.toml
- ✅ Release workflow is set up
- ✅ README has installation instructions
- ⏳ Ready to execute: `cargo publish`

## First Release Commands

```bash
# Execute the first publish
cargo publish

# Create the first release tag
git tag v0.1.0
git push origin v0.1.0

# GitHub Actions will automatically build and upload binaries
```

That's it! Your project will be:
- Published on crates.io at https://crates.io/crates/sql-stream
- Documented at https://docs.rs/sql-stream
- Available via `cargo install sql-stream`
- Released on GitHub with binary downloads
