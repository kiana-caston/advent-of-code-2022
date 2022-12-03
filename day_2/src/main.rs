use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufReader};

const ROCK_SCORE: u32 = 1;
const PAPER_SCORE: u32 = 2;
const SCISSORS_SCORE: u32 = 3;

const WIN: u32 = 6;
const DRAW: u32 = 3;

const ROCK: char = 'A';
const PAPER: char = 'B';
const SCISSORS: char = 'C';

fn main() {
  part_1().ok();
  part_2().ok();
}

fn part_2() -> io::Result<()>{
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);

  let mut score = 0;
  
  for line in reader.lines() {
    let l = line.unwrap();
    let mut chars = l.chars();

    let opp = chars.next().unwrap();
    let ending = chars.last().unwrap();

    match ending {
      'X' => score += score_loss(opp),
      'Y' => score += score_draw(opp),
      'Z' => score += score_win(opp),
      _ => println!("invalid: {}", ending)
    }
  }

  println!("part 2 answer: {}", score);

  Ok(())
}

fn score_loss(opp: char) -> u32 {
  match opp {
    ROCK => SCISSORS_SCORE, 
    PAPER => ROCK_SCORE,
    SCISSORS => PAPER_SCORE,
    _ => 0,
  }
}

fn score_draw(opp: char) -> u32 {
  match opp {
    ROCK => DRAW + ROCK_SCORE, 
    PAPER => DRAW + PAPER_SCORE,
    SCISSORS => DRAW + SCISSORS_SCORE,
    _ => 0,
  }
}

fn score_win(opp: char) -> u32 {
  match opp {
    ROCK => WIN + PAPER_SCORE, 
    PAPER => WIN + SCISSORS_SCORE,
    SCISSORS => WIN + ROCK_SCORE,
    _ => 0,
  }
}

fn part_1() -> io::Result<()>{
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);

  let mut score = 0;
  
  for line in reader.lines() {
    let l = line.unwrap();
    let mut chars = l.chars();

    let opp = chars.next().unwrap();
    let me = chars.last().unwrap();

    match me {
      'X' => score += score_rock(opp),
      'Y' => score += score_paper(opp),
      'Z' => score += score_scissors(opp),
      _ => println!("invalid")
    }
  }

  println!("part 1 answer: {}", score);

  Ok(())
}

fn score_rock(opp: char) -> u32 {
  match opp {
    ROCK => ROCK_SCORE + 3,
    PAPER => ROCK_SCORE,
    SCISSORS => ROCK_SCORE + 6,
    _ => 0,
  }
}

fn score_paper(opp: char) -> u32 {
  match opp {
    ROCK => PAPER_SCORE + WIN,
    PAPER => PAPER_SCORE + DRAW,
    SCISSORS => PAPER_SCORE,
    _ => 0,
  }
}

fn score_scissors(opp: char) -> u32 {
  match opp {
    ROCK => SCISSORS_SCORE,
    PAPER => SCISSORS_SCORE + WIN,
    SCISSORS => SCISSORS_SCORE + DRAW,
    _ => 0,
  }
}
