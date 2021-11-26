//! Based on The Rust book's implementation of a Thread Pool and workers.  https://doc.rust-lang.org/book/ch20-02-multithreaded.html

use std::{fmt, thread};
use std::sync::{mpsc, Arc, Mutex};
use std::fmt::{Debug};

/// Thread pool struct for managing a pool of worker and distributing workloads.
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError>{
        if size < 1 {
            return Err(PoolCreationError);
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        return Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static,  {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {

            if let Some(thread) =  worker.thread.take(){
                  thread.join().unwrap();
            };
            //println!("worker {}, has successfully shut down", worker.id)
        }
    }
}

#[derive(Debug)]
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to create Pool")
    }
}



struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker{
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn( move ||  {
            //println!("Thread {} is alive and ready to receive work", id);
            loop {
                let message = receiver.lock().expect("Worker Cannot obtain lock, the mutex might be poisoned").recv().expect("The sender is no longer available");
                match message {
                    Message::NewJob(job) => {
                        //println!("worker {} performing new task", id);
                        job()
                    }
                    Message::Terminate => {
                        break;
                    }
                }
            }
        });
        Worker{id, thread: Some(thread)}
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message{
    NewJob(Job),
    Terminate,
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn thread_pool_test(){
        let pool = ThreadPool::new(10).unwrap();

        for i in 0..100 {
            pool.execute(move || {
                let  _o = i + i;
            })
        }
    }

}