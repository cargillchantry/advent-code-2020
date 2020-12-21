use crate::file_util::read_lines;

#[derive(Debug)]
struct Block {
    id: u16,
    rows: [u16; 10],
    left: u16,
    right: u16
}

impl Block {
    fn borders(&self) -> [u16; 4] { [self.rows[0], self.left, self.right, self.rows[9]] }
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
    let signatures = read_image_signatures(&mut iter);
    let mut corners = Vec::new();
    for (idx, signature) in signatures.iter().enumerate() {
        let mut count = 0;
        for side in signature.borders().iter() {
            let reversed = side.flip_side();
            let matches = signatures.iter().enumerate().any(|(other_idx, x)|
                other_idx != idx &&
                x.borders().iter().any(|y| y == side || *y == reversed)
            );
            if matches {
                count += 1;
            }
        }
        if count < 3 {
            corners.push(signature.id);
        }
    }
    println!("Part 1 {:?}", corners.iter().map(|x| *x as usize).product::<usize>());
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
            rows
        }
    )
}
