use std::{thread, sync::{mpsc::{self, Receiver}, Mutex, Arc}};

pub struct ThreadPool{
    threads: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

enum Message{
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool{
    pub fn new(size: usize) -> ThreadPool{ 
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let mutex_receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&mutex_receiver)));
        }

        ThreadPool{
            threads: workers,
            sender,
        }
    }
    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self){
        println!("Sending terminate message to all workers.");
        for _ in &mut self.threads{
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");
        for worker in &mut self.threads{
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
                println!("Worker {} is down", worker.id);
            }
        } 
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker{
    fn new(id: usize, job_receiver: Arc<Mutex<Receiver<Message>>>) -> Worker{
        let thread = thread::spawn(move || 
            loop{
                let message = job_receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} received a job; Executing...", id);
                        job();
                    }
                    Message::Terminate =>{
                        println!("Worker {} was told to terminate", id);
                        break;
                    }
                }
            });
        Worker{
            id,
            thread:Some(thread),
        }
    }
}
