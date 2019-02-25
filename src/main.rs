use typenum::*;
use std::marker::PhantomData;
use std::sync::{Mutex, MutexGuard};
use std::ops::Add;

pub struct Token<'a, Level> {
    level: PhantomData<Level>,
    lifetime: PhantomData<&'a ()>
}

impl<'a, Level> Drop for Token<'a, Level> {
    fn drop(&mut self) {
    }
}

fn root_token() -> Token<'static, U0> {
    Token { level: PhantomData, lifetime: PhantomData}
}

pub struct Lock<T, Level> {
    mutex: Mutex<T>,
    level: PhantomData<Level>,
}

impl<T, Level> Lock<T, Level> {
    pub fn new(inner: T) -> Self {
        Lock { level: PhantomData, mutex: Mutex::new(inner) }
    }
    pub fn lock<'a, TokenLevel>(&self, _token: &'a mut Token<TokenLevel>) -> (MutexGuard<T>, Token<'a, Level>)
    where TokenLevel: IsLess<Level, Output = True> {
        let guard = self.mutex.lock().unwrap();
        (guard, Token { level: PhantomData, lifetime: PhantomData})
    }
}
/*
fn do_stuff<Level: Add<B1>>(token: &mut Token<Level>) {
    let l = Lock::<u32, Add1<Level>>::new(0);
    l.lock(token);
}*/

fn main() {
    let mut token = root_token();
    let x = Lock::<u32, U5>::new(0);
    let y = Lock::<u32, U3>::new(0);
    let mut t2 = y.lock(&mut token);
    x.lock(&mut t2.1);
}
