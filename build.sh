#!/bin/bash

OVMF_CODE="${OVMF_CODE_OVERRIDE:-/usr/share/OVMF/OVMF_CODE.fd}"
EFIVARS="./efivars.fd"
EFIVARS_SIZE="512K"

_SOCKETS=1
_CORES=2
_THREADS=2
_TOTAL_CPU=$((${_SOCKETS} * ${_CORES} * ${_THREADS}))

_CPU_OPTS=(
    "host"          # match cpu to host model
    "-hypervisor"   # remove hypervisor flag
    "migratable=no" # do not allow migration; required for +invtsc
    "+invtsc"       # enable invariant TSC so we can disable hpet
)
_MACHINE_OPTS=(
    "q35"
    "accel=kvm"
    "usb=off"
    "vmport=off"
)
_SMP_OPTS=(
    "${_TOTAL_CPU}"
    "sockets=${_SOCKETS}"
    "cores=${_CORES}"
    "threads=${_THREADS}"
)
_FIRMWARE_OPTS=(
    "file=${OVMF_CODE}"
    "if=pflash"
    "format=raw"
    "unit=0"
    "readonly=on"
)
_FIRMWARE_NVRAM_OPTS=(
    "file=${EFIVARS}"
    "if=pflash"
    "format=raw"
)

SMP_OPTS=$(IFS=,; echo "${_SMP_OPTS[*]}")
CPU_OPTS=$(IFS=,; echo "${_CPU_OPTS[*]}")
MACHINE_OPTS=$(IFS=,; echo "${_MACHINE_OPTS[*]}")
FIRMWARE_OPTS=$(IFS=,; echo "${_FIRMWARE_OPTS[*]}")
FIRMWARE_NVRAM_OPTS=$(IFS=,; echo "${_FIRMWARE_NVRAM_OPTS[*]}")

QEMU_OPTS=(
    # Disable all default options
    -nographic
    -nodefaults
    -no-user-config

    # Enable QEMU QAPI access with QMP
    -qmp unix:./qmp-reEFId.sock,server,wait=off
    #-monitor stdio

    # Basic VM configuration
    -name guest=reEFId,debug-threads=on
    -machine "${MACHINE_OPTS}" # Example: -machine q35,accel=kvm
    -m 256M       # 256MiB of memory will be available
    -mem-prealloc # Pre-allocate the memory on the host

    # UEFI firmware flash blob and nvram setup
    -drive "${FIRMWARE_OPTS}"
    -drive "${FIRMWARE_NVRAM_OPTS}"

    # CPU configuration
    -smp "${SMP_OPTS}" # Example: -smp 8,sockets=1,cores=4,threads=2
    -cpu "${CPU_OPTS}" # Example: -cpu host,-hypervisor
    -no-hpet           # Disable high-precision event timer
    -enable-kvm        # Enable KVM hardware acceleration

    # Setup serial
    -serial unix:./reEFId.sock,server,wait=on

    # Network
    -netdev user,id=nic0,tftp=./target/x86_64-unknown-uefi/debug,bootfile=re_efi_d.efi
    -device netdev=nic0,driver=e1000,bootindex=1
)

_launch() {
    # If you source this script and call `_launch` directly, any failure will
    # exit your main shell. Use the wrapper function `safe_launch` instead
    set -euxEo pipefail

    if [[ ! -f "${EFIVARS}" ]]; then
        fallocate -l "${EFIVARS_SIZE}" "${EFIVARS}"
    fi
    cargo build
    qemu-system-x86_64 "${QEMU_OPTS[@]}"
}

safe_launch() {
    # Use this function when calling from other scripts
    # your main shell. This wrapper ensures that all of the shell changes
    # happen in a subshell and do not propogate to the main bash shell.
    (_launch "$@")
}

# This checks if we are sourcing this script, or executing it directly
# The equivalent in python would be ```if __name__ == '__main__':```
[[ "${BASH_SOURCE[0]}" == "${0}" ]] && safe_launch "$@"
