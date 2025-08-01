/*
 Timecalc_rs
 Copyright 2021 Peter Pearson.

 Licensed under the Apache License, Version 2.0 (the "License");
 You may not use this file except in compliance with the License.
 You may obtain a copy of the License at

 http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
 ---------
*/

use chrono::Timelike;
use std::fmt;

macro_rules! comment {
    ($($t:tt)*) => {$($t)*};
}
macro_rules! comment {
    ($($t:tt)*) => {};
}

/// this represents a point-in-time, i.e. an actual time of the day,
/// essentially the time elapsed after midnight
#[derive(Debug, Default)]
struct TimePoint {
    hours:      u32,
    minutes:    u32,
    seconds:    u32
}

impl TimePoint {
    fn new() -> Self {
        Default::default()
    }

    fn get_total_time_point_in_seconds(&self) -> u32 {
        let mut final_val = self.hours * 60 * 60;
        final_val += self.minutes * 60;
        final_val += self.seconds;
        final_val
    }

    fn is_null(&self) -> bool {
        self.hours == 0 && self.minutes == 0 && self.seconds == 0
    }
}

/// This represents a time period, or duration
#[derive(Debug, Default, PartialEq)]
struct TimePeriod {
    hours:      u32,
    minutes:    u32,
    seconds:    u32
}

impl TimePeriod {
    fn new() -> Self {
        Default::default()
    }

    #[cfg(test)]
    fn add_hours(mut self, hours: u32) -> TimePeriod {
        self.hours += hours;
        self
    }

    #[cfg(test)]
    fn add_minutes(mut self, minutes: u32) -> TimePeriod {
        self.minutes += minutes;
        self
    }

    fn accumulate(&mut self, time_period: &TimePeriod) {
        self.hours += time_period.hours;
        self.minutes += time_period.minutes;
        self.seconds += time_period.seconds;
        self.normalise_units();
    }

    fn add_time_point_delta_in_seconds(&mut self, tp_delta_secs: u32) {
        self.seconds += tp_delta_secs;
        self.normalise_units();
    }

    fn normalise_units(&mut self) {
        if self.seconds > 59 {
            let num_normalised_minutes = self.seconds / 60;
            self.seconds %= 60;

            self.minutes += num_normalised_minutes;
        }

        if self.minutes > 59 {
            let num_normalised_hours = self.minutes / 60;
            self.minutes %= 60;

            self.hours += num_normalised_hours;
        }
    }

    fn is_null(&self) -> bool {
        self.hours == 0 && self.minutes == 0 && self.seconds == 0
    }
}

impl fmt::Display for TimePeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.hours > 0 && self.seconds > 0 {
            write!(f, "{} {}, {} {}, {} {}",
                    self.hours, if self.hours == 1 {"hour"} else {"hours"},
                    self.minutes, if self.minutes == 1 {"minute"} else {"minutes"},
                    self.seconds, if self.seconds == 1 {"second"} else {"seconds"})
        }
        else if self.hours > 0 {
            // no seconds to worry about
            write!(f, "{} {}, {} {}",
                    self.hours, if self.hours == 1 {"hour"} else {"hours"},
                    self.minutes, if self.minutes == 1 {"minute"} else {"minutes"})
        }
        else if self.seconds > 0 {
            // just minutes and seconds
            write!(f, "{} {}, {} {}",
                    self.minutes, if self.minutes == 1 {"minute"} else {"minutes"},
                    self.seconds, if self.seconds == 1 {"second"} else {"seconds"})
        }
        else {
            // hopefully just minutes
            write!(f, "{} {}",
                    self.minutes, if self.minutes == 1 {"minute"} else {"minutes"})
        }
    }
}

pub fn calculate_duration(times_string: &str) {
    let mut total_time_period = TimePeriod::new();

    if times_string.contains('+') {
        // it's a special type, which is a time period addition.
        total_time_period = calculate_time_period_additions(times_string);

        println!("Total time: {}.", total_time_period);
        return;
    }

//comment!
{
    // see if we have more than one pairs of times
    if times_string.contains(",") {
        // we have multiple time pairs
        let string_pairs: Vec<&str> = times_string.split(',').collect();

        for pair in &string_pairs {
            let pair_time_period = calculate_time_period_from_tp_pair(pair.as_ref());

            if pair_time_period.is_null() {
                println!("Error calculating time period from supplied input value.");
                return;
            }

            total_time_period.accumulate(&pair_time_period);
        }
    }
    else {
        // we only have a single pair...
        total_time_period = calculate_time_period_from_tp_pair(times_string);
    }
}

comment!
{
    // TODO: need to work out how we propogate / print errors with this version...
    
    total_time_period = times_string.split(",")
    .map(|pair|  calculate_time_period_from_tp_pair(&pair.to_string()))
    .fold(TimePeriod::new(), |mut acc, elem| {
        acc.accumulate(&elem);
        acc
    });
}
    println!("Total time: {}.", total_time_period);
}

