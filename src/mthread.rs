// This module contains a thread pool. 

use std::thread;

// Worker is an unit for executing given task 
#[allow(unused)]
struct Worker {
	id: usize, 
	thread: Option<thread::JoinHandle<()>>, 
}

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{
	Receiver, SendError, 
};

type Job = Box<dyn FnOnce() + Send + 'static>;
type Result = std::result::Result<(), SendError<Signal>>;

#[allow(unused)]
pub enum Signal {
	Task(Job), 
	Terminate, 
}
use self::Signal::Task;
use self::Signal::Terminate;

impl Worker {
	pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Signal>>>) ->Worker {
		let thread = thread::spawn(move || {
			loop {
				let signal = receiver.lock().unwrap().recv().unwrap();

				match signal {
					Task(job) => job(), 
					Terminate => break, 
				};
			}

			// log::info!("worker {} exit", id);
			println!("worker {} exit", id);
		});

		Worker { id, thread: Some(thread)} 
	}
}

use std::sync::mpsc::Sender;

pub struct ThreadPool {
	_threads: Vec<Worker>, 
	sender: Sender<Signal>, 
}

const MAX_THREAD_NUM: usize = 16usize;

use std::sync::mpsc;

impl ThreadPool {
	pub fn new(size: usize) ->ThreadPool {
		// make sure size is valid 
		assert!(size > 0 && size < MAX_THREAD_NUM);

		// initialize sender/receiver channel
		let (sender, receiver) = mpsc::channel();
		let receiver = Arc::new(Mutex::new(receiver));

		// initialize Worker vector 
		let mut threads = Vec::with_capacity(size);
		for id in 0..size {
			let worker = Worker::new(id, Arc::clone(&receiver));

			threads.push(worker);
		}

		ThreadPool { _threads: threads, sender}
	}

	pub fn execute<F>(&mut self, f: F) ->Result
		where F: FnOnce() + Send + 'static
	{
		let job = Box::new(f);

		self.sender.send(Task(job))
	}

	pub fn terminate(&mut self) ->Result {
		for _ in &self._threads {
			self.sender.send(Terminate).unwrap();
		}

		for worker in &mut self._threads {
			if let Some(thread) = worker.thread.take() {
				thread.join().unwrap();
			}
		}

		Ok(())
	}
}

// I think that ThreadPool will never be dropped, since we'll run 
// it continuously in main() until SIGINT is received. 
impl Drop for ThreadPool {
	fn drop(&mut self) {
		self.terminate().unwrap();
	}
}