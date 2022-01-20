#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use blog_os::{allocator, println};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
  use blog_os::memory::{self, BootInfoFrameAllocator};
  use x86_64::VirtAddr;

  println!("Hello {}", "World!");
  blog_os::init();

  let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
  let mut mapper = unsafe { memory::init(phys_mem_offset) };
  let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

  allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

  let x = Box::new(41);

  #[cfg(test)]
  test_main();

  println!("It did not crash!");
  blog_os::hlt_loop();
}

#[cfg(not(test))] // 新しく追加した属性
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  blog_os::test_panic_handler(info)
}
