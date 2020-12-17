use crate::day_seventeen::Block::{Active, Inactive};
use crate::file_util::read_non_blank_lines;
use std::collections::HashMap;
use std::iter::repeat;
use itertools::Itertools;

#[derive(Eq, PartialEq, Clone)]
enum Block {
    Active, Inactive
}

struct Board {
    dimensions: u32,
    blocks: HashMap<Vec<isize>, Block>
}

impl Board {
    fn new(dimensions: u32) -> Self {
        Board {
            dimensions,
            blocks: HashMap::new()
        }
    }

    fn neighbours(&self, coord: &Vec<isize>) -> Vec<(Vec<isize>, &Block)> {
        let mut iterators = Vec::new();
        for i in 0..self.dimensions {
            iterators.push(
                repeat(0_isize).take(3_usize.pow(i))
                    .chain(repeat(1).take(3_usize.pow(i)))
                    .chain(repeat(-1).take(3_usize.pow(i)))
                    .cycle()
            );
        }
        (0 .. 3_usize.pow(self.dimensions) * self.dimensions as usize)
            .filter_map(|i| iterators[i % (self.dimensions as usize)].next())
            .batching(|iter| {
                let mut indexes = Vec::new();
                for i in 0..self.dimensions {
                    indexes.push(coord[i as usize] + iter.next()?)
                }
                Some(indexes)
            })
            .skip(1)
            .map(|x| {
                let block = self.get_block(&x);
                (x, block)
            })
            .collect()
    }

    fn set_active(&mut self, coord: &Vec<isize>) {
        self.blocks.insert(coord.clone(), Active);
    }

    fn set_inactive(&mut self, coord: &Vec<isize>) {
        self.blocks.remove(coord);
    }

    fn get_block(&self, coord: &Vec<isize>) -> &Block {
        self.blocks.get(coord).unwrap_or(&Inactive)
    }

    fn get_active_blocks(&self) -> Vec<&Vec<isize>> {
        self.blocks.keys().collect()
    }
}

fn iterate_board(board: &mut Board) {
    let mut changes = Vec::new();
    let active_blocks = board.get_active_blocks();
    active_blocks.iter().for_each(|block| {
        let neighbours = board.neighbours(block);
        let active_neighbours = neighbours.iter()
            .filter(|x| *x.1 == Active)
            .count();
        if active_neighbours < 2 || active_neighbours > 3 {
            changes.push(((*block).clone(), Inactive));
        }
        neighbours.iter()
            .filter(|x| *x.1 == Inactive && board.neighbours(&x.0)
                .iter()
                .filter(|y| *y.1 == Active)
                .count() == 3
            )
            .for_each(|x| changes.push((x.0.clone(), Active)));
    });
    changes.into_iter().for_each(|it|
        if it.1 == Active {
            board.set_active(&it.0);
        } else {
            board.set_inactive(&it.0);
        }
    )
}

#[allow(dead_code)]
pub fn run_day_seventeen() {
    let mut board = Board::new(4);
    read_non_blank_lines("assets/day_seventeen")
        .enumerate()
        .for_each(|(x, line)|
            line.chars().enumerate().for_each(|(y, c)| {
                if c == '#' {
                    board.set_active(&vec!(x as isize, y as isize, 0, 0));
                }
            })
        );
    for _ in 0..6 {
        iterate_board(&mut board);
    }
    let result_part_two = board.get_active_blocks().iter().count();
    println!("Result part two {}", result_part_two);
}
