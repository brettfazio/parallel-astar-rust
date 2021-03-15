use std::sync::mpsc::{channel, Sender, Receiver};
use std::clone::Clone;
use std::sync::{Arc, Mutex};
use std::mem::drop;

#[derive(PartialEq)]
enum Barrier {
	AtCheckPoint,
	Exit
}

pub struct DynamicBarrier {
	tx: Vec<Sender<Barrier>>,
	receivers: Vec<Arc<Mutex<Receiver<Barrier>>>>,
	rx: Arc<Mutex<Receiver<Barrier>>>,
	threads: usize,
	cur: isize,
	count: usize,
}

impl DynamicBarrier {
	pub fn new(threads: usize) -> DynamicBarrier {
		let mut receivers: Vec<Arc<Mutex<Receiver<Barrier>>>> = Vec::with_capacity(threads);
		let mut transmitters: Vec<Sender<Barrier>> = Vec::with_capacity(threads);
		
		for _i in 0..threads {
    		let (tx, rx) = channel::<Barrier>();

    	 	transmitters.push(tx.clone());
    		receivers.push(Arc::new(Mutex::new(rx)));
    	}

		DynamicBarrier {
			receivers,
			tx: transmitters,
			threads: threads - 1,
			count: 0,
			rx: Arc::new(Mutex::new(channel::<Barrier>().1)),
			cur: -1,
		}
	}

	fn wait(&mut self) {
	    let rx = self.rx.lock().unwrap();
		self.count = 0;
	
		for tx in &self.tx {
			match tx.send(Barrier::AtCheckPoint) {
				_ => continue
			}
		}

		loop {
			match rx.recv() {
				Ok(barrier) => if barrier == Barrier::AtCheckPoint { self.count += 1 } else { self.threads -= 1 },
				Err(_) => continue
			}

			if self.count >= self.threads {
				break;
			}
		}
	}

	fn exit(self) {
		for tx in &self.tx {
			match tx.send(Barrier::Exit) {
				Ok(_) => drop(tx),
				Err(_) => continue
			}
		}
	}
}

impl Clone for DynamicBarrier {
	fn clone(&self) -> Self {
	    // Must wrap receivers in a mutex
	    let rx = self.receivers[(self.cur + 1) as usize].clone();
	    
		DynamicBarrier {
			rx,
			tx: self.tx.clone(),
			cur: self.cur + 1,
			..*self
		}
	}
}