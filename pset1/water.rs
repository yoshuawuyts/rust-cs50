// Write, in a file called water.c in your ~/workspace/pset1 directory, a
// program that prompts the user for the length of his or her shower in minutes
// (as a positive integer) and then prints the equivalent number of bottles of
// water (as an integer) per the sample output below, wherein underlined text
// represents some user’s input.
//
// username@ide50:~/workspace/pset1 $ ./water
// minutes: 10
// bottles: 120
//
// a 1-minute shower is akin to using 192 ÷ 16 = 12 bottles of water
//
// http://stackoverflow.com/questions/28528998/how-do-i-read-a-single-string-from-standard-input

use std::io;
use std::io::prelude::*;

fn main () {
  let mut line = String::new();
  let stdin = io::stdin();

  stdin.lock().read_line(&mut line).unwrap();
}
