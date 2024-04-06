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
        *head = Some(new_node.clone());
        return Some(new_node.lock().unwrap().elem.clone());
    }

    let mut position = 0;
    while let Some(node) = current {
        let locked_node = node.lock().unwrap();
        println!("node: {:?}", locked_node.elem.tag);
        println!("next: {:?}", locked_node.next);
        if locked_node.as_ref() >= new_node.lock().unwrap().as_ref() {
            println!("Inserting present at position: {}", position);

            let mut new_node_clone = new_node.lock().unwrap();
            // update the head
            new_node_clone.next.replace(node.clone());
            if let Some(prev) = prev {
                println!("Inserting present in middle");
                let mut locked_prev = prev.lock().unwrap();
                locked_prev.next.replace(new_node.clone());
                return None;
            } else {
                println!("Inserting present at beginning");
                *head = Some(new_node.clone());
                return Some(new_node_clone.elem.clone());
            }
        }
        prev = Some(node.clone());
        current = locked_node.next.clone();
        position += 1;
    }
    // end of list
    println!("Inserting present at end");
    let prev = prev.unwrap();
    let mut locked_prev = prev.lock().unwrap();
    locked_prev.next.replace(new_node.clone());

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
            println!("Thread {} is checking the next present {}", serf, next.lock().unwrap().elem.tag);
            node = Some(next);
        } else {
            println!("No next present, returning None");
            break;
        }
    }
    None
}