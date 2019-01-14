#![feature(try_trait, drain_filter)]

mod extensions;

use std::{collections::VecDeque, num::ParseIntError, option::NoneError};

use array_macro::array;
use regex::Regex;

fn solve(input: &str) -> usize {
    let regex = Regex::new(r"pos=<(?P<x>-?\d+),(?P<y>-?\d+),(?P<z>-?\d+)>, r=(?P<r>\d+)").unwrap();
    let nanobots: Vec<_> = input
        .trim()
        .lines()
        .map(|l| {
            let caps = regex.captures(l)?;
            let x = caps.name("x")?.as_str().parse()?;
            let y = caps.name("y")?.as_str().parse()?;
            let z = caps.name("z")?.as_str().parse()?;
            let radius = caps.name("r")?.as_str().parse()?;
            let pos = (x, y, z);
            Ok(Nanobot { pos, radius })
        })
        .collect::<Result<_, ParsingError>>()
        .unwrap();

    let x_min = nanobots.iter().map(|n| n.pos.0).min().unwrap();
    let x_max = nanobots.iter().map(|n| n.pos.0).max().unwrap();
    let y_min = nanobots.iter().map(|n| n.pos.1).min().unwrap();
    let y_max = nanobots.iter().map(|n| n.pos.1).max().unwrap();
    let z_min = nanobots.iter().map(|n| n.pos.2).min().unwrap();
    let z_max = nanobots.iter().map(|n| n.pos.2).max().unwrap();
    let region = Region {
        nanobots: nanobots.iter().collect(),
        bounds: Bounds {
            pos: (x_min, y_min, z_min),
            length: (
                (x_max - x_min + 1) as usize,
                (y_max - y_min + 1) as usize,
                (z_max - z_min + 1) as usize,
            ),
        },
    };

    let mut regions = VecDeque::with_capacity(3_000_000);
    regions.push_back(region);
    while let Some(r) = regions.pop_front() {
        regions.extend(
            r.divide()
                .into_iter()
                .filter(|r| r.bounds.length.0 > 0 && r.bounds.length.1 > 0 && r.bounds.length.2 > 0)
                .filter(|r| r.nanobots.len() > 890)
                .cloned(),
        );
        if regions.iter().all(|r| r.bounds.length == (1, 1, 1)) {
            break;
        }
    }

    let pos = regions
        .into_iter()
        .max_by(|a, b| {
            let a_len = a.nanobots.len();
            let b_len = b.nanobots.len();

            if a_len == b_len {
                let b_dist = b.bounds.pos.0.abs() + b.bounds.pos.1.abs() + b.bounds.pos.2.abs();
                let a_dist = a.bounds.pos.0.abs() + a.bounds.pos.1.abs() + a.bounds.pos.2.abs();
                b_dist.cmp(&a_dist)
            } else {
                a_len.cmp(&b_len)
            }
        })
        .map(|r| r.bounds.pos)
        .unwrap();

    println!("chosen pos: {:?}", pos);

    (pos.0.abs() + pos.1.abs() + pos.2.abs()) as usize
}

#[derive(Debug)]
struct Nanobot {
    pos: (isize, isize, isize),
    radius: usize,
}

impl Nanobot {
    fn can_reach(&self, pos: (isize, isize, isize)) -> bool {
        ((self.pos.0 - pos.0).abs() + (self.pos.1 - pos.1).abs() + (self.pos.2 - pos.2).abs())
            as usize
            <= self.radius
    }

    fn intersects(&self, bounds: &Bounds) -> bool {
        bounds.corners().iter().any(|&c| self.can_reach(c))
            || self.can_reach(bounds.center())
            || self.edges().iter().any(|&e| bounds.surrounds(e))
    }

    fn edges(&self) -> [(isize, isize, isize); 6] {
        let (x, y, z) = self.pos;
        [
            (x, y, z - self.radius as isize),
            (x, y, z + self.radius as isize),
            (x, y - self.radius as isize, z),
            (x, y + self.radius as isize, z),
            (x - self.radius as isize, y, z),
            (x - self.radius as isize, y, z),
        ]
    }
}

