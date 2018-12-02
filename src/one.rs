mod input_fetcher;

fn main() {
    let input = input_fetcher::fetch_or_exit(1);
    println!("{}", a(input.as_str()));
}

fn a(input: &str) -> i32 {
    input.lines().map(|l| l.parse::<i32>().unwrap()).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(a("+1\n+1\n+1"), 3);
        assert_eq!(a("+1\n+1\n-2"), 0);
        assert_eq!(a("-1\n-2\n-3"), -6);
    }
}
