#[macro_export]
macro_rules! bootstrap {
    ($x:expr) => {
        fn main() {
            use $crate::input_fetcher;

            let input = input_fetcher::fetch_or_exit($x);
            println!("{}", solve(input.as_str()));
        }
    };
}
