---
layout: default
title: Neutron API
parent: Neutron
---

## NeutronAPI Userspace

Most things can be built with `rust-std` or bare neutron service suite with rei. However apps that are graphical in nature should use `arckit` which bundles `arcwm`, general wrapper classes and an object persistence library for storing and manipulating data.

When building executables/libraries with rust for `target_os=neutron_arc64`:

```rust
// in neutronapi::

struct ServiceResult<T> {
    data: Option<T>,
    status_code,
    // should actually make this a 'some_lifetime &str
    // and give ownership to the return function
    message: Option<String>,
}

// wrappers around asm calls and more checks (that arent in the kernel cause we want to keep it lean)
fn sys_read(arg1, arg2) -> ServiceResult {
    // checks
    if arg1 is uninitialised or out of scope {
        return fail
    }

    if arg1 + arg2 is uninitialised or out of scope {
        return fail 
    }

    unsafe {
        // make the syscall
        asm!("...");

        // asm should load the return val in r0, load it into a variable
        let result = u64;
        asm!("ld {result} r0");
    }

    ServiceResult {
        result,
        SUCCESS,
        None,
    }
}

```

## Kernel Overview

When writing kernel level drivers or kernel modules, one should use `neutron_api`.
