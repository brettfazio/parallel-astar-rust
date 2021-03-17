use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::mem::drop;

// Channel receiver types.
type RxVec = Option<Vec<Arc<Mutex<Receiver<Barrier>>>>>;
type Rx = Option<Arc<Mutex<Receiver<Barrier>>>>;

// Internal struct for channel receiver.
struct BarrierRecv
{
	receivers: RxVec,
	receiver: Rx,
}

// Signal to send in channels at wait and exit.
#[derive(PartialEq)]
enum Barrier
{
	AtCheckPoint,
	Exit,
}

// Structure for dynamic barrier.
pub struct DynamicHurdle
{
	tx: Vec<Sender<Barrier>>,
	rx: BarrierRecv,
	threads: usize,
	count: usize,
	cur: isize,
}

// Dynamic Barrier implementation for synchronizing threads.
impl DynamicHurdle
{
	// Creates channel for the given given number of threads.
	// Returns new barrier for the main thread.
	pub fn new(threads: usize) -> DynamicHurdle
	{
		let mut receivers: Vec<Arc<Mutex<Receiver<Barrier>>>> = Vec::with_capacity(threads);
		let mut transmitters: Vec<Sender<Barrier>> = Vec::with_capacity(threads);
		
		for _ in 0..threads
		{
			let (tx, rx) = channel::<Barrier>();

			transmitters.push(tx);
			receivers.push(Arc::new(Mutex::new(rx)));
		}

		DynamicHurdle
		{
			tx: transmitters,
			rx: BarrierRecv { receivers: Some(receivers), receiver: None },
			threads,
			count: 0,
			cur: -1,
		}
	}

	// Clones barrier from the main thread and increments
	// internal counter representing the current thread number.
	pub fn create(&mut self) -> DynamicHurdle
	{
		self.cur += 1;
		self.clone()
	}

	// Forces current thread to wait for all other threads in the original
	// count, minus the threads that have called `exit`.
	pub fn wait(&mut self)
	{
		let rx = self.rx.receiver.as_ref().unwrap();
		self.count = 0;
	
		for tx in &self.tx
		{
			tx.send(Barrier::AtCheckPoint).ok();
		}

		loop
		{
			if self.count >= self.threads
			{
				break;
			}

			match rx.lock().unwrap().recv()
			{
				Ok(barrier) =>
				{
					if barrier == Barrier::AtCheckPoint
					{
						self.count += 1;
					}
					else
					{
						self.threads -= 1;
					}
				},
				Err(_) => continue
			}
		}
	}

	// Exits current thread and let all other threads know.
	pub fn exit(&self)
	{
		for tx in self.tx.clone()
		{
			match tx.send(Barrier::Exit)
			{
				Ok(_) => drop(tx),
				Err(_) => continue
			}
		}
		println!("Exiting in Dynamic Barrier");
	}
}

// Clones barrier with correct receiver for child thread.
impl Clone for DynamicHurdle
{
	fn clone(&self) -> Self
	{
		let receivers = self.rx.receivers.as_ref();

		DynamicHurdle
		{
			rx: BarrierRecv { receivers: None, receiver: Some(receivers.unwrap()[self.cur as usize].clone()) },
			tx: self.tx.clone(),
			cur: self.cur,
			..*self
		}
	}
}

// impl Drop for DynamicHurdle {
// 	fn drop(&mut self)
// 	{
// 		self.exit();
// 	}
// }