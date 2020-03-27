use std::sync::{Arc, Condvar, Mutex};

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let mutex = Mutex::new(None);
    let cond_var = Condvar::new();
    let arc = Arc::new((mutex, cond_var));
    (Sender { pair: arc.clone() }, Receiver { pair: arc })
}

pub struct Sender<T> {
    pair: Arc<(Mutex<Option<T>>, Condvar)>,
}

pub struct Receiver<T> {
    pair: Arc<(Mutex<Option<T>>, Condvar)>,
}

impl<T> Sender<T> {
    pub fn send(&self, t: T) -> Result<(), ()> {
        let (mutex, condvar) = &*self.pair;
        let mut m = mutex.lock().unwrap();
        *m = Some(t);
        condvar.notify_one();
        Ok(())
    }
}

impl<T> Receiver<T> {
    pub fn recv(&self) -> Option<T> {
        let (mutex, condvar) = &*self.pair;
        let mut m = mutex.lock().unwrap();
        while !m.is_some() {
            m = condvar.wait(m).unwrap();
        }
        m.take()
    }
}
