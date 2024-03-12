use crate::List::{Cons, Nil};
use::std::ops::Deref;


#[derive(Debug)]
enum List{
    Cons(i64,Box<List>),
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
    let b = Box::new(5);
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
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


fn testing_drop(){
    let c = MyPointer{
        data: String::from("CCCC")
    };
    let d= MyPointer{
        data: String::from("hello again")
    };
    println!("pointers created");
    drop(c);
    println!("Released")
}


fn main() {
    testing_drop()
}
