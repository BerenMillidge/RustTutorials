extern crate rand;

use std::io;
use std::cmp::Ordering;

use rand::Rng; // Rng is apparently a trait - still not that sure about that - which defines methods that random number generators implement?

fn main() {
	println!("Guess the number!");

	println!("Please input your guess.");

	let secret_number = rand::thread_rng().gen_range(1,101); // rust has immutability by defualt - forbetter compiler optimizations?

	println!("Secret umber: {}", secret_number);

	loop {

	let mut guess = String::new();

	io::stdin().read_line(&mut guess).expect("failed to read line!"); // error handling?

	// switches to a match for error handling. this large focus on error handling from the beginning
	// seems to be a featureof rust, and it is avery good one!
	let guess: u32 = match guess.trim().parse() {
		Ok(num) => num,
		Err(_) => continue,
	};
	// this is an interesting error handling thing. it is a good point about the eixstence and importance of tooling
	// generally having the compiler tell you where there should be errors should realistically be super useful, but who realy knows?

	println!("You guessed: {}", guess);

	// now check it... this uses the special match syntax from functional languages
	match guess.cmp(&secret_number) {
		Ordering::Less => println!("Too small!"),
		Ordering::Greater => println!("Too big!"),
		Ordering::Equal => {
			println!("You win!");
			break;
		}
	}
}
}
// in rust you must declare function anotations in the function definition - this means that unlike java you rarely need them selsewhere
// does rust also have return types?
// you can specify their function type after an arrow - i.e. fn five() -> i32 or smoething... like that
// rust returns the lat value of corect type automatically. can also use return so who knows!
// statements are ended by semicolongs in rust, and evaluated expressoins, which can be return values aren't
// I do still just prefer the return convention though so it is clear, as well as the type annotation, just hwhat is being retuerned!

// the mainthing about rust is that it doesn't have a garbage collector but safely deals with memory and allows manual memory management - ideal for device drivers? amd osses
// since it does not have a runtime but also allows safety which is the key thing really1
// it is cool how so many models of memory management and the like can be used - so instead of manual memory allocation
// rust manages memory through a system of ownership with rules that can be checked at compile time!

// these are the ownershiprules... fairly straightforward
      //  Each value in Rust has a variable that’s called its owner.
      //  There can only be one owner at a time.
      //  When the owner goes out of scope, the value will be dropped.

 // hmm... so when reassigning heap variables, reassinging them devalidates them to prevent double free errors, which is cool
 // i.e. thisi s not allowed:
 let s1 = String::from("Hello!");
 let s2 = s1 ;
 println!(s1);
 // this is not allowed since 'ownership' of the heap allocated variable the string has been transferred to s1
 // and therefore s1 cannot be used again. s1 refrerence has been invalidated - rust automatically shallow copies
 // heap alocated variables, and obviously deep copies stack allocated variables.
 // it is clever how people haven't figured this out before rust... so who knows...
 // automatic data copying can be assumed to therefore be inexpensive, although you can deep copy and clone data
 // so you need to deep copy so do something likethis
 let s3 = s2.clone();
 println(s3)
 println(s2)
 // this is okay, because s2 has been deep copied, and thus not invalidated!
 // clone is a signal deep copying is occuring which my be runtime expensive, so be warned about that!
 // rust also has a trait system. automatically stack allocated variable shave the 'copy' tait which means automatic deep copying
 // and no automatic freeing since they are assumed to be on the stack, while heap objects implement the 'drop' trait
 // which then automatical frees it going out so who knows?
 // if you don't want a variable to go out of scope when passed to a function that then finished - which happens otherwise
 // you do a deep copy - i.e. you do a pass by reference and send in a reference instead!
 // which is the & syntax!
 // references allow you to pass the pointer without the new scope taking ownership of the oject referenced by the pointer. which makes a whole load of sense and is cool!
 // this means that references are and must bei mmutatble within their referenced functions - you can't reference it, change it,and then send it bac... that's not allowed since then the amount of memory would change
 // instead you'll need to replace it or something... at least by default.. you do have mutable references don't you , so that's god!
 // utable references are only allowed if the original variable is utable, which is obviously required!
 // also can only have one mutable reference to a particular piece of data in a particular scope.
 // this allows rust to prevent data races at cmopile time! which is good and allows much safer concurrency
 // rust also prevents dangling references at compile time which is really nice... so who knows!

