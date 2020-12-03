use crate::file_util::read_lines;

const BOARD_SIZE: usize = 31;

struct Position {
    horizontal: usize,
    count: usize
}

fn calculate_collisions<'a>(
    board_size: usize,
    horizontal_skip: usize,
    vertical_skip: usize,
    iterator: impl Iterator<Item = &'a String>
) -> Position {
    iterator.map(|line| line
        .chars()
        .enumerate()
        .filter_map(|(index, block)| match block {
            '.' => None,
            _ => Some(index)
        })
        .collect::<Vec<usize>>()
    )
        .enumerate()
        .fold(
            Position { horizontal: 0, count: 0 },
            |position, (index, tree_positions)| {
                if index % vertical_skip != 0 {
                    return position
                }

                Position {
                    horizontal: (position.horizontal + horizontal_skip) % board_size,
                    count: position.count + if tree_positions.contains(&position.horizontal) {
                        1
                    } else {
                        0
                    }
                }
            }
        )
}

#[allow(dead_code)]
pub fn run_day_three() {
    let lines = read_lines("assets/day_three").collect::<Vec<String>>();
    let result = calculate_collisions(
        BOARD_SIZE,
        3,
        1,
        lines.iter()
    );
    println!("Result Task 1 {}", result.count);

    let second_result: usize = [[1, 1], [1, 5], [1, 7], [2, 1]]
        .iter()
        .map(|slope|
            calculate_collisions(
                BOARD_SIZE,
                slope[1],
                slope[0],
                lines.iter()
            ).count
        )
        .product();

    println!("Result Task 2 {}", second_result * result.count)
}

#[cfg(test)]
mod tests {
    use crate::day_three::*;

    #[test]
    fn should_calculate_collisions_with_horizontal_skip() {
        let result = calculate_collisions(11, 3, 1, vec!(
            &String::from("..##......."),
            &String::from("#...#...#.."),
            &String::from(".#....#..#."),
            &String::from("..#.#...#.#"),
            &String::from(".#...##..#."),
            &String::from("..#.##....."),
            &String::from(".#.#.#....#"),
            &String::from(".#........#"),
            &String::from("#.##...#..."),
            &String::from("#...##....#"),
            &String::from(".#..#...#.#")
        ).into_iter());
        assert_eq!(result.count, 7)
    }

    #[test]
    fn should_calculate_collisions_with_vertical_skip() {
        let result = calculate_collisions(11, 1, 2, vec!(
            &String::from("..##......."),
            &String::from("....#...#.."),
            &String::from(".#....#..#."),
            &String::from("..#........"),
            &String::from("..#........")
        ).into_iter());
        assert_eq!(result.count, 2)
    }
}