fn extract_tp_from_string(time_string: &str) -> TimePoint {
    let num_colons = time_string.matches(':').count();

    let mut hours = 0;
    let mut minutes = 0;
    let mut seconds = 0;

    if num_colons == 0 {
        // see if string is "now"

        if time_string.eq("now") {
            let current_time = chrono::offset::Local::now();
            hours = current_time.time().hour();
            minutes = current_time.time().minute();
            // don't bother with seconds for the moment...
//          seconds = current_time.time().second();
        }
    }
    else if num_colons == 1 {
        // only hours and minutes

        let string_items: Vec<&str> = time_string.split(':').collect();
        // make sure we have valid strings we can parse
        if string_items.iter().all(|s| s.parse::<u32>().is_ok()) {
            hours = string_items[0].parse().unwrap();
            minutes = string_items[1].parse().unwrap();
        }
    }
    else if num_colons == 2 {
        // hours and minutes and seconds

        let string_items: Vec<&str> = time_string.split(':').collect();
        // make sure we have valid strings we can parse
        if string_items.iter().all(|s| s.parse::<u32>().is_ok()) {
            hours = string_items[0].parse().unwrap();
            minutes = string_items[1].parse().unwrap();
            seconds = string_items[2].parse().unwrap();
        }
    }

    let tp = TimePoint{hours, minutes, seconds};
    tp
}

fn calculate_time_period_from_tp_pair(tp_pair: &str) -> TimePeriod {
    let mut tp = TimePeriod{hours: 0, minutes: 0, seconds: 0};

    if tp_pair.contains('-') {
        let string_items: Vec<&str> = tp_pair.split('-').collect();

        let start_time = extract_tp_from_string(string_items[0]);
        let end_time = extract_tp_from_string(string_items[1]);

        if start_time.is_null() {
            println!("Error: unrecognised value in: '{}'", string_items[0]);
        }
        else if end_time.is_null() {
            println!("Error: unrecognised value in: '{}'", string_items[1]);
        }
        else {
            // otherwise, we got valid values from both pairs, so add the delta to the time period
            let tp_delta_seconds = end_time.get_total_time_point_in_seconds() - start_time.get_total_time_point_in_seconds();
            tp.add_time_point_delta_in_seconds(tp_delta_seconds);
        }
    }

    tp
}

fn calculate_time_period_additions(time_periods: &str) -> TimePeriod {
    let mut tp = TimePeriod::new();

    // we should have a series of time periods
    let string_items = time_periods.split('+');
    for time_period_str in string_items.enumerate() {
        if let Some(time_period) = extract_time_period_from_string(time_period_str.1) {
            tp.accumulate(&time_period);
        }
    }

    tp
}

fn extract_time_period_from_string(time_period_string: &str) -> Option<TimePeriod> {
    // only support hours and minutes so far...
    if !time_period_string.contains('h') && !time_period_string.contains('m') {
        // can't find either, so not valid.
        return None;
    }

    let mut time_period = TimePeriod::default();
    let mut num_buffer = String::new();

    for c in time_period_string.chars() {
        if c.is_ascii_digit() {
            num_buffer.push(c);
        }
        else if c.is_ascii_alphabetic() {
            if let Ok(num) = num_buffer.parse::<u32>() {
                if c == 'h' {
                    time_period.hours += num;
                }
                else if c == 'm' {
                    time_period.minutes += num;
                }
                else {
                    eprintln!("Unexpected time period character: '{}'", c);
                }
            }
            num_buffer.clear();
        }
    }

    Some(time_period)
}

//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_tp_from_string_01() {
        let tp_result = extract_tp_from_string(&String::from("07:32"));
        assert_eq!(tp_result.hours, 7);
        assert_eq!(tp_result.minutes, 32);
    }

    #[test]
    fn extract_time_period_extraction_from_string_01() {
        
        assert_eq!(extract_time_period_from_string(""), None);

        assert_eq!(extract_time_period_from_string("3h"), Some(TimePeriod::new().add_hours(3)));

        assert_eq!(extract_time_period_from_string("3h5m"), Some(TimePeriod::new().add_hours(3).add_minutes(5)));
    }
}
