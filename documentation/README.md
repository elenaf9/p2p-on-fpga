# Documentation

## Introduction

Field Programmable Gate Arrays (FPGAs) are general-purposed chips that consist of a matrix of configurable blocks that allow programming hardware logic with software.  
The central goal of this work with FPGAs is to evaluate how they can be used together with different open-source projects to facilitate the implementation of a Peer-to-Peer network.
The combination of FPGAs and a Peer-to-Peer network serves the purpose of exemplifying how FPGAs can be integrated in the Internet-of-Things.  
Additionally, regarding the Peer-to-Peer network, using FPGAs as hardware also presents the major advantage that the whole application, including its underlying low hardware logic and its firmware, it completely open-source and verifiable.

During the course of the work, the implementation of a Linux-capable system on the FPGA has proven to be more complex than expected, not least because the initially used hardware, the [IOTA Crypto Core (ICCFPGA)](https://gitlab.com/iccfpga-rv) turned out to be unfeasible, and the [Xilinx Arty A7-35T](https://reference.digilentinc.com/reference/programmable-logic/arty-a7/reference-manual) development board had to be used instead.
Eventually, the majority of work was devoted to researching and testing different projects in the ecosystem on the hardware, which included a significant amount of *Trial-and-Error* regarding what core and SoC implementation to use.

## Chances for future work

There are multiple options for continuing the work with FPGAs based on the insights described in this folder.
The least time-consuming one is most likely to continue the work with the Renode simulation of the SiFive Unleashed board (this is briefly described in [Used-Hardware/README.md](Hardware/README.md). With the presumption that the application currently only fails to execute because of the version of the GNU C library, it could be attempted to update its version and then validate that the P2P Network works as expected.
Alternatively for the Arty 35-T board, the [Linux-on-LiteX-Rocket](documentation/FPGAs/3_Linux-on-RISCV/Linux-on-Litex-Rocket.md) project was recently extended with support for the Arty 100-T board, which is a larger version of the 35-T.
It could be tested whether support for the 35-T version could also be possible, though the README.md of the project states:
> The `a7-35` variant is probably too small to fit Rocket.

Potentially, the [Linux-on-LiteX-VexRiscv](documentation/FPGAs/3_Linux-on-RISCV/Linux-on-LiteX-VexRiscv.md) could be used for comparison since this project supports the Arty A7-35T within a LiteX Soc, even though it is using a different soft-CPU.

Independently of any further work on the firmware of the FPGA, Rust already includes initial support for the `riscv32gc-unknown-linux-gnu` target, with 32-bit RISC-V, Linux kernel version 5.4, and GNU C library version 2.33. The Rust standard library is currently not provided for this target.
If support for std can be added, the `Linux-on-LiteX-VexRiscv` project could be tested, provided that the required version for Linux kernal and glibc are satisfy or can be updated.

## Content of this folder

This folder contains documentation and notes that were collected during the research.

Folder / File | Description
-|-
[FPGA](./FPGA) | Documentation of different open-source projects that implement a SoC with RISC-V Soft-CPU on FPGA, and facilitate running Linux.
[Used Hardware](./Used-Hardware) | Description of Trial-and-Error work with the two boards ICCFPGA and Xilinx Arty-A7, in the attempt of implementing a system for which the Rust Standard library is supported.
