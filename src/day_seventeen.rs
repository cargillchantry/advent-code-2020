use crate::day_seventeen::Block::{Active, Inactive};
use crate::file_util::read_non_blank_lines;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone)]
enum Block {
    Active, Inactive
}

type Coord = (isize, isize, isize);

struct Board {
    blocks: HashMap<Coord, Block>
}

impl Board {
    fn new() -> Self {
        Board {
            blocks: HashMap::new()
        }
    }

    fn neighbours(&self, coord: Coord) -> Vec<(Coord, &Block)> {
       vec!(
            // above
            (coord.0 - 1, coord.1 + 1, coord.2 + 1),
            (coord.0, coord.1 + 1, coord.2 + 1),
            (coord.0 + 1, coord.1 + 1, coord.2 + 1),
            (coord.0 - 1, coord.1, coord.2 + 1),
            (coord.0, coord.1, coord.2 + 1),
            (coord.0 + 1, coord.1, coord.2 + 1),
            (coord.0 - 1, coord.1 - 1, coord.2 + 1),
            (coord.0, coord.1 - 1, coord.2 + 1),
            (coord.0 + 1, coord.1 - 1, coord.2 + 1),
            // same
            (coord.0 - 1, coord.1 + 1, coord.2),
            (coord.0, coord.1 + 1, coord.2),
            (coord.0 + 1, coord.1 + 1, coord.2),
            (coord.0 - 1, coord.1, coord.2),
            (coord.0 + 1, coord.1, coord.2),
            (coord.0 - 1, coord.1 - 1, coord.2),
            (coord.0, coord.1 - 1, coord.2),
            (coord.0 + 1, coord.1 - 1, coord.2),
            // below
            (coord.0 - 1, coord.1 + 1, coord.2 - 1),
            (coord.0, coord.1 + 1, coord.2 - 1),
            (coord.0 + 1, coord.1 + 1, coord.2 - 1),
            (coord.0 - 1, coord.1, coord.2 - 1),
            (coord.0, coord.1, coord.2 - 1),
            (coord.0 + 1, coord.1, coord.2 - 1),
            (coord.0 - 1, coord.1 - 1, coord.2 - 1),
            (coord.0, coord.1 - 1, coord.2 - 1),
            (coord.0 + 1, coord.1 - 1, coord.2 - 1)
        )
            .into_iter()
            .map(|x| (x, self.get_block(&x)))
            .collect()
    }

    fn set_active(&mut self, coord: (isize, isize, isize)) {
        self.blocks.insert(coord, Active);
    }

    fn set_inactive(&mut self, coord: &(isize, isize, isize)) {
        self.blocks.remove(&coord);
    }

    fn get_block(&self, coord: &(isize, isize, isize)) -> &Block {
        self.blocks.get(&coord).unwrap_or(&Inactive)
    }

    fn get_active_blocks(&self) -> Vec<&(isize, isize, isize)> {
        self.blocks.keys().collect()
    }
}

fn iterate_board(board: &mut Board) {
    let mut changes = Vec::new();
    let active_blocks = board.get_active_blocks();
    active_blocks.iter().for_each(|block| {
        let neighbours = board.neighbours(**block);
        let active_neighbours = neighbours.iter()
            .filter(|x| *x.1 == Active)
            .count();
        if active_neighbours < 2 || active_neighbours > 3 {
            changes.push((**block, Inactive));
        }
        neighbours.iter()
            .filter(|x| *x.1 == Inactive && board.neighbours(x.0)
                .iter()
                .filter(|y| *y.1 == Active)
                .count() == 3
            )
            .for_each(|x| changes.push((x.0, Active)));
    });
    changes.iter().for_each(|it|
        if it.1 == Active {
            board.set_active(it.0);
        } else {
            board.set_inactive(&it.0);
        }
    )
}

#[allow(dead_code)]
pub fn run_day_seventeen() {
    let mut board = Board::new();
    read_non_blank_lines("assets/day_seventeen")
        .enumerate()
        .for_each(|(x, line)|
            line.chars().enumerate().for_each(|(y, c)| {
                if c == '#' {
                    board.set_active((x as isize, y as isize, 0));
                }
            })
        );
    for _ in 0..6 {
        iterate_board(&mut board);
    }
    let result_part_one = board.get_active_blocks().iter().count();
    println!("Result part one {}", result_part_one);
}
