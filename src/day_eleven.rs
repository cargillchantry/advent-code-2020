use crate::file_util::read_non_blank_lines;
use crate::day_eleven::Seat::{TAKEN, EMPTY, FLOOR};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Seat {
    TAKEN, EMPTY, FLOOR
}

trait SafeGet<'a, T> {
    fn safe_get(self, index: isize) -> Option<&'a T>;
}

impl <'a, T> SafeGet<'a, T> for &'a [T] {
    fn safe_get(self, index: isize) -> Option<&'a T> {
        if index < 0 {
            None
        } else {
            self.get(index as usize)
        }
    }
}

#[allow(dead_code)]
pub fn run_day_eleven() {
    let mut rows = read_non_blank_lines("assets/day_eleven");
    let first_row = rows.by_ref().next().unwrap();
    let row_width = first_row.len();
    let mut seats = first_row.chars()
        .chain(rows.flat_map(|line| line.chars().collect::<Vec<char>>().into_iter()))
        .map(|seat| match seat {
            'L' => EMPTY,
            '#' => TAKEN,
            _ => FLOOR
        })
        .collect::<Vec<Seat>>();
    let mut seats_part_2 = seats.clone();

    let mut revisions = 0;
    while revise_seats(row_width, 3, &mut seats, get_adjacent_seats) {
        revisions += 1;
    }
    println!(
        "Part 1 seat revisions {} after {} revisions",
         seats.iter().filter(|x| **x == TAKEN).count(),
         revisions
    );

    revisions = 0;
    while revise_seats(row_width, 4, &mut seats_part_2, get_non_floor_adjacent_seats) {
        revisions += 1;
    }
    println!(
        "Part 2 seat revisions {} after {} revisions",
        seats_part_2.iter().filter(|x| **x == TAKEN).count(),
        revisions
    )
}

fn revise_seats(
    row_width: usize,
    max_taken: usize,
    seats: &mut[Seat],
    adjacent_seat_resolver: impl Fn(usize, usize, &[Seat]) -> [Option<&Seat>; 8]
) -> bool {
    let mut changes = Vec::new();
    for index in 0..seats.len() {
        if seats[index] == EMPTY
            && !is_any_taken(&adjacent_seat_resolver(index, row_width, &seats)){
            changes.push((index, TAKEN));
        } else if seats[index] == TAKEN
            && number_taken(&adjacent_seat_resolver(index, row_width, &seats)) > max_taken {
            changes.push((index, EMPTY));
        }
    }

    let changed = !changes.is_empty();
    for (index, change) in changes.into_iter() {
        seats[index] = change;
    }
    changed
}

fn number_taken(seats: &[Option<&Seat>]) -> usize {
    seats.iter().filter(|x|
        if let Some(seat) = x {
            **seat == TAKEN
        } else {
            false
        }
    ).count()
}

fn is_any_taken(seats: &[Option<&Seat>]) -> bool {
    seats
        .iter()
        .any(|x|
            if let Some(seat) = x {
                **seat == TAKEN
            } else {
                false
            }
        )
}

fn find_non_floor_seat(mut index: isize, row_width: isize, seats: &[Seat], direction: impl Fn(isize, isize) -> Option<isize>) -> Option<&Seat> {
    while let Some(seat_location) = direction(index, row_width) {
        let seat = seats.safe_get(seat_location);
        if match seat { Some(x) => *x != FLOOR, None => true } {
            return seat
        }
        index = seat_location;
    }
    None
}

fn on_left(index: isize, row_width: isize) -> bool {
    index % row_width == 0
}

fn on_right(index: isize, row_width: isize) -> bool {
    (index + 1) % row_width == 0
}

fn get_top_left_index(i: isize, row_width: isize) -> Option<isize> {
    if on_left(i, row_width) { None } else { Some(i - row_width - 1) }
}

fn get_top_right_index(i: isize, row_width: isize) -> Option<isize> {
    if on_right(i, row_width) { None } else { Some(i - row_width + 1) }
}

fn get_top_middle_index(i: isize, row_width: isize) -> Option<isize> {
    Some(i - row_width)
}

fn get_left_index(i: isize, row_width: isize) -> Option<isize> {
    if on_left(i, row_width) { None } else { Some(i - 1) }
}

fn get_right_index(i: isize, row_width: isize) -> Option<isize> {
    if on_right(i, row_width) { None } else { Some(i + 1) }
}

