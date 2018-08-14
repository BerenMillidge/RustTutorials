// so, an iterator just lets you perform an action on each item of collectoin in turn
// does logic of iterating over item and determining when sequence has fniished
// you don't reimplement that logic yourself
// iterators are lazy - have no effect until you call methods to consume them!
// all iterators implement a trait called Iterator!

// defined like:
trait Iterator {
	type Item;

	fn next(&mut self) -> Option<Self::Item>;
}
// essentially iterators only have one method - the next method, which then returns one item of the iterator wrapped in some
// and thenn at the end return NOne
// iter only provides immutable references - if you want to take ownership and returns owned values then use into_iter
// also iterate over mutable references call iter_mut

// iterators have consuming methods - which use them up and dereference them so they can't be called again, and adaptors
// which map them into different ones so by chaining you can create complex iterators!
// i.e. use a map iterator to get map function

let v1: Vec<i32> = vec![1,2,3];
let result = v1.iter().map(|x| x+1).collect(); // collect is a consuming method, map is an adaptor method!

// similarly filter can be implemented in this way!

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}

// you can obviously define your own iterator - for instance a simple counter to cont from 1 to 5

struct Counter {
	count: u32,
}

impl Cotner {
	fn new() -> Counter {
		Counter { count: 0 }
	}
}

impl Iterator for Counter {
	type Item = u32;

	fn next(&mut self) -> Option <Self::Item> {
		self.count+=1;
		if self.count < 6 {
			Some(self.count)
			// if less thna 6 return next
		}
		else {
			None
			// if greater stop  returning!
		}
	}
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}
// now you can use iterator with any kind of standard method... like this!
// it is cool how these function compositions can be made, like this so who knows!
// since the standard methods just implement next... which is nice!