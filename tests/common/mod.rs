/// Common functionality for all integration tests
/// Note: If you encounter `error[E0463]: can't find crate for 'test'`, rememmber to add
/// `harness = false` to the [[test]] section of cargo.toml
pub extern crate alloc;
pub use alloc::string::String;
pub use alloc::vec::Vec;
pub use hermit::{print, println};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
	Success = 0x10,
	Failed = 0x11,
}

pub fn exit(failure: bool) -> ! {
	// temporarily make this public. FIXME: we could also pass an argument to main indicating uhyve or qemu
	if hermit::environment::is_uhyve() {
		match failure {
			//ToDo: Add uhyve exit code enum
			true => hermit::sys_exit(1),
			false => hermit::sys_exit(0),
		}
	} else {
		match failure {
			true => exit_qemu(QemuExitCode::Failed),
			false => exit_qemu(QemuExitCode::Success),
		}
	}
}

/// Debug exit from qemu with a returncode
/// '-device', 'isa-debug-exit,iobase=0xf4,iosize=0x04' must be passed to qemu for this to work
pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
	use x86_64::instructions::port::Port;

	unsafe {
		let mut port = Port::new(0xf4);
		port.write(exit_code as u32);
	}
	println!("Warning - Failed to debug exit qemu - exiting via sys_exit()");
	hermit::sys_exit(0) //sys_exit exitcode on qemu gets silently dropped
}

// ToDo: Maybe we could add a hard limit on the length of `s` to make this slightly safer?
unsafe fn parse_str(s: *const u8) -> Result<String, ()> {
	let mut vec: Vec<u8> = Vec::new();
	let mut off = s;
	while *off != 0 {
		vec.push(*off);
		off = off.offset(1);
	}
	let str = String::from_utf8(vec);
	match str {
		Ok(s) => Ok(s),
		Err(_) => Err(()), //Convert error here since we might want to add another error type later
	}
}

// Workaround since the "real" runtime_entry function (defined in libstd) is not available,
// since the target-os is hermit-kernel and not hermit
#[no_mangle]
extern "C" fn runtime_entry(argc: i32, argv: *const *const u8, _env: *const *const u8) -> ! {
	extern "Rust" {
		/// main functions of integration test get their arguments as a Vec<String> and
		/// must return a Result<(), ()> indicating success or failure of the tests
		fn main(args: Vec<String>) -> Result<(), ()>;
	}

	let mut str_vec: Vec<String> = Vec::new();
	let mut off = argv;
	for i in 0..argc {
		let s = unsafe { parse_str(*off) };
		unsafe {
			off = off.offset(1);
		}
		match s {
			Ok(s) => str_vec.push(s),
			Err(_) => println!(
				"Warning: Application argument {} is not valid utf-8 - Dropping it",
				i
			),
		}
	}

	let res = unsafe { main(str_vec) };
	match res {
		Ok(_) => exit(false),
		Err(_) => exit(true),
	}
}
