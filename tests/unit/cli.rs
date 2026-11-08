use clap::{Parser, error::ErrorKind};
use tubix::app::cli::Cli;

#[test]
fn no_arguments_selects_dashboard() {
    assert!(Cli::try_parse_from(["tubix"]).is_ok());
}

#[test]
fn help_and_version_are_successful_early_exits() {
    for (flag, kind) in [
        ("--help", ErrorKind::DisplayHelp),
        ("--version", ErrorKind::DisplayVersion),
    ] {
        let error = Cli::try_parse_from(["tubix", flag]).unwrap_err();
        assert_eq!(error.kind(), kind);
        assert_eq!(error.exit_code(), 0);
    }
}

#[test]
fn version_comes_from_package_metadata() {
    let output = Cli::try_parse_from(["tubix", "--version"])
        .unwrap_err()
        .to_string();
    assert_eq!(
        output.trim(),
        format!("tubix {}", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn future_commands_are_not_advertised_or_accepted() {
    assert!(Cli::try_parse_from(["tubix", "doctor"]).is_err());
    assert!(Cli::try_parse_from(["tubix", "--fix"]).is_err());
}
