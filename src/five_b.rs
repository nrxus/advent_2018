mod polymer_len;

use self::polymer_len::polymer_len;

fn solve(input: &str) -> usize {
    let input = input.trim();

    [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ]
    .iter()
    .map(|e| {
        input
            .chars()
            .filter(|c| &c.to_ascii_lowercase() != e)
            .collect::<String>()
    })
    .map(|s| polymer_len(&s))
    .min()
    .unwrap()
}

common::bootstrap!(5);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "dabAcCaCBAcCcaDA\n";
        assert_eq!(solve(input), 4);
    }
}
