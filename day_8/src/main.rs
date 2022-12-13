use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufReader};

fn main() {
  part_1().ok();
  part_2().ok();
}

fn part_2() -> io::Result<()>{
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);

	let mut tree_grid: Vec<Vec<char>> = Vec::new();
	let mut max_scenic_score = 0;

  for line in reader.lines() {
    let row = line.unwrap();
    let tree_row = row.chars().collect();

    tree_grid.push(tree_row);
  }

	let grid_length = tree_grid.len();

	for x in 1..grid_length - 1 {
		for y in 1..grid_length - 1 {
			let tree = tree_grid[x][y];

			let scenic_score = north_score(&tree_grid, tree, x, y) * 
				south_score(&tree_grid, tree, x, y) * 
				east_score(&tree_grid, tree, x, y) * 
				west_score(&tree_grid, tree, x, y);

			if scenic_score > max_scenic_score {
				max_scenic_score = scenic_score;
			}
		}
	}

  println!("part 2 answer: {}", max_scenic_score);

  Ok(())
}

fn part_1() -> io::Result<()>{
  let file = File::open("input.txt")?;
  let reader = BufReader::new(file);

	let mut tree_grid: Vec<Vec<char>> = Vec::new();
	let mut visible_trees = 0;

  for line in reader.lines() {
    let row = line.unwrap();
    let tree_row = row.chars().collect();

    tree_grid.push(tree_row);
  }

	let grid_length = tree_grid.len();

	for x in 0..grid_length {
		for y in 0..grid_length {
			if x == 0 || y == 0 || x == grid_length - 1 || y == grid_length - 1 {
				visible_trees += 1;
			} else {
				let tree = tree_grid[x][y];

				if north_visible(&tree_grid, tree, x, y) ||
					south_visible(&tree_grid, tree, x, y) ||
					east_visible(&tree_grid, tree, x, y) ||
					west_visible(&tree_grid, tree, x, y) {

					visible_trees += 1;
				} else {
				}
			}
		}
	}
  println!("part 1 answer: {}", visible_trees);

  Ok(())
}

fn north_score(tree_grid: &Vec<Vec<char>>, tree: char, x: usize, mut y: usize) -> usize {
	let mut score = 0;

	loop {
		score += 1;
		y -= 1;

		let north_tree = tree_grid[x][y];

		if y == 0 || north_tree >= tree {
			break;
		}
	}

	return score; 
}

fn south_score(tree_grid: &Vec<Vec<char>>, tree: char, x: usize, mut y: usize) -> usize {
	let mut score = 0;

	loop {
		score += 1;
		y += 1;

		let south_tree = tree_grid[x][y];

		if y == tree_grid.len() - 1 || south_tree >= tree {
			break;
		}
	}

	return score; 
}

fn east_score(tree_grid: &Vec<Vec<char>>, tree: char, mut x: usize, y: usize) -> usize {
	let mut score = 0;

	loop {
		score += 1;
		x += 1;

		let east_tree = tree_grid[x][y];

		if x == tree_grid.len() - 1 || east_tree >= tree {
			break;
		}
	}

	return score;
}

fn west_score(tree_grid: &Vec<Vec<char>>, tree: char, mut x: usize, y: usize) -> usize {
	let mut score = 0;

	loop {
		score += 1;
		x -= 1;

		let west_tree = tree_grid[x][y];

		if x == 0 || west_tree >= tree {
			break;
		}
	}

	return score;
}


fn north_visible(tree_grid: &Vec<Vec<char>>, tree: char, x: usize, mut y: usize) -> bool {
	let mut visible = true;

	loop {
		if y == 0 {
			break;
		}

		let north_tree = tree_grid[x][y - 1];

		if north_tree >= tree {
			visible = false;
			break;
		}

		y -= 1;
	}

	return visible; 
}

fn south_visible(tree_grid: &Vec<Vec<char>>, tree: char, x: usize, mut y: usize) -> bool {
	let mut visible = true;

	loop {
		if y ==  tree_grid.len() - 1 {
			break;
		}

		let south_tree = tree_grid[x][y + 1];

		if south_tree >= tree {
			visible = false;
			break;
		}

		y += 1;
	}

	return visible; 
}

fn east_visible(tree_grid: &Vec<Vec<char>>, tree: char, mut x: usize, y: usize) -> bool {
	let mut visible = true;

	loop {
		if x == tree_grid.len() - 1 {
			break;
		}

		let east_tree = tree_grid[x + 1][y];

		if east_tree >= tree {
			visible = false;
			break;
		}

		x += 1;
	}

	return visible; 
}

fn west_visible(tree_grid: &Vec<Vec<char>>, tree: char, mut x: usize, y: usize) -> bool {
	let mut visible = true;

	loop {
		if x == 0 {
			break;
		}

		let west_tree = tree_grid[x - 1][y];

		if west_tree >= tree {
			visible = false;
			break;
		}

		x -= 1;
	}

	return visible; 
}