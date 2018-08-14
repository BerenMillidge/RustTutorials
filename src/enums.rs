// so just some stuff abuot enums
// power of mathc comes from its pattern matching syntax and the fact that compiler ensures that all eventualities are coered - which is a very nice guarantee

// i.e. something like
enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
	match coin {
		Coin::Penny => 1
		Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
	}
}
// compiler on match ensures all attributes are covered!

// option<T> is a generic that allows matching with virtually any sort of type
// also is nullable - and defines stuff like:
// matches have placeholder _ - i.e. like in haskell! for all other cases, so that's nice!
enum Option<T> {
	Some(T),
	None,
}
// it is really that simple, and included in the prelude!

// if let is like a match just for a single value - i.e.
// for instance suppose you want to match some value to 3 - why not use if?
// anyhow
let val = Some(0u8);
match val {
	Some(3) => println!("Three!"),
	_ => (),
}
// this is using standard match format, but is verbose. instead use if let cosntruct

if let Some(3) = val {
	println!("Three!");
}
// which is much less verbose! it's just syntactic sugar for the first one though!
// structs and enums are quite simple and that is nice and good - yay!

// result enum is generally defined to handle normal exceptions that are recoverable from. defined as follows
enum Result<T,E> {
	Ok(T),
	Err(E),
}

// suppose you want to match some file! and you want to do robust error handling, you need to match on the types
// and this stuff is hideously verbose... I'm not really likeing this so much... so let'sgo

// the ? operator can only be used in functinos that return Result, whic makes sense!