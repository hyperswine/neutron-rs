---
layout: default
title: Everything is a Function
parent: Neutron
---

## Everything is a Function?

Functional programming patterns allow strong control over mutability of data. Most operations and routines should operate on immutable data and only create new data, never modify existing data. But we do eventually want to modify existing data so we restrict that to mutable routines that are highly controlled.

### Iterators, Map, Reduce, etc

A cool thing about functional programming is a lot of the map/reduce/lambda/accumulate type methods work on Iterator types. We can often represent an operation in more concise and mathematically meaningful forms.

### Functions - First Class Objects

In Neutron, functions are first class objects. Data storage is highly controlled and all operations involve passing around functions and using them to handle service calls, driver routines and abstraction routines.

### Functional Kernel

The entire kernel doesnt have to be based on functional principles. But some key areas / modules where it could be useful are places like io management.

## Everything is a File?

A lot of people are used to the idea of everything being a file. Esp. programmers. It is a very convenient way of interacting with the OS esp through the shell.

Take devices for example. To interact with them you can create a device file to represent an underlying device. Load the driver functions for it. Then when a process makes a `read` call to a readable device file, you can expect it to behave like any other file. The process waits for the device to respond and etc.

Through higher level code like C and Rust however, it starts to lose a bit of meaning. You can just use `std::fs` and etc. to interact with the underlying OS.

## Everything is a URL?

I think its a pretty neat concept. Instead of files we represent most things with a url like `scheme:/location`. By virtual of a HFS it works nicely to access disk files.

Idk how well it'll work for implementing languages and apps though. Which is the other big feature other than the shell. Rust, Go, Kotlin, Python... heavily rely on the standard library in order to do useful things. I think a good way to design a kernel is to allow higher level abstractions like language compilers to easily and efficiently make use of kernel services to do its thing.

## Everything Appears on the Filesystem

If it exists and can/should be interactable through userspace means. It should be on the filesystem. Otherwise it can just be some arbitrary object in memory/on another file for the kernel to use.

That means everything should adhere to unix path conventions (NOT DOS!). And be placed, accessed and shown like any other file. But operations on specific types of files dont need to be the same.

Thats where neutron syscalls comes in. It has a strong differentiation between what you CAN do on a specific file type through syscalls or else you get an error.

### File Types

1. regular file. In whatever encoding like ASCII, UTF-8
2. directory. Stores a bunch of other files, including itself through `.` and its parent `..`
3. symlink. In contrast to a 'hard link' like a dir
4. named pipe. Kinda like a reg file but can only be accessed by processes in the same parent space (or execution space in neutron)
5. domain socket. Kind of like a named pipe but only allow one way data flow from opener -> listener
6. device file. Block files can be opened, lseeked and closed as if you were seeking and reading/writing to disk. Char files though, can only accept a serial stream with `SerialStream` which basically sends the data to it in a stream rather than copy it block by block. In userspace though, its basically the same when you open, close, read, write. Just the kernel manages it

NOTE: no doors or any weird stuff. Also I think it might be cool to merge the concept of the named pipe with the socket. Maybe open a socket in 1 way mode or something.

### Named Pipes

A named pipe is pretty useful for sharing volatile data on the fly. If you want to tell another process something. You can just create a named pipe file somewhere and tell the other process where it is.

Then if one file wants to send the other something. It writes to the file and when its done it writes EOF. Which the other process should then intercept and read from the file until they reach EOF and move on.

Data is volatile so each time the file is read the data should be moved or copied from the virtual file to the process' virtual address space depending on how many processes are reading from it. I guess CoW memory works well here.

### Device Files

Like pipes and any other file. To send data we send a block or stream with an EOF end. Then the kernel handles the syscall and finds the correct driver functions to interact with the device.

When device IO is done, the process carries on like usual.

### Ember File

An ember file is just a regular file that can either be encoded in UTF-8 `.ember` or binary compressed form `.embin`.
