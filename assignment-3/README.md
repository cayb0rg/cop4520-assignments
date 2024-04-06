# Programming Assignment 3

## Instructions

Make sure you have Rust installed.

Run the following:

```[language=bash]
git clone git@github.com:cayb0rg/cop4520-assignments.git
&& cd assignment-3
```

The program takes in one argument: `cargo run [problem_number]`

### Problem 1

```[language=Rust]
cargo run 1
```

The program will output the number of presents added and the number of thank you cards written at its completion. Please see program comments for more information.

### Problem 2

```[language=Rust]
cargo run 2
```

The program will output files in the format `report-<report_number>.txt` every hour. Please see program comments for more information and sample reports made.

To test with a shorter hour (such as 1 min), change `LENGTH_OF_HOUR_IN_SECS` in program to a multiple of 60. (60 means one "hour" will be a minute)

## Problem 2 efficiency, correctness, and progress guarantee

This program is efficient because each thread is responsible for a single sensor. It is correct because each thread collects temperature readings after sleeping for one minute every time it collects a new reading. This ensures that readings are exactly a minute apart. Along the same lines, the report is generated after every hour, on the dot. It checks the time interval by looking at data entries 80 elements apart (8 sensors * 10 minutes). It finds the lowest/highest by sorting the arrays and getting the lowest 5/highest 5 respectively.
