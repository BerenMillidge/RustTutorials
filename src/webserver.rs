// okay, the multithreaded webserver tutorial from scratch!!! this is realy cool and exciting... let's get on this!
/// including write the basic http server and thread pool manually which is wonderful for me as previously the ywere effectively
// 'magic'
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
//use std::fs::read_to_string;

fn my_handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();
    // reading presumably does not consume it as it's a read only operation

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    // now make a response. make the simplest response possible

    let simplestOk = "HTTP/1.1 200 OK\r\n\r\n";

    //get html response from file
    let filename = "hello.html";
    println!("about to open file!");
    let mut f = fs::File::open(filename).expect("File not found!");
    println!("opened file!");
    let mut contents=  String::new();
    let fileContents = f.read_to_string(&mut contents).expect("File read failed!");
    println!("{}", fileContents.clone());
    let fileResponse = format!("HTTP/1.1 200 OK\r\n\r\n {}", fileContents);
    println!("about to printn file response!");
    println!("{}",fileResponse.clone());
    stream.write(fileResponse.as_bytes()).unwrap(); //panic in case of failure
    stream.flush().unwrap(); // makes the program wiats until all bytes are written to the connection!
}

fn handle_connection_unrefactored(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // okay, so try to parse on get to see if that works
    // I have no idea why the string doesn't work!?
    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
	    let contents = fs::read_to_string("hello.html").unwrap();
	    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
	    stream.write(response.as_bytes()).unwrap();
	    stream.flush().unwrap();
	} else {
		// do something else!
		// okay, now try to return to a general error page!
		let status_line = "HTTP/1.1 404 NOT FOUND \r\n\r\n";
		let contents = fs::read_to_string("404.html").unwrap();
		let response = format!("{}{}", status_line, contents);
		stream.write(response.as_bytes()).unwrap();
		stream.flush().unwrap();
	}

}
//refactor to have more efficiency here!
fn handle_connection(mut stream: TcpStream) {
    // --snip--

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// it is pretty cool how simple a webserver can be to be honest... nothing here is too horrendously complicated
// similarly, I wonder if my eye tracking thing can come in with similarly low levels of complexity?

fn main() {
	let port = "127.0.0.1:7878";
	let listener = TcpListener::bind(port).unwrap(); // sett up connect listener. can't realy do anything else if fails!

	//then it gets an incoming steram it will print connection establisehd
	for stream in listener.incoming() {
		let stream = stream.unwrap(); // if this fials, probably should be error handler but who knows
		println!("Connection establisehd!");
		handle_connection(stream); // don't we need to pass in a reference... I suppose not... the stram gets consumed by thatfunction
		// but w don't want to use it later so it doesn't matter!
}
}

// this iscool... if you can you can implement async i/o model like in node in rust... which is cool
// maybe I should look into finding a way to do this... all it does it have one main thread and then sends off the workers
// to the other threads, but who knows? so at the moment ths uses a thread pool which is a good implementation!

// so you need to implement a thread pool, which is really cool and fun!

// a simple implementation without a threadpool would just spawn a thread for ach request. would look something like
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}


// so ideally we want some kind of interface like this:
// so we can start up and assign a poolwith N threads at the start, and then just executre a function
// which gets passed to a rhread. so this is the basic interface we want:

use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

// so let's implement a basic therad pool struct
// also need to implement this using channels, which could be fun. it is a cool implementation doing all of this
// and it is quite low level, which is really nice to see how to make it not magic
use std::sync::mpsc; 
use std::sync::Arc;
use std::sync::Mutex;


pub struct ThreadPool {
	threads: Vec<Worker>,
	sender: mpsc::Sender<Message>,
}

impl ThreadPool {
	pub fn new(size: usize) -> ThreadPool {
		assert!(size > 0);
		let mut threads = Vec::with_capacity(size);

		let (sender, receiver) = mpsc::channel();
		let receiver = Arc::new(Mutex::new(receiver));

		for i in 0..size {
			threads.push(Worker::new(i, Arc::clone(&receiver)));
		}
		ThreadPool {
			threads
		}
	}
	pub fn execute<F>(&self, f:F)
		where F: FnOnce() + Send + 'static {
			let job = Box::new(f);
			self.sender.send(Message::NewJob(job)).unwrap();

		}
	// imeplement a custom drop to handle gracefully and shut down

}

//send a mesasge to the therad either a closure or a termiante enum

enum Message {
	NewJob(Job),
	Terminate,
}


impl Drop for ThreadPool {
	fn drop(&mut self){ 
		for worker in &mut self.workers {
			println!("Shutting down worker {}", worker.id);

			for _ in &nut self.workers {
				self.sender.send(Message::Terminate).unwrap();
				// send a terminate message to all workers...
				// I suppose this is why threadpool implementations are generally cryptic and arcance!
			}

			if let Some(thread) = worker.thread.take() {
				thread.join().unwrap(); // rust can be pretty cryptic with all this stuff
				// I'd imagine it makes perfect sense when you get used to it though!
			}
			// wait for each worker to finsihe when it goes out of self!
		}
	}
}

struct Job;

// we also need to implement a worker behaviour to spawn the threads correctly... this is because
// currently the spawning doesn't give them anything to do
// and wewant the threads to wait to be given behaviou
// so create another worker instance!
// we can't do that though... since the mpsc model is single consumer multiple producers... and we want the opposite
// so a new idea is instead to share the receiver object via mutexes - so it uses all possible concurrency models concurrently(!)

pub struct Worker {
	id: u32,
	handle: Option<thread::JoinHandle<()>>
}
impl Worker {
	pub fn new(id: u32, receiver:: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
		// that is one amazing parametric type signature!
		let threadHandle = spawn::thread(move || {
			// loop forever!
			loop {
				let message = receiver.lock().unwrap().recv().unwrap();
				println!("Worker {} got a job; executing...", id);
				match message {
					Message::NewJob(job) => {
						(*job)();
					},
					Message:Terminate => {
						println!("Worker {} was told to terminate.", id);
						break; 
						// break out of the infinite loop!
					}
				}
			}
		}); // spawn a new thread with an emty closure
		return Worker {
			id,
			Some(threadHandle),
		}
	}
}

// and that's all folks... a pretty impressive rust webserver miplementation - this is cool... I wonder if it works interestingly or not?
// almost certainly not!