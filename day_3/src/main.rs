use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufReader};

const ITEMS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
  part_1().ok();
  part_2().ok();
}

fn part_2() -> io::Result<()>{
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);

  let mut elves = 0;
  let mut priorities = 0;
  let mut groups: Vec<Vec<String>> = Vec::new();
  let mut v: Vec<String> = Vec::new();


  for line in reader.lines() {
    let elf = line.unwrap();
    v.push(elf);
    elves += 1;

    if elves == 3 {
      groups.push(v.clone());
      v.clear();
      elves = 0;
    }
  }

  for group in groups {
    let elf1: Vec<char> = group[0].chars().collect();
    let elf2: Vec<char> = group[1].chars().collect();
    let elf3: Vec<char> = group[2].chars().collect();

    let (shortest, longer1, longer2) = if elf1.len() < elf2.len() && elf1.len() < elf3.len() {
      (elf1, elf2, elf3)
    } else if elf2.len() < elf1.len() && elf2.len() < elf3.len() {
      (elf2, elf1, elf3)
    } else {
      (elf3, elf1, elf2)
    };

    let mut i = 0;

    loop {
      let item = shortest[i];

      if longer1.contains(&item) && longer2.contains(&item) {
        let priority = ITEMS.find(item).unwrap() + 1;
        priorities += priority;
        break;
      }

      i+= 1;
    }
  }

  println!("part 2 answer: {}", priorities);

  Ok(())
}

fn part_1() -> io::Result<()>{
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);

  let mut priorities = 0;

  for line in reader.lines() {
    let rucksack = line.unwrap();
    let rucksack_size = rucksack.len();
    let compartment_size = rucksack_size / 2;

    let compartment1: Vec<char> = rucksack.chars().take(compartment_size).collect();
    let compartment2: Vec<char> = rucksack.chars().skip(compartment_size).collect();
    
    let mut i = 0;

    loop {
      let item = compartment2[i];
      if compartment1.contains(&item) {
        let priority = ITEMS.find(item).unwrap() + 1;
        priorities += priority;
        break;
      }

      i += 1;
    }
  }

  println!("part 1 answer: {}", priorities);

  Ok(())
}