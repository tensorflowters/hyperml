# Creating a bootloader

Creating a bootloader for a Rust-based hypervisor involves several steps, as the bootloader is responsible for setting up the environment in which your kernel will run.

Here's a basic guide on how to create a simple bootloader:

## 1. Understand the Boot Process

The boot process is the sequence of events that starts from powering on the machine and leads up to your kernel code running.
For x86 systems, this typically involves the BIOS or UEFI firmware.

## 2. Choose Between BIOS and UEFI

### BIOS (Legacy)

Older but simpler. Boots via the Master Boot Record (MBR) or a boot sector.

### UEFI

Modern, more complex, but more capabilities. Boots using the EFI System Partition.

## 3. Setting Up the Development Environment

Ensure Rust is installed and configured for cross-compiling.

Install tools like QEMU for emulation and GDB for debugging.

### Cross-Compiling

#### Cross-Compiling - Definition

Cross-compiling is the process of compiling a program on one platform (the host) to run on another platform (the target). In the context of hypervisor development, you typically compile on your development machine (e.g., x86_64) for a different architecture or bare-metal target.

#### Cross-Compiling - Purpose

Allows you to develop and build software that runs in an environment different from your development machine, such as a minimal Rust kernel for a hypervisor.

### QEMU

#### QEMU - Definition

QEMU is an open-source machine emulator and virtualizer. It can emulate a full system, including a processor and various peripherals, or it can virtualize user-level processes from one architecture to run on another.

#### QEMU - Purpose

Enables you to test your hypervisor or kernel in a controlled, emulated hardware environment, without needing to run it on actual hardware.

### GDB

#### GDB - Definition

GDB (GNU Debugger) is a powerful tool for debugging programs. It allows you to see what is going on 'inside' a program while it executes or what a program was doing at the moment it crashed.

#### GDB - Purpose

Assists in debugging your kernel or hypervisor, especially when dealing with low-level code and hardware interactions.

## 4. Writing a Simple Bootloader

### For BIOS

Write a boot sector in Assembly that fits within 512 bytes, the size of the MBR.
The boot sector should initialize CPU registers and a stack.
Load the kernel from a predetermined location on the disk into memory.

### For UEFI

Write a UEFI application in Rust using the uefi-rs crate.
UEFI applications are easier to write in Rust as they can use some Rust standard library features.
The application should locate and load the kernel file from a filesystem.

## 5. Compile and Test the Bootloader

Use NASM to compile the Assembly code for a BIOS bootloader.

For a UEFI bootloader, compile using cargo with the appropriate target for UEFI.

Test the bootloader using QEMU or on actual hardware.

## 6. Load and Execute the Kernel

The bootloader should load the kernel into memory and then jump to its entry point.

Ensure the kernel is loaded at the correct memory address expected by the kernel.

## 7. Pass Control to the Kernel

After loading the kernel into memory, the bootloader should pass control to it, typically by jumping to its entry point.

### Example Code for BIOS Bootloader

Here's an example of what the beginning of a BIOS bootloader might look like in Assembly:

```asm
asm
Copy code
section .text
bits 16
org 0x7C00

start:
    cli             ; Clear interrupts
    mov ax, 0x07C0
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; Set up stack...
    ; Load the kernel...
    ; Jump to the kernel...

times 510-($-$$) db 0   ; Pad the bootloader to 510 bytes
dw 0xAA55               ; Boot signature at the end
```

## Next Steps

### Integrate with Your Kernel

Once your bootloader can successfully load a kernel into memory, you'll need to ensure that your kernel is prepared to take control. This includes setting up a correct execution environment in Rust.

### Iterate and Enhance

As your hypervisor project evolves, you may need to return to your bootloader to add new features or capabilities, such as passing system information to the kernel.
