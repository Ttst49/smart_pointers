use crate::List::{Cons, Nil};
use::std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
enum List{
    Cons(i64,Rc<List>),
    Nil,
}

struct MyPointer{
    data:String
}

impl Drop for MyPointer {
    fn drop(&mut self) {
        println!("Cleaning your pointer `{}` !", self.data);
    }
}


struct MyBox<T>(T);
impl<T> MyBox<T>{
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(unused)]
fn box_learning(){
    let b = Rc::new(5);
    let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));
    println!("list={:?}",list);


    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5,x);
    assert_eq!(5,*y);
}


#[allow(unused)]
fn greets(name:&str){
    println!("Hey,{}",name)
}


#[allow(unused)]
fn testing_drop(){
    let c = MyPointer{
        data: String::from("CCCC")
    };
    #[allow(unused_variables)]
    let d= MyPointer{
        data: String::from("hello again")
    };
    println!("pointers created");
    drop(c);
    println!("Released")
}

#[allow(unused)]
fn multi_references_lists(){
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after a creation = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after b creation = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after c creation = {}", Rc::strong_count(&a));
    }
    println!("count after c has been released = {}", Rc::strong_count(&a));
}


fn main() {
    multi_references_lists()
}
