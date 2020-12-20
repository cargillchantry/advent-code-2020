use crate::file_util::read_lines;

type Block = [u16; 5];

#[allow(dead_code)]
pub fn run_day_twenty() {
    let mut iter = read_lines("assets/day_twenty");
    let signatures = read_image_signatures(&mut iter);
    let mut corners = Vec::new();
    for (idx, signature) in signatures.iter().enumerate() {
        let mut count = 0;
        for side in signature.iter().skip(1) {
            let reversed = (*side).reverse_bits() >> 6;
            let matches = signatures.iter().enumerate().any(|(other_idx, x)|
                other_idx != idx &&
                x.iter().skip(1).any(|y| y == side || *y == reversed)
            );
            if matches {
                count += 1;
            }
        }
        if count < 3 {
            corners.push(signature[0]);
        }
    }
    println!("{:?}", corners.iter().map(|x| *x as usize).product::<usize>());
}

fn read_image_signatures(iter: &mut impl Iterator<Item = String>) -> Vec<Block> {
    let mut signatures = Vec::new();
    while let Some(block) = read_image_signature(iter) {
        signatures.push(block);
    }
    signatures
}

fn read_image_signature(iter: &mut impl Iterator<Item = String>) -> Option<Block> {
    let name = iter.next()?
        .chars()
        .skip(5)
        .take_while(|it| *it != ':')
        .collect::<String>()
        .parse::<u16>()
        .ok()?;
    let mut block_iter = iter
        .take_while(|it| !it.trim().is_empty());
    let first_line = block_iter.next()?;
    let top = read_line_as_signature_part(&first_line);
    let mut left = if first_line.chars().next()? == '#' { 1 } else { 0 };
    let mut right = if first_line.chars().last()? == '#' { 1 } else { 0 };
    let mut digit = 1;
    let mut last = first_line;
    for line in block_iter {
        digit <<= 1;
        left += if line.chars().next()? == '#' { digit } else { 0 };
        right += if line.chars().last()? == '#' { digit } else { 0 };
        last = line;
    }
    let bottom = read_line_as_signature_part(&last);
    Some([name, top, left, right, bottom])
}

fn read_line_as_signature_part(line: &str) -> u16 {
    let mut part = 0;
    let mut digit = 1_u16;
    for c in line.chars(){
        if c == '#' {
            part += digit;
        }
        digit <<= 1
    }
    part
}
