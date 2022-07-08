---
layout: default
parent: Logic Design
title: Combinational Circuits
grand_parent: Spectre Hardware
---

## Overview

Basically, it is a circuit that combines different gates. Examples of combination circuits include encoders, decoders, multiplexers and demuxes.

![](/assets/img/combinational_circuit_blockdiagram.jpg)

Muxes take multiple inputs and 1 ouput. Demuxes take 1 input and multiple outputs.

### Decoders

Basically we have n inputs and up to 2^n outputs.

Stuff like BCD -> 7 segment circuits are examples of decoders.

### Encoder

Reverse of a decoder. So up to 2^n inputs and therefore n outputs.

Decimal -> BCD is an encoder. Also octal/hex -> binary are encoders too. Notice how you have a lot of stuff in the output sometimes, as 1 hex = 4 bits. Since hex has 16 possible vals and only one line should be on at a time and binary is only 2 and all lines can be on at the same time. So a 1:4 is actually a 16:4.

E.g. 0x400 to binary is 0100 0000 0000. So you start with 3 * 16 inputs and you need 12 outputs.

Actually no. You basically have 4 bits out thats true, each bit is 0/1 so 4 lines. But in you need 16. So you have 16 in and 4 out.
