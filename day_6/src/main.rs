
use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufReader};

// part 1 and 2 used the same solution
// just had to change MESSAGE_LENGTH from 4 to 14

const MESSAGE_LENGTH: usize = 14; 

fn main() -> io::Result<()> {
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);

  let mut signal: Vec<char> = Vec::new();

  for line in reader.lines() {
    let l = line.unwrap();
    signal = l.chars().collect();
  }

  let mut start = 0;
  let mut end = MESSAGE_LENGTH;
  
  loop {
    let mut subset = signal.get_mut(start..end).unwrap().to_vec();

    subset.sort();
    subset.dedup();

    if subset.len() ==  MESSAGE_LENGTH {
      break;
    }

    start += 1;
    end += 1;
  }

  println!("answer: {}", end );

  Ok(())
}