fn get_bottom_left_index(i: isize, row_width: isize) -> Option<isize> {
    if on_left(i, row_width) { None } else { Some(i + row_width - 1) }
}

fn get_bottom_middle_index(i: isize, row_width: isize) -> Option<isize> {
    Some(i + row_width)
}

fn get_bottom_right_index(i: isize, row_width: isize) -> Option<isize> {
    if on_right(i, row_width) { None } else { Some(i + row_width + 1) }
}

fn get_non_floor_adjacent_seats(index: usize, row_width: usize, seats: &[Seat]) -> [Option<&Seat>; 8] {
    let index = index as isize;
    let row_width = row_width as isize;

    return [
        find_non_floor_seat(index, row_width, &seats, get_top_left_index),
        find_non_floor_seat(index, row_width, &seats, get_top_middle_index),
        find_non_floor_seat(index, row_width, &seats, get_top_right_index),
        find_non_floor_seat(index, row_width, &seats, get_left_index),
        find_non_floor_seat(index, row_width, &seats, get_right_index),
        find_non_floor_seat(index, row_width, &seats, get_bottom_left_index),
        find_non_floor_seat(index, row_width, &seats, get_bottom_middle_index),
        find_non_floor_seat(index, row_width, &seats, get_bottom_right_index)
    ];
}

fn retrieve(maybe_index: Option<isize>, seats: &[Seat]) -> Option<&Seat> {
    maybe_index.and_then(|i| seats.safe_get(i))
}

fn get_adjacent_seats(index: usize, row_width: usize, seats: &[Seat]) -> [Option<&Seat>; 8] {
    let index = index as isize;
    let row_width = row_width as isize;
    return [
        retrieve(get_top_left_index(index, row_width), &seats),
        retrieve(get_top_middle_index(index, row_width), &seats),
        retrieve(get_top_right_index(index, row_width), &seats),
        retrieve(get_left_index(index, row_width), &seats),
        retrieve(get_right_index(index, row_width), &seats),
        retrieve(get_bottom_left_index(index, row_width), &seats),
        retrieve(get_bottom_middle_index(index, row_width), &seats),
        retrieve(get_bottom_right_index(index, row_width), &seats)
    ];
}

#[cfg(test)]
mod tests {
    use crate::day_eleven::*;

    #[test]
    fn should_get_seat_positions() {
        let under_test = [
            TAKEN, EMPTY, TAKEN, EMPTY,
            TAKEN, FLOOR, TAKEN, FLOOR,
            TAKEN, EMPTY, TAKEN, EMPTY,
            EMPTY, EMPTY, FLOOR, EMPTY,
        ];
        assert_eq!(
            get_adjacent_seats(9, 4, &under_test),
            [
                Some(&under_test[4]),
                Some(&under_test[5]),
                Some(&under_test[6]),
                Some(&under_test[8]),
                Some(&under_test[10]),
                Some(&under_test[12]),
                Some(&under_test[13]),
                Some(&under_test[14])
            ]
        );
        assert_eq!(
            get_adjacent_seats(4, 4, &under_test),
            [
                None,
                Some(&under_test[0]),
                Some(&under_test[1]),
                None,
                Some(&under_test[5]),
                None,
                Some(&under_test[8]),
                Some(&under_test[9])
            ]
        );
        assert_eq!(
            get_adjacent_seats(7, 4, &under_test),
            [
                Some(&under_test[2]),
                Some(&under_test[3]),
                None,
                Some(&under_test[6]),
                None,
                Some(&under_test[10]),
                Some(&under_test[11]),
                None
            ]
        );
    }

    #[test]
    fn should_get_seat_positions_part_2() {
        let under_test = [
            TAKEN, EMPTY, TAKEN, EMPTY,
            TAKEN, FLOOR, TAKEN, FLOOR,
            TAKEN, EMPTY, TAKEN, EMPTY,
            EMPTY, EMPTY, FLOOR, EMPTY,
        ];
        assert_eq!(
            get_non_floor_adjacent_seats(9, 4, &under_test),
            [
                Some(&under_test[4]),
                Some(&under_test[1]),
                Some(&under_test[6]),
                Some(&under_test[8]),
                Some(&under_test[10]),
                Some(&under_test[12]),
                Some(&under_test[13]),
                None
            ]
        );
    }
}
