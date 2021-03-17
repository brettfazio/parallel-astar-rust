use crossbeam::channel::{unbounded, Sender, Receiver};

// Internal struct for channel receiver.
struct BarrierRecv
{
	receivers: Option<Vec<Receiver<Barrier>>>,
	receiver: Option<Receiver<Barrier>>,
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
		let mut receivers: Vec<Receiver<Barrier>> = Vec::with_capacity(threads);
		let mut transmitters: Vec<Sender<Barrier>> = Vec::with_capacity(threads);
		
		for _ in 0..threads
		{
			let (tx, rx) = unbounded();

			transmitters.push(tx);
			receivers.push(rx);
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
		if (self.cur as usize) >= self.threads
		{
			panic!("Exceeded number of threads");
		}

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

			match rx.recv()
			{
				Ok(barrier) =>
				{
					match barrier
					{
						Barrier::AtCheckPoint => self.count += 1,
						Barrier::Exit => self.threads -= 1,
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
			tx.send(Barrier::Exit).ok();
		}
	}
}

// Clones barrier with correct receiver for child thread.
impl Clone for DynamicHurdle
{
	fn clone(&self) -> Self
	{
		let receivers = self.rx.receivers.as_ref().unwrap();

		DynamicHurdle
		{
			rx: BarrierRecv { receivers: None, receiver: Some(receivers[self.cur as usize].clone()) },
			tx: self.tx.clone(),
			cur: self.cur,
			..*self
		}
	}
}

impl Drop for DynamicHurdle
{
	fn drop(&mut self)
	{
		self.exit();
	}
}