mod input_fetcher;

fn main() {
    let input = input_fetcher::fetch_or_exit(1);
    println!("{:?}", input);
}

#[cfg(test)]
mod test {}
