use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufReader};

fn main() {
  part_1().ok();
  part_2().ok();
}


// (x1, y1)
// (x2, y2)
// overlaps if any number in x1 to y1 is in x2 to  y2
//  if y1 = x2 

fn part_2() -> io::Result<()>{
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);

  let mut total = 0;

	for line in reader.lines() {
    let pairs = line.unwrap();

    let vec_pairs: Vec<&str> = pairs.split(',').collect();
    
    let elf1: Vec<&str> = vec_pairs[0].split('-').collect();
    let elf2: Vec<&str> = vec_pairs[1].split('-').collect();

		let x1 = elf1[0].parse::<i32>().unwrap();
    let y1 = elf1[1].parse::<i32>().unwrap();
    let x2 = elf2[0].parse::<i32>().unwrap();
    let y2 = elf2[1].parse::<i32>().unwrap();

		if !(y1 < x2 || y2 < x1) {
			total += 1;
		}
	}

  println!("part 2 answer: {}", total);

  Ok(())
}

// (x1, y1)
// (x2, y2)
// overlaps if x2 >= x1 and y2 <= y1
// x1 >= x2 and y1 <= y2

fn part_1() -> io::Result<()>{
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);

  let mut total = 0;

  for line in reader.lines() {
    let pairs = line.unwrap();

    let vec_pairs: Vec<&str> = pairs.split(',').collect();
    
    let elf1: Vec<&str> = vec_pairs[0].split('-').collect();
    let elf2: Vec<&str> = vec_pairs[1].split('-').collect();

    let x1 = elf1[0].parse::<i32>().unwrap();
    let y1 = elf1[1].parse::<i32>().unwrap();
    let x2 = elf2[0].parse::<i32>().unwrap();
    let y2 = elf2[1].parse::<i32>().unwrap();

    if (x2 >= x1 && y2 <= y1) || (x1 >= x2 && y1 <= y2) {
      total += 1;
    }
  }

  println!("part 1 answer: {}", total);

  Ok(())
}