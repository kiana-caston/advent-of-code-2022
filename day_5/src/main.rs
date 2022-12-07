use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufReader};

const SPACE_BETWEEN_CRATES: usize = 4;

fn main() {
  part_1().ok();
  part_2().ok();
}

fn part_2() -> io::Result<()>{
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);

  let mut instructions: Vec<Vec<usize>> = Vec::new();
  let mut stacks: Vec<Vec<char>> = Vec::new();
  let mut crates: Vec<Vec<char>> = Vec::new();

  let mut get_crates = true;

  for line in reader.lines() {
    let row = line.unwrap();

    if row.is_empty() {
      get_crates = false;
      continue;
    }
    
    if get_crates {
      let split_row: Vec<char> = row.chars().collect();
      crates.push(split_row[1..split_row.len()-1].to_vec());
    }

    if !get_crates {
      let split_instruction: Vec<&str> = row.split_whitespace().collect();
      let mut vec: Vec<usize> = Vec::new();

      vec.push(split_instruction[1].parse::<usize>().unwrap());
      vec.push(split_instruction[3].parse::<usize>().unwrap());
      vec.push(split_instruction[5].parse::<usize>().unwrap());

      instructions.push(vec);
    }
  }

  let stack_numbers = crates.pop().unwrap();
  let first_crate_row = crates.pop().unwrap();
  let mut crate_pos = 0;

  for n in stack_numbers {
    if n.is_numeric() {
      let mut stack = Vec::new();

      stack.push(first_crate_row[crate_pos]);
      stacks.push(stack);

      crate_pos += SPACE_BETWEEN_CRATES;
    }
  }

  loop {
    if crates.is_empty() {
      break;
    }

    let crate_row = crates.pop().unwrap();

    crate_pos = 0;

    for i in 0..stacks.len() {
      let c = crate_row[crate_pos];
      if c.is_alphabetic() {
        stacks[i].push(c);
      }
      crate_pos += SPACE_BETWEEN_CRATES;
    }
  }

  for instruction in instructions {
    let number_to_move = instruction[0];
    let from_num = instruction[1] - 1;
    let to_num = instruction[2] - 1;
    let size = stacks[from_num].len() - number_to_move;
    let mut crates_to_move = stacks[from_num].split_off(size);

    stacks[to_num].append(&mut crates_to_move);

  }

  let mut answer = "".to_string();

  for mut stack in stacks {
    let top_crate = stack.pop().unwrap();

    answer.push(top_crate.clone());
  } 

  println!("part 2 answer: {}", answer);

  Ok(())
}

fn part_1() -> io::Result<()>{
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);

  let mut instructions: Vec<Vec<usize>> = Vec::new();
  let mut stacks: Vec<Vec<char>> = Vec::new();
  let mut crates: Vec<Vec<char>> = Vec::new();

  let mut get_crates = true;

  for line in reader.lines() {
    let row = line.unwrap();

    if row.is_empty() {
      get_crates = false;
      continue;
    }
    
    if get_crates {
      let split_row: Vec<char> = row.chars().collect();
      crates.push(split_row[1..split_row.len()-1].to_vec());
    }

    if !get_crates {
      let split_instruction: Vec<&str> = row.split_whitespace().collect();
      let mut vec: Vec<usize> = Vec::new();

      vec.push(split_instruction[1].parse::<usize>().unwrap());
      vec.push(split_instruction[3].parse::<usize>().unwrap());
      vec.push(split_instruction[5].parse::<usize>().unwrap());

      instructions.push(vec);
    }
  }

  let stack_numbers = crates.pop().unwrap();
  let first_crate_row = crates.pop().unwrap();
  let mut crate_pos = 0;

  for n in stack_numbers {
    if n.is_numeric() {
      let mut stack = Vec::new();

      stack.push(first_crate_row[crate_pos]);
      stacks.push(stack);

      crate_pos += SPACE_BETWEEN_CRATES;
    }
  }

  loop {
    if crates.is_empty() {
      break;
    }

    let crate_row = crates.pop().unwrap();

    crate_pos = 0;

    for i in 0..stacks.len() {
      let c = crate_row[crate_pos];
      if c.is_alphabetic() {
        stacks[i].push(c);
      }
      crate_pos += SPACE_BETWEEN_CRATES;
    }
  }

  for instruction in instructions {
    let mut number_to_move = instruction[0];
    let from_num = instruction[1] - 1;
    let to_num = instruction[2] - 1;
    
    loop {
      if number_to_move == 0 {
        break;
      }

      let crate_to_move = stacks[from_num].pop().unwrap();
      stacks[to_num].push(crate_to_move);

      number_to_move -= 1;
    }
  }

  let mut answer = "".to_string();

  for mut stack in stacks {
    let top_crate = stack.pop().unwrap();

    answer.push(top_crate.clone());
  } 

  println!("part 1 answer: {}", answer);

  Ok(())
}
