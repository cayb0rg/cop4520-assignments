use std::sync::{Arc, Mutex};
use std::{thread, fs};
use std::time::Instant;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // if the user provides a number, use that for the number of threads
    if args.len() < 2 {
        get_primes_multithreaded(8);
    } else {
        let num_threads = &args[1];
        match num_threads.parse::<i32>() {
            Ok(num) => get_primes_multithreaded(num),
            Err(_) => println!("Incorrect number of threads entered. Please use an integer.")
        }
    }
}

// Given the number of threads, calculate the prime numbers up to 10^8 using multithreading
// Store the elapsed time, number of primes, and last ten prime numbers in 'primes.txt'
fn get_primes_multithreaded(num_threads: i32) {
    // start counter at 3
    let counter = Arc::new(Mutex::new(3));
    // start sum at 2 (since 2 is a prime number)
    let sum = Arc::new(Mutex::new(2));

    let max = (10 as u64).pow(8);

    let mut handles = vec![];

    // initiate prime array with number '2'
    let primes = Arc::new(Mutex::new(vec![2]));

    // start the timer
    let now = Instant::now();
    for _ in 0..num_threads {
        let counter = Arc::clone(&counter);
        let primes = Arc::clone(&primes);
        let sum = Arc::clone(&sum);
        let handle = thread::spawn(move || {
            loop {
                let cur_num;
                {
                    // acquire lock to the counter
                    let mut num = counter.lock().unwrap();
                    // if counter has reached 10^8, exit loop (and this thread)
                    if *num > max {
                        break;
                    }
                    // save the num for use later
                    cur_num = *num;
                    // increment the counter by 2, skipping even numbers
                    *num += 2;
                }
                // counter lock is dropped

                // do work
                if is_prime(cur_num) {
                    // lock the primes vector and sum to make changes
                    let mut vector = primes.lock().unwrap();
                    let mut sum = sum.lock().unwrap();
                    vector.push(cur_num);
                    *sum += cur_num;
                }

            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // stop the timer
    let elapsed = now.elapsed();
    let result = primes.lock().unwrap();
    let num_of_primes = (*result).len();

    // summarize results in primes.text
    let mut file_contents = format!("{:?} {:?} {:?}", elapsed, num_of_primes,*sum.lock().unwrap());
    file_contents.push('\n');
    for i in (1..11).rev() {
        file_contents.push_str(&(*result)[num_of_primes - i].to_string());
        file_contents.push(' ');
    }

    fs::write("./primes.txt", file_contents).expect("Unable to write to file.");

    println!("Results stored in primes.txt");

}

// determines whether a number is prime
fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    if num == 2 || num == 3 {
        return true;
    }

    // check whether num is divisible by any number between 0 and num's square root
    let max = (num as f64).sqrt() as u64 + 1;

    for i in 2..max {
        if num % i == 0 {
            return false;
        }
    }

    return true;
}