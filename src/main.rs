#![feature(
    int_roundings,
    panic_info_message,
)]
#![no_std]
#![no_main]

mod efi;
#[macro_use]
mod print;

use core::arch::asm;
use core::panic::PanicInfo;

use crate::efi::{
    EfiHandle,
    EfiSystemTable,
    EfiStatus,
};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    if let Some(location) = info.location() {
        println!(
            "{}:{}:{}",
            location.file(),
            location.line(),
            location.column(),
        );
    }
    if let Some(message) = info.message() {
        println!("{}", message);
    }
    loop {
        unsafe {
            asm!["hlt"];
        }
    }
}

#[no_mangle]
extern fn efi_main(image_handle: EfiHandle, system_table: *mut EfiSystemTable) -> EfiStatus {
    unsafe {
        efi::register_system_table(system_table);
    }
    let (mm, _, _, _) = efi::get_memory_map();
    let memory_map = mm.get_slice();
    println!("Memory Map Size was: {}", memory_map.len());

    /*
    let mm = match efi::exit_boot_services(image_handle) {
        Ok(mm) => mm,
        Err(e) => panic!("{e}"),
    };
    // we cannot print because we have exited boot services (TODO setup serial driver)
    let _memory_map = mm.get_slice();
    */

    println!("IF YOU SEE THIS EVERYTHING WORKED!!");
    panic!("panic so we halt the cpu and dont boot to efi menu");
    EfiStatus(0)
}
