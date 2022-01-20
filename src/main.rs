#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
  use blog_os::memory::{self, BootInfoFrameAllocator};
  use x86_64::{structures::paging::Page, VirtAddr};

  println!("Hello {}", "World!");
  blog_os::init();

  let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
  let mut mapper = unsafe { memory::init(phys_mem_offset) };
  let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

  // map an unused page
  let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
  // let page = Page::containing_address(VirtAddr::new(0));
  memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

  // write the string `New!` to the screen through the new mapping
  let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
  unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

  // let addresses = [
  //   // the identity-mapped vga buffer page
  //   0xb8000,
  //   // some code page
  //   0x201008,
  //   // some stack page
  //   0x0100_0020_1a10,
  //   // virtual address mapped to physical address 0
  //   boot_info.physical_memory_offset,
  // ];

  // for &address in &addresses {
  //   let virt = VirtAddr::new(address);
  //   let phys = mapper.translate_addr(virt);
  //   println!("{:?} -> {:?}", virt, phys);
  // }

  // let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

  // for (i, entry) in l4_table.iter().enumerate() {
  //   if !entry.is_unused() {
  //     println!("L4 Entry {}: {:?}", i, entry);
  //   }

  //   if !entry.is_unused() {
  //     println!("L4 Entry {}: {:?}", i, entry);

  //     // get the physical address from the entry and convert it
  //     let phys = entry.frame().unwrap().start_address();
  //     let virt = phys.as_u64() + boot_info.physical_memory_offset;
  //     let ptr = VirtAddr::new(virt).as_mut_ptr();
  //     let l3_table: &PageTable = unsafe { &*ptr };

  //     // print non-empty entries of the level 3 table
  //     for (i, entry) in l3_table.iter().enumerate() {
  //       if !entry.is_unused() {
  //         println!("  L3 Entry {}: {:?}", i, entry);
  //       }
  //     }
  //   }
  // }

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
