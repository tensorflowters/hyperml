# Roadmap

Welcome to Operating Systems Development

## Phase 0: Introduction

Main article: [Introduction](link-to-introduction)

You should consult all the basic documentation before starting writing an operating system.

## Building the latest GCC

Main article: [Building GCC](link-to-building-gcc)

You may wish to upgrade your system compiler to the latest version before you start out with operating systems development and build the cross-compiler.

## Phase I: Beginning

In this phase we will set up a toolchain and create a basic kernel that will become the core of the new operating system.

## Setting up a Cross-Toolchain

Main article: [GCC Cross Compiler](link-to-gcc-cross-compiler)

The first thing you will like to do is set up a cross-compiler for your operating system. The compiler on your local system is not able to produce programs for your operating system because it hasn't been invented yet. At first, you would like to do is create a compiler that produces executables that will run directly on your target hardware.

## Creating a Hello World kernel

Main article: [Bare Bones](link-to-bare-bones)

See also Bare Bones for other platforms
Your next task is to make a simple hello world kernel that is able to start up, print a message to the output device, and then loop endlessly. While simple and useless, it will serve as a great example and starting point for a real system, as well as confirm that your testing environment works correctly.

## Setting up a Project

Main article: [Meaty Skeleton](link-to-meaty-skeleton)

With a basic working example, your next task is to set up a build infrastructure using whatever build system you see fit. Be careful in your choices of technology, GNU Make is easier to port than Python.

## Calling Global Constructors

Main article: [Calling Global Constructors](link-to-calling-global-constructors)

The compiler expects you to perform various program initialization tasks, such as calling the constructors on global C++ objects. Normally you would use a kernel_early_main function to set up the minimal kernel features, then perform all these initialization tasks, and then jump to the actual kernel_main function.

## Terminal Support

Main article: [Formatted Printing](link-to-formatted-printing)

You will often need to debug your operating system. Your very best friend is a printf function that is able to print strings and integers to the screen onto a terminal-like buffer. It is crucial to add a printf function to your kernel early on as you will certainly need it later for debugging.

## Stack Smash Protector

Main article: [Stack Smashing Protector](link-to-stack-smashing-protector)

Early is not too soon to think about security and robustness. You can take advantage of the optional stack smash protector offered by modern compilers that detect stack buffer overruns rather than behaving unexpectedly (or nothing happening, if unlucky).

## Multiboot

Main article: [Multiboot](link-to-multiboot)

It's useful to know what features and information the bootloader offers the kernel, as this may help you get memory maps, set video modes, and even kernel symbol tables.

## Global Descriptor Table

Main article: [Global Descriptor Table](link-to-global-descriptor-table)

The Global Descriptor Table is an important part of the processor state and it should as such be one of the first things that are initialized. It probably makes a lot of sense to set up it even prior to kernel_early.

## Memory Management

Main article: [Memory Management](link-to-memory-management)

Memory allocation and management is one of the most basic functions in an operating system. You need to keep track of physical page frames, what ranges of virtual memory are used, and implementing a heap (malloc, free) upon it for internal kernel use.

## Interrupts

Main article: [Interrupts](link-to-interrupts)

Your kernel needs to handle asynchronous events sent by the hardware to function properly.

## Multithreaded Kernel

Main article: [Multithreaded Kernel](link-to-multithreaded-kernel)

It is best to go multithreaded early in the development of your kernel or you'll end up rewriting parts of your kernel. We'll certainly need this when we add processes later on.

## Keyboard

Main article: [Keyboard](link-to-keyboard)

Your operating system will certainly need support for reading input from the computer operator so it can adapt its behavior to his wishes.

## Internal Kernel Debugger

Main article: [Internal Kernel Debugger](link-to-internal-kernel-debugger)

It is very useful for a multithreaded kernel to have built-in debugging facilities early on. You could have a magic key that stops the entire kernel and dumps the user to a mini-kernel with a command line interface for debugging. It could know the data structures used by the scheduler to list all the threads and perform call traces.

## Filesystem Support

Main articles: [Filesystem](link-to-filesystem), [Initialization Ramdisk](link-to-initialization-ramdisk)

It'll be useful to have support for filesystems early on and transferring files onto your operating system using an initialization ramdisk.

## Phase II: User-Space

In this phase, we'll expand your operating system into user-space and add support for programs, which is enough for your project to qualify as a small operating system. You need to work on system calls, program loading, memory management, and rework parts of your system early in this phase.

## User-Space

Main article: [User-Space](link-to-user-space)

Your processor has until now run in kernel mode, where the code is able to do everything. Processes are normally run with no permissions at all, except being able to execute code and use its designated memory. The first part of implementing user-space is switching the processor to user mode.

## Program Loading

Main article: [Dynamic Linker](link-to-dynamic-linker)

The first task you will need to complete is loading a program into memory. This involves parsing the program headers, allocating memory at the right virtual addresses, and copying the contents of the segments to the right virtual addresses. You'll also need to fill up entries in the GOT according to the relocation tables.

## System Calls

Main article: [System Calls](link-to-system-calls)

You are now able to load programs into memory and switch to user mode. Processes communicate with the kernel using system calls, which is the next feature you will want to add.

## OS Specific Toolchain

Main article: [OS Specific Toolchain](link-to-os-specific-toolchain)

