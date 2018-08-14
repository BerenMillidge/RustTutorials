// some things to see how closures areimplemented - Im' not totally sure on the theory of these - they come from 
// functional languages in any case

use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
	println!("Calculating slowly...");
	thread::sleep(Duratoin::from_secs(2));
	intensity;
}


fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}

// so what is a closure... a closure stores a result in a variable, which only has to be called once, and can be evaulated at any other point
// but then once evaluated the result i sstored in that variable without having to be evaluated again, this allows essentially smoething to be only evluated the first time it is needed
// and saved thereafter. that's really clever!

// closures are just defined using the straight parantheses (i.e. pipe!) syntax

let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
// can add type annotations to closures ify ou want



let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};

fn main() {
	let simulated_user_specified_value = 10;
	let simulated_random_number = 7;

	generate_workout(
		simulated_user_specified_value,
		simulated_random_number);
}
