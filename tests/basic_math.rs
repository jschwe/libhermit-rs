#![feature(test)]
//#![feature(start)]
#![no_std]
#![no_main]
#![test_runner(test_runner)]
#![feature(custom_test_frameworks)]
#![feature(start)]
//#![cfg_attr(target_os = "hermit", reexport_test_harness_main = "test_main")]
#![reexport_test_harness_main = "test_main"]

/// Regarding `#[test]` and `#[test_case]` this comment explains the current implementation
/// https://github.com/rust-lang/rust/issues/50297#issuecomment-524180479
/// This is of course subject to change, since the whole feature is not stable
///
//extern crate hermit;
//extern crate x86_64;

//#[macro_use]
//extern crate float_cmp;
//mod common;
//use common::*;
//use core::hint::black_box;
// Either use black_box from core::hint or the value_fence definition
// core hint is a nop, but possibly only prevents dead code elimination
// value_fence has higher overhead but should be a bit safer regarding preventing optimizations
/*pub fn black_box<T>(x: T) -> T {
	common::value_fence::<T>(x)
}*/
use hermit::{print, println};
pub trait Testable {
	fn run(&self) -> ();
}

impl<T> Testable for T
where
	T: Fn(),
{
	fn run(&self) {
		print!("{}...\t", core::any::type_name::<T>());
		self();
		println!("[ok]");
	}
}

pub fn test_runner(tests: &[&dyn Testable]) {
	println!("Running {} tests", tests.len());
	for test in tests {
		test.run();
	}
	exit(false);
}

#[test_case]
fn add1() {
	let x = 1 + 2;
	assert_eq!(x, 3);
}

//#[test]
/*
fn test_f64_arithmetic() {
	let x = black_box::<f64>(65.2);
	let y = black_box::<f64>(89.123);
	let z = x * y;
	assert!(approx_eq!(f64, z, 5810.8196f64, ulps = 1));
	let z = z * y;
	assert!(approx_eq!(f64, z, 517877.6752108f64, ulps = 1));
	let z = z * y;
	assert!(approx_eq!(f64, z, 46154812.047812128f64, ulps = 2));
	let z = z * y;
	assert!(approx_eq!(f64, z, 4113455314.137160319f64, ulps = 3));

	let z = black_box(z) / y;
	assert!(approx_eq!(f64, z, 46154812.047812128f64, ulps = 2));
	assert!(!approx_eq!(f64, z, 46154812.047812128f64, ulps = 1)); // If we haven't lost any precision, the something is fishy

	let z = black_box(z) / y;
	assert!(approx_eq!(f64, z, 517877.6752108f64, ulps = 2));
	assert!(!approx_eq!(f64, z, 517877.6752108f64, ulps = 1));

	// Division
	let x = black_box::<f64>(4.0);
	let y = black_box::<f64>(5.0);
	let z = x / y;
	assert!(approx_eq!(f64, z, 0.8f64, ulps = 0));
	let z = black_box(z) / y;
	assert!(approx_eq!(f64, z, 0.16f64, ulps = 0));
	// 0100011110101110000101000111101011100001010001111011 exp 01111111100
	let z = black_box(z) / y;
	assert!(approx_eq!(f64, z, 0.032f64, ulps = 0));
	let z = black_box(z) / y;
	assert!(approx_eq!(f64, z, 0.0064f64, ulps = 0));
	//1010001101101110001011101011000111000100001100101101 exp 01111110111
	let z = black_box(z) / y;
	assert!(approx_eq!(f64, z, 0.00128f64, ulps = 0));
	//0b0011111101010100111110001011010110001000111000110110100011110001

	let z = black_box(z) * black_box(y) * black_box(y) * black_box(y);
	assert!(approx_eq!(f64, z, 0.16f64, ulps = 0));
	let z = black_box(z * y);
	assert!(approx_eq!(f64, z, 0.8f64, ulps = 0));
}
*/
//ToDo - add a testrunner so we can group multiple similar tests

#[no_mangle]
extern "C" fn runtime_entry(_argc: i32, _argv: *const *const u8, _env: *const *const u8) -> ! {
	// extern "Rust" {
	// 	fn test_main();
	// }
	unsafe {
		test_main();
	}
	exit(false);
}

// #[start]
// pub fn main(c: isize, argv: *const *const u8) -> isize {
// 	//test_main();
// 	0
// }

/*
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
	test_main();

	loop {}
}*/

pub fn exit(failure: bool) -> ! {
	// temporarily make this public. FIXME: we could also pass an argument to main indicating uhyve or qemu
	if hermit::environment::is_uhyve() {
		match failure {
			//ToDo: Add uhyve exit code enum
			true => hermit::sys_exit(1),
			false => hermit::sys_exit(0),
		}
	} else {
		unimplemented!();
	}
}
