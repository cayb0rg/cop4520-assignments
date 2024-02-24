use std::{env, sync::{Arc, Mutex}, thread, time::Instant};
use std::collections::VecDeque;

const COUNTER: i32 = 1;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        // get the problem number from input
        let problem_number = match args[1].parse::<u32>() {
            Ok(num) => num,
            Err(_err) => {
                println!("Incorrect problem number entered. Please enter 1 or 2.");
                0
            }
        };

        let mut num_threads = 8;

        // if the user provides a number, use that for the number of threads
        if args.len() > 2 {
            num_threads = match args[2].parse::<i32>() {
                Ok(num) => num,
                Err(_err) => {
                    println!("Incorrect number of threads entered. Please use an integer.");
                    0
                }
            };
        }

        match problem_number {
            1 => start_party(num_threads),
            2 => vase_queue_strat(num_threads),
            _ => println!("Incorrect problem number entered. Please enter 1 or 2.")
        };
    }
}

fn start_party(num_threads: i32) {
    // start the timer
    let now = Instant::now();
    let mut handles = vec![];
    let counter_count = Arc::new(Mutex::new(0));
    let cupcake = Arc::new(Mutex::new(true));
    let minotaur_is_inviting = Arc::new(Mutex::new(true));

    for guest in 0..(num_threads) {
        let cupcake = Arc::clone(&cupcake);
        let counter_count = Arc::clone(&counter_count);
        let minotaur_is_inviting = Arc::clone(&minotaur_is_inviting);
        let handle = thread::spawn(move || {
            let mut ate_cupcake = false;
            loop {
                let mut cupcake_guard = cupcake.lock().unwrap();
                let mut minotaur_guard = minotaur_is_inviting.lock().unwrap();
                // check if guest is the counter
                if guest == COUNTER {
                    println!("Counter enters labyrinth.");
                    if !*cupcake_guard {
                        // Counter replaces the cupcake
                        *cupcake_guard = true;
                        let mut count = counter_count.lock().unwrap();
                        *count += 1;
                        println!("Counter: Replaced cupcake.");
                        // Counter checks if all guests have eaten a cupcake
                        if *count >= num_threads - 1 {
                            println!("Counter: All guests have eaten a cupcake.");
                            *minotaur_guard = false;
                            break;
                        }
                    } else {
                        println!("Counter: There's a cupcake here, so I'm going to do nothing.");
                    }
                    println!("Counter leaves labryinth.");
                }
                else if *minotaur_guard {
                    println!("Guest {} enters the labryinth.", guest);
                    if !ate_cupcake {
                        if *cupcake_guard {
                            // Eat the cupcake
                            *cupcake_guard = false;
                            ate_cupcake = true;
                            println!("Guest {}: Ate cupcake.", guest);
                        }
                        else {
                            println!("Guest {}: I've not eaten a cupcake, but there's none here to eat. I won't do anything.", guest);
                        }
                    }
                    else {
                        println!("Guest {}: I've already eaten a cupcake. I won't do anything.", guest);
                    }
                    println!("Guest {} leaves labryinth.", guest);
                }
                else {
                    break;
                }
                drop(cupcake_guard); // drop the lock (leave the labyrinth)

                thread::sleep(std::time::Duration::from_secs(1));
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // stop the timer
    let elapsed = now.elapsed();
    println!("Completed in {:?}", elapsed);
}

// use queue method (strategy 3)
fn vase_queue_strat(num_threads: i32) {
    let queue: Arc<Mutex<VecDeque<i32>>> = Arc::new(Mutex::new(VecDeque::new()));

    // start the timer
    let now = Instant::now();
    let mut handles = vec![];

    for guest in 0..(num_threads-1) {
        let queue_clone = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            loop {
                let mut q = queue_clone.lock().unwrap();
                // join the queue
                q.push_back(guest);
                drop(q); // release the lock on queue

                // wait for my turn
                while {
                    let q = queue_clone.lock().unwrap();
                    let is_my_turn = q.front().unwrap() == &guest;
                    !is_my_turn
                } {
                    thread::sleep(std::time::Duration::from_secs(1));
                }

                // enter the showroom
                println!("Guest {} enters the showroom", guest);

                // simulate some time spent in the showroom
                thread::sleep(std::time::Duration::from_secs(2));

                // leave the showroom
                println!("Guest {} leaves the showroom", guest);

                // notify the next guest in the queue
                let mut q = queue_clone.lock().unwrap();
                q.pop_front();
                if let Some(next_guest) = q.front() {
                    println!("Guest {} notifies Guest {} that the showroom is available", guest, next_guest);
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
    println!("Completed in {:?}", elapsed);
}