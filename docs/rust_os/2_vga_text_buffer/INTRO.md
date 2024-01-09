# The VGA Text Buffer

To print a character to the screen in VGA text mode, one has to write it to the text buffer of the VGA hardware.

The VGA text buffer is a two-dimensional array with typically 25 rows and 80 columns, which is directly rendered to the screen.

The VGA text buffer is accessible via memory-mapped I/O to the address 0xb8000.

This means that reads and writes to that address don’t access the RAM but directly access the text buffer on the VGA hardware. This means we can read and write it through normal memory operations to that address.

Note that memory-mapped hardware might not support all normal RAM operations.

For example, a device could only support byte-wise reads and return junk when a u64 is read.

Fortunately, the text buffer supports normal reads and writes, so we don’t have to treat it in a special way.

## Example

<https://github.com/phil-opp/blog_os/tree/post-03>

## A Rust Module

The VGA text buffer will be implemented as a Rust module.

For the content of this module, we create a new src/vga_buffer.rs file.
