#![no_std] // We don't use the standard library in kernel modules
#![feature(allocator_api, global_asm)] // Kernel features needed for Rust

use kernel::prelude::*;
use kernel::chrdev;
use kernel::file_operations::FileOperations;

module! {
    type: SimpleCharDevice,
    name: b"simple_rust_driver",
    author: b"Hao Ho",
    description: b"A simple Rust character device driver",
    license: b"GPL",
}

struct SimpleCharDevice {
    major: u32,
}

impl KernelModule for SimpleCharDevice {
    fn init() -> Result<Self> {
        pr_info!("Initializing the Rust character device driver!\n");

        // Register the character device
        let major = chrdev::builder(b"simple_rust_driver")
            .build::<SimpleFileOperations>()?;

        pr_info!("Character device registered with major number {}\n", major);

        Ok(Self { major })
    }
}

impl Drop for SimpleCharDevice {
    fn drop(&mut self) {
        pr_info!("Cleaning up the Rust character device driver.\n");
    }
}

struct SimpleFileOperations;

impl FileOperations for SimpleFileOperations {
    fn read(
        this: &Self,
        _file: &kernel::file_operations::File,
        _data: &mut kernel::user_ptr::UserSlicePtrWriter,
    ) -> kernel::Result<usize> {
        pr_info!("Read operation called on the device.\n");
        Ok(0) // No data to return
    }

    fn write(
        this: &Self,
        _file: &kernel::file_operations::File,
        _data: &kernel::user_ptr::UserSlicePtrReader,
    ) -> kernel::Result<usize> {
        pr_info!("Write operation called on the device.\n");
        Ok(0) // No data written
    }
}
