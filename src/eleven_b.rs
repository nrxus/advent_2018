mod extensions;

use self::extensions::cart_product;
use rayon::prelude::*;

fn power_level(x: usize, y: usize, serial: i32) -> i32 {
    let rack_id = x as i32 + 10;
    (rack_id * y as i32 + serial) * rack_id / 100 % 10 - 5
}

fn solve(input: &str) -> Answer {
    let serial: i32 = input.trim().parse().unwrap();
    let mut grid: [i32; 300 * 300] = [0; 300 * 300];
    grid.iter_mut()
        .enumerate()
        .for_each(|(i, v)| *v = power_level(i % 300 + 1, i / 300 + 1, serial));

    (1..301_usize)
        .into_par_iter()
        .flat_map(|s| {
            cart_product(0..301 - s, 0..301 - s)
                .par_bridge()
                .map(move |(x, y)| (x, y, s))
        })
        .max_by_key(|&(x, y, s)| {
            cart_product(0..s, 0..s)
                .map(|(dx, dy)| (x + dx, y + dy))
                .map(|(x, y)| grid[y * 300 + x])
                .sum::<i32>()
        })
        .map(|(x, y, s)| Answer(x + 1, y + 1, s))
        .unwrap()
}

#[derive(PartialEq, Eq, Debug)]
struct Answer(usize, usize, usize);

use std::fmt;

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.0, self.1, self.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve("18"), Answer(90, 269, 16));
        assert_eq!(solve("42"), Answer(232, 251, 12));
    }
}

common::read_main!();
//common::bootstrap!(9);
