#!/usr/bin/env python3

import time
import argparse
import subprocess
from subprocess import Popen, PIPE, STDOUT
import os, os.path

SMP_CORES = 1  # Number of cores
MEMORY_MB = 64  # amount of memory
# Path if libhermit-rs was checked out via rusty-hermit repository
BOOTLOADER_PATH = '../loader/target/x86_64-unknown-hermit-loader/debug/rusty-loader'


# ToDo add test dependent section for custom kernel arguments / application arguments
# Idea: Use TOML format to specify things like should_panic, expected output
# Parse test executable name and check tests directory for corresponding toml file
# If it doesn't exist just assure that the return code is not a failure

# ToDo Think about always being verbose, or hiding the output
def run_test(process_args):
    print(os.getcwd())
    abs_bootloader_path = os.path.abspath(BOOTLOADER_PATH)
    print("Abspath: ", abs_bootloader_path)
    p = Popen(process_args, stdout=PIPE, stderr=STDOUT, text=True)
    output: str = ""
    for line in p.stdout:
        dec_line = line
        output += dec_line
        print(line, end='')  # stdout will already contain line break
    rc = p.wait()
    # ToDo: add some timeout
    return rc, output


print("Test runner called")
parser = argparse.ArgumentParser(description='See documentation of cargo test runner for custom test framework')
parser.add_argument('runner_args', type=str, nargs='*')
args = parser.parse_args()
print("Arguments: {}".format(args.runner_args))

qemu_base_arguments = ['qemu-system-x86_64',
                       '-display', 'none',
                       '-smp', str(SMP_CORES),
                       '-m', str(MEMORY_MB) + 'M',
                       '-serial', 'stdio',
                       '-kernel', BOOTLOADER_PATH,
                       # skip initrd - it depends on test executable
                       '-cpu', 'qemu64,apic,fsgsbase,rdtscp,xsave,fxsr'
                       ]
rc_list: int = []
# This is assuming test_runner only passes executable files as parameters
for arg in args.runner_args:
    assert isinstance(arg, str)
    curr_qemu_arguments = qemu_base_arguments.copy()
    # ToDo: assert that arg is a path to an executable before calling qemu
    # ToDo: Add addional test based arguments for qemu / uhyve
    curr_qemu_arguments.extend(['-initrd', arg])
    rc, output = run_test(curr_qemu_arguments)
    rc_list.append(rc)
    #Todo: print some status information about the test

# todo print something ala x/y tests failed etc.
#  maybe look at existing standards (TAP?)
#  - TAP: could use tappy to convert to python style unit test output (benefit??)


