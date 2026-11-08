# Roadmap

This document separates the implemented foundation from future work. Subsequent
phases require their own scoped implementation and review.

## Phase 0: Repository bootstrap

- README-only initial main commit, with develop branched from main.
- Foundation work isolated on a feature branch and reviewed through a PR.

## Phase 1: Foundation

- Single Cargo package, stable Rust, locked dependencies and MIT license.
- Dark dashboard, responsive empty state, footer links/version, keyboard help
  and clean global exit.
- Async bounded event flow, controlled redraws, cancellation, terminal restoration
  and optional file diagnostics.
- CLI help/version, offline tests and Linux/macOS/Windows CI.

## Future: Downloads and persistence

- yt-dlp download service, with three concurrent downloads by default.
- Controlled pause on exit and persistent history in SQLite.
- Playlist parent/child jobs, progress events and FFmpeg processing.
- Configuration, download directory validation and actionable failures.

## Future: Doctor and interaction

- `tubix doctor` detects OS/CPU, installed package managers, yt-dlp, FFmpeg and
  ffprobe; validates executable versions/paths, config, database and destination.
- `tubix doctor --fix` proposes OS-specific repairs and asks for explicit
  confirmation before system changes. CLI and TUI reuse one repair service.
- Expand global shortcuts, modal flows and a command palette as needed.

## Future: Verified releases and distribution

- GitHub Releases with precompiled Windows, macOS and Linux binaries; x86_64
  and ARM64 where supported and validated.
- SHA256 checksums, Cosign keyless signatures with documented verification,
  and GitHub build provenance attestations.
- Release Please for automated versioning and publication of the single `tubix`
  package to crates.io.
- Automatically update `Formula/tubix.rb` in
  [madebydaniz/homebrew-tap](https://github.com/madebydaniz/homebrew-tap)
  only after verified releases. No manual formula edits in the release flow.
- Planned Homebrew command: `brew install madebydaniz/tap/tubix`.
- Review [Linguasteg](https://github.com/madebydaniz/linguasteg) as a reference
  when implementing release automation; adapt it to Tubix's single-package
  architecture rather than copying workflows.

No release, repair, download, playlist or FFmpeg pipeline is implemented in Phase 1.
