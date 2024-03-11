use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List{
    Cons(i64,Box<T>),
    Nil,
}


fn boxLearning(){
    let b = Box::new(5);
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list={:?}",list)
}


fn main() {
    boxLearning()
}
