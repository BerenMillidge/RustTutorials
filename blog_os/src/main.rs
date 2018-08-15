// this is going to be the main function starting to create a free standing binary which can be our initial os kernel
// of course it's going to be significantly more complex in reality, but creating these sorts of simple projects
// can be really fun!

// so rust needs to be pared down to its minimal runtime. this principally means no standard library
// since most of those functions depend upon the standard runtime environment...
// so let's try to operate this!

#![feature(panic_implementation)]
#![no_std] // removes standard library support(!)
#![no_main] /// use our own entrypoint for the thing!

// without the standard library there is no panic function for what to do when the program panics
// so you need to define that function yourself, as well as turning off the standard stack unwinding procedure
// which is necessary since that once again assumes already the existence ofan os

use core::panic::PanicInfo;
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
	loop {}
}
// creates a panic function which should never return (marking by the never type (!), and just loops forever!

// so, how does it work. Rust binary that links standard library - executino starts in a C runtime library called crt0
// C runtime zero which sets up environment for C... i.e. creating a stack and placing arugments in right registers
// then it envokes the entry point of the rust tuntime, which is marked by the start language item
// which can be found here https://github.com/rust-lang/rust/blob/bb4d1491466d8239a7a5fd68bd605e3276e97afb/src/libstd/rt.rs#L32-L73 -
// it's so cool you can actuall ysee the start of the rust runtime... that is fantastic
// although it immediately importats a whole bunch of stuff!
// runtime then calls the main function
// however the freestanding binary executable does not have accesss to rust runtimeor crt0 so it needs to devol
// own enty point, which is fantastic!
// need to add no main attribute!

// so use the linux entry point - default entry point is called start... which is what we ues for our kernel
// but I don't nkow since we don't even have a kernel to define in the fist place.. so who knows
// but let's try this

#[no_mangle]
pub extern "C" fn _start() -> ! {
	loop {}
	// another non returning system
	// also extern C to use C caling convention instead of currently unknown rust conention!
}

// that is pretty reasonable... start should be freestanding and not do too much... now to create the rest of the kernel
// which is incredibly tiny... so who knows how it should work!
// so ideally this kernel code should be launched immediately from the bootloader which will immediately define start