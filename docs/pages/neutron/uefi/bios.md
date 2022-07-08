---
layout: default
title: BIOS
parent: UEFI
grand_parent: Neutron
---

## BIOS?

Basic Input/Output System. It is a legacy specification that a lot of system firmware implemented in the past. Though I think UEFI actually includes BIOS as part of its spec so it is still part of newer systems.

## Concepts

### Core

ASCIIZ => zero terminated byte array. Each byte interpreted as an ASCII char

BDA => Bios Data Area. In RAM. Used by BIOS to manage various peripherals and resources on the system. Like RAM, CPU, MMIO etc. Starts at the segment address 0x40 and maybe lasts like a few MB

### Devices

BAID => BIOS aware IPL device. Any device that can boot an OS. Requires BIOS to have specific code to support it. So disk drives, ssds, floops, cd roms, network adapters

IPL Device => Initial Program Load Device. Any device in the system that can boot and load an OS

Legacy Card => Any standard ISA card that contains no PnP compatibility. So no PnP expansion header

PnP => Plug and Play. Anything defined by the Plug and Play BIOS spec or the PnP ISA spec. PnP cards contain an option ROM with a PnP expansion header

PFA => PCI Function Address. A unique number assigned to a PCI function on a PCI device. Consists of a function number, device number, bus number

CSN => Card Select Number. Identifies a PnP ISA card. Used to communicate to that card. Assigned by the PnP BIOS program to each PnP ISA card plugged in the system

CDR => Conflict Dection Resolution. Method by which a PnP BIOS detects the resource requirements for PnP cards. Then allocates them in a conflict free way. Like if they want a section of RAM for MMIO or some resource ID that may already been taken

### Pointers

BCV => Boot Connection Vector. Just a pointer that points to code inside the option ROM. This option ROM performs device initialisation and detects if a peripheral is attached. Then hook INT 0x13 if needed. The BCV resides in a PnP option ROM expansion Header. E.g. an SCSI (kinda like PCI) controller

BEV => Bootstrap Entry Vector. A pointer that points to code inside an option ROM that will directly load an OS. The BEV resides in a PnP option ROM expansion Header. E.g. an ethernet controller

DV => Disconnect Vector. A pointer that points to code inside the option ROM. This code will perform cleanup after the BCV. It resides in a PnP option ROM expansion header. The SCSI controller also has a DV

### Programs

POST => Power on Self Test. Part of the BIOS program. Takes control immediately after the computer turns on. Initialises the computer hardware so that an OS can be loaded

Setup => The setup program part of the BIOS program. Executed after a user specified a specific hot key during BIOS initialisation. Allows user to setup and configure the system and select the IPL priority of the system

DDIM => Device Driver Initialisation Model. A method of initialising an option ROM whereby the option ROM is first copied to shadow RAM. Then its initialisation vector is called with shadow RAM write-enabled. When the option ROM completes initialisation, it may dispose of code not needed at runtime. By resizing the ROM memory footprint. Finally the option ROM returns and the BIOS regains control, the ROM is write protected

Shadow RAM => just a section of ram that is reserved for the BIOS. Copy the stuff in ROM to it so it can be accessed more quickly, up to like 2x faster (10ns for CL16 3200MHz).
