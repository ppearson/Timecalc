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

use chrono::{Timelike};

struct TimeCalc;

#[derive(Debug)]
#[derive(Default)]
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
        return final_val
    }

    fn is_null(&self) -> bool {
        return self.hours == 0 && self.minutes == 0 && self.seconds == 0
    }
}

#[derive(Debug)]
#[derive(Default)]
struct TimePeriod {
    hours:      u32,
    minutes:    u32,
    seconds:    u32
}

impl TimePeriod {
    fn new() -> Self {
        Default::default()
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
        return self.hours == 0 && self.minutes == 0 && self.seconds == 0
    }
}

impl TimeCalc {
    pub fn calculate_duration(times_string: &String) {
        let mut total_time_period = TimePeriod::new();
        // see if we have more than one pairs of times
        if times_string.contains(",") {
            // we have multiple time pairs
            let string_pairs: Vec<&str> = times_string.split(',').collect();

            for pair in &string_pairs {
                let pair_time_period = TimeCalc::calculate_time_period_from_tp_pair(&pair.to_string());

                if pair_time_period.is_null() {
                    println!("Error calculating time period from supplied input value.");
                    return
                }

                total_time_period.accumulate(&pair_time_period);
            }
        }
        else {
            // we only have a single pair...
            total_time_period = TimeCalc::calculate_time_period_from_tp_pair(&times_string);
        }

        if total_time_period.hours > 0 && total_time_period.seconds > 0 {
            println!("Total time: {} {}, {} {}, {} {}.",
                    total_time_period.hours, if total_time_period.hours == 1 {"hour"} else {"hours"},
                    total_time_period.minutes, if total_time_period.minutes == 1 {"minute"} else {"minutes"},
                    total_time_period.seconds, if total_time_period.seconds == 1 {"second"} else {"seconds"});
        }
        else if total_time_period.hours > 0 {
            // no seconds to worry about
            println!("Total time: {} {}, {} {}.",
                    total_time_period.hours, if total_time_period.hours == 1 {"hour"} else {"hours"},
                    total_time_period.minutes, if total_time_period.minutes == 1 {"minute"} else {"minutes"});
        }
        else if total_time_period.seconds > 0 {
            // just minutes and seconds
            println!("Total time: {} {}, {} {}.",
                    total_time_period.minutes, if total_time_period.minutes == 1 {"minute"} else {"minutes"},
                    total_time_period.seconds, if total_time_period.seconds == 1 {"second"} else {"second"});
        }
        else {
            // hopefully just minutes
            println!("Total time: {} {}.",
                    total_time_period.minutes, if total_time_period.minutes == 1 {"minute"} else {"minutes"});
        }
    }

    fn extract_tp_from_string(time_string: &String) -> TimePoint {

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
//                let seconds = current_time.time().second();
            }
        }
        else if num_colons == 1 {
            // only hours and minutes

            let string_items: Vec<&str> = time_string.split(':').collect();
            hours = string_items[0].parse::<u32>().unwrap();
            minutes = string_items[1].parse::<u32>().unwrap();
        }
        else if num_colons == 2 {
            // hours and minutes and seconds

            let string_items: Vec<&str> = time_string.split(':').collect();
            hours = string_items[0].parse::<u32>().unwrap();
            minutes = string_items[1].parse::<u32>().unwrap();
            seconds = string_items[2].parse::<u32>().unwrap();
        }

        let tp = TimePoint{hours: hours, minutes: minutes, seconds: seconds};
        return tp
    }

    fn calculate_time_period_from_tp_pair(tp_pair: &String) -> TimePeriod {
        let mut tp = TimePeriod{hours: 0, minutes: 0, seconds: 0};

        if tp_pair.contains('-') {
            let string_items: Vec<&str> = tp_pair.split('-').collect();

            let start_time = TimeCalc::extract_tp_from_string(&string_items[0].to_string());
            let end_time = TimeCalc::extract_tp_from_string(&string_items[1].to_string());

            if start_time.is_null() {
                println!("Error: unrecognised value in: '{}", string_items[0]);
            }
            else if end_time.is_null() {
                println!("Error: unrecognised value in: '{}", string_items[1]);
            }

            let tp_delta_seconds = end_time.get_total_time_point_in_seconds() - start_time.get_total_time_point_in_seconds();
            tp.add_time_point_delta_in_seconds(tp_delta_seconds);
        }

        return tp
    }
}
