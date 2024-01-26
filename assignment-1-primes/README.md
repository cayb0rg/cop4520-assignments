## Problem Statement
> "Your non-technical manager assigns you the task to find all primes between 1 and 108. The assumption is that your company is going to use a parallel machine that supports eight concurrent threads. Thus, in your design you should plan to spawn 8 threads that will perform the necessary computation. Your boss does not have a strong technical background but she is a reasonable person. Therefore, she expects to see that the work is distributed such that the computational execution time is approximately equivalent among the threads."

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
<<<<<<< HEAD:assignment-1-primes/README

## Evaluation
Each thread is **well-formed**. Critical sections are locked using Rust's `Arc` struct, a thread-safe reference-counting pointer. We enter a new scope and call `lock()` on the shared counter before incrementing it. We save a copy of this value to use later. Upon leaving the scope, the lock drops automatically. We then do our `is_prime()` calculations (outside of any critical sections). If the number is prime, we call `lock()` on the array of primes and total sum before updating those.

Because the `is_prime` calculation takes place outside of any locks, the program can be run more efficiently.

The program is **deadlock and starvation-free**. Work is consistently distributed across the threads each run. This was tested using the `Instant` crate and summing the time spent in critical sections by each thread in an array. This is left out of the final program because it creates an additional lock which takes extra computational time (about 7-10 seconds more) but can be found in the `test_execution_speeds` function. Example output during testing:

> Number of primes found by each thread:
> Thread 1: 719234 in 5911810 microsec
> Thread 2: 720816 in 5925949 microsec
> Thread 3: 719152 in 5913603 microsec
> Thread 4: 720767 in 5921654 microsec
> Thread 5: 721963 in 5915342 microsec
> Thread 6: 720162 in 5916540 microsec
> Thread 7: 719175 in 5924184 microsec
> Thread 8: 720185 in 5916393 microsec
=======
>>>>>>> 6974169917dc8d9d10b38beaf7955bbdea687e00:assignment-1-primes/README.md
