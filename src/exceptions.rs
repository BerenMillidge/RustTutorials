
use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
	let f = File::open("hello.txt");
	let f = match f {
		Ok(file) => file,
		Err(ref error) if error.kind() == ErrorKind::NotFound => {
			match File::create("hello.txt") {
				Ok(fc) => fc,
				Err(e) => {
					panic!("Tried to create file but failed {:?}", e)
				},
			}
		},
		Error(error) => {
			panic!("There was an unspecified problem opening file: {:?}", error)
		},
	};

	// this is a bit of a nightmare, but simply tries to create the file instead... who knows if this works?
}

// let's have a look at the exceptions again - this time to read a username from file
fn read_username_from_file() -> Result<String, io::Error> {
	let f = File::open("hello.txt");
	let mut f = match f {
		Ok(file) => file,
		Err(e) => return Err(e) // this propagates the exception upwards
	};

	let mut s = String::new(); // generate a new string
	match f.read_to_string(&mut s) {
		Ok(_) => Ok(s);
		Err(e) => Err(e);
	}
}

// we can also use the question mark operator to result immediately if there is an error, to propagate the error
// i.e.
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
// cn even chain together to make it a lot shorter... this is really good, although it adds a lot more mental overhead
// to parse what this function actually does initially, but with practice will probably become minimal


fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
