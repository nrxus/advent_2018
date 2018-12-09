#![feature(range_contains)]

fn solve(input: &str) -> usize {
    let input = input.trim();

    let mut index = None;
    let mut ranges = vec![];
    let mut building = false;

    for (next, c) in input.chars().enumerate() {
        let i = match index {
            None => {
                if building {
                    building = false;
                    ranges.push(0..next);
                }
                index = Some(next);
                continue;
            }
            Some(i) => i,
        };

        let prev = input.chars().nth(i).unwrap();
        if prev.eq_ignore_ascii_case(&c) && prev != c {
            let i = if ranges.last().map(|r: &std::ops::Range<_>| &r.end) == Some(&i) {
                ranges.pop().unwrap().start
            } else {
                i
            };
            index = i.checked_sub(1);
            building = true;
        } else {
            if building {
                ranges.push(i + 1..next);

                index = Some(next);
                building = false;
            } else {
                index = Some(i + 1);
            }
        }
    }

    if building {
        let start = index.map(|i| i + 1).unwrap_or(0);
        let range = start..input.len();
        ranges.push(range);
    }

    let skip_len: usize = ranges.iter().map(|r| r.end - r.start).sum();

    input.len() - skip_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!(solve(input), 10);
    }

    #[test]
    fn test_2() {
        let input = "cBaAbdDB";
        assert_eq!(solve(input), 2);
    }

    #[test]
    fn test_3() {
        let input = "ZzNnMYytTtfFTINnkKimYUOouyPpvVokKZzqQQqTtMmmIiMjJryYvVRztJjiIfFyGbBgYTgGLfFHhlsysSYoOsSiIfpPFcCrRVvbeEuUAXx";
        assert_eq!(solve(input), 5);
    }

    #[test]
    fn test_4() {
        let input = "aBbA\n";
        assert_eq!(solve(input), 0);
    }
}

common::bootstrap!(5);
