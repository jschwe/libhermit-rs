#![no_std]
#![no_main]
//#![test_runner(hermit::test_runner)]
//#![feature(custom_test_frameworks)]
//#![reexport_test_harness_main = "test_main"]

//use core::panic::PanicInfo;
extern crate hermit;
use hermit::{print, println};

//ToDO: Find out how the runtime_entry function works
// and why it works in rusty-demo without being defined (maybe part of std?)


//#[test_case]
#[no_mangle]
pub fn main(argc: isize, argv: *const *const u8) {
	println!("hey we made it to the test function :O");
	hermit::sys_exit(-1);
}

// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
// 	hermit::test_panic_handler(info)
// }
