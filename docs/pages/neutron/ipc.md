---
layout: default
title: IPC
parent: Neutron
---

## Overview

IN UNIX:

File => stored on disk. Or synthesised on demand by a file server.

Memory Mapped File => file mapped to RAM. Modified by writing to the memory addresses directly instead of outputting to a stream. Basically like a file of a certain size but can be increased / decreased with writes() and lseeks() of larger addresses

Message Queue => kind of like a socket. But preserves message boundaries (no out of bound channels). Should be implemented in the OS. Allows two or more processes to read/write to the message queue by simply binding to the queue in read/write mode

Message Passing => message queues or non-OS managed channels. Basically a 1 - 1 process queue where one sends a message until EOF and so on

Shared Memory => multiple processes are given access to the same block of memory. Can be synchronised with mutexes or semaphores

Pipes => named or anonymous. A named pipe is basically a file but we read/write to its file descriptor rather than stdin/out. An anon pipe is a one way data channel using stdin/out with EOF semantics

Sockets => a standard socket allows data to be sent over a 'network interface'. Which can either be to another process or another computer on the network. Either stream oriented (TCP) or message oriented (UDP). UNIX domain sockets are just sockets where the communication occurs in kernelspace. They use the VFS as their address space (so basically paths to processes like /proc/..). A process references a domain socket as a specific inode and multiple processes can communicate with one socket

Signals => A system message sent from one process to another. Not usually used to transfer data but to remotely command the partnered process. Good for quick actions like kill or sleep / wake another process. Should require higher privilege levels to do. They are also called 'async system traps'

### Message Queue

Observer pattern where a process can register itself into a specified queue number as an observer. If a publisher wants to broadcast their message, they can register a queue with a unique number. And allow processes to subscribe to the queue.

Also allows specific permissions needed for the processes so no eaves dropping.

Can be used synchronously like HTTP or async like AJAX to update a part of a web page with new info on the fly. With HTTP you prob want everything on the page rendered, at least the static elements.

### Socket

A network interface is a point of interconnection between a computer and a private / public network. It can either be virtual or physical in form, like a loopback interface or an actual ethernet interface.

Network interfaces are usually ip addresses. Ipv4:port is the most common. A process can expose an ipv4:port by creating a socket (endpoint) and binding its address / pid to it. The kernel handles the rest, or maybe just hands out ip addresses when socket::new() is called. If you are creating a client socket, that means you want to send data first so you should `connect()` to another socket's ipv4:port then `write()` and `read()` as you go along. With EOF semantics.

Locally, its prob `127.0.0.1`. You then bind a the client/server process to a port. If using multiple sockets, use multiple different and unique ports.

## Neutron

I think I will just support channels and sockets.
