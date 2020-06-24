use std::{io, mem};

pub struct Node {
    item: String,
    next: Link
}
type Link = Option<Box<Node>>;

struct StringStack {
    head: Link,
    size: u32
}

impl StringStack {
    pub fn new() -> StringStack {
        StringStack{
            head: None,
            size: 0
        }
    }
    pub fn push(&mut self, item: String) {
        let new_node = Box::new( Node{
            item,
            next: mem::replace(&mut self.head, None)
        });
        self.head = Some(new_node);
        self.size += 1;
    }
    pub fn pop(&mut self) -> Option<String> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                self.size -= 1;
                Some(node.item)
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        match &self.head {
            Some(_) => false,
            None => true
        }
    }
}

impl Drop for StringStack {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}

fn main() {
    let mut stack = StringStack::new();
    loop {
        println!("Enter an integer: ");
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                for i in input.trim().split(' ') {
                    if i == "-" {
                        stack.pop();
                    } else {
                        stack.push(String::from(i));
                    }
                }
            },
            Err(error) => println!("Error reading input: {}", error)
        }
        break
    }
    while !stack.is_empty() {
        if let Some(word) = stack.pop() {
            println!("{}", word);
        }
    }
}
