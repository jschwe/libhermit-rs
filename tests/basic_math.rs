#![no_std]
#![no_main]

extern crate hermit;
use hermit::{print, println};

// Workaround since the "real" runtime_entry function (defined in libstd) is not available,
// since the target-os is hermit-kernel and not hermit
#[no_mangle]
extern "C"
fn runtime_entry(argc: i32, argv: *const *const u8, _env: *const *const u8) -> ! {
    let res = main(argc as isize, argv);
    match res {
        Ok(_) => hermit::sys_exit(0),
        Err(_) => hermit::sys_exit(1),
    }
}
//ToDo - add a testrunner so we can group multiple similar tests

//ToDo - Idea: pass some values into main - compute and print result to stdout
//ToDo - add some kind of assert like macro that returns a result instead of panicing
pub fn main(_argc: isize, _argv: *const *const u8) -> Result<(), ()>{
    let x = 25;
    let y = 310;
    let z = 25 * 310;
    println!("25 * 310 = {}", z);
    match z {
        7750 => Ok(()),
        _ => Err(()),
    }
}

