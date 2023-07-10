// This file should be a minimal implementation of EFI, enough to safely exit_boot_services,
// Alone this might be useless, but it will let me setup the environment so that I can load a
// non-EFI binary. This file will let me massage the environment into what the non-EFI binary is
// expecting and then safely call it. That will mean the non-EFI binary will get embedded into the
// main binary, or I will need to implement some network fetching code (not using UEFI functions)

use core::{
    result::Result,
    sync::atomic::{
        AtomicPtr,
        Ordering,
    },
};

static EFI_SYSTEM_TABLE: AtomicPtr<EfiSystemTable> = AtomicPtr::new(core::ptr::null_mut());

#[repr(C)]
pub struct EfiHandle(usize);

#[derive(Debug)]
#[repr(C)]
pub struct EfiStatus(pub usize);

#[repr(C)]
struct EfiInputKey {
    scan_code:    u16,
    unicode_char: u16,
}

#[repr(C)]
struct EfiTableHeader {
    signature:   u64,
    revision:    u32,
    header_size: u32,
    crc32:       u32,
    reserved:    u32,
}

enum EfiAllocateType {
   AllocateAnyPages,
   //AllocateMaxAddress,
   //AllocateAddress,
   //MaxAllocateType,
}

impl From<EfiAllocateType> for usize {
    fn from(val: EfiAllocateType) -> Self {
        match val {
            EfiAllocateType::AllocateAnyPages   => 0,
            //EfiAllocateType::AllocateMaxAddress => 1,
            //EfiAllocateType::AllocateAddress    => 2,
            //EfiAllocateType::MaxAllocateType    => 3,
        }
    }
}

#[repr(C)]
struct EfiBootServices {
    _header: EfiTableHeader,

    // Task Priority Services
    _raise_tpl:   usize,
    _restore_tpl: usize,

    // Memory Services
    allocate_pages: unsafe fn(
        allocate_type:        usize,
        memory_type:          u32,
        pages:                usize,
        efi_physical_address: &mut usize,
    ) -> EfiStatus,

    free_pages: unsafe fn(
        memory: usize,
        pages:  usize,
    ) -> EfiStatus,

    get_memory_map: unsafe fn(
        memory_map_size:    &mut usize,
        memory_map:         *mut u8,
        map_key:            &mut usize,
        descriptor_size:    &mut usize,
        descriptor_version: &mut u32,
    ) -> EfiStatus,

    _allocate_pool: usize,
    _free_pool:     usize,

    // Event & Timer Services
    _create_event:   usize,
    _set_timer:      usize,
    _wait_for_event: usize,
    _signal_event:   usize,
    _close_event:    usize,
    _check_event:    usize,

    // Protocol Handler Services
    _install_protocol_interface:   usize,
    _reinstall_protocol_interface: usize,
    _uninstall_protocol_interface: usize,
    _handle_protocol:              usize,
    _reserved:                     usize,
    _register_protocol_notify:     usize,
    _locate_handle:                usize, 
    _locate_device_path:           usize,
    _install_configuration_table:  usize,

    // Image Services
    _load_image:   usize,
    _start_image:  usize,
    _unload_image: usize,
    _exit:         usize,
    exit_boot_services: unsafe fn(
        image_handle: EfiHandle,
        map_key:      usize,
    ) -> EfiStatus,

    // Miscellaneous Services
    _get_next_monotonic_count: usize,
    _stall:                    usize,
    _set_watchdog_timer:       usize,

    // DriverSupport Services
    _connect_controller:    usize,
    _disconnect_controller: usize,

    // Open and Close Protocol Services
    _open_protocol:             usize,
    _close_protocol:            usize,
    _open_protocol_information: usize,

    // Library Services
    _protocols_per_handle:                   usize,
    _locate_handle_buffer:                   usize,
    _locate_protocol:                        usize,
    _install_multiple_protocol_interfaces:   usize,
    _uninstall_multiple_protocol_interfaces: usize,

    // 32-bit CRC Services
    _calculate_crc32: usize,

    // Miscellaneous Services
    _copy_mem:        usize,
    _set_mem:         usize,
    _create_event_ex: usize,
}

#[repr(C)]
struct EfiSimpleTextInputProtocol {
    reset: unsafe fn(
        this: *const EfiSimpleTextInputProtocol,
        extend_verification: bool,
    ) -> EfiStatus,

    read_keystroke: unsafe fn(
        this: *const EfiSimpleTextInputProtocol,
        key:  *mut EfiInputKey,
    ) -> EfiStatus,
    
    _wait_for_key: usize,
}

#[repr(C)]
struct EfiSimpleTextOutputProtocol {
    reset: unsafe fn(
        this: *const EfiSimpleTextOutputProtocol,
        extend_verification: bool,
    ) -> EfiStatus,

