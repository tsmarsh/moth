# Homebrew tap for macOS distribution

Distribute moth to macOS users via brew.

## End User Experience
```bash
# Install
brew tap moth-tracker/tap
brew install moth

# Or direct
brew install moth-tracker/tap/moth
```

Long-term goal: get into homebrew-core for just `brew install moth`.

## Formula

`Formula/moth.rb`:
```ruby
class Moth < Formula
  desc "Simple file-based issue tracker that lives in your git repo"
  homepage "https://github.com/moth-tracker/moth"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/moth-tracker/moth/releases/download/v0.1.0/moth-0.1.0-aarch64-apple-darwin.tar.gz"
      sha256 "..."
    else
      url "https://github.com/moth-tracker/moth/releases/download/v0.1.0/moth-0.1.0-x86_64-apple-darwin.tar.gz"
      sha256 "..."
    end
  end

  def install
    bin.install "moth"
  end

  test do
    system "#{bin}/moth", "--version"
  end
end
```

## Repository Structure
```
homebrew-tap/
├── Formula/
│   └── moth.rb
└── README.md
```

Hosted at: `github.com/moth-tracker/homebrew-tap`

## Implementation Tasks

### 1. Create tap repository
- [ ] Create `moth-tracker/homebrew-tap` repo
- [ ] Add Formula directory
- [ ] Add README with install instructions

### 2. Build macOS binaries
- [ ] GitHub Actions: build for x86_64-apple-darwin
- [ ] GitHub Actions: build for aarch64-apple-darwin
- [ ] Sign binaries (optional but recommended)
- [ ] Notarize with Apple (optional but prevents Gatekeeper warnings)

### 3. Release artifacts
- [ ] Create tar.gz archives with just the binary
- [ ] Upload to GitHub Releases
- [ ] Generate sha256 checksums

### 4. Formula automation
- [ ] Script to update formula version
- [ ] Script to update sha256 hashes
- [ ] Triggered on new git tag

### 5. CI/CD pipeline
- [ ] On git tag: build both architectures
- [ ] Create GitHub Release
- [ ] Upload artifacts
- [ ] Auto-PR to update homebrew-tap formula

### 6. homebrew-core submission (future)
- [ ] Must have notable number of users/stars
- [ ] Must meet homebrew quality standards
- [ ] Submit PR to homebrew/homebrew-core
- [ ] Remove tap instructions once accepted

## Build Targets

| Target | Runner | Notes |
|--------|--------|-------|
| x86_64-apple-darwin | macos-13 | Intel Macs |
| aarch64-apple-darwin | macos-14 | Apple Silicon |

## Signing & Notarization (Optional)

For Gatekeeper-friendly distribution:
- [ ] Apple Developer account
- [ ] Generate Developer ID certificate
- [ ] Sign binary with `codesign`
- [ ] Notarize with `xcrun notarytool`
- [ ] Staple ticket (not applicable for CLI tools distributed via tar)

May not be necessary for Homebrew installs (brew handles quarantine).

## Edge Cases

- [ ] Rosetta 2: x86 binary works on Apple Silicon via emulation
- [ ] Old macOS versions: set minimum deployment target
- [ ] Xcode CLI tools dependency: moth shouldn't require them at runtime

## Acceptance Criteria

- [ ] `brew tap moth-tracker/tap && brew install moth` works on Intel Mac
- [ ] `brew tap moth-tracker/tap && brew install moth` works on Apple Silicon
- [ ] `brew upgrade moth` fetches new versions
- [ ] `brew uninstall moth` cleanly removes
- [ ] Formula passes `brew audit --strict moth`