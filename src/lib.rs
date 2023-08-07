// threadpool_sub made on github.com

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

// mod thread_pool;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(0 < size);
        let (sender, reciever) = mpsc::channel();
        let reciever = Arc::new(Mutex::new(reciever));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    // fn new(id: usize, reciever: mpsc::Receiver<Job>) -> Worker {
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            reciever;
        });
        Worker { id, thread }
    }
}
