# Architecture

Tubix is one Cargo package containing a binary entry point and a library of
internal application modules. Phase 1 contains only the dashboard foundation.

## Responsibilities

| Module | Responsibility |
| --- | --- |
| `app` | State, commands, modal state, lifecycle and event orchestration |
| `app::cli` | Clap parsing; metadata exits before terminal initialization |
| `app::event` | Async input producer and bounded event messages |
| `app::runtime` | Single-owner state updates and rendering schedule |
| `tui::input` | Global keyboard shortcuts translated into commands |
| `tui::view` | Pure, responsive Ratatui rendering and dark theme |
| `tui::TerminalSession` | Interactive-terminal validation and restoration guard |
| `system` | File-only diagnostics and OS shutdown signals |

Future `core`, `download`, and `storage` modules will own domain models,
yt-dlp orchestration, and SQLite/configuration respectively. They are not created
until they have real responsibilities. No download or persistence code exists yet.

## Event flow

```text
Crossterm EventStream ── input task ──┐
                                    ├── bounded mpsc (64) ── app runtime ── terminal
OS shutdown signals ── signal task ──┘                          │
                                                              └── state + commands
```

The Tokio multi-thread runtime hosts background producers. Only the application
loop owns and draws the terminal. Input, shutdown, errors, and redraw requests
travel through the bounded queue. Future background services can use the same
boundary without obtaining a terminal handle.

The initial frame is drawn immediately. Subsequent frames require a dirty state
and a deadline at least 1/30 second after the previous draw. Updates within a frame
are coalesced. Unrecognized input and an idle dashboard do not cause redraws or
periodic wakeups. A ready frame deadline takes priority to avoid starvation under
continuous event traffic.

Keyboard input maps to commands before state mutation. Global quit works over
the help modal. The optional modal state supplies a small foundation for future
dialogs; there is no command palette or component framework.

## Shutdown and failures

`q`, `Ctrl+C`, OS interrupt, and Unix termination signals request clean shutdown.
End-of-input also requests shutdown; event errors propagate to the caller. The
application cancels input and signal producers using a watch channel and joins
them before returning. Cancellation also interrupts a producer waiting for queue
capacity. Closing the event receiver unblocks pending sends.

The terminal guard restores raw mode, the alternate screen and cursor visibility
on normal return and error paths, including partial initialization. Ratatui's panic
hook restores the terminal before printing panic diagnostics; guard destruction
also restores it during unwinding. Forced process termination cannot run cleanup.
Error messages print only after the guard has been dropped.

Tracing defaults to a sink. `TUBIX_LOG` enables append-only file diagnostics;
workers never print to stdout or stderr. File setup occurs before terminal setup.

## Tests and CI

`tests/suite.rs` registers unit, integration and TUI tests from root `tests/`.
The suite tests public behavior using Clap, Ratatui's `TestBackend`, deterministic
streams and paused Tokio time. It needs no YouTube access or external services.
Empty fixture folders are intentionally absent.

CI runs formatting, Clippy, all tests and a release build using stable Rust on
Linux, macOS and Windows. Lockfile resolution is fixed in CI with `--locked`.
