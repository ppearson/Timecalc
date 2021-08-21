Timecalc
========

Timecalc is a simple CLI application to work out time deltas based off timestamps, reporting the result as a human readable duration.

There are currently two implementations (in separate subdirectories), the original C++ version, and a newer Rust version. Both currently have identical functionality.

Usage:

    ./timecalc 08:30-12:45

Returns:

    Total time: 4 hours, 15 minutes.

Usage:

    ./timecalc 08:30-12:45,13:50-17:00

Returns:

    Total time: 7 hours, 25 minutes.

You can also use the string 'now' as an alias for the current time.


Possible future work
--------------------

* Support for dates

