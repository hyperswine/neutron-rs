---
layout: default
title: Root NeFS Hierarchy
parent: NeFS
grand_parent: Neutron
---

## Neutron Filesystem Hierarchy

Like Linux and Unix derived OSes, NeFS places system files, binaries, static and shared libraries (for dev or runtimes), newly installed apps in their own separate dirs.

Base hierarchy:

```bash
/
    /home
    /sys
    /packages
    /dev
    /mnt
    /live
    /snapshots
```

Explanations of each dir:

```bash
# ~ redirects to /home (on default single user config)
/home
    # quick access apps and widgets contained in workspaces
    /desktop
        /workspace_0
            .workspace_settings
            /widgets
    /documents
        /downloads
        /images
# system config files, kinda like windows registry and /etc
# combines config for /usr into the system itself, no distinction
# note, no "login manager" per se, just a screen to ensure you are who you say you are
/sys
    /process
    /vars
    /logs
    /config # basically /etc and windows reg type thing (userspace editor)
# installed packages using Neutron Bundler (NB)
# userland should search this dir for the dock and app drawer apps and executables
# executables are auto namespaced by their package name and can be searched from the searcher like "package_name/<some_exe>"
# .apps are kinda like archives with a single executable and an icon + preferences and settings
/packages
    /package_name
        /some_app.app
        /some_app2.app
        /some_exe.elf
        /lib
    # shared libraries, mainly for executables in packages
    /shared_libraries
# mounted devices and null dev for throwing away stuff
/dev
    null
    mouse_<uuid>
    kb_<uuid>
    usb_drive<uuid>
/mnt
    /nvme0p1
    /nvme0p2
    /usb_flash0
# a VFS mainly for storing temporary process data and live logs
/live
    kernel.log
    process.log
/snapshots
    # binary compressed snapshots (CoW) of the root dir (not mounted dirs)
    # just like any other file stored on the rootfs
    root_21-04-2022-15-44.snapshot
```
