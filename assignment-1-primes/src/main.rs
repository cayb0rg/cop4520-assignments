use std::sync::{Arc, Mutex};
use std::{env, fs, thread};
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    // if the user provides a number, use that for the number of threads
    if args.len() < 2 {
        get_primes(8);
    } else {
        let num_threads = &args[1];
        match num_threads.parse::<i32>() {
            Ok(num) => get_primes(num),
            Err(_) => println!("Incorrect number of threads entered. Please use an integer.")
        }
    }
}

// Given the number of threads, calculate the prime numbers up to 10^8 using multithreading
// Store the elapsed time, number of primes, and last ten prime numbers in 'primes.txt'
fn get_primes(num_threads: i32) {
    let counter = Arc::new(Mutex::new(3));
    let primes = Arc::new(Mutex::new(vec![2]));
    let sum = Arc::new(Mutex::new(2));
    let max = (10 as u64).pow(8);

    let mut handles = vec![];

    // start the timer
    let now = Instant::now();
    for _ in 0..num_threads {
        let counter = Arc::clone(&counter);
        let primes = Arc::clone(&primes);
        let sum = Arc::clone(&sum);

        // spawn the thread
        let handle = thread::spawn(move || {
            loop {
                let cur_num;
                {
                    let mut num = counter.lock().unwrap();  // enter critical section
                    cur_num = *num;  // in critical section
                    *num += 2;       // in critical section
                }                    // leave critical section

                // if counter has reached 10^8, exit thread
                if cur_num > max {
                    break;
                }
                if is_prime(cur_num) {
                    {
                        let mut primes = primes.lock().unwrap(); // enter critical section
                        primes.push(cur_num);  // in critical section
                    } // leave critical section
                    {
                        let mut sum = sum.lock().unwrap(); // enter critical section
                        *sum += cur_num; // in critical section
                    } // leave critical section
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
    let primes = primes.lock().unwrap();
    let num_of_primes = (*primes).len();

    // summarize results in primes.txt
    let mut file_contents = format!("{:?} {:?} {:?}", elapsed, num_of_primes,*sum.lock().unwrap());
    file_contents.push('\n');
    // get top ten maximum primes
    for i in (1..11).rev() {
        file_contents.push_str(&(*primes)[num_of_primes - i].to_string());
        file_contents.push(' ');
    }

    fs::write("./primes.txt", file_contents).expect("Unable to write to file.");

    println!("Completed in {:?}", elapsed);
    println!("Results stored in primes.txt");

}

fn is_prime(num: u64) -> bool {
    // check whether num is divisible by any number between 0 and num's square root
    let max = (num as f64).sqrt() as u64 + 1;

    for i in (3..max).step_by(2) {
        if num % i == 0 {
            return false;
        }
    }

    return true;
}

// Tests time spent in each critical section and number of primes by each thread
// fn test_execution_speed(num_threads: i32) {
//     let counter = Arc::new(Mutex::new(3));
//     let sum = Arc::new(Mutex::new(2));
//     let max = (10 as u64).pow(8);

//     let mut handles = vec![];
//     let primes = Arc::new(Mutex::new(vec![2]));
//     let execution_times = Arc::new(Mutex::new(vec![0; num_threads as usize]));
//     let primes_found = Arc::new(Mutex::new(vec![0; num_threads as usize]));

//     // start the timer
//     let now = Instant::now();
//     for th in 0..num_threads {
//         let counter = Arc::clone(&counter);
//         let primes = Arc::clone(&primes);
//         let sum = Arc::clone(&sum);
//         let primes_found = Arc::clone(&primes_found);
//         let execution_times = Arc::clone(&execution_times);

//         let handle = thread::spawn(move || {
//             let mut execution_time = 0;
//             loop {
//                 let cur_num;
//                 {
//                     let mut num = counter.lock().unwrap();  // enter critical section
//                     let now = Instant::now();
//                     cur_num = *num;  // in critical section
//                     *num += 2;       // in critical section
//                     execution_time += now.elapsed().as_micros();
//                 }                    // leave critical section

//                 // if counter has reached 10^8, exit thread
//                 if cur_num > max {
//                     break;
//                 }
//                 if is_prime(cur_num) {
//                     {
//                         let mut primes_found = primes_found.lock().unwrap();
//                         primes_found[th as usize] += 1;
//                     }
//                     {
//                         let mut primes = primes.lock().unwrap(); // enter critical section
//                         let now = Instant::now();
//                         primes.push(cur_num);  // in critical section
//                         execution_time += now.elapsed().as_micros();
//                     } // leave critical section
//                     {
//                         let mut sum = sum.lock().unwrap(); // enter critical section
//                         let now = Instant::now();
//                         *sum += cur_num; // in critical section
//                         execution_time += now.elapsed().as_micros();
//                     } // leave critical section
//                 }

//             }
//             let mut execution_times = execution_times.lock().unwrap();
//             execution_times[th as usize] = execution_time;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     // stop the timer
//     let elapsed = now.elapsed();
//     let result = primes.lock().unwrap();
//     let num_of_primes = (*result).len();

//     // summarize results in primes.text
//     let mut file_contents = format!("{:?} {:?} {:?}", elapsed, num_of_primes,*sum.lock().unwrap());
//     file_contents.push('\n');
//     for i in (1..11).rev() {
//         file_contents.push_str(&(*result)[num_of_primes - i].to_string());
//         file_contents.push(' ');
//     }

//     fs::write("./primes.txt", file_contents).expect("Unable to write to file.");

//     let primes_found = primes_found.lock().unwrap();
//     let execution_times = execution_times.lock().unwrap();
//     println!("Number of primes found by each thread:");
//     for i in 0..num_threads {
//         println!("Thread {}: {} in {} microsec", i + 1, primes_found[i as usize], execution_times[i as usize]);
//     }

//     println!("Completed in {:?}", elapsed);
//     println!("Results stored in primes.txt");

// }