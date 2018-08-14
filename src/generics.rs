// okay, now for generics!
// so generics obviously just stop you having to write silly multiple functions with the same body but with different types
// I think julia's multiple dispathc handles this a whole lot better, but hey it's going for the C++/Java tradition here!

// generic list function
// generic type T

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
	let mut largest = list[0];

	for &item in list.iter() {
		if item > largest {
			largest = item;
		}
	}
	return largest;

}

// but this won't immediately work. what about possible types which don't have ordering possibilities
// this means we will need to define a trait. a trat is just a property or tag of a type that it implements the required
// functionality, in this case order comparisons which you can implement more on traits!

// can also define generic structs
struct Point<T> {
	x: T,
	y: T,
}

// or different types obviously
struct Point<T,U> {
	x: T,
	y: U,
}

// can define generic methods too
impl<T,U> Point<T,U>{
	fn mixup<V,W> (self, other: Point<V,W>) -> Point<T,W> {
		Point {
			x: self.x,
			y:other.y,
		}
	}
}

// also in enum definitions
enum Option<X,Y> {
	x(X),
	y(Y)
}

// rust generics aren't slower since it does monomorphisation of the code at compile time and not runtime

// so, a types behaviour is just the methods callable on that type. certain methods can be abstracted and callable
// as traits.. i.e. comparable, numeric and so forth. these are traits

// a trait is just essentially a contract that a type or struct if it implements the trait will have certain methods
// it's an interface in java. implement as follows

pub trait Summary {
	fn summarize(&self) -> String; // this, like an interface, just defines the methods the trait must use
	// all types implementing summary trait must have their own custom implementation of summarize method
	// this is checked by the compiler at compile time!
}

// traits are implemented on types by using the impl <Trait> for <Type> syntax. i.e.

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/// can also implement a default method for the trait, just in the initial trait definition block
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// we can define in method parameter that an argument needs a specific trait for the type in something like:
// use the impl trait syntax
pub fn notify(item: impl Summary) {
	println!("Breadking news! {}", item.summarize());
}

// can be used to generif suf also
// i.e. 
pub fn notify_generic<T: Summary>(item: T) {
	println!(item.summarize());
}
// can add multiple traits with +  - i.e. 

fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
	...
}

// can simplify presentation using where clause:
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{

...}
// ca nalso use impl trait in return type as well, as expected!

// another kind of generic, which is seemingly unique to rust is lifetimes
// how does the rust borrow checker work?
// basically rust prevents dangling references y lifetimes, and essntially it does not allow a vriable to take a pointer or reference
// from a variable with a shorter lifetime, this is to prevent the other variable being deallocated, and thus essentially a null pointer
// developing. the lifetime is simply the length of closure execution.
// you can set the lifetime of seome varibales to be the same as the lifetime as others to overwrite this default borrow chcking behvaiour
// by using apostaphe syntax

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// the lifetime extended variable will then be deallocated when the extended lifetime is up!