use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::net::{AddrParseError, Ipv6Addr};

#[derive(Debug)]
enum UpstreamError {
  IO(std::io::Error),
  Parsing(AddrParseError),
}

impl fmt::Display for UpstreamError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl From<io::Error> for UpstreamError {
  fn from(error: io::Error) -> Self {
    UpstreamError::IO(error)
  }
}

impl From<AddrParseError> for UpstreamError {
  fn from(error: AddrParseError) -> Self {
    UpstreamError::Parsing(error)
  }
}

impl error::Error for UpstreamError {}

fn main() -> Result<(), UpstreamError> {
  let _f = File::open("invisible.txt")?;

  let _localhost = "::1".parse::<Ipv6Addr>()?;

  Ok(())
}
