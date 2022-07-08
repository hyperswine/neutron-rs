---
layout: default
title: NeutronQuickFS
parent: Neutron
---

## Overview

NQFS is a modified version of exFAT and ext2 meant to store content that hardly changes.

- optimised for smaller partitions

## Design

The design of quickfs is quite simple and optimised for speed on smaller, less volatile partitions.

```rust
struct NQFSSuperBlock;
```
