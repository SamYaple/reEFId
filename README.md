# EFI Minimal Implementation in Rust

**NOTE:** For a comprehensive interaction with UEFI in Rust, consider using a Rust crate like `uefi`, which provides extensive UEFI protocol support. Much of this code is for learning purposes, you almost certainly do not want to be here.

This is a minimal implementation of UEFI (Unified Extensible Firmware Interface) services in Rust. The code aims to safely exit the boot services provided by UEFI, which is a crucial step before transitioning the system from UEFI to a standalone kernel or application that can execute in bare-metal mode. The implementation prepares the environment, enabling the subsequent execution of a non-UEFI binary.

## Background

UEFI is a specification that defines a software interface between an operating system and platform firmware, serving as a replacement for the traditional BIOS (Basic Input/Output System) in modern computer systems. UEFI provides boot and runtime services that can be used by the operating system or its loader.

A critical function of the UEFI boot service is `ExitBootServices()`, which a boot loader application calls before handing over control to the operating system kernel. After `ExitBootServices()` is called, the UEFI boot services are no longer available. 

## Key Concepts

The main highlights from the discussion in this repository include:

1. **Interacting with UEFI Boot Services:** The primary focus of this implementation is to interact with UEFI Boot Services, particularly the memory services (`allocate_pages`, `free_pages`) and the `get_memory_map` function.

2. **Calling Non-UEFI Binaries:** The eventual goal of this implementation is to transition from the UEFI environment to a non-UEFI binary. This could be a standalone kernel or application that can execute in bare-metal mode. The non-UEFI binary may be embedded into the main binary or fetched over the network.

## Usage

The codebase is a starting point for implementing a Rust-based bootloader or operating system loader that can interact with UEFI, manage memory, and transition to executing a non-UEFI binary.

It can serve as a foundation for a larger system, where additional functionalities, like network operations, filesystem interactions, and more, can be implemented on top of the base UEFI interactions.

Currently, I am using it to make a shim to workaround a firmware not following the UEFI spec.

```
    # Build the project and launch a QEMU instance
    $ ./build.sh
```

```
    # Connect to the serial output of the QEMU instance in another terminal
    $ minicom -D unix\#./reEFId.sock
```

The serial terminal output should look like:

```
    Memory Map Size was: 5952
    IF YOU SEE THIS EVERYTHING WORKED!!
    AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
    src/main.rs:61:5
    panic so we halt the cpu and dont boot to efi menu
```
