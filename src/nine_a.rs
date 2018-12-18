fn solve(input: &str) -> u32 {
    let mut input = input.split_whitespace();
    let players: usize = input.next().unwrap().parse().unwrap();
    let mut players = vec![0; players];
    let mut player_iter = players.iter_mut();
    let marbles: u32 = input.rev().skip(1).next().unwrap().parse().unwrap();
    let mut board = Vec::with_capacity((marbles - marbles / 23) as usize);
    board.push(0);
    let mut current = 0;
    for marble in 1..=marbles {
        let player = {
            match player_iter.next() {
                Some(p) => p,
                None => {
                    player_iter = players.iter_mut();
                    player_iter.next().unwrap()
                }
            }
        };
        current = if marble % 23 == 0 {
            let next = (current + board.len() - 7) % board.len();
            *player += marble + board.remove(next);
            next
        } else {
            let next = ((current + 2) % board.len()) as usize;
            board.insert(next, marble);
            next
        };
    }
    players.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r"9 players; last marble is worth 25 points";
        assert_eq!(solve(input), 32);

        let input = r"10 players; last marble is worth 1618 points";
        assert_eq!(solve(input), 8317);

        let input = r"13 players; last marble is worth 7999 points";
        assert_eq!(solve(input), 146373);

        let input = r"17 players; last marble is worth 1104 points";
        assert_eq!(solve(input), 2764);

        let input = r"21 players; last marble is worth 6111 points";
        assert_eq!(solve(input), 54718);

        let input = r"30 players; last marble is worth 5807 points";
        assert_eq!(solve(input), 37305);
    }
}

common::bootstrap!(9);
