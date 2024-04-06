// if turn_to_write_ty {
                //     let mut list = list.lock().unwrap();
                //     // println!("Thread {} is writing a thank you note", serf);
                //     let mut present = match list.pop_back() {
                //         Some(present) => present,
                //         None => continue,
                //     };
                //     present.card = true;
                //     list.push_back(present);

                //     turn_to_write_ty = false;
                // } else {
                //     let present;
                //     {
                //         let mut bag = bag.lock().unwrap();
                //         // println!("Thread {} is adding a present to the chain", serf);
                //         present = match bag.pop() {
                //             Some(present) => present,
                //             None => break,
                //         };
                //     }
                //     let list_clone;
                //     {
                //         let mut list = list.lock().unwrap();
                //         if list.len() == 0 {
                //             list.push_back(present);
                //             turn_to_write_ty = true;
                //             continue;
                //         } else {
                //             list_clone = list.clone();
                //         }
                //     }

                //     for (i, p) in list_clone.iter().enumerate() {
                //         if p.tag > present.tag {
                //             let mut list = list.lock().unwrap();
                //             let mut second_split = list.split_off(i);
                //             list.push_back(present);
                //             list.append(&mut second_split);
                //             break;
                //         }
                //     }

                //     turn_to_write_ty = true;
                // }
                // 1. take a present from the bag and add it to the chain
                // match bag.try_lock() {
                //     Ok(mut bag) => {
                //         println!("Thread {} is adding a present to the chain", serf);
                //         let present = match bag.pop() {
                //             Some(present) => present,
                //             None => break,
                //         };
                //         let mut list = list.lock().unwrap();
                //         if list.len() == 0 {
                //             list.push_back(present);
                //         } else {
                //             let mut second_split = list.split_off(present as usize);
                //             list.push_back(present);
                //             list.append(&mut second_split);
                //         }
                //     },
                //     Err(_) => {
                //         // 2. write a thank you note and remove present from chain
                //         match list.lock() {
                //             Ok(mut list) => {
                //                 println!("Thread {} is writing a thank you note", serf);
                //                 let present = match list.pop_front() {
                //                     Some(present) => present,
                //                     None => continue,
                //                 };
                //                 if list.len() == 0 {
                //                     list.push_back(present);
                //                 } else {
                //                     let mut second_split = list.split_off(present as usize);
                //                     list.push_back(present);
                //                     list.append(&mut second_split);
                //                 }
                //             },
                //             Err(_) => {
                //                 let counter = counter.lock().unwrap();
                //                 println!("Request made by the Minotaur to find present with tag {} in the chain", counter);
                //                 let list = list.lock().unwrap();
                //                 for present in list.iter() {
                //                     if *present == *counter {
                //                         println!("Present with tag {} found in the chain", counter);
                //                         break;
                //                     }
                //                 }
                //             },
                //         }
                //     },
                // };


