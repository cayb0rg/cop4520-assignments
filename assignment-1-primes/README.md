## Problem Statement
> "Your non-technical manager assigns you the task to find all primes between 1 and 108. The assumption is that your company is going to use a parallel machine that supports eight concurrent threads. Thus, in your design you should plan to spawn 8 threads that will perform the necessary computation. Your boss does not have a strong technical background but she is a reasonable person. Therefore, she expects to see that the work is distributed such that the computational execution time is approximately equivalent among the threads. Finally, you need to provide a brief summary of your approach and an informal statement reasoning about the correctness and efficiency of your design. Provide a summary of the experimental evaluation of your approach. Remember, that your company cannot afford a supercomputer and rents a machine by the minute, so the longer your program takes, the more it costs. Feel free to use any programming language of your choice that supports multi-threading as long as you provide a ReadMe file with instructions for your manager explaining how to compile and run your program from the command prompt."

## Instructions
Make sure you have Rust installed.

Navigate to a directory you would like this repository installed. Then, run the commands:
```
git clone git@github.com:cayb0rg/cop4520-assignments.git &&
cd cop4520-assignments/assignment-1-primes &&
cargo run
```

You may also pass a thread count argument like so: `cargo run 9`. The program will create eight threads if no arguments are given.

## Output
The program outputs the execution time, total number of primes found, the sum of all primes, and the top ten maximum primes.

The program output is stored in `primes.txt`.
