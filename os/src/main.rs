#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os::println;
use bootloader::{BootInfo, entry_point};


entry_point!(kernel_main);



#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use os::memory;
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Hello World{}", "!");

    os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe {
        memory::init(phys_mem_offset)
    };
    // let addresses = [
    //     0xb8000,
    //     0x201008,
    //     0x0100_0020_1a10,
    //     boot_info.physical_memory_offset,
    // ];
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {
        memory::init(phys_mem_offset)
    };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};
    // let l4_table = unsafe {
    //     active_level_4_table(phys_mem_offset)
    // };

    // for (i, entry) in l4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", i, entry);

    //         let phys = entry.frame().unwrap().start_address();
    //         let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         let ptr = VirtAddr::new(virt).as_mut_ptr();
    //         let l3_table: &PageTable = unsafe {
    //             &*ptr
    //         };

    //         for (i, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 println!(" L3 Entry {}: {:?}", i, entry);
    //             }
    //         }
    //     }
    // }
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // };
    // x86_64::instructions::interrupts::int3();

    // fn stack_overflow(){
    //     stack_overflow();
    // }

    // stack_overflow();
    // use x86_64::registers::control::Cr3;

    // let (level_4_page_table, _) = Cr3::read();
    // println!("Level 4 page table at: {:?}", level_4_page_table.start_address());
    

    // let ptr = 0xdeadbeaf as *mut u8;
    // unsafe {*ptr = 42;}

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    // loop {
    //     use os::print;
    //     print!("-");
    // }
    os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    // loop {}
    os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}