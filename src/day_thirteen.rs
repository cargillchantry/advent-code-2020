use crate::file_util::read_non_blank_lines;
use std::str::FromStr;

#[allow(dead_code)]
pub fn run_day_thirteen() {
    let mut lines = read_non_blank_lines("assets/day_thirteen");
    let departure_time = lines.next()
        .and_then(|x| usize::from_str(x.as_str()).ok())
        .unwrap();
    let buses = lines.next().unwrap().split(",")
        .enumerate()
        .filter(|(_, x)| *x != "x")
        .filter_map(|(index, x)| Some((index, usize::from_str(x).ok()?)))
        .collect::<Vec<(usize, usize)>>();
    let earliest_bus = buses.iter()
        .map(|(_, bus)| (bus, bus - (departure_time % bus)))
        .min_by(|bus, other| bus.1.cmp(&other.1))
        .unwrap();
    let product: usize = buses.iter().map(|(_, b)| b).product();
    let result_part_two: usize = buses.iter()
        .map(| (index, bus)| {
            let factors = product / bus;
            index * factors * inverse(factors, *bus)
        })
        .sum();

    println!("Result {}", earliest_bus.0 * earliest_bus.1);
    println!("Result Part 2 {}", product - (result_part_two % product));
}

fn inverse(mut x: usize, mut y: usize) -> usize {
    (1..y).find(|z| (z * x).rem_euclid(y) == 1).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use crate::day_thirteen::*;

    #[test]
    fn should_() {

    }
}
