use std::collections::{HashMap, HashSet};

trait OptionMerge<T> {
    fn merge<S>(self, b: Option<S>) -> Option<(T, S)>;
}

impl<T> OptionMerge<T> for Option<T> {
    fn merge<S>(self, b: Option<S>) -> Option<(T, S)> {
        self.and_then(|a| b.map(|b| (a, b)))
    }
}

trait ResultMerge<T, E> {
    fn merge<S>(self, b: Result<S, E>) -> Result<(T, S), E>;
}

impl<T, E> ResultMerge<T, E> for Result<T, E> {
    fn merge<S>(self, b: Result<S, E>) -> Result<(T, S), E> {
        self.and_then(|a| b.map(|b| (a, b)))
    }
}

trait AbsDiff {
    fn abs_diff(self, other: u16) -> u16;
}

impl AbsDiff for u16 {
    fn abs_diff(self, other: u16) -> u16 {
        if self > other {
            self - other
        } else {
            other - self
        }
    }
}

trait IteratorExt: Iterator {
    fn uniq_min_by_key<B: Ord>(self, f: impl FnMut(&Self::Item) -> B) -> Option<Self::Item>;
}

impl<I: Iterator> IteratorExt for I {
    fn uniq_min_by_key<B: Ord>(mut self, mut f: impl FnMut(&Self::Item) -> B) -> Option<I::Item> {
        let first = self.next()?;
        let value = f(&first);
        let first = (first, value, true);

        let (min_e, _, uniq) = self.fold(first, |(min_e, min_v, uniq), e| {
            let v = f(&e);
            if v < min_v {
                (e, v, true)
            } else {
                let uniq = uniq && min_v != v;
                (min_e, min_v, uniq)
            }
        });
        if !uniq {
            None
        } else {
            Some(min_e)
        }
    }
}

fn solve(input: &str) -> u16 {
    let coords: Vec<(u16, u16)> = input
        .trim()
        .lines()
        .map(|l| l.split(","))
        .filter_map(|mut l| l.next().merge(l.next()))
        .map(|(c, r)| (c.trim(), r.trim()))
        .map(|(c, r)| c.parse().merge(r.parse()))
        .collect::<Result<_, _>>()
        .unwrap();

    let min_x = coords.iter().map(|(c, _)| c).min().cloned().unwrap();
    let min_y = coords.iter().map(|(_, r)| r).min().cloned().unwrap();

    let max_x = coords.iter().map(|(c, _)| c).max().cloned().unwrap();
    let max_y = coords.iter().map(|(_, r)| r).max().cloned().unwrap();

    let mut grid: HashMap<(u16, u16), u16> = HashMap::with_capacity(coords.len());
    let mut infinite_points: HashSet<(u16, u16)> = HashSet::new();

    for c in 0..=max_x {
        for r in 0..=max_y {
            let closest = coords
                .iter()
                .uniq_min_by_key(|(c1, r1)| c1.abs_diff(c) + r1.abs_diff(r))
                .cloned();
            match closest {
                None => continue,
                Some(closest) => {
                    if c == min_x || c == max_x || r == min_y || r == max_y {
                        infinite_points.insert(closest);
                    } else {
                        *(grid.entry(closest).or_default()) += 1;
                    }
                }
            }
        }
    }

    grid.into_iter()
        .filter(|(k, _)| !infinite_points.contains(k))
        .map(|(_, v)| v)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

        assert_eq!(solve(input), 17);
    }
}

common::bootstrap!(6);
