extern crate typemap;

use std::sync::{Arc, RwLock};
use std::thread;
use typemap::{Key, ShareDebugMap};
use std::sync::mpsc;
use std::time;


struct Test {
    cache: RwLock<ShareDebugMap>,
}

impl Test {
    fn new() -> Test {
        Test {
            cache: RwLock::new(ShareDebugMap::custom()),
        }
    }

    fn get_a(&self) -> Arc<Vec<A>> {
        loop {
            // it is important to use read() insted of try_read() to avoid high CPU usage
            // as read() will block the thread until try_write() will release the lock
            match self.cache.read() {
                Ok(map) => {
                    if map.contains::<A>() {
                        let a = map.get::<A>().unwrap();
                        return a.clone();
                    }
                },
                Err(_) => {}
            }
    
            match self.cache.try_write() {
                Ok(mut map) => {
                    if !map.contains::<A>() {
                        thread::sleep(time::Duration::new(4, 0));
                        let val = A { a: "a".into()};
                        map.insert::<A>(Arc::new(vec![val]));
                    }
                },
                Err(_) => continue,
            }
        }
    }
}

#[derive(Debug)]
struct A {
    a: String
}


#[derive(Debug)]
struct B {
    b: String,
}

impl Key for A { type Value = Arc<Vec<A>>; }
impl Key for B { type Value = Arc<Vec<B>>; }

    
fn main() {
    let (tx, rx) = mpsc::channel();

    let t = Arc::new(Test::new());

    for _ in 0..100 {
        let t = t.clone();  
        let tx = tx.clone();

        thread::spawn(move || {
            let r = t.get_a();
            tx.send(r).unwrap();
        });
    }

    for i in 0..100 {
        println!("{} - {:?}", i, rx.recv().unwrap());
    }

}
    
