use std::{
    fs::OpenOptions,
    io::{self, Write},
    sync::Mutex,
};

pub fn init() -> io::Result<()> {
    let writer: Box<dyn Write + Send> = match std::env::var_os("TUBIX_LOG") {
        Some(path) => Box::new(OpenOptions::new().create(true).append(true).open(path)?),
        None => Box::new(io::sink()),
    };

    tracing_subscriber::fmt()
        .with_writer(Mutex::new(writer))
        .with_ansi(false)
        .with_max_level(tracing::Level::DEBUG)
        .try_init()
        .map_err(io::Error::other)
}
