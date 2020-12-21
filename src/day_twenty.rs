use crate::file_util::read_lines;

#[derive(Debug)]
struct Block {
    id: u16,
    rows: [u16; 10],
    left: u16,
    right: u16,
    matching_ids: [u16; 4]
}

impl Block {
    fn borders(&self) -> [u16; 4] { [self.rows[0], self.left, self.right, self.rows[9]] }
    fn add_matching_sides(&mut self, block: &mut Block) -> &mut Self {
        if self.id == block.id {
            return self;
        }
        for (id, side) in self.borders().iter().enumerate() {
            let flipped_side = side.flip_side();
            for (other_id, other_side) in block.borders().iter().enumerate() {
                if side == other_side || flipped_side == *other_side {
                    self.matching_ids[id] = block.id;
                    block.matching_ids[other_id] = self.id;
                    return self;
                }
            }
        }
        self
    }
    fn missing_sides(&self) -> usize {
        self.matching_ids.iter().filter(|x| **x != 0).count()
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

    println!(
        "Part 1 {:?}",
        signatures.iter()
            .filter(|x| x.missing_sides() == 2)
            .map(|it| it.id as usize)
            .product::<usize>()
    );
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
    let mut left = 0;
    let mut right = 0;
    let mut outer_digit = 1_u16;
    for (i, block) in block_iter.enumerate() {
        left += if block.chars().next()? == '#' { outer_digit } else { 0 };
        right += if block.chars().last()? == '#' { outer_digit } else { 0 };
        let mut digit = 1_u16;
        for c in block.chars(){
            if c == '#' {
                rows[i] += digit;
            }
            digit <<= 1
        }
        outer_digit <<= 1;
    }

    Some(
        Block {
            id,
            left,
            right,
            rows,
            matching_ids: [0; 4]
        }
    )
}
