// conins available: 25, 10, 5, 1
// figure out which coins must be returned
// use floats

use std::io;
use std::io::prelude::*;

fn main () {
  loop {
    print!("amount: ");
    io::stdout().flush().unwrap();

    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();

    match line.trim().parse::<f32>() {
      Err(_) => println!("Please provide a valid number"),
      Ok(n) => {
        let change = calculate_change(n);
        match change {
          1 => println!("{} coin", change),
          _ => println!("{} coins", change),
        };
        break;
      },
    };
  }

  fn calculate_change (n: f32) -> i32 {
    let mut remaining: f32 = n;
    let mut i: i32 = 0;
    loop {
      match n {
        _ if remaining >= 0.25 => {
          remaining -= 0.25;
          i += 1;
        },
        _ if remaining >= 0.10 => {
          remaining -= 0.10;
          i += 1;
        },
        _ if remaining >= 0.05 => {
          remaining -= 0.05;
          i += 1;
        },
        _ if remaining >= 0.01 => {
          remaining -= 0.01;
          i += 1;
        },
        0.0|_ => break,
      };
    };
    return i;
  }
}
