use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value : i32,
    next : RefCell<Option<Rc<Node>>>
}

fn main() {
    let a = Rc::new(Node{value:29, next:RefCell::new(None)});
    let b = Rc::new(Node{value:89, next:RefCell::new(Some(a.clone() )) });
    *a.next.borrow_mut() = Some(b.clone());
    println!("a = {:?}", a);
}
