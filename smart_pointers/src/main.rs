enum List {
    Cons(i32, Box<List>),
    Nil,
}
use crate::List::{Cons, Nil};
use std::ops::Deref;

fn main() {
    {
        let b = Box::new(5);
        println!("b = {}", b);
    }

    {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }

    {
        struct MyBox<T>(T);
        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }

        impl<T> Deref for MyBox<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        let x = 5;
        let y = MyBox::new(x);
        *y;
    }
}
