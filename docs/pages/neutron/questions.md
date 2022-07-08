---
layout: default
title: Questions
parent: Neutron
---

## Userspace vs Kernelspace

Should more things be in the userspace, or the kernel space? I think generally userspace. Or even the firmware if possible.

A filesystem does not have to reside in kernel space. But from the principle of least responsibility and abstraction building. I think it kinda makes sense.

As long as everything is in one language and is easy to build. Which is doable with rust modules and cargo. That means it doesnt really matter where you write a specific component. Just that it does exist. And isnt in the way of other things.

Basically a case by case basis? Maybe not all filesystem drivers have to be in the kernel or userspace. Some can be in the kernel. Some in userspace. If the code is generally lean and clean. As well as generic enough to support extensions. And easy enough to refactor to be more generic or better. I dont see a big problem.

## Syscalls on Single User vs Multi User

On single user mode, what syscalls are good to have? But, wait, isnt syscalls only good with multi users? Or maybe just in case for programs that you dont fully trust. Esp browsers that may do some funny things. I guess they can just run on wasmer. But I do also want a full OS without quantii. Just that quantii is optional when used with ArcHypervisor.

So rn im assuming literally just one person will be using it natively. When they start neutron vanilla, they have access to a simplified but pretty complete view of the system. Then some startup scripts will run with root permissions. But a poorly worded program or a virus could easily take over and do things you dont want it to do. So the first `init` will set up a shell and DE. The user interacts with the shell or DE. And if a malicious program wants to run and take over, the user should be able to check it first, as it is a non-trusted program.

When using guest containers/software, what needs to be done to ensure safety and noone does something stupid or problematic?

If they use quantii, then they can run a WASM VM on Neutron-WASI hypervisor. Then they have access to multiuser mode. Neutron-WASI uses the core syscalls to implement other features. And wasm software implements things like multiple different users, workspaces, dirs, permissions, ownership of files, groups, etc.

So I think:

- provides structured access so you dont have to do it yourself. Although you could also just implement a version of rust std or libc
- has some level of protection over stupid code. Code that tries to allocate a lot of memory with `brk` could be rejected by kernel analysis routines
