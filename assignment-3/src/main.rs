use std::{env, fs::File, sync::{Arc, Mutex}, thread, time::{Duration, Instant}};
use rand::{random, Rng};
use std::io::Write;

use assignment_3::list::{add_in_order, write_thank_you_note, List, Node, Present};

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

        match problem_number {
            1 => servant_behavior(4, 100),
            2 => atmospheric_temp_reading(8),
            _ => println!("Incorrect problem number entered. Please enter 1 or 2.")
        };
    }
}

/** Create a random bag of size <num_presents> */
fn create_bag(num_presents: u32) -> Vec<Present> {
    let mut bag = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..num_presents {
        let present = Present {
            tag: rng.gen_range(0..100),
            card: false
        };
        bag.push(present);
    }
    bag

}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Turn {
    WRITE_THANK_YOU,
    ADD_PRESENT,
    CHECK_FOR_PRESENT
}

// Problem 1
/** Servants (threads) perform one of the following tasks:
 * 1. Sort presents into a linked list
 * 2. Write thank you notes for presents
 * 3. Check whether a gift with a particular tag is present in the chain
 */
fn servant_behavior(num_threads: i32, num_presents: u32) {
    // start the timer
    let now = Instant::now();
    // create the bag of presents
    let bag = Arc::new(Mutex::new(create_bag(num_presents)));
    // create the list of threads
    let mut handles = vec![];
    // create the linked list of presents
    let list = List::<Present>::new();
    // get the head of the list
    let head = list.head_ref();

    // track the number of presents added / number of thank you cards written
    let num_presents_added = Arc::new(Mutex::new(0));
    let num_thank_yous = Arc::new(Mutex::new(0));

    // track whether bag is empty
    let bag_empty = Arc::new(Mutex::new(false));

    // start the threads
    for serf in 0..(num_threads) {
        let bag = Arc::clone(&bag);
        let head = head.clone();
        let num_presents_added = Arc::clone(&num_presents_added);
        let num_thank_yous = Arc::clone(&num_thank_yous);
        let bag_empty = Arc::clone(&bag_empty);
        let handle = thread::spawn(move || {
            // track which action to perform next
            let mut turn = Turn::ADD_PRESENT;
            let mut head = head.clone();
            loop {
                // randomly decide whether Minotaur requests to find a tag
                let random_num_below_100 = rand::random::<u32>() % 100;
                if random_num_below_100 < 10 {
                    turn = Turn::CHECK_FOR_PRESENT;
                }
                // 1. write a thank you note
                if turn == Turn::WRITE_THANK_YOU {
                    // println!("Thread {} is writing a thank you note", serf);
                    let mut wrote_thanks = true;
                    let head = head.clone();
                    match write_thank_you_note(head, serf) {
                        Some(node) => (),
                        None => {
                            let bag_empty = bag_empty.lock().unwrap();
                            if *bag_empty {
                                break;
                            }
                            wrote_thanks = false;
                            // head.clone() // start from the beginning
                        },
                    };

                    if wrote_thanks {
                        println!("Thread {} wrote a thank you note", serf);
                        let mut num_thank_yous = num_thank_yous.lock().unwrap();
                        *num_thank_yous += 1;
                    }

                    turn = Turn::ADD_PRESENT;
                }
                // 2. add a present to the chain
                else if turn == Turn::ADD_PRESENT {
                    let present;
                    {
                        let mut bag = bag.lock().unwrap();
                        // println!("Thread {} is adding a present to the chain", serf);
                        present = match bag.pop() {
                            Some(present) => present,
                            None => {
                                let mut bag_empty = bag_empty.lock().unwrap();
                                *bag_empty = true;
                                turn = Turn::WRITE_THANK_YOU;
                                continue;
                            },
                        };
                    }
                    println!("Present {}", present.tag);
                    // add present to the chain in the correct position
                    match add_in_order(&mut head, present.clone()) {
                        Some(present) => {
                            // update head to contain the new node
                            head.replace(Arc::new(Mutex::new(Node {
                                elem: present,
                                next: head.clone(),
                            })));
                        },
                        None => ()
                    }

                    println!("Thread {} added present to the chain", serf);

                    let mut num_presents_added = num_presents_added.lock().unwrap();
                    *num_presents_added += 1;

                    turn = Turn::WRITE_THANK_YOU;
                }
                // 3. check whether a gift with a particular tag in present in the chain
                else if turn == Turn::CHECK_FOR_PRESENT {
                    println!("Thread {} is checking for a present in the chain", serf);
                    let mut current = head.clone();
                    let mut found = false;
                    let tag_to_find = rand::random::<u32>() % num_presents;
                    while let Some(node) = current {
                        let locked_node = node.lock().unwrap();
                        if locked_node.elem.tag == tag_to_find {
                            println!("Thread {} found present with tag {}", serf, tag_to_find);
                            found = true;
                            break;
                        }
                        current = locked_node.next.clone();
                    }
                    if !found {
                        println!("Thread {} did not find present with tag {}", serf, tag_to_find);
                    }
                    turn = Turn::WRITE_THANK_YOU;
                }
            }
            println!("Thread {} exited loop", serf);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // stop the timer
    let elapsed = now.elapsed();
    println!("Completed in {:?}", elapsed);

    println!("Num presents added: {}", *num_presents_added.lock().unwrap());
    println!("Num thank you notes written: {}", *num_thank_yous.lock().unwrap());
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct DataCaptureFrame {
    pub temp: f32,
    pub time: Duration,
}

const LENGTH_OF_HOUR_IN_SECS: u64 = 3600; // in seconds

// Problem 2
// use queue method (strategy 3)
fn atmospheric_temp_reading(num_threads: i32) {
    // start the timer
    let start = Instant::now();
    let mut handles = vec![];
    // create shared memory
    let data = Arc::new(Mutex::new(Vec::<DataCaptureFrame>::new()));
    let mut num_reports = 0;

    let report_data = Arc::clone(&data);
    let report_handle = thread::spawn(move || {
        loop {
            // check every hour
            thread::sleep(Duration::from_secs(LENGTH_OF_HOUR_IN_SECS as u64));
            compile_report(&report_data, &mut num_reports);
             // clear shared memory for next hour
            report_data.lock().unwrap().clear();
        }
    });
    handles.push(report_handle);


    for _serf in 0..(num_threads) {
        let data = data.clone();
        let handle = thread::spawn(move || {
            loop {
                // Simulate temperature reading by generating random number
                let temperature = rand::random::<f32>() * 170.0 - 100.0; // Range: -100F to 70F

                let capture = DataCaptureFrame {
                    time: start.elapsed(),
                    temp: temperature
                };

                // Store the temperature reading in shared memory
                data.lock().unwrap().push(capture);

                // Sleep for 1 minute
                thread::sleep(Duration::from_secs(LENGTH_OF_HOUR_IN_SECS / 60));
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // stop the timer
    let elapsed = start.elapsed();
    println!("Completed in {:?}", elapsed);
}

pub fn compile_report(data: &Arc<Mutex<Vec<DataCaptureFrame>>>, num_reports: &mut i32) {
    // get the lowest 5 temps
    let mut data = data.lock().unwrap();

    if data.len() < 5 {
        println!("Not enough data to compile report");
        return;
    }

    let mut lowest_five_temps = Vec::<DataCaptureFrame>::with_capacity(5);
    let mut highest_five_temps = Vec::<DataCaptureFrame>::with_capacity(5);
    let mut interval = Vec::<DataCaptureFrame>::with_capacity(2);

    println!("Data: ");
    for frame in data.iter() {
        println!("Time: {:?}, Temp: {}\n", frame.time, frame.temp);
    }

    // get the 10 minute interval of time when the largest temperature difference was observed
    // BEFORE sorting
    let mut greatest_temp_diff = 0.0;
    if data.len() < 80 {
        println!("Not enough data to compile report");
        return;
    }
    for (i, frame) in data.iter().enumerate().step_by(80) {
        if i == 0 {
            continue;
        } else {
            let diff = frame.temp - data[i - 10].temp;
            if diff > greatest_temp_diff {
                if interval.len() < 2 {
                    interval.push(data[i - 80]);
                    interval.push(*frame);
                } else {
                    interval[0] = data[i - 80];
                    interval[1] = *frame;
                }
                greatest_temp_diff = diff;
            }
        }
    }

    // sort the data by temp
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!("Data: ");
    for frame in data.iter() {
        println!("Time: {:?}, Temp: {}\n", frame.time, frame.temp);
    }

    // get the highest and lowest 5 temps
    for i in 0..5 {
        lowest_five_temps.push(data[i].clone());
        highest_five_temps.push(data[data.len() - i - 1].clone());
    }

    // Make the report
    let mut report = String::from("Report:\n");
    let mut file = File::create(format!("report-{:?}.txt", *num_reports)).unwrap();
    *num_reports += 1;

    report.push_str("Lowest 5 temperatures:\n");
    for frame in lowest_five_temps {
        report.push_str(&format!("Time: {:?}, Temp: {}\n", frame.time, frame.temp));
    }
    report.push_str("Highest 5 temperatures:\n");
    for frame in highest_five_temps {
        report.push_str(&format!("Time: {:?}, Temp: {}\n", frame.time, frame.temp));
    }
    report.push_str("10 minute interval with greatest temperature difference:\n");
    for frame in interval {
        report.push_str(&format!("Time: {:?}, Temp: {}\n", frame.time, frame.temp));
    }

    match file.write_all(report.as_bytes()) {
        Ok(_) => println!("Report written to file"),
        Err(_) => println!("Error writing report to file")
    };
}