use crate::file_util::read_lines;

const BOARD_SIZE: usize = 31;

struct Position {
    horizontal: usize,
    count: usize
}

fn calculate_collisions(board_size: usize, iterator: impl Iterator<Item = String>) -> Position {
    iterator.map(|line| line
        .chars()
        .enumerate()
        .filter_map(|(index, block)| match block {
            '.' => None,
            _ => Some(index)
        })
        .collect::<Vec<usize>>()
    )
        .fold(Position { horizontal: 0, count: 0 }, |position, tree_positions| {
            Position {
                horizontal: (position.horizontal + 3) % board_size,
                count: position.count + if tree_positions.contains(&position.horizontal) {
                    1
                } else {
                    0
                }
            }
        })
}

#[allow(dead_code)]
pub fn run_day_three() {
    let result = calculate_collisions(BOARD_SIZE, read_lines("assets/day_three"));
    println!("Result {}", result.count)
}

#[cfg(test)]
mod tests {
    use crate::day_three::*;

    #[test]
    fn should_calculate_collisions() {
        let result = calculate_collisions(11, vec!(
            String::from("..##......."),
            String::from("#...#...#.."),
            String::from(".#....#..#."),
            String::from("..#.#...#.#"),
            String::from(".#...##..#."),
            String::from("..#.##....."),
            String::from(".#.#.#....#"),
            String::from(".#........#"),
            String::from("#.##...#..."),
            String::from("#...##....#"),
            String::from(".#..#...#.#")
        ).into_iter());
        assert_eq!(result.count, 7)
    }
}
