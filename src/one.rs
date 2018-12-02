mod input_fetcher;

fn main() {
    let input = input_fetcher::fetch_or_exit(1);
    println!("{}", b(input.as_str()));
}

fn a(input: &str) -> i32 {
    sanitize(input).sum()
}

fn b(input: &str) -> i32 {
    use std::collections::HashSet;

    #[derive(Default, Debug)]
    struct State {
        partial_sums: HashSet<i32>,
        sum: i32,
    }

    impl State {
        fn is_done(&self) -> bool {
            self.partial_sums.contains(&self.sum)
        }

        fn add_number(&mut self, num: i32) {
            let sum = self.sum + num;
            self.partial_sums.insert(self.sum);
            self.sum = sum;
        }
    }

    sanitize(input)
        .cycle()
        .scan(State::default(), |state, x| {
            if state.is_done() {
                None
            } else {
                state.add_number(x);
                Some(state.sum)
            }
        })
        .last()
        .unwrap()
}

fn sanitize<'a>(input: &'a str) -> impl Iterator<Item = i32> + Clone + 'a {
    input.lines().map(|l| l.parse::<i32>().unwrap())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(
            a(r#"+1
+1
+1"#),
            3
        );

        assert_eq!(
            a(r#"+1
+1
-2"#),
            0
        );

        assert_eq!(
            a(r#"-1
-2
-3"#),
            -6
        );
    }

    #[test]
    fn test_b() {
        assert_eq!(
            b(r#"+1
-1"#),
            0
        );

        assert_eq!(
            b(r#"+3
+3
+4
-2
-4"#),
            10
        );

        assert_eq!(
            b(r#"-6
+3
+8
+5
-6"#),
            5
        );

        assert_eq!(
            b(r#"+7
+7
-2
-7
-4"#),
            14
        );
    }
}
