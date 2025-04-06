use core::fmt;

use rand::{rng, Rng};
use std::fmt::Display;

fn one_in(denominator: u32) -> bool {
  rng().random_ratio(1, denominator)
}

#[derive(Debug, PartialEq)]
pub enum FileState {
  Open,
  Closed,
}

impl Display for FileState {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      FileState::Open => write!(f, "OPEN"),
      FileState::Closed => write!(f, "CLOSED"),
    }
  }
}

#[derive(Debug)]
pub struct File {
  name: String,
  data: Vec<u8>,
  state: FileState,
}

impl Display for File {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "<{} ({})>", self.name, self.state)
  }
}

impl File {
  fn new(name: &str) -> File {
    File {
      name: String::from(name),
      data: Vec::new(),
      state: FileState::Closed,
    }
  }

  fn new_with_data(name: &str, data: &Vec<u8>) -> File {
    let mut f = File::new(name);
    f.data = data.clone();
    f
  }

  fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
    if self.state != FileState::Open {
      return Err(String::from("File must be open for reading"));
    }
    let mut tmp = self.data.clone();
    let read_length = tmp.len();

    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    Ok(read_length)
  }
}

fn open(mut f: File) -> Result<File, String> {
  if one_in(10_000) {
    let err_msg = String::from("Permission denied");
    return Err(err_msg);
  }
  f.state = FileState::Open;
  Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
  if one_in(10_000) {
    let err_msg = String::from("Interrupted by signal");
    return Err(err_msg);
  }
  f.state = FileState::Closed;
  Ok(f)
}

fn main() {
  let f_data: Vec<u8> = vec![114, 117, 115, 116, 33];
  let mut f = File::new_with_data("file.txt", &f_data);

  let mut buffer: Vec<u8> = vec![];

  f = open(f).unwrap();
  let f_length = f.read(&mut buffer).unwrap();
  f = close(f).unwrap();

  let text = String::from_utf8_lossy(&buffer);

  println!("{:?}", &f);
  println!("{}", &f);
  println!("{} is {} bytes long", &f.name, f_length);
  println!("{}", text);
}
