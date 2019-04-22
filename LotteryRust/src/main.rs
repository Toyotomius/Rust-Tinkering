mod lotto649;
mod lottoexample;

use lotto649::lotto649;
use lottoexample::lottoexample;
use std::thread;
use std::time::Instant;

fn main() {
	let start = Instant::now();

	// Create vector to hold thread children that are spawned
	let mut children = vec![];
	
	
	for i in 0..30 {
		// Spin up a new thread for each index of loop. Pass index to function for tracking.
		children.push(thread::spawn(move || {
			lotto649(&i);
		}));
		children.push(thread::spawn(move || {
			lottoexample(&i);
		}));
	}

	for child in children {
		let _ = child.join();
	}

	println!(
		"(non-Handles) Total time elapsed: {} ms",
		start.elapsed().as_millis()
	);

	// 	let handles = (0..30)
	// 	.into_iter()
	// 	.map(|i| {
	// 		println!("Submitted {}", i);
	// 		thread::spawn(move || lotto649(i))
	// 	})
	// 	.collect::<Vec<_>>();

	// handles.into_iter().for_each(|h| h.join().unwrap());
}