    output_string: unsafe fn(
        this:   *const EfiSimpleTextOutputProtocol,
        string: *const u16,
    ) -> EfiStatus,

    test_string: unsafe fn(
        this:   *const EfiSimpleTextOutputProtocol,
        string: *const u16,
    ) -> EfiStatus,

    _query_mode:          usize,
    _set_mode:            usize,
    _set_attribute:       usize,
    _clear_screen:        usize,
    _set_cursor_position: usize,
    _enable_cursor:       usize,
    _mode:                usize,
}

#[repr(C)]
pub struct EfiSystemTable {
    _header:            EfiTableHeader,
    _firmware_vendor:   *const u16,
    _firmware_revision: u32,
    console_in_handle:  EfiHandle,
    console_in:         *const EfiSimpleTextInputProtocol,
    console_out_handle: EfiHandle,
    console_out:        *const EfiSimpleTextOutputProtocol,
    console_err_handle: EfiHandle,
    console_err:        *const EfiSimpleTextOutputProtocol,
    _runtime_services:  usize,
    boot_services:      *const EfiBootServices,
}

#[derive(Debug)]
#[repr(C)]
enum EfiMemoryType {
    ReservedMemoryType,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    ConventionalMemory,
    UnstableMemory,
    ACPIReclaimMemory,
    ACPIMemoryNVS,
    MemoryMappedIO,
    MemoryMappedIOPortSpace,
    PalCode,
    PersistentMemory,
    UnacceptedMemoryType,
}

impl From<EfiMemoryType> for u32 {
    fn from(val: EfiMemoryType) -> Self {
        match val {
            EfiMemoryType::ReservedMemoryType       =>  0,
            EfiMemoryType::LoaderCode               =>  1,
            EfiMemoryType::LoaderData               =>  2,
            EfiMemoryType::BootServicesCode         =>  3,
            EfiMemoryType::BootServicesData         =>  4,
            EfiMemoryType::RuntimeServicesCode      =>  5,  
            EfiMemoryType::RuntimeServicesData      =>  6,
            EfiMemoryType::ConventionalMemory       =>  7,
            EfiMemoryType::UnstableMemory           =>  8,
            EfiMemoryType::ACPIReclaimMemory        =>  9,
            EfiMemoryType::ACPIMemoryNVS            => 10,
            EfiMemoryType::MemoryMappedIO           => 11,
            EfiMemoryType::MemoryMappedIOPortSpace  => 12,
            EfiMemoryType::PalCode                  => 13,
            EfiMemoryType::PersistentMemory         => 14,
            EfiMemoryType::UnacceptedMemoryType     => 15,
        }
    }
}

impl From<u32> for EfiMemoryType {
    fn from(val: u32) -> Self {
        match val {
             0 => EfiMemoryType::ReservedMemoryType,
             1 => EfiMemoryType::LoaderCode,
             2 => EfiMemoryType::LoaderData,
             3 => EfiMemoryType::BootServicesCode,
             4 => EfiMemoryType::BootServicesData,
             5 => EfiMemoryType::RuntimeServicesCode,
             6 => EfiMemoryType::RuntimeServicesData,
             7 => EfiMemoryType::ConventionalMemory,
             8 => EfiMemoryType::UnstableMemory,
             9 => EfiMemoryType::ACPIReclaimMemory,
            10 => EfiMemoryType::ACPIMemoryNVS,
            11 => EfiMemoryType::MemoryMappedIO,
            12 => EfiMemoryType::MemoryMappedIOPortSpace,
            13 => EfiMemoryType::PalCode,
            14 => EfiMemoryType::PersistentMemory,
            15 => EfiMemoryType::UnacceptedMemoryType,
             _ => EfiMemoryType::UnacceptedMemoryType,
        }
    }
}

pub unsafe fn register_system_table(system_table: *mut EfiSystemTable) {
    match EFI_SYSTEM_TABLE.compare_exchange(
        core::ptr::null_mut(),
        system_table,
        Ordering::SeqCst,
        Ordering::SeqCst,
    ) {
        Ok(_)  => return,
        Err(_) => panic!("Failed to get EFI system_table"),
    };
}

pub fn output_string(string: &str) {
    let st = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);
    if st.is_null() {
        return;
    }
    let out = unsafe {
        (*st).console_out
    };

    // TODO: Kill this buffer
    let mut tmp = [0u16; 32]; // UCS-2, not UTF-16
    let mut in_use = 0;
    for chr in string.encode_utf16() {
        if chr == b'\n' as u16 {
            tmp[in_use] = b'\r' as u16;
            in_use += 1;
        }
        tmp[in_use] = chr;
        in_use += 1;

        if in_use == (tmp.len() - 2) {
            tmp[in_use] = 0;
            unsafe {
                ((*out).output_string)(out, tmp.as_ptr());
            }
            in_use = 0;
        }
    }
    if in_use > 0 {
        tmp[in_use] = 0;
        unsafe {
            ((*out).output_string)(out, tmp.as_ptr());
        }
    }
}

