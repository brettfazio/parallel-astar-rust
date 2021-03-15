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
	rx: Option<Arc<Mutex<Receiver<Barrier>>>>,
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

    	 	transmitters.push(tx);
    		receivers.push(Arc::new(Mutex::new(rx)));
    	}

		DynamicBarrier {
			receivers,
			tx: transmitters,
			threads: threads - 1,
			count: 0,
			rx: None,
			cur: -1,
		}
	}
	
	fn create(&mut self) -> DynamicBarrier {
		self.cur += 1;
		self.clone()
	}

	fn wait(&mut self) {
		let rx = self.rx.as_ref().clone();
		self.count = 0;
	
		for tx in &self.tx {
			tx.send(Barrier::AtCheckPoint).ok();
		}

		loop {
			match rx.unwrap().lock().unwrap().recv() {
				Ok(barrier) => if barrier == Barrier::AtCheckPoint { self.count += 1 } else { self.threads -= 1 },
				Err(_) => continue
			}

			if self.count >= self.threads {
				break;
			}
		}
	}

	fn exit(self) {
		for tx in self.tx.clone() {
			match tx.send(Barrier::Exit) {
				Ok(_) => drop(tx),
				Err(_) => continue
			}
		}
	}
}

impl Clone for DynamicBarrier {
	fn clone(&self) -> Self {
		let rx = self.receivers[self.cur as usize].clone();

		DynamicBarrier {
			rx: Some(rx),
			tx: self.tx.clone(),
			cur: self.cur,
			receivers: vec![],
			..*self
		}
	}
}