use std::{
    borrow::Cow,
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use lazy_static::lazy_static;

type MutexHashMapCow = Mutex<HashMap<Cow<'static, str>, usize>>;

lazy_static! {
    static ref METRICS: MutexHashMapCow = Mutex::new(HashMap::new());
}

fn arc_mutex_hashmap_cow() -> Arc<MutexHashMapCow> {
    Arc::new(Mutex::new(HashMap::new()))
}

fn main() {
    let metrics: Arc<MutexHashMapCow> = arc_mutex_hashmap_cow();
    for _ in 0..32 {
        let m = metrics.clone();
        thread::spawn(move || {
            let mut g = m.lock().unwrap();
            let data = &mut *g;
            let entry = data.entry("hello".into()).or_insert(0);
            *entry += 1;
        });
    }
    thread::sleep(Duration::from_millis(100));
    println!("metrics: {:?}", metrics.lock().unwrap());
}
