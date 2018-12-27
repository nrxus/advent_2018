fn solve(input: &str) -> usize {
    let input = input.trim().as_bytes();
    let mut scores = vec![b'3', b'7'];
    let mut first = 0;
    let mut second = 1;

    loop {
        let new = scores[first] - b'0' + scores[second] - b'0';
        if new > 9 {
            scores.extend(&[b'1', b'0' + (new % 10)]);
        } else {
            scores.push(b'0' + new);
        }
        first = (first + 1 + (scores[first] - b'0') as usize) % scores.len();
        second = (second + 1 + (scores[second] - b'0') as usize) % scores.len();

        if let Some((i, _)) = scores
            .windows(input.len())
            .enumerate()
            .skip(scores.len().checked_sub(input.len() + 1).unwrap_or(0))
            .find(|(_, w)| *w == input)
        {
            break i;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve("51589"), 9);
        assert_eq!(solve("01245"), 5);
        assert_eq!(solve("92510"), 18);
        assert_eq!(solve("59414"), 2018);
        assert_eq!(solve("515891"), 9);
    }
}

common::read_main!();
//common::bootstrap!(14);
