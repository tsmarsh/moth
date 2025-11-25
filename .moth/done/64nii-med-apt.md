## Implementation Tasks

### 1. Debian package build
- [ ] Create `debian/` directory structure
- [ ] Write `debian/control` (package metadata)
- [ ] Write `debian/rules` (build instructions)
- [ ] Write `debian/copyright`
- [ ] Build with `dpkg-buildpackage` or `cargo-deb`

### 2. Cross-compilation
- [ ] Set up GitHub Actions for linux/amd64
- [ ] Set up GitHub Actions for linux/arm64
- [ ] Use `cross` or native runners

### 3. GPG signing
- [ ] Generate dedicated signing key for moth releases
- [ ] Store private key as GitHub secret
- [ ] Sign Release files in CI

### 4. Repository generation
- [ ] Script to generate Packages files (`dpkg-scanpackages`)
- [ ] Script to generate Release file
- [ ] Script to sign and create InRelease

### 5. Hosting
- [ ] Option A: GitHub Pages from `gh-pages` branch
- [ ] Option B: S3 bucket with CloudFront
- [ ] Option C: Self-hosted on moth.dev

### 6. CI/CD pipeline
- [ ] On git tag: build .deb for both architectures
- [ ] Generate repo metadata
- [ ] Sign everything
- [ ] Push to hosting

### 7. Documentation
- [ ] Add install instructions to README
- [ ] Add to website (if exists)

## Tools/Crates to Evaluate

- `cargo-deb` - generates .deb directly from Cargo.toml
- `reprepro` - manages APT repos
- `aptly` - alternative repo manager

## Edge Cases

- [ ] Upgrades: ensure version ordering works
- [ ] Downgrades: should work with `apt install moth=0.1.0`
- [ ] Removal: clean uninstall with `apt remove moth`
- [ ] Conflicts: don't conflict with any existing package

## Acceptance Criteria

- [ ] Fresh Ubuntu 22.04 can install via documented instructions
- [ ] Fresh Debian 12 can install via documented instructions
- [ ] `apt update` fetches latest version
- [ ] `apt upgrade` upgrades moth when new version released
- [ ] GPG signature validates correctly