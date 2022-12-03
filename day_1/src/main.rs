use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufReader};

// part 2 solution

fn main() -> io::Result<()>{
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);

  let mut v: Vec<i32> = Vec::new();
  let mut total = 0;

  for line in reader.lines() {
    let calorie  = line.unwrap();

    if calorie.is_empty() {
      v.push(total);
      total = 0;
    } else {
      total += calorie.parse::<i32>().unwrap();
    }
  }

  v.push(total);
  v.sort();

  let mut count = 0;
  let mut max = 0;

  loop {
    if count == 3 {
      break;
    }
    max += v.pop().unwrap();

    count += 1;
  }

  println!("answer: {}", max);
  
  Ok(())
}


// 
// part 1 solution
// post working on part 2
// 

/*** 

fn main() -> io::Result<()>{
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);

  let mut max = 0;
  let mut total = 0;

  for line in reader.lines() {
    let calorie  = line.unwrap();

    if calorie.is_empty() {
      if total > max {
        max = total;
      }
      total = 0;
    } else {
      total += calorie.parse::<i32>().unwrap();
    }
  }

  if total > max {
    max = total;
  }

  println!("answer: {}", max);
  
  Ok(())
}

***/

//
// part 1 solution (before bug fix)
// (working on part 2 found a bug where the last line is skipped)
//

/***

fn main() -> io::Result<()>{
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);

  let mut max = 0;
  let mut total = 0;

  for line in reader.lines() {
    let calorie  = line.unwrap();

    if calorie.is_empty() {
      if total > max {
        max = total;
        total = 0;
      }
      else {
        total = 0;
      }
    } else {
      total += calorie.parse::<i32>().unwrap();
    }
  }

  println!("answer: {}", max);
  
  Ok(())
}

***/
