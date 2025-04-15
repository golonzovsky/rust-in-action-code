use chrono::DateTime;
use chrono::Local;
use clap::{Arg, Command};

// struct with no fields is known as a zero-sized type or ZST
struct Clock;

impl Clock {
  fn get() -> DateTime<Local> {
    Local::now()
  }

  fn set() -> ! {
    unimplemented!()
    //todo!()
  }
}

fn main() {
  let app = Command::new("clock")
    .version("0.1")
    .about("Gets and (aspirationally) sets the time.")
    .arg(Arg::new("action").value_parser(["get", "set"]).default_value("get"))
    .arg(
      Arg::new("std")
        .short('s')
        .long("use-standard")
        .value_parser(["rfc2822", "rfc3339", "timestamp"])
        .default_value("rfc3339"),
    )
    .arg(Arg::new("datetime").help(
      "When <action> is 'set', apply <datetime>. \
       Otherwise, ignore.",
    ));

  let args = app.get_matches();

  let action = args.get_one::<String>("action").expect("default form clap");
  let std = args.get_one::<String>("std").expect("default from clap");

  if action == "set" {
    unimplemented!()
  }

  let now = Clock::get();
  match std.as_str() {
    "timestamp" => println!("{}", now.timestamp()),
    "rfc2822" => println!("{}", now.to_rfc2822()),
    "rfc3339" => println!("{}", now.to_rfc3339()),
    _ => unreachable!(),
  }
}
