// this one is important because concurrency is an important part of why rust is so cool,
// and it is so necessary for creating real time streaming i/o os systems and other systems that do this within one computer
// this is one place where rust and julia's concurrency models differ and rusts is better - julia assumes distribted concurrency
// mostly - i.e.e you're running one program on many compuers in a datacenter. Rust assumes multiple programmes on one computer
// one computer has multiple streams of real time i/o data it needs to handle, like an os, and that's why it'sso important!
// yeah, so it is true, ownership and type checking and memory safety make concurency much easier to reason about
// which is necessary, as it automatically prevents various race conditions from happening!
// rust has multiple parralel ways essentially all ways to implement concurrency possible as it is a low level language
// these include - threads, mesasge passing concurrency, shared state concurrency, and sync and send types
// including often you don't need concurrency/huge distributed systems if instead you have one big efficient one!

// so first things first... threading:
// threading has issues because you can't guarantee the orde in which your program execute saymore - leads to race conditions, deadlocks,
// and heisenbugs.
// so two different ways to implement threading - 1.) direct to OS - i.e. OS provides a threading api, 1 programming language thread
// to one OS thread - this is 1:1.
// or alternaitvely language runtime environment can create threads themselves - and run M threads on N OS threads
// M:N model. - also knows as green threads
// so yeah, so Rust tries to have a really small runtime, since it is required for OS programmign where it simplycan't hae a runtime
// so M:N model needs a largerlanguage runtime to manage threads, so standard library only provides 1:1 threading. by default
// but creates for M:N threads...

// threading;

use std::thread;
use std::time::Duration;
/*
fn main() {
	// this is a closure!
	let handle = thread::spawn(|| {
		for i in 1..10 {
			println!("number {} from spawned thread", i);
			thread::sleep(Duration::from_millis(1));
		}
	});
	for i in 1..5 {
		println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
	}
	handle.join().unwrap();
}
*/
// so the trouble is, the spawned thread dies the moment the main thread is finished, even if it is not done...
// you can fix this by saving the handle into a variable and then using the join method in it, which waits
// for it's thread to finish!
// so therads are fairly straightforward in this but have a lot of complexity in getting stuff correct.
// so instead you can use message passing whereby therads communicateto each other only through messaging instead of 
// shared memory which can cause all sorts of race conditions and other difficulty! - it's the new thing, from the arising
// of languages like erlang in the general consciousness!
// so rust does this through a channel model. i.e. so , messages are sent through channels. a channel has two halves
// a transmitter and a receiver. transmitter sends messages -i.e. thread calls methods on the transmitter with the data
// and the other thread checks for arriving messages - channels is closed if one half drops out.
// can obviously be used to make chat systems or things like it in rust which makes sense!

use std::sync::mpsc;

// just create a channel
fn main() {
	let (tx, rx) = mpsc::channel(); // creates a trnsmitter and receiver!
	// mpsc i smultiple producer single consumer - i.e. multiple sending ends but only one receiving end!
	// so lets move transmitter into another thread to send back to main thred
	thread::spawn(move || {
		let val = String::from("hi!");
		tx.send(val).unwrap();
	});

	// then receive the message
	let received = rx.recv().unwrap();
	println!("Got {}", received);
}

// you can create multiple producers by just cloning the transmitter!

fn main() {

let (tx, rx) = mpsc::channel();

let tx1 = mpsc::Sender::clone(&tx);
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}
}

// that is fairly straightforward and generally rusts guarantees around ownership help to remove some of the issues
// and concurrent errors that can crop up with this!

// the alternatie is shared memory concurrency which you can also do in rust, and where rusts rules help again!
// to do shared memory right, to try to avoid interference and various race conditions you instead use mutexes
// these essentially ensure that only one thread can write to the data at a time or read from it!
// to do this it essentially locks the data... to use the mutex you need to:
// 1.) attempt to aquirethe lock before using the data
// 2.) once done you've must unlock the mutex again so other threads can use it 
// which is really nothard just a lot of annoying bookkeeping.

//rust does this with the muetx type

use std::sync::Mutex;

fn main() {
	let me = Mutex::new(5); // stores data 'five' in a mutex

	{ 
		let mut num = m.lock.unwrap();
		// unlock the mutex
		*num = 6;
		//overwrite data to 6
	}
	println!("m = {:?}", m);
}
// lock release happens automatically in rust when the mutex guard goes out of scope and is drpped!

// now let's try to have 10 threads implement a counter

use std::rc::Rc;

fn main() {
	let counter = Rc::new(Mutex::new(0));
	let mut handles = vec![];

	for _ in 0..10 {
		let counter = Rc::clone(&counter);
		let handle = thread::spawn(move || {
			let mut num = counter.lock().unwrap();
			*num +=1;
		});
		handles.push(handle);
	}
	for handle in handles {
		handles.join().unwrap();
	}
	// makes sure all handles run to completino instead of just ending when the main thread does!

	println!("Result : {}", *counter.lock().unwrap()); // get the data from the mutex here!
}

// the trouble is here we've giving the mutex mnultiple owners, one for each thread, andgenerally that isn't possible
// we need to use the smart pointer Rc<T> to solve this!
// the trouble with this is that Rc is not thread safe. it's own updatescan be interrupted by other threads...
// how does one deal with this? by using an atomic reference coutning type which is thread safe!
// in return for thread safety there is more overhead here of using the concurrency primitives, so that is an issue!

// using atomic Rc
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// mutexes aren't be all and end all. and aren't completely safe. deadlocks are still very possible to happen, so 
// you need to find out deadlock mitigation strategies and how you might solve it!
// also in rust ll of the concurrency features are implemented in rust via the standard library and not hardbaked into the language
// this flexibility means that you can create your own concurrency features as you want!
// anyhow, it has syn and send trats in the language!

// send marker trait indicates that ownership of type implementing send can be transferred between threads
// almost every rust type is send!
// sync is that the trait /object is thread safe, and so can be called at the same time from multiple traits!