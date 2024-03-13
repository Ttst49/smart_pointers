pub trait Messenger {
    fn send(&self,msg:&str);
}

pub struct LimitTracker<'a,T:Messenger>{
    messenger:&'a T,
    value: usize,
    max: usize
}

impl<'a, T> LimitTracker<'a,T>
    where T:Messenger
{
    pub fn new(messenger:&T,max:usize)->LimitTracker<T>{
        LimitTracker{
            messenger,
            value:0,
            max
        }
    }

    pub fn set_value(&mut self, value:usize){
        self.value = value;

        let max_percent = self.value as f64 / self.max as f64;

        if max_percent >= 1.0 {
            self.messenger.send("You reached your quota limit")
        }else if max_percent >= 0.9 { 
            self.messenger.send("You almost reached your quota limit")
        }else if max_percent >= 0.75 {
            self.messenger.send("You used 75% of your quota limit")
        }

    }
}