As your operating system is now becoming a real operating system, it is time to treat it as such. We'll like to teach the cross-compiler about your operating system and its conventions, so we can easily cross-compile programs.

## Creating a C Library

Main article: [Creating a C Library](link-to-creating-a-c-library)

At this point, you can decide to use an existing C library or write your own C library. If you go the custom route, you will want to set up some basic features that the cross-compiler needs for libgcc. With this in place, you can now easily cross-compile programs.

## Fork and Execute

Main article: [Fork](link-to-fork)

With basic program loading in place, we are almost ready to create a multitasking operating system. The missing primitives is allowing a process to create new processes

. The standard Unix primitive is fork, which allows a process to create a perfect copy of itself. This copy is then able to call the program loader and replace its own memory with that of another program image.

## Shell

Main article: [Shell](link-to-shell)

This is a very exciting point in your operating system. It is able to run programs and create new processes. So far, the behavior of the system has possibly been determined when it was compiled. By writing a shell, you can run multiple programs and decide which one to run at runtime. This is the point where you reach the level that many newcomers dream of: a basic operating system with a working command line.

## Phase III: Extending your Operating System

With these basic features in place, you can now start writing your operating system and all its wonderful features. You'll add games, editors, test programs, command line utilities, drivers, and whatever you can imagine. Your skill and imagination are the limit here. You can delay many of these features until later in your operating system and make them in almost any order.

## Time

Main article: [Time](link-to-time)

Time is a complicated concept in computing, however modern operating systems are expected to have functions for converting timestamps to parsed time and back, as well as providing system clocks (real time, monotonic time, user CPU time, ..) and timers on these clocks with events happening on expiration.

## Threads

Main article: [Thread](link-to-thread)

Operating systems should expose a threading API such as pthreads.

## Thread Local Storage

Main article: [Thread Local Storage](link-to-thread-local-storage)

Thread local variables require runtime support.

## Symmetric Multiprocessing

Main article: [SMP](link-to-smp)

It's a very good idea to add support for multiple CPUs to your kernel early on, or you will likely need to redo a lot of your kernel because it wasn't SMP-ready in many places.

## Secondary Storage

Main article: [Secondary](link-to-secondary)

You will likely want to support common block devices such as hard disks, CD-ROMs, floppies, and whatever storage devices your operating system needs support for.

## Real Filesystems

Main article: [File Systems](link-to-file-systems)

It's a good idea to add proper filesystem support early on.

## Graphics

Main article: [How do I set a graphics mode](link-to-graphics-mode)

Real operating systems don't operate in the basic text mode, but have bitmapped graphics. Writing real graphics drivers is a bunch of work, although some virtual machines offer some useful shortcuts.

## User Interface

Main articles: [User Interface](link-to-user-interface), [Compositing](link-to-compositing)

You will certainly need to impress the operating systems development community with your flashy graphics and usable user-interface.

## Networking

Main article: [Networking](link-to-networking)

The uses for networking support are obvious, so you will likely want to do this.

## Sound

Main article: [Sound](link-to-sound)

Sound is an important part of the computing experience and depending on your needs, you may well wish to support sound on common hardware.

## Universal Serial Bus

Main article: [USB](link-to-usb)

If you need to communicate with modern peripherals, you will likely need a USB stack and support for the various common USB controllers.

## Phase IV: Bootstrapping

You now have your basic operating system in place and you are ready to move onto the next level. In this phase, we will start porting software onto your operating system such that you can become self-hosting. You already begun your effort toward being self-hosting when you set up your OS-specific toolchain and it pays off now.

## Porting Software

Main article: [Cross-Porting Software](link-to-cross-porting-software)

While not all pieces of software are easy to port, most Unix software comes with an autoconf-generated configure script. You can provide these scripts with the --host=mycpu-myos option and if your operating system offers the needed features, you can cross-compile the software onto your operating system. You already met examples of how to port software when setting up the OS-specific toolchain. While the difficulty of cross-compiling software differs greatly, you will likely be using the same process for adapting new packages.

## Porting GCC

Main article: [Porting GCC to your OS](link-to-porting-gcc-to-your-os)

You already began the work porting binutils and gcc when you set up the OS-specific toolchain. We'll finish the process and cross-compile them onto your operating system such that it can compile the C hello world program.

## Compiling your OS under your OS

The next task is to port your entire build system. You may need to port GNU Make, port some command line utilities (coreutils) or write your own, port a real shell or finish yours, and more. You may also need to fix a number of bugs in your operating system such that the compiler runs correctly. You will need to deal with how to transfer the newly compiled version onto permanent storage such that a reboot of the computer will run the next version. Your operating system will now qualify as self-compiling.

## Fully Self-hosting

Now that you can build your entire operating system under your operating system, you also need to be able to do the rest. You need to be able to also build your compiler under your operating system. You need to be able to develop under your operating system, so you'll port your favorite text editor or write one. You need networking so you can release the newest version (build on itself) onto the internet. You'll port lots of programs, libraries, games, and whatever else you desire, such that the entire development process can happen on your operating system. You can now uninstall your original operating system and replace it with your new glorious operating system.

## Phase V: Profit

You have now successfully created a real operating system that is fully self-hosting and the envy of the entire operating systems development community. You have ported quake, have OpenGL programs, a working browser, a thriving community of contributors, and much success. You can now start over and develop the next operating system from your own operating system.
