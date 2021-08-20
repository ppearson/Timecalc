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

// module would be better, but...
include!("timeCalc.rs");

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: timecalc requires a command line argument consisting of at least one pair of time strings, i.e. '08:25-14:50'");
        return
    }

    let time_arg = &args[1];

    TimeCalc::calculate_duration(time_arg);
}


//

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_extract_tp_from_string_01() {
        let tp_result = TimeCalc::extract_tp_from_string(&String::from("07:32"));
        assert_eq!(tp_result.hours, 7);
        assert_eq!(tp_result.minutes, 32);
    }
}