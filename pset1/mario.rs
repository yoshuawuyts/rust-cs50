// prompt input
// non negative
// repeat on incorrect input
// int < 23
// top is 2 blocks
// right aligned

use std::io;
use std::io::prelude::*;

fn main () {
  loop {
    print!("height: ");
    io::stdout().flush().unwrap();

    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();

    // handle input err cases
    let height: i32 = match line.trim().parse::<i32>() {
      Err(_) => {
        println!("Input must be a number");
        continue;
      },
      Ok(n) => match n {
        n if n < 0 => {
          println!("Number must be positive");
          continue;
        },
        n if n > 23 => {
          println!("Number must be smaller than 23");
          continue;
        },
        0...23 => n,
        _ => panic!("no matches found")
      },
    };

    print_blocks(height);
    break;
  }
}

fn print_blocks(height: i32) {
  let max: i32 = height + 1;
  for i in 2..height + 1 {
    let blocks = i;
    let white = max - i;
    for _ in 0..white {
      print!(" ");
    }
    for _ in 0..blocks {
      print!("#");
    }
    print!("\n");
  };
}
