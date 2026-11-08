use clap::Parser;
use tubix::{app::cli::Cli, system::diagnostics, tui::TerminalSession};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::process::ExitCode {
    let _cli = Cli::parse();
    let result = async {
        diagnostics::init()?;
        let mut session = TerminalSession::enter()?;
        tubix::app::start(session.terminal_mut()).await
    }
    .await;

    match result {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("tubix: {error}");
            std::process::ExitCode::FAILURE
        }
    }
}
