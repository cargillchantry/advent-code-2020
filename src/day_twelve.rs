use crate::file_util::read_non_blank_lines;
use std::str::FromStr;
use crate::day_twelve::Direction::{Forward, Backward};
use crate::day_twelve::Heading::{East, West, North, South};

enum Heading { North, East, South, West }
#[derive(PartialEq, Eq)]
enum Direction { Forward, Backward }

#[allow(dead_code)]
pub fn run_day_twelve() {
    let directions = read_non_blank_lines("assets/day_twelve")
        .filter_map(|line|
            Some(
                (line.chars().next().unwrap(), isize::from_str(&line[1..]).ok()?)
            )
        )
        .collect::<Vec<(char, isize)>>();
    let result = solve_part_one(&directions);
    let result2 = solve_part_two(&directions);

    println!("Result part 1 {}", result.0.abs() + result.1.abs());
    println!("Result part 2 {}", result2.0.abs() + result2.1.abs());
}

fn solve_part_one(inst: &[(char, isize)]) -> (isize, isize) {
    inst.iter()
        .fold((East, (0, 0)), |previous, (instruction, number)| {
            match instruction {
                'F' => translate(previous, number, Forward),
                'B' => translate(previous, number, Backward),
                'L' => rotate(previous, number, Forward),
                'R' => rotate(previous, number, Backward),
                'N' => (previous.0, (previous.1.0, previous.1.1 + number)),
                'E' => (previous.0, (previous.1.0 + number, previous.1.1)),
                'S' => (previous.0, (previous.1.0, previous.1.1 - number)),
                'W' => (previous.0, (previous.1.0 - number, previous.1.1)),
                _ => previous
            }
        }).1
}

fn solve_part_two(inst: &[(char, isize)]) -> (isize, isize) {
    inst.iter()
        .fold(((10_isize, 1_isize), (0_isize, 0_isize)), |(way_point, ship), (instruction, number)| {
            match instruction {
                'F' => (way_point, (ship.0 + number * way_point.0, ship.1 + number * way_point.1)),
                'B' => (way_point, (ship.0 - number * way_point.0, ship.1 - number * way_point.1)),
                'L' => (rotate_around(way_point, number, Forward), ship),
                'R' => (rotate_around(way_point, number, Backward), ship),
                'N' => ((way_point.0, way_point.1 + number), ship),
                'E' => ((way_point.0 + number, way_point.1), ship),
                'S' => ((way_point.0, way_point.1 - number), ship),
                'W' => ((way_point.0 - number, way_point.1), ship),
                _ => (way_point, ship)
            }
        }).1
}

fn rotate_around(way_point: (isize, isize), amount: &isize, direction: Direction) -> (isize, isize) {
    let quarter_turns = (amount/90).rem_euclid(4);

    match quarter_turns {
        1 => if direction == Forward { (-way_point.1, way_point.0) } else { (way_point.1, -way_point.0) },
        2 => (-way_point.0, -way_point.1),
        3 => if direction == Forward { (way_point.1, -way_point.0) } else { (-way_point.1, way_point.0) },
        _ => way_point
    }
}

fn rotate(position: (Heading, (isize, isize)), amount: &isize, direction: Direction) -> (Heading, (isize, isize)) {
    let current_heading = position.0;
    let mut degrees = match current_heading {
        East => 0,
        North => 1,
        West => 2,
        South => 3
    };
    degrees += if direction == Forward { 1 } else { -1 } * amount / 90;
    let next_heading = match degrees.rem_euclid(4) {
        0 => East,
        1 => North,
        2 => West,
        _ => South
    };
    (next_heading, position.1)
}

fn translate(position: (Heading, (isize, isize)), amount: &isize, direction: Direction) -> (Heading, (isize, isize)) {
    let translate = if direction == Forward { 1 } else { -1 } * amount;
    let heading = position.0;
    match heading {
        East => (heading, (position.1.0 + translate, position.1.1)),
        West => (heading, (position.1.0 - translate, position.1.1)),
        North => (heading, (position.1.0, position.1.1 + translate)),
        _ => (heading, (position.1.0, position.1.1 - translate))
    }
}

#[cfg(test)]
mod tests {
    use crate::day_twelve::*;

    #[test]
    fn should_get_final_position() {
        let data = vec!(
            ('F', 10),
            ('N', 3),
            ('F', 7),
            ('R', 90),
            ('F', 11)
        );
        let result = solve_part_one(&data);
        assert_eq!(result.0, 17);
        assert_eq!(result.1, -8);
    }

    #[test]
    fn should_get_final_position_part_two() {
        let data = vec!(
            ('F', 10),
            ('N', 3),
            ('F', 7),
            ('R', 90),
            ('F', 11)
        );
        let result = solve_part_two(&data);
        assert_eq!(result.0, 214);
        assert_eq!(result.1, -72);
    }

    #[test]
    fn should_rotate_around_ship() {
        assert_eq!(
            rotate_around((1, 2), &90, Forward),
            (-2, 1)
        );
        assert_eq!(
            rotate_around((1, 2), &180, Forward),
            (-1, -2)
        );
        assert_eq!(
            rotate_around((1, 2), &270, Forward),
            (2, -1)
        );
        assert_eq!(
            rotate_around((-1, 2), &90, Forward),
            (-2, -1)
        );
        assert_eq!(
            rotate_around((-1, 2), &180, Forward),
            (1, -2)
        );
        assert_eq!(
            rotate_around((-1, 2), &270, Forward),
            (2, 1)
        );
    }
}
