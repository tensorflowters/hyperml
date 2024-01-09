# Kernel Development

## 1. Create a Bootloader

Purpose: The bootloader is responsible for loading your kernel into memory and starting it.
Implementation: You can use an existing bootloader like bootloader or uefi-rs for UEFI systems. These bootloaders can load a Rust kernel from a disk and start it.

## 2. Set Up a Rust Environment for Kernel Development

Rust Environment: Ensure your Rust environment is set up to compile kernel code. This typically means compiling for a no_std environment, as the standard library is not available.
Configuration: Modify your Cargo.toml to use the #![no_std] attribute and configure the appropriate target for cross-compilation.

## 3. Entry Point

Kernel Entry Point: Write the entry point of your kernel. This is the first Rust code that will run.
Assembly Wrapper: Initially, you may need a small assembly stub to jump to your Rust entry point, especially to correctly set up the environment for Rust code.

## 4. Minimal Runtime

Runtime Requirements: Implement the minimal runtime requirements for your kernel, such as a stack and possibly a heap.
Memory Management: Basic memory management setup is essential at this stage.

## 5. Console Output

Debugging: Implement a way to output text to the console for debugging purposes. This could be as simple as writing to a fixed memory-mapped I/O address for text output.

## 6. Hardware Initialization

CPU and Memory: Initialize essential hardware, like the CPU in a known state and configuring memory.

## 7. Test and Iterate

Testing: Regularly test your kernel on real hardware or a virtual machine.
Iterative Development: Develop iteratively, adding and testing new features one at a time.
Starting the Development:
Project Structure: Set up your Rust project structure, including the Cargo.toml file with the necessary dependencies and configurations.

Bootloader Integration: Choose and integrate a bootloader. If you're using UEFI, uefi-rs is a good choice. For a more traditional BIOS-based system, you might use something like bootloader.

Write the Entry Point: Begin coding with the entry point and the minimal runtime environment.

Tools and Resources:
QEMU: Use an emulator like QEMU for testing your hypervisor.
GDB: Use GDB or another debugger for low-level debugging of your kernel.

## Next Steps

After you have a basic kernel that boots and provides output, you can proceed to more advanced features like memory management, CPU virtualization, and the integration of your specific hypervisor functionalities.

Refer to the [post-02 branch](https://github.com/phil-opp/blog_os/tree/post-02)