pub struct SimplePage {
    buffer: &'static mut [u8],
    pages:  usize,
    size:   usize,
}

impl SimplePage {
    pub fn get_slice(&self) -> &[u8] {
        &self.buffer[..self.size]
    }

    pub fn get_mut_slice(&mut self) -> &mut [u8] {
        &mut self.buffer[..self.size]
    }
}

impl Drop for SimplePage {
    fn drop(&mut self) {
        let st = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);
        if st.is_null() {
            return;
        }
        unsafe {
            let ret = ((*(*st).boot_services).free_pages)(
                self.buffer.as_ptr() as usize,
                self.pages,
            );
            assert!(ret.0 == 0, "{:x?}", ret);
        }
    }
}

/// Takes a `usize` of bytes and returns a buffer with at least that much space
fn simple_allocate(min_size: usize) -> SimplePage {
    let st = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);
    if st.is_null() { panic!("i dont know what to do"); }

    let page_size = 4096;
    let pages = min_size.div_ceil(4096);
    let size = pages * page_size;

    let buffer = unsafe {
        let mut buffer: usize = 0;
        let ret = ((*(*st).boot_services).allocate_pages)(
            EfiAllocateType::AllocateAnyPages.into(),
            EfiMemoryType::LoaderData.into(),
            pages,
            &mut buffer,
        );
        assert!(ret.0 == 0, "{:x?}", ret);
        let ptr = buffer as *mut u8;
        core::slice::from_raw_parts_mut(ptr, size)
    };
    SimplePage {
        buffer,
        pages,
        size,
    }
}

pub fn get_memory_map() -> (SimplePage, usize, usize, u32) {
    let st = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);
    if st.is_null() {
        panic!("i dont know what to do");
    }

    let memory_map_size = unsafe {
        let mut size = 0;
        let mut _k = 0;
        let mut _s = 0;
        let mut _v = 0;
        let ret = ((*(*st).boot_services).get_memory_map)(
            &mut size,
            core::ptr::null_mut(),
            &mut _k,
            &mut _s,
            &mut _v,
        );
        assert!(ret.0 == 0x8000000000000005, "{:x?}", ret);
        if size == 0 {
            panic!("the returned buffer size was 0; we wont proceed; fix the firmware");
        }
        size
    };

    let mut memory_map = simple_allocate(memory_map_size);
    let mut size = core::mem::size_of_val(memory_map.buffer);
    let mut key = 0;
    let mut mdesc_size = 0;
    let mut mdesc_version = 0;

    unsafe {
        let ret = ((*(*st).boot_services).get_memory_map)(
            &mut size,
            memory_map.buffer.as_mut_ptr(),
            &mut key,
            &mut mdesc_size,
            &mut mdesc_version,
        );
        assert!(ret.0 == 0, "{:x?}", ret);
    }
    memory_map.size = size;

    /*
    println!("the mdesc_size {mdesc_size} and the struct size {}", core::mem::size_of::<EfiMemoryDescriptor>());
    let mm = memory_map.get_slice();
    for offset in (0..mm.len()).step_by(mdesc_size) {
        let entry = core::ptr::read_unaligned(
            mm[offset..].as_ptr() as *const EfiMemoryDescriptor
        );
        let type_: EfiMemoryType = entry.type_.into();
        println!(
            "{:016x} {:016x} {:?}",
            entry.physical_start,
            entry.number_of_pages * 4096,
            type_,
        );
    }
    */
    (memory_map, key, mdesc_size, mdesc_version)
}

#[repr(C)]
struct EfiMemoryDescriptor {
    type_:           u32,
    physical_start:  u64,
    virtual_start:   u64,
    number_of_pages: u64,
    attribute:       u64,
}

pub fn exit_boot_services(image_handle: EfiHandle) -> Result<SimplePage, &'static str> {
    let (mm, key, _, _) = get_memory_map();
    /*
    if we print after we get_memory_map then the `key`
    will change and we will fail to `exit_boot_services`
    */

    let st = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);
    if st.is_null() {
        return Err("Failed to get EFI system table");
    }

    unsafe {
        let ret = ((*(*st).boot_services).exit_boot_services)(image_handle, key);
        if ret.0 != 0 {
            return Err("Failed to exit EFI boot services");
        }
    }

    Ok(mm)
}
