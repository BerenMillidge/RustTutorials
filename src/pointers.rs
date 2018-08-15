// so references in rust  i.e. &x are effectively pointers, and they just refer to
// there are also smart pointers which have additional metadata and provide additional fucntionality

// so, some smart pointer examples - Box<T> puts normal stack data on the heap and the stack just contains a point
// to the heap data... doesn't really have any overhead except for being in the heap

// the final product is a multithreaded webservice effectively from scratch... which is really cool
// and definitely useful to see how that works. that's really awesome to try to do and see if it would be interesting
// so who knows! could easily turn that into a separate repository to see how it is!

// for instance, suppose you want to store 5 - a i32 on the heap, you do

fn main() {
	let b = Box::new(5);
	println!("b = {}", b);
}
// this is pretty useless, but boxes generaly allow us to define other types that can't be implemented otherwise!
