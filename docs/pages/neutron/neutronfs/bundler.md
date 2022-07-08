---
layout: default
title: Neutron Bundler
parent: NeFS
grand_parent: Neutron
---

## Overview

The recommended way to install apps. Packages in quantii should be bundled as `.package` archives.

A package can contain zero or more apps. It needs to have at least one 'object' which is either a:

- executable app file `.app` that contains its own thumbnail, config settings, and files that are small
- executable `.executable` files that may not be self contained, requiring extra folders on the fs to run

If developing an app or library, use the language's packaging system instead. E.g. `cargo`, `npm`, `pip`, `bundle`. These should create their own dirs and package things their own way.

- as well as setting any env variables. It is usually not recommended to install executables through a language's package manager
- recommended to only install deps (usually libs) to the project dir
- any extra tools should be installed by `neutron bundler` and be versioned automatically. To use a tool of a specific version, neutron tools should be executed like `<tool> -use <version>`
