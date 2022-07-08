---
layout: default
title: Rei Shell
parent: Neutron
---

## Overview

Rei shell is a shell application with a builtin gui as well as a default vga text buffer output.

On a default build, one can expect:

- `reis` to be the default terminal emulator, stored in `/packages/reis`
- `rei service` to be online and accepting socket connections via ssh

## Commands

Rei shell (reis) is an interpreter for rei with direct access to Neutron services and functionalities such as pipes, builtin commands and autocompletion.

Unlike standard reic, reis is interpreted and is compiled with NeutronAPI directly.

## Regex-like

I like regex. `r""` can be used as a shortform for `regex""` or `regex()`. By default, we use a form of enhanced search that kinda looks like sql, bash and google search:

`""` => exact search

`*` => wildcard

`?` => zero or one of the prev group

`:` => sub specification, e.g. `with date: before 2022`

So we make good use of natural language.
