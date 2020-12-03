use crate::file_util::read_lines;

const BOARD_SIZE: usize = 31;

struct Position {
    horizontal: usize,
    count: usize
}

fn convert_to_collision_vec(iterator: impl Iterator<Item = String>) -> Vec<Vec<usize>> {
    iterator.map(|line| line
        .chars()
        .enumerate()
        .filter_map(|(index, block)| match block {
            '.' => None,
            _ => Some(index)
        })
        .collect()
    ).collect()
}

fn calculate_collisions(
    board_size: usize,
    horizontal_step: usize,
    vertical_step: usize,
    slope: &[Vec<usize>]
) -> Position {
    slope
        .iter()
        .step_by(vertical_step)
        .fold(
            Position { horizontal: 0, count: 0 },
            |position, tree_positions| Position {
                    horizontal: (position.horizontal + horizontal_step) % board_size,
                    count: position.count + if tree_positions.contains(&position.horizontal) {
                        1
                    } else {
                        0
                    }
                }
        )
}

#[allow(dead_code)]
pub fn run_day_three() {
    let lines = convert_to_collision_vec(read_lines("assets/day_three"));
    let result = calculate_collisions(
        BOARD_SIZE,
        3,
        1,
        &lines
    );
    println!("Result Task 1 {}", result.count);

    let second_result: usize = [[1, 1], [1, 5], [1, 7], [2, 1]]
        .iter()
        .map(|slope|
            calculate_collisions(
                BOARD_SIZE,
                slope[1],
                slope[0],
                &lines
            ).count
        )
        .product();

    println!("Result Task 2 {}", second_result * result.count)
}

#[cfg(test)]
mod tests {
    use crate::day_three::*;

    #[test]
    fn should_convert_lines_to_collision_vec() {
        let result = convert_to_collision_vec(
            vec!(
                String::from("..##......."),
                String::from("#...#...#.."),
                String::from(".#....#..#.")
            ).into_iter()
        );
        assert_eq!(
            vec!(
                vec!(2, 3),
                vec!(0, 4, 8),
                vec!(1, 6, 9)
            ),
            result
        )
    }

    #[test]
    fn should_calculate_collisions_with_horizontal_skip() {
        // we use the exact strings provided by advent here
        let slope = convert_to_collision_vec(
            vec!(
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
            ).into_iter()
        );
        let result = calculate_collisions(11, 3, 1, &slope);
        assert_eq!(result.count, 7)
    }

    #[test]
    fn should_calculate_collisions_with_vertical_skip() {
        let result = calculate_collisions(11, 1, 2, &[
            vec!(),
            vec!(0, 1, 2, 3, 4, 5),
            vec!(1),
            vec!(0, 1, 2, 3, 4, 5),
            vec!(2)
        ]);
        assert_eq!(result.count, 2)
    }
}
