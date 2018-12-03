#![feature(try_trait)]

use std::{num::ParseIntError, option::NoneError, str::FromStr};

fn solve(input: &str) -> usize {
    let fabric = Fabric {
        grid: [FabricState::Unclaimed; 1000 * 1000],
    };
    input
        .lines()
        .map(|l| l.parse::<Claim>().unwrap())
        .fold(fabric, |mut fabric, c| {
            fabric.claim_all(c.iter());
            fabric
        })
        .grid
        .iter()
        .filter(|&&s| s == FabricState::Conflict)
        .count()
}

struct Fabric {
    grid: [FabricState; 1000 * 1000],
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum FabricState {
    Unclaimed,
    Claimed,
    Conflict,
}

impl FabricState {
    fn claim(&mut self) {
        use self::FabricState::*;

        *self = match self {
            Unclaimed => Claimed,
            _ => Conflict,
        }
    }
}

impl Fabric {
    fn claim_all(&mut self, claims: impl Iterator<Item = (usize, usize)>) {
        claims.for_each(|c| self.claim(c));
    }

    fn claim(&mut self, (column, row): (usize, usize)) {
        let index = row * 1000 + column;
        self.grid[index].claim();
    }
}

#[derive(Debug)]
struct Claim {
    bottom_right: (usize, usize),
    top_left: (usize, usize),
}

impl Claim {
    fn iter(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        ClaimIter {
            claim: self,
            pointer: self.top_left,
        }
    }
}

struct ClaimIter<'c> {
    claim: &'c Claim,
    pointer: (usize, usize),
}

impl Iterator for ClaimIter<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pointer.0 == self.claim.bottom_right.0 {
            self.pointer.0 = self.claim.top_left.0;
            self.pointer.1 += 1;
        }

        if self.pointer.1 == self.claim.bottom_right.1 {
            None
        } else {
            let pointer = self.pointer;
            self.pointer.0 += 1;
            Some(pointer)
        }
    }
}

impl FromStr for Claim {
    type Err = ClaimParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut line = line.split("@").nth(1)?.split(":");
        let mut top_left = line.next()?.trim().split(",");
        let left = top_left.next()?.parse()?;
        let top = top_left.next()?.parse()?;
        let mut size = line.next()?.trim().split("x");
        let right = left + size.next()?.parse::<usize>()?;
        let bottom = top + size.next()?.parse::<usize>()?;
        Ok(Claim {
            top_left: (left, top),
            bottom_right: (right, bottom),
        })
    }
}

#[derive(Debug)]
struct ClaimParseError;

impl From<NoneError> for ClaimParseError {
    fn from(_: NoneError) -> Self {
        ClaimParseError
    }
}

impl From<ParseIntError> for ClaimParseError {
    fn from(_: ParseIntError) -> Self {
        ClaimParseError
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = r#"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"#;
        assert_eq!(solve(input), 4);
    }
}

common::bootstrap!(3);