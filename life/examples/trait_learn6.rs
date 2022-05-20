use std::{
    cell::RefCell,
    fmt::Error,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn arc_mutex<T>(t: T) -> Arc<Mutex<T>> {
    Arc::new(Mutex::new(t))
}

fn rc_ref<T>(t: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(t))
}

fn arc_mutext_is_send_sync() {
    let a = arc_mutex(1);
    let b = a.clone();
    let c = a.clone();
    for _ in 0..100 {
        let c = c.clone();
        thread::spawn(move || {
            let mut g = c.lock().unwrap();
            *g += 1;
            let c2 = c.clone();
            drop(g);
            let h = thread::spawn(move || {
                let mut g = c2.lock().unwrap();
                *g += 1;
            });
            h.join().unwrap();
        });
    }
    {
        let mut g = b.lock().unwrap();
        *g += 1;
    }
    thread::sleep(Duration::from_secs(1));
    // handle.join().unwrap();
    println!("a={:?}", a);
}

fn main() {
    arc_mutext_is_send_sync();
}
