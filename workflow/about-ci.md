# GitHub Actions workflows

This repository uses GitHub Actions for CI, automated dependency updates, and releases.

Workflow files live under:

- `.github/workflows/ci.yml`
- `.github/workflows/release.yml`
- `.github/workflows/auto-tag.yml`
- `.github/workflows/version-bump.yml`
- `.github/workflows/gitbot-deps.yml`

## Overview

### CI (`.github/workflows/ci.yml`)

**Triggers**

- `push` to `main`
- `pull_request` targeting `main`

**What it does**

- **Security audit**: runs `cargo-audit` using `cargo +stable` to avoid MSRV breakage when `cargo-audit` bumps its required Rust version.
- **MSRV**: runs `cargo test --all` on Rust `1.81.0`.
- **Test + Build (debug)**: runs `cargo test --all` and `cargo build --profile dev`.
- **Format + Clippy**: runs `cargo fmt -- --check` and `cargo clippy ... -D warnings`.
- **Dependency policy**: installs `cargo-deny` and runs `cargo +stable deny check all`.

**Notes**

- This workflow is meant to keep `main` green and enforce formatting/lints/policy.

### Release (`.github/workflows/release.yml`)

**Triggers**

- `push` tag matching `v*` (recommended)
- `workflow_dispatch` (manual) with `tag` input

**What it builds**

- Linux (runner host build)
- macOS (runner host build)
- Android aarch64 (cross-compiled): `aarch64-linux-android`

**Build steps (per platform job)**

- Tests: `cargo test --all`
- Builds:
  - `cargo build --profile dev --locked`
  - `cargo build --profile release --locked`
- Checks:
  - `cargo fmt -- --check`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo +stable deny check all`

**Packaging output**

Each build produces:

- `cosmostrix-bin-<platform>.tar.xz`
- `cosmostrix-bin-<platform>.tar.xz.sha512`

Where `<platform>` is one of:

- `linux-amd64`
- `macos-arm64`
- `android-aarch64`

The archive contains:

- `cosmostrix` binary
- `README.md`
- `LICENSE`

**Checksums**

Checksum files are generated using:

- `sha512sum` when available, else
- `shasum -a 512`

Verification examples:

```bash
# Linux
sha512sum -c cosmostrix-bin-linux-amd64.tar.xz.sha512

# macOS
shasum -a 512 -c cosmostrix-bin-macos-arm64.tar.xz.sha512
```

**Release publishing**

The `publish_release` job:

- downloads all build artifacts
- generates release notes from git history (since previous `v*` tag)
- creates a GitHub Release and uploads all `*.tar.xz` and `*.tar.xz.sha512` files

### Auto Tag (`.github/workflows/auto-tag.yml`)

**Trigger**

- `push` to `main`

**Condition**

- only runs when the latest commit subject starts with `release:`

**What it does**

- extracts the version from the first line of the commit message:
  - example: `release: 1.0.1`
- validates the version format
- verifies `Cargo.toml` version matches the `release:` version
- creates and pushes an annotated tag `v<version>`

### Auto Version Bump (`.github/workflows/version-bump.yml`)

**Triggers**

- scheduled daily
- manual `workflow_dispatch`
- `push` to `main` when the head commit message matches `chore(deps): auto-update dependencies...`

**What it does**

- counts dependency update commits since last `v*` tag
- every 10 such commits, it bumps `Cargo.toml` patch version and creates a signed commit:
  - `release: X.Y.(Z+1)`

This release commit will then be picked up by **Auto Tag**.

### Auto Update Dependencies (`.github/workflows/gitbot-deps.yml`)

**Triggers**

- scheduled daily
- manual `workflow_dispatch` with inputs:
  - `force_update` (continue even if tests fail)
  - `strategy` (`direct` push-to-main, or `pr`)

**What it does**

- runs `cargo update`
- runs a best-effort `cargo audit`
- builds with `cargo build --profile release`
- runs `cargo test --all` (can be forced non-blocking)
- commits and pushes updates to `main` (default) or opens a PR

## Typical release flow

```bash
# 1) Update Cargo.toml version
# 2) Commit with the release version
git commit -am "release: 1.0.1"

# optional prerelease examples:
# git commit -am "release: 1.0.1-rc.1"
# git commit -am "release: 1.0.1-alpha.1"

# 3) Push to main
git push origin main
```

- The **Auto Tag** workflow will create and push tag `v1.0.1`.
- The **Release** workflow will run on that tag.
- If the version contains `-` (e.g. `-rc.1`, `-alpha.1`), the GitHub Release is marked as a prerelease.
