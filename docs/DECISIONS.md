# Architecture decisions

## 2026-11-08: Foundation

1. **Single package.** `tubix` contains a small binary and modular library.
   Separate publishable crates add overhead without a current consumer.
2. **Stable dependencies.** Verified Rust stable 1.99.1 and crates.io stable
   versions on 2026-11-08: Ratatui 0.30.2, Crossterm 0.29.0, Tokio 1.54.1,
   Clap 4.6.7, tracing 0.1.44, tracing-subscriber 0.3.23 and futures-util 0.3.34.
   All declared minimum Rust versions are compatible with the 1.99 baseline.
   Ratatui explicitly selects its Crossterm 0.29 backend. Cargo.lock records the
   resolved graph; stable toolchain and CI track stable Rust updates.
3. **One terminal owner.** Background tasks send events over a queue of 64.
   A watch channel coordinates cancellation without adding a cancellation crate.
   Input uses Crossterm's async stream; no application polling thread is needed.
4. **Redraw on change.** A dirty flag and a one-shot frame deadline coalesce
   changes at approximately 30 FPS. An always-running frame ticker would wake
   the application unnecessarily while idle.
5. **Small command and modal model.** Input translates into commands and a
   single optional modal. Help exercises these boundaries without implementing
   a command palette, download actions or an extensible UI framework.
6. **Safe terminal lifecycle.** Reject redirected input/output before enabling
   raw mode. Use a guard, initialization rollback and Ratatui's panic hook.
   Log to an optional file, never over the active dashboard.
7. **Tests outside production source.** Root integration-test entry points
   register nested tests. Test public boundaries with deterministic inputs and
   a test backend, without test-only production API or inline test modules.
8. **Open-source baseline.** MIT license, English project documentation and
   Conventional Commits. Bootstrap main has README only. All foundation work
   lives on `feat/project-foundation`, following maintainer branch conventions,
   with a PR to develop; integration requires maintainer approval.

References: [Ratatui installation](https://ratatui.rs/installation/),
[Crossterm events](https://docs.rs/crossterm/0.29.0/crossterm/event/index.html),
[Tokio shutdown](https://tokio.rs/tokio/topics/shutdown),
[Clap](https://docs.rs/clap/4.6.7/clap/).
