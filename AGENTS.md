# Development rules

- Keep Tubix a single Cargo package with focused internal modules. Preserve
  existing boundaries and avoid unused scaffolding and speculative abstractions.
- Develop on `feat/*` branches from `develop`. Open PRs targeting `develop`;
  merge only with required checks/reviews and explicit maintainer approval.
  Preserve branch protections and signing. Never force-push or rewrite history.
- Validate and present local changes first. Wait for the maintainer to check
  them and explicitly approve continuation before committing, pushing, or
  creating or updating a PR.
- Use Conventional Commits for commits and PR titles. Commit messages contain
  one imperative subject of at most 70 characters, without bodies or trailers.
  Keep PR descriptions empty unless essential context is required.
- Keep all project-owned tests under root `tests/`. Register nested tests through
  Cargo-discoverable entry points. Do not add inline tests to production files
  or expose internals solely to test them. Tests must be offline and deterministic.
- Maintain one terminal owner. Render without blocking work. Use async tasks,
  bounded channels, cancellation and event-driven redraws capped near 30 FPS.
  Workers must never write to the terminal. Restore it on errors and panics.
- Use compatible stable Rust and dependencies; verify versions before upgrades.
  Commit `Cargo.lock`. Do not implement future roadmap features implicitly.
- Run formatting, Clippy, focused tests, all tests and release build before
  finishing. Report changed files, validation and unresolved issues.
- Write documentation and necessary code comments in English. Keep docs concise
  and consistent with implemented behavior. Communicate with the maintainer in
  Finglish. Use TokenSave for indexed code exploration and Context7 for library
  API documentation; fall back to targeted source inspection when unavailable.
- Do not modify unrelated files. Keep optional diagnostics in a file, never on
  the active TUI. Require explicit confirmation before future system repairs.
