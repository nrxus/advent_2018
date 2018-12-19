#[derive(Default, Debug)]
struct Marble {
    prev: usize,
    next: usize,
    value: u32,
}

#[derive(Debug)]
struct Board {
    marbles: Vec<Marble>,
    current: usize,
}

impl Board {
    pub fn new(capacity: usize) -> Self {
        let mut marbles = Vec::with_capacity(capacity);
        marbles.push(Marble::default());
        Board {
            marbles,
            current: 0,
        }
    }

    pub fn skip_clock(&mut self, skipped: usize) -> &mut Board {
        for _ in 0..skipped {
            self.current = unsafe { self.marbles.get_unchecked(self.current) }.next;
        }
        self
    }

    pub fn skip_counterclock(&mut self, skipped: usize) -> &mut Board {
        for _ in 0..skipped {
            self.current = unsafe { self.marbles.get_unchecked(self.current) }.prev;
        }
        self
    }

    pub fn insert(&mut self, value: u32) {
        let current = self.marbles.len();
        let prev_marble = unsafe { self.marbles.get_unchecked_mut(self.current) };
        let new_marble = Marble {
            prev: self.current,
            next: prev_marble.next,
            value,
        };
        prev_marble.next = current;
        let next_marble = unsafe { self.marbles.get_unchecked_mut(new_marble.next) };
        next_marble.prev = current;

        self.marbles.push(new_marble);
        self.current = current;
    }

    pub fn remove(&mut self) -> u32 {
        let removed = self.marbles.swap_remove(self.current);

        //fix swap
        if self.current < self.marbles.len() {
            unsafe {
                let swapped = self.marbles.get_unchecked(self.current);
                let prev = swapped.prev;
                let next = swapped.next;
                self.marbles.get_unchecked_mut(prev).next = self.current;
                self.marbles.get_unchecked_mut(next).prev = self.current;
            }
        }

        self.current = removed.next;
        unsafe {
            self.marbles.get_unchecked_mut(removed.prev).next = removed.next;
            self.marbles.get_unchecked_mut(removed.next).prev = removed.prev;
        }

        removed.value
    }
}

fn solve(input: &str) -> u32 {
    let mut input = input.split_whitespace();
    let players: usize = input.next().unwrap().parse().unwrap();
    let mut players = vec![0; players];
    let marbles = input.rev().skip(1).next().unwrap().parse::<u32>().unwrap() * 100;
    let capacity = (marbles - 2 * (marbles / 23) + if marbles % 23 == 0 { 1 } else { 0 }) as usize;
    let mut board = Board::new(capacity);
    let mut player = 0;
    for marble in 1..=marbles {
        if marble % 23 == 0 {
            let removed = board.skip_counterclock(7).remove();
            player = player % players.len();
            unsafe {
                *(players.get_unchecked_mut(player)) += marble + removed;
            }
        } else {
            board.skip_clock(1).insert(marble);
        }
        player += 1;
    }
    players.into_iter().max().unwrap()
}

common::read_main!();
//common::bootstrap!(9);
