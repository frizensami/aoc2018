extern crate chrono;

use chrono::prelude::*;
use chrono::Duration;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::vec::Vec;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum LogEvent {
    WakeUp,
    FallAsleep,
    BeginShift(u32),
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct LogEntry {
    timestamp: DateTime<Utc>,
    event: LogEvent,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Interval {
    start_sleep: DateTime<Utc>,
    end_sleep: DateTime<Utc>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Guard {
    id: u32,
    sleep_intervals: Vec<Interval>,
}

impl std::fmt::Debug for LogEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}] {:?}\n", self.timestamp, self.event)
    }
}

impl std::fmt::Debug for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<{} - {}>\n", self.start_sleep, self.end_sleep)
    }
}

impl std::fmt::Debug for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "#{} {:?}\n", self.id, self.sleep_intervals)
    }
}

const INPUT_FILENAME: &str = "input.txt";

/*
    Given a security log, process the period that guards are sleeping
    - Need to sort the log entries by time before further processing

*/
fn main() -> Result<()> {
    println!("Reading AOC Day 4 Input file: input.txt");

    // include_str! is also viable here, but wanted to learn File operations explicitly
    let f = File::open(INPUT_FILENAME)?;
    let mut logentries: Vec<LogEntry> = Vec::new();

    // Parse all log entries
    for line in BufReader::new(f).lines() {
        let line_parse = line.unwrap();
        let new_logentry = line_to_logentry(&line_parse);
        logentries.push(new_logentry);
    }

    // Sort them by timestamp ordering
    logentries.sort();
    println!("{:?}", logentries);

    // Parse the sorted log entries into a set of Guard objects
    let guards: Vec<Guard> = guards_from_sorted_logentries(&logentries);
    println!("{:?}", guards);

    // Sum up all sleep intervals to find the sleepiest guard
    let max_guard: (&Guard, Duration) = guards
        .iter()
        .map(|g| {
            (
                g,
                g.sleep_intervals
                    .iter()
                    .map(|s| s.end_sleep.signed_duration_since(s.start_sleep))
                    .fold(Duration::zero(), |sum, i| sum + i),
            )
        })
        .max_by_key(|x| x.1)
        .unwrap();

    println!("Max guard: {:?}", max_guard);

    // Given the guard with the most sleep duration: we can find out the minutes he's asleep
    let minute_ranges: Vec<(u32, u32)> = max_guard.0.sleep_intervals.iter().map(|i| (i.start_sleep.minute(), i.end_sleep.minute())).collect(); 

    println!("Minute ranges for guard: {:?}", minute_ranges);

    let mut minute_sleep_frequencies: HashMap<u32, u32> = HashMap::new();
    for (start, end) in minute_ranges {
        for i in start..end {
            minute_sleep_frequencies.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }
    }

    println!("Minute frequencies: {:?}", minute_sleep_frequencies);
    println!("(Max minute, Frequency): {:?}", minute_sleep_frequencies.iter().max_by_key(|x| x.1).unwrap());

    // PART 2
    // Given the guard with the most sleep duration: we can find out the minutes he's asleep
    let guards_minute_ranges: Vec<(u32, Vec<(u32, u32)>)> = guards.iter().map(|g| (g.id, g.sleep_intervals.iter().map(|i| (i.start_sleep.minute(), i.end_sleep.minute())).collect())).collect(); 

    println!("All minute ranges: {:?}", guards_minute_ranges);

    let mut max_guardid: u32 = 0;
    let mut max_minutefreq_overall: u32 = 0;
    let mut max_minute_overall = 0;
    for (guardid, minute_ranges) in guards_minute_ranges {
        let mut minute_sleep_frequencies: HashMap<u32, u32> = HashMap::new();
        for (start, end) in minute_ranges {
            for i in start..end {
                minute_sleep_frequencies.entry(i).and_modify(|x| *x += 1).or_insert(1);
            }
        }

        let (max_minute, max_minute_freq): (&u32, &u32) = minute_sleep_frequencies.iter().max_by_key(|x| x.1).unwrap();
        if *max_minute_freq > max_minutefreq_overall {
            max_minutefreq_overall = *max_minute_freq;
            max_guardid = guardid;
            max_minute_overall = *max_minute;
        }
    }

    println!("Max guardid: {}, max minute freq: {}, max minute: {}, answer: {}", max_guardid, max_minutefreq_overall, max_minute_overall, max_guardid * max_minute_overall);

    Ok(())
}

fn line_to_logentry(line: &str) -> LogEntry {
    // Get datetime for log entry
    let l_sq_bracket_idx = line.find('[').unwrap();
    let r_sq_bracket_idx = line.find(']').unwrap();
    let datetime_string = &line[(l_sq_bracket_idx + 1)..r_sq_bracket_idx];
    let year: i32 = datetime_string[0..4].parse().unwrap();
    let month: u32 = datetime_string[5..7].parse().unwrap();
    let day: u32 = datetime_string[8..10].parse().unwrap();
    let hour: u32 = datetime_string[11..13].parse().unwrap();
    let min: u32 = datetime_string[14..16].parse().unwrap();
    let datetime = Utc.ymd(year, month, day).and_hms(hour, min, 0);

    // Parse type of log entry
    let log_entry_string = &line[(r_sq_bracket_idx + 2)..];
    let event: LogEvent = if log_entry_string.starts_with("falls") {
        LogEvent::FallAsleep
    } else if log_entry_string.starts_with("wakes") {
        LogEvent::WakeUp
    } else {
        let begins_shift_idx = log_entry_string.find('b').unwrap();
        let guard_number: u32 = log_entry_string[7..(begins_shift_idx - 1)].parse().unwrap();
        LogEvent::BeginShift(guard_number)
    };
    return LogEntry {
        timestamp: datetime,
        event: event,
    };
}

fn guards_from_sorted_logentries(logentries: &Vec<LogEntry>) -> Vec<Guard> {
    let mut guards: Vec<Guard> = Vec::new();
    let mut guard_intervals: HashMap<u32, Vec<Interval>> = HashMap::new();
    let mut current_guard_id: u32 = 0;
    let mut start_sleep: DateTime<Utc> = Utc::now();
    for logentry in logentries {
        match logentry.event {
            /*
            LogEvent::BeginShift(id) => { guards.push(current_guard); current_guard = Guard { id: id, sleep_intervals: Vec::new() };}
            LogEvent::FallAsleep => { start_sleep = logentry.timestamp; }
            LogEvent::WakeUp => { current_guard.sleep_intervals.push(Interval { start_sleep: start_sleep, end_sleep: logentry.timestamp }) }
            */
            LogEvent::BeginShift(id) => {
                current_guard_id = id;
            }
            LogEvent::FallAsleep => {
                start_sleep = logentry.timestamp;
            }
            LogEvent::WakeUp => {
                guard_intervals
                    .entry(current_guard_id)
                    .and_modify(|e| {
                        e.push(Interval {
                            start_sleep: start_sleep,
                            end_sleep: logentry.timestamp,
                        })
                    })
                    .or_insert(Vec::new());
            }
        }
    }

    for (id, intervals) in guard_intervals.drain() {
        guards.push(Guard {
            id: id,
            sleep_intervals: intervals,
        });
    }

    guards
}