#[derive(Clone, Copy, Debug)]
struct Bounds {
    pos: (isize, isize, isize),
    length: (usize, usize, usize),
}

impl Bounds {
    fn divide(&self) -> [Bounds; 8] {
        let (left, right) = Bounds::ldiv(self.length.0);
        let (up, down) = Bounds::ldiv(self.length.1);
        let (inwards, outwards) = Bounds::ldiv(self.length.2);
        let (x, y, z) = self.pos;

        [
            Bounds {
                pos: (x, y, z),
                length: (left, up, inwards),
            },
            Bounds {
                pos: (x, y, z + inwards as isize),
                length: (left, up, outwards),
            },
            Bounds {
                pos: (x, y + up as isize, z),
                length: (left, down, inwards),
            },
            Bounds {
                pos: (x, y + up as isize, z + inwards as isize),
                length: (left, down, outwards),
            },
            Bounds {
                pos: (x + left as isize, y, z),
                length: (right, up, inwards),
            },
            Bounds {
                pos: (x + left as isize, y, z + inwards as isize),
                length: (right, up, outwards),
            },
            Bounds {
                pos: (x + left as isize, y + up as isize, z),
                length: (right, down, inwards),
            },
            Bounds {
                pos: (x + left as isize, y + up as isize, z + inwards as isize),
                length: (right, down, outwards),
            },
        ]
    }

    fn corners(&self) -> [(isize, isize, isize); 8] {
        let (x, y, z) = self.pos;
        let (dx, dy, dz) = self.length;
        let (dx, dy, dz) = (dx as isize, dy as isize, dz as isize);

        [
            (x, y, z),
            (x, y, z + dz - 1),
            (x, y + dy - 1, z),
            (x, y + dy - 1, z + dz - 1),
            (x + dx - 1, y, z),
            (x + dx - 1, y, z + dz - 1),
            (x + dx - 1, y + dy - 1, z),
            (x + dx - 1, y + dy - 1, z + dz - 1),
        ]
    }

    fn center(&self) -> (isize, isize, isize) {
        let (x, y, z) = self.pos;
        let (dx, dy, dz) = self.length;
        (
            x + dx as isize / 2,
            y + dy as isize / 2,
            z + dz as isize / 2,
        )
    }

    fn ldiv(len: usize) -> (usize, usize) {
        if len % 2 == 0 {
            (len / 2, len / 2)
        } else {
            (len / 2, len / 2 + 1)
        }
    }

    fn surrounds(&self, (x, y, z): (isize, isize, isize)) -> bool {
        let (left, top, inwards) = self.pos;
        let (right, bottom, outwards) = (
            left + self.length.0 as isize,
            top + self.length.1 as isize,
            inwards + self.length.2 as isize,
        );
        x >= left && x < right && y >= top && y < bottom && z >= inwards && z < outwards
    }
}

#[derive(Debug, Clone)]
struct Region<'n> {
    bounds: Bounds,
    nanobots: Vec<&'n Nanobot>,
}

impl<'n> Region<'n> {
    fn divide(&self) -> [Region<'n>; 8] {
        let boxes = self.bounds.divide();
        array![|i| Region {
            bounds: boxes[i],
            nanobots: self.nanobots.iter().filter(|n| n.intersects(&boxes[i])).cloned().collect()
        }; 8]
    }
}

#[derive(Debug)]
struct ParsingError;

impl From<regex::Error> for ParsingError {
    fn from(_: regex::Error) -> Self {
        ParsingError
    }
}

impl From<NoneError> for ParsingError {
    fn from(_: NoneError) -> Self {
        ParsingError
    }
}

impl From<ParseIntError> for ParsingError {
    fn from(_: ParseIntError) -> Self {
        ParsingError
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_a() {
//         let input = r"pos=<10,12,12>, r=2
// pos=<12,14,12>, r=2
// pos=<16,12,12>, r=4
// pos=<14,14,14>, r=6
// pos=<50,50,50>, r=200
// pos=<10,10,10>, r=5";
//         assert_eq!(solve(input), 36);
//     }
// }

common::read_main!();
//common::bootstrap!(16);
