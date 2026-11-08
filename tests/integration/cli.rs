use std::process::{Command, Stdio};

#[test]
fn cli_metadata_does_not_require_a_terminal() {
    for flag in ["--help", "--version"] {
        let output = Command::new(env!("CARGO_BIN_EXE_tubix"))
            .arg(flag)
            .output()
            .unwrap();
        assert!(output.status.success());
        assert!(output.stderr.is_empty());
        assert!(!output.stdout.contains(&0x1b));
    }
}

#[test]
fn noninteractive_startup_fails_without_escape_sequences() {
    let output = Command::new(env!("CARGO_BIN_EXE_tubix"))
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("interactive terminal"));
    assert!(!output.stdout.contains(&0x1b));
}
