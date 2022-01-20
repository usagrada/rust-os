#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("Hello {}", "World!");
  // panic!("Some panic message");
  blog_os::init();

  use x86_64::registers::control::Cr3;

  let (level_4_page_table, _) = Cr3::read();
  println!(
    "Level 4 page table at: {:?}",
    level_4_page_table.start_address()
  );

  // new
  // let ptr = 0xdeadbeaf as *mut u32;
  // unsafe {
  // *ptr = 42;
  // }

  // let ptr = 0x205141 as *mut u32;
  // // read from a code page
  // unsafe {
  //   let x = *ptr;
  // }
  // println!("read worked");

  // // write to a code page
  // unsafe {
  //   *ptr = 42;
  // }
  // println!("write worked");

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
