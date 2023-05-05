use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn box_learning() {
    println!(" ");
    println!("### Box ###");

    let b = Box::new(5);
    // we can access the data in the box
    // similar to how we would if this data were on the stack
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:?}", list);
}

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct MyType {
    s: String,
}

impl Deref for MyType {
    type Target = String;

    fn deref(&self) -> &String {
        &self.s
    }
}

pub fn deref_trait() {
    println!(" ");
    println!("### Deref Trait ###");

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let xx = 5;
    let yy = Box::new(xx);
    assert_eq!(xx, *yy);

    let xxx = 10;
    let yyy = MyBox::new(xxx);
    assert_eq!(10, xxx);
    assert_eq!(10, *yyy);

    fn hello(n: &str) {
        println!("Hello, {n}!");
    }

    let my_string = MyBox::new(String::from("Ismael"));

    let my_ref = &my_string;

    hello(my_ref);

    let my_value = MyType {
        s: "Hello, Rust!".to_string(),
    };

    let my_ref = &my_value;

    // Implicit dereferencing using the * operator
    println!("{:?}", *my_ref);
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn drop_trait() {
    println!(" ");
    println!("### Drop Trait ###");

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    // d.drop(); // explicit destructor calls not allowed
    println!("CustomSmartPointers created.");
    drop(d);
    println!("CustomSmartPointer dropped before the end of main.");
}

use std::rc::Rc;

use crate::ListRc::{Consa, Nila};

#[derive(Debug)]
enum ListRc {
    Consa(i32, Rc<ListRc>),
    Nila,
}

pub fn rc_ref_counted() {
    println!(" ");
    println!("### Rc<T> ###");
    let a = Rc::new(Consa(5, Rc::new(Consa(10, Rc::new(Nila)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Consa(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Consa(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

use std::cell::RefCell;

pub fn ref_cell() {
    println!(" ");
    println!("### RefCell<T> ###");

    let data = RefCell::new(5);
    let mut mutable_borrow = data.borrow_mut();
    println!("Immutable borrow: {:?}", mutable_borrow);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        box_learning();
    }

    #[test]
    fn test_02() {
        deref_trait();
    }

    #[test]
    fn test_03() {
        drop_trait();
    }

    #[test]
    fn test_04() {
        rc_ref_counted();
    }

    #[test]
    fn test_05() {
        ref_cell();
    }
}
