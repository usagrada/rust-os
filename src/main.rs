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

  fn stack_overflow() {
    stack_overflow(); // 再帰呼び出しのために、リターンアドレスがプッシュされる
  }

  // スタックオーバーフローを起こす
  stack_overflow();

  #[cfg(test)]
  test_main();

  println!("It did not crash!");
  loop {}
}

#[cfg(not(test))] // 新しく追加した属性
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  blog_os::test_panic_handler(info)
}
