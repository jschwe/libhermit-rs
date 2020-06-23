#![feature(test)]
#![no_std]
#![no_main]
#![test_runner(test_runner)]
#![feature(custom_test_frameworks)]
#![feature(start)]
#![reexport_test_harness_main = "test_main"]

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
}

#[test_case]
fn add_one() {
	let x = 1 + 2;
	assert_eq!(x, 3);
}

// After initializing the kernel calls this function
#[no_mangle]
extern "C" fn runtime_entry(_argc: i32, _argv: *const *const u8, _env: *const *const u8) -> ! {
	//Toggling this changes the error from compile time to link time
	// extern "Rust" {
	// 	fn test_main();
	// }
	unsafe {
		test_main();
	}
	hermit::sys_exit(0)
}
