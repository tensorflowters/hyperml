# Testing

This post explores unit and integration testing in no_std executables. We will use Rust’s support for custom test frameworks to execute test functions inside our kernel. To report the results out of QEMU, we will use different features of QEMU and the bootimage tool.

The complete source code for this post can be found in the [post-04](https://github.com/phil-opp/blog_os/tree/post-04) branch.
