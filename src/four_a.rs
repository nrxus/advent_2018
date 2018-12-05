#![feature(try_trait)]
#![feature(try_from)]

use std::{collections::HashMap, convert::TryFrom, num::ParseIntError, option::NoneError};

fn solve(input: &str) -> u16 {
    let mut entries: Vec<_> = input.lines().map(|l| Entry::try_from(l).unwrap()).collect();
    entries.sort_unstable_by(|a, b| a.date().cmp(b.date()));
    let guards = Schedule::guard_schedules(&entries).unwrap();

    let (id, schedules) = guards
        .iter()
        .max_by_key(|(_, schedules)| {
            schedules
                .iter()
                .flat_map(|s| s.minutes.iter())
                .filter(|&&s| s == GuardState::Asleep)
                .count()
        })
        .unwrap();

    let minute = schedules
        .iter()
        .flat_map(Schedule::asleep_minutes)
        .fold(HashMap::<usize, u16>::new(), |mut acc, m| {
            let count = acc.entry(m).or_default();
            *count += 1;
            acc
        })
        .iter()
        .max_by_key(|(_, &c)| c)
        .map(|(m, _)| m)
        .cloned()
        .unwrap();

    id * minute as u16
}

struct Schedule {
    minutes: [GuardState; 60],
}

impl Schedule {
    fn new() -> Self {
        Schedule {
            minutes: [GuardState::Awake; 60],
        }
    }

    fn guard_schedules(entries: &[Entry]) -> Result<HashMap<u16, Vec<Self>>, EntryParseError> {
        let mut id = if let Entry::BeginShift(_, id) = entries[0] {
            Ok(id)
        } else {
            Err(EntryParseError)
        }?;
        let mut schedule = Schedule::new();
        let mut guards: HashMap<_, Vec<_>> = HashMap::new();

        for entry in entries[1..].iter() {
            match entry {
                Entry::BeginShift(_, next_id) => {
                    guards.entry(id).or_default().push(schedule);

                    id = *next_id;
                    schedule = Schedule::new();
                }
                Entry::FallingAsleep(d) => {
                    let minute = d[14..16].parse()?;
                    schedule.set_after(minute, GuardState::Asleep);
                }
                Entry::WakingUp(d) => {
                    let minute = d[14..16].parse()?;
                    schedule.set_after(minute, GuardState::Awake);
                }
            }
        }

        Ok(guards)
    }

    fn asleep_minutes(&self) -> impl Iterator<Item = usize> + '_ {
        self.minutes
            .iter()
            .enumerate()
            .filter(|(_, s)| **s == GuardState::Asleep)
            .map(|(m, _)| m)
    }

    fn set_after(&mut self, minute: usize, state: GuardState) {
        self.minutes[minute..].iter_mut().for_each(|m| {
            *m = state;
        });
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GuardState {
    Awake,
    Asleep,
}

#[derive(Debug)]
enum Entry<'s> {
    BeginShift(&'s str, u16),
    FallingAsleep(&'s str),
    WakingUp(&'s str),
}

impl Entry<'_> {
    fn date(&self) -> &str {
        match self {
            Entry::BeginShift(d, _) => &d,
            Entry::FallingAsleep(d) => &d,
            Entry::WakingUp(d) => &d,
        }
    }
}

impl<'s> TryFrom<&'s str> for Entry<'s> {
    type Error = EntryParseError;

    fn try_from(raw: &'s str) -> Result<Self, Self::Error> {
        let date = &raw[1..17];
        let action = &raw[19..];
        if action.contains("falls asleep") {
            Ok(Entry::FallingAsleep(date))
        } else if action.contains("wakes up") {
            Ok(Entry::WakingUp(date))
        } else {
            let id = action[7..].split(" ").nth(0)?.parse::<u16>()?;
            Ok(Entry::BeginShift(date, id))
        }
    }
}

#[derive(Debug)]
struct EntryParseError;

impl From<NoneError> for EntryParseError {
    fn from(_: NoneError) -> Self {
        EntryParseError
    }
}

impl From<ParseIntError> for EntryParseError {
    fn from(_: ParseIntError) -> Self {
        EntryParseError
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = r"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

        assert_eq!(solve(input), 240);
    }
}

common::bootstrap!(4);
