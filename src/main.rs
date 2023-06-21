mod calculate;
mod operations;
mod parse;
mod time;

use crate::calculate::calculate;

use std::process::ExitCode;

/*
  Grammar for calculator

  nanoseconds  -> "ns" | "nanoseconds"
  milliseconds -> "ms" | "milliseconds"
  microseconds -> "us" | "microseconds"
  seconds      -> "s"  | "seconds" | "sec"

  time_unit -> nanoseconds | milliseconds | microseconds | seconds

  number -> (0-9)+

  time_value -> number time_unit

  // Just supporting simple operators for now
  operator ->  "+" | "-"

  // A calculation can be converted to a specific unit of time
  conversion ->  time_value "as" time_unit

  // An expression can be a value or other expressions that return values
  expr -> time_value | time_value operator time_value | time_value conversion

*/

fn main() -> ExitCode {
    let args = std::env::args();
    if args.len() == 1 {
        eprintln!("usage: timecalc <EXPR>");
        return ExitCode::FAILURE;
    }

    let input = args.skip(1).collect::<Vec<String>>().join("");

    let res = calculate(&input);
    match res {
        Ok(value) => {
            println!("Result: {}", value);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("For input '{}'\n{}", input, e);
            ExitCode::FAILURE
        }
    }
}
