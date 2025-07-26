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

mod timeCalc;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: timecalc requires a command line argument consisting of at least one pair of time strings, i.e. '08:25-14:50',");
        eprintln!("  or alternatively a series of added short string time period durations, i.e. '3h14m+6h45m'.");
        return;
    }

    let main_arg = &args[1];
    if main_arg.contains("-version") {
        println!("Timecalc 1.0 (Rust version).");
        return;
    }

    timeCalc::calculate_duration(main_arg);
}
