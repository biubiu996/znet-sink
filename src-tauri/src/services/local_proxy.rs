use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};

use crate::errors::{AppError, AppResult};

const LOCAL_PROXY_WAIT_TIMEOUT: Duration = Duration::from_secs(5);
const LOCAL_PROXY_WAIT_INTERVAL: Duration = Duration::from_millis(100);
const LOCAL_PROXY_CONNECT_TIMEOUT: Duration = Duration::from_millis(250);

pub(crate) fn wait_until_listening(host: &str, port: u16) -> AppResult<()> {
    let started = Instant::now();
    let mut last_error = None;

    while started.elapsed() < LOCAL_PROXY_WAIT_TIMEOUT {
        match probe(host, port) {
            Ok(()) => return Ok(()),
            Err(error) => {
                last_error = Some(error);
                std::thread::sleep(LOCAL_PROXY_WAIT_INTERVAL);
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        AppError::internal(format!("local proxy is not listening on {host}:{port}"))
    }))
}

fn probe(host: &str, port: u16) -> AppResult<()> {
    let address = format!("{host}:{port}");
    let mut addrs = address
        .to_socket_addrs()
        .map_err(|error| AppError::internal(format!("invalid local proxy endpoint: {error}")))?;
    let addr = addrs.next().ok_or_else(|| {
        AppError::internal(format!("cannot resolve local proxy endpoint: {address}"))
    })?;

    TcpStream::connect_timeout(&addr, LOCAL_PROXY_CONNECT_TIMEOUT)
        .map(|_| ())
        .map_err(|error| {
            AppError::internal(format!(
                "local proxy is not listening on {host}:{port}: {error}"
            ))
        })
}
