use std::sync::{Arc, Mutex};
use std::ops::Deref;
use std::io::Write;

#[derive(Clone)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Arc<Mutex<Node<T>>>>;

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub elem: T,
    pub next: Link<T>,
}

impl<T> Node<T> {
    fn as_ref(&self) -> &T {
        &self.elem
    }
}

#[derive(Clone, Debug)]
pub struct Present {
    pub tag: u32,
    pub card: bool,
}

impl Ord for Present {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.tag.cmp(&other.tag)
    }
}

impl PartialOrd for Present {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Present {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag
    }
}

impl Eq for Present {}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        let new_node = Arc::new(Mutex::new(Node {
            elem: elem,
            next: self.head.clone(),
        }));
        List { head: Some(new_node) }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| {
                let locked_node = node.lock().unwrap();
                locked_node.next.clone()
            }),
        }
    }

    pub fn head_ref(&self) -> Link<T> {
        self.head.clone()
    }

    pub fn head(&self) -> Option<T>
    where
        T: Clone,
    {
        self.head.as_ref().map(|node| {
            let locked_node = node.lock().unwrap();
            locked_node.elem.clone()
        })
    }



    // pub fn add_in_order(&mut self, elem: T)
    // where
    //     T: Clone + PartialOrd,
    // {
    //     println!("Adding node");
    //     std::io::stdout().flush().unwrap();
    //     let new_node = Arc::new(Mutex::new(Node {
    //         elem: elem,
    //         next: None,
    //     }));

    //     let mut current = self.head.clone();
    //     let mut prev = None;
    // locked_nodep();
    //         if locked_node.as_ref() >= new_node.lock().unwrap().as_ref() {
    //             println!("Found a node with a greater value");
    //             break;
    //         }
    //         prev = Some(node.clone());
    //         current = locked_node.next.clone();
    //     }

    //     if let Some(prev) = prev {
    //         println!("Adding node in the middle of the list");
    //         let mut locked_prev = prev.lock().unwrap();
    //         let mut new_node = new_node.lock().unwrap();
    //         new_node.next = locked_prev.next.clone();
    //         locked_prev.next = Some(Arc::new(Mutex::new(new_node.clone())));
    //     } else {
    //         println!("Adding node at the beginning of the list");
    //         let mut new_node = new_node.lock().unwrap();
    //         new_node.next = self.head.clone();
    //         self.head = Some(Arc::new(Mutex::new(new_node.clone())));
    //     }
    // }

    // pub fn iter(&self) -> Iter<'_, T>
    // where
    //     T: Clone,
    // {
    //     Iter {
    //         next: self.head.as_ref().map(|node| {
    //             let locked_node = node.lock().unwrap();
    //             locked_node.as_ref()
    //         }),
    //     }
    // }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if Arc::strong_count(&node) == 1 {
                head = node.lock().unwrap().next.take();
            } else {
                break;
            }
        }
    }
}

// pub struct Iter<'a, T> {
//     next: Option<&'a Node<T>>,
// }

// impl<'a, T> Iterator for Iter<'a, T> {
//     type Item = &'a T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.next.map(|node| {
//             self.next = node.next.as_ref().map(|n| &*n.lock().unwrap());
//             &node.elem
//         })
//     }
// }

pub fn add_in_order(head: &mut Link<Present>, present: Present) -> Option<Present> {
    println!("Adding present: {:?}", present.tag);
    let new_node = Arc::new(Mutex::new(Node {
        elem: present,
        next: None,
    }));

    let mut current = head.clone();
    let mut prev: Option<Arc<Mutex<Node<Present>>>> = None;

    if let None = current {
        println!("Empty list, returning new head");
        return Some(new_node.lock().unwrap().elem.clone());
    }

    let mut position = 0;
    while let Some(node) = current {
        let locked_node = node.lock().unwrap();
        if locked_node.as_ref() >= new_node.lock().unwrap().as_ref() {
            println!("Inserting present at position: {}", position);

            if let Some(prev) = prev {
                println!("Inserting present in middle");
                let mut locked_prev = prev.lock().unwrap();
                let mut new_node = new_node.lock().unwrap();
                locked_prev.next = Some(Arc::new(Mutex::new(new_node.clone())));
                new_node.next = Some(Arc::new(Mutex::new(locked_node.clone())));
            } else {
                println!("Inserting present at beginning");
                // update the head
                let mut new_node = new_node.lock().unwrap();
                new_node.next = Some(Arc::new(Mutex::new(locked_node.clone())));
            }
        }
        prev = Some(node.clone());
        current = locked_node.next.clone();
        position += 1;
    }

    None
}

pub fn write_thank_you_note(node: Link<Present>, serf: i32) -> Link<Present> {
    let mut node = node;
    while let Some(node_ref) = node {
        let mut locked_node = node_ref.lock().unwrap();
        println!("Thread {} is checking present {}", serf, locked_node.elem.tag);
        if !locked_node.elem.card {
            println!("Thread {} is writing a thank you note for present {}", serf, locked_node.elem.tag);
            locked_node.elem.card = true;
            return Some(node_ref.clone());
        }
        if let Some(next) = locked_node.next.clone() {
            node = Some(next);
        } else {
            break;
        }
    }
    None
}