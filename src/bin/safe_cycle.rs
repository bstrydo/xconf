use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value : String,
    next : RefCell<Weak<Node>>
}

fn main() {
    let a = Rc::new(Node{value:String::from("29"), next:RefCell::new(Weak::new())});
    {
        let b = Rc::new(Node{value:String::from("89"), next:RefCell::new(Rc::downgrade(&a) ) });
        *a.next.borrow_mut() = Rc::downgrade(&b);
        let z = a.next.borrow_mut();
        println!("z = {:?}", z.upgrade());
    }
    let z = a.next.borrow_mut();
    println!("z = {:?}", z.upgrade());
}
