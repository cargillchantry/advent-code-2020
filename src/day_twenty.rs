use crate::file_util::read_lines;

#[derive(Debug)]
struct Block {
    id: u16,
    rows: [u16; 10],
    border_clockwise: [u16; 4],
    border_anti_clockwise: [u16; 4],
    matching_ids: [Option<(u16, bool)>; 4]
}

#[derive(Eq, PartialEq)]
enum Flip {
    FlipX,
    FlipY,
    FlipXY,
    Identity
}

impl Block {
    fn new(id: u16, rows: [u16; 10]) -> Self {
        let mut left = 0;
        let mut right = 0;
        let mut multiplier = 1_u16;
        // clock-wise bit arrangement
        for i in 0..10 {
            left += if rows[10 - i - 1] & 1 == 1 { multiplier } else { 0 };
            right += if rows[i] & 512 == 512 { multiplier } else { 0 };
            multiplier <<= 1;
        }
        Block {
            id,
            rows,
            border_clockwise: [rows[0], right, rows[9].reverse_bits() >> 6, left],
            border_anti_clockwise: [
                rows[0].reverse_bits() >> 6,
                right.reverse_bits() >> 6,
                rows[9],
                left.reverse_bits() >> 6
            ],
            matching_ids: [None; 4]
        }
    }
    fn transformed(&self, flip: Flip) -> [u16; 10] {
        let mut result = [0; 10];
        match flip {
            Flip::Identity => self.rows.iter()
                .enumerate()
                .for_each(|(i, r)| result[i] = *r),
            Flip::FlipX => {
                self.rows.iter()
                    .enumerate()
                    .for_each(|(i, r)| result[i] = r.reverse_bits() >> 6)
            },
            Flip::FlipY => {
                let mut i = 1_u16;
                for _ in 0..10 {
                    for y in 0..10 {
                        result[y] += self.rows[10 - y - 1] & i
                    }
                    i <<= 1;
                }
            },
            Flip::FlipXY => {
                for y in 0..10 {
                    let mut i = 1_u16;
                    let reversed = self.rows[10 - y - 1].reverse_bits() >> 6;
                    for _ in 0..10 {
                        result[y] += reversed  & i;
                        i <<= 1;
                    }
                }
            }
        }
        result
    }
    fn add_matching_sides(&mut self, block: &mut Block) -> &mut Self {
        let iter = self.border_clockwise.iter()
            .zip(self.border_anti_clockwise.iter()).enumerate();
        for (id, (side, flipped_side)) in iter {
            for (other_id, other_side) in block.border_clockwise.iter().enumerate() {
                if side == other_side || flipped_side == other_side {
                    self.matching_ids[id] = Some((block.id, flipped_side == other_side));
                    block.matching_ids[other_id] = Some((self.id, flipped_side == other_side));
                    return self;
                }
            }
        }
        self
    }
    fn missing_sides(&self) -> usize {
        self.matching_ids.iter().filter(|x| x.is_none()).count()
    }
}

trait FlipSide {
    fn flip_side(self) -> Self;
}

impl FlipSide for u16 {
    fn flip_side(self) -> Self { self.reverse_bits() >> 6 }
}

#[allow(dead_code)]
pub fn run_day_twenty() {
    let mut iter = read_lines("assets/day_twenty");
    let mut signatures = read_image_signatures(&mut iter);
    populate_matches(&mut signatures);

    let corners: Vec<&Block> = signatures.iter()
        .filter(|x| x.missing_sides() == 2)
        .collect();
    println!(
        "Part 1 {:?}",
        corners.iter().map(|it| it.id as usize).product::<usize>()
    );

}

fn print(x: &[u16; 10], y: &[u16; 10]) {
    for i in 0..10 {
        println!("{:010b} {:010b}", x[i], y[i]);
    }
    println!();
}

fn populate_matches(signatures: &mut Vec<Block>) {
    for i in 1..signatures.len() {
        let split = signatures.split_at_mut(i);
        let left = split.0;
        let right = split.1;
        if let Some(signature )= left.last_mut() {
            for other_signature in right.iter_mut(){
                signature.add_matching_sides(other_signature);
            }
        }
    }
}

fn read_image_signatures(iter: &mut impl Iterator<Item = String>) -> Vec<Block> {
    let mut signatures = Vec::new();
    while let Some(block) = read_image_signature(iter) {
        signatures.push(block);
    }
    signatures
}

fn read_image_signature(iter: &mut impl Iterator<Item = String>) -> Option<Block> {
    let id = iter.next()?
        .chars()
        .skip(5)
        .take_while(|it| *it != ':')
        .collect::<String>()
        .parse::<u16>()
        .ok()?;
    let block_iter = iter
        .take_while(|it| !it.trim().is_empty());
    let mut rows = [0; 10];
    for (i, block) in block_iter.enumerate() {
        let mut digit = 1_u16;
        for c in block.chars(){
            if c == '#' {
                rows[i] += digit;
            }
            digit <<= 1
        }
    }

    Some(Block::new(id, rows))
}
