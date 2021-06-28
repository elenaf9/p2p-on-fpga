# Documentation

This folder contains documentation and notes that were collected during the research on the thesis.

Folder / File | Description
-|-
[B.Sc. Thesis](./Bsc-Thesis) | Initial idea for the thesis and tracking of the workflow.
[FPGAs](./FPGAs) | FPGA terminology, architecture and toolchains.
[iccfpga-rv](./iccfpga-rv) | IOTA Crypto Core Manual.
[Rocket Core](./litex-rocket) | Rocket Chip Generator paper, and blog post on Linux on Rocket Chip with LiteX.
[VexRiscv](./litex-vexriscv) | LiteX, VexRiscv, Linux-on-Litex (on the Xilinx Arty A7).
[RISC-V](./RISC-V) | RISC-V Instruction Set Architecture and System Binary Interface.

## Introduction

Field Programmable Gate Arrays (FPGAs) are integrated circuits that fundamentally differ from most other semiconductor devices, because the predominant purpose of an FPGA is not to provide any hardwired design logic, but instead to allow the programming of such logic with software.
The topic of FGPAs is very broad, because this whole concept breaks the typically rather strict barrier between hardware and software.  
Usually, a system relies upon certain functionalities being implemented in hardware.
Mainstream computing mostly uses either high-performant Application Specific Integrated Circuits (ASICs) that are designed for only one specific purpose, or General Purpose Processors (GPP) that already include e.g. a Control Unit, an Arithmetic Logic Unit, and Registers, and also assure that these components are properly wired up in the first place.
An embedded software engineer can build on this hardware to provide a solid basis and to predetermine fundamental aspects like the Instruction Set Architecture (ISA), which in return also gives assurance about the underlying system and its reliability.

Opposite to this, FPGAs are general-purposed chips that only consist of a matrix of configurable blocks, without any intended field of application.
Any desired functionality has to explicitly be programmed into them.
The implementer has to take the full responsibility for the whole system from the low level of combinatorial logic and wires, up to a potential CPU and its ISA, virtual memory, clock frequency, and much more.
The use of FPGAs comes with an increased liability, but this in return also grants a huge empowerment.  
First of all, using FPGAs offers great flexibility that would not be possible otherwise, since there are no predetermined decisions that narrow the amount of use cases. The potential application is primarily limited by the amount of blocks in the FPGA, which means that the same FPGA can be used to implement image processing, cryptocoin mining, or a System on Chip (SoC) with multiple CPUs.  
Additionally, the whole system in its implementation is completely transparent and the software can be published, reviewed and modified, which resulted in a whole ecosystem around FPGAs in the open-source community.
While with hardwired functionalities the user has to trust the vendors technical description and the actual detailed specification is often kept closed and hidden behind an Intellectual Property (IP), the nature of FPGAs allows to write sovereign implementations and to use existing project without being restricted by a licence.
These aspects make FPGAs especially suited for research purposes and prototyping.

The central goal of this work with FPGAs is to evaluate how they can be used together with different open-source projects to facilitate the implementation of a Peer-to-Peer network.
The combination of FPGAs and a Peer-to-Peer network serves the purpose of exemplifying how FPGAs can be integrated in the Internet-of-Things.  
Additionally, regarding the Peer-to-Peer network, using FPGAs as hardware also presents the major advantage that the whole application, including its underlying low hardware logic and its firmware, it completely open-source and verifiable.

During the course of the work, the implementation of a Linux-capable system on the FPGA has proven to be more complex than expected, not least because the initially used hardware, the [IOTA Crypto Core (ICCFPGA)](https://gitlab.com/iccfpga-rv) turned out to be unfeasible, and the [Xilinx Arty A7-35T](https://reference.digilentinc.com/reference/programmable-logic/arty-a7/reference-manual) development board had to be used instead.
Eventually, the majority of work was devoted to researching and testing different projects in the ecosystem on the hardware, which included a significant amount of *Trial-and-Error* regarding what core and SoC implementation to use.

## Chances for future work

There are multiple options for continuing the work with FPGAs based on the insights described in this folder.
The least time-consuming one is most likely to continue the work with the Renode simulation of the SiFive Unleashed board. With the presumption that the application currently only fails to execute because of the version of the GNU C library, it could be attempted to update its version and then validate that the P2P Network works as expected.
Alternatively for the Arty 35-T board, the *Linux-on-LiteX-Rocket* project that currently supports Linux on the Rocket Chip for range of other boards, could be extended with support for the Arty. Because the core is embedded in a LiteX SoC, it could be further analysed how *Linux-on-LiteX-VexRiscv* is implemented for the Arty, and this knowledge could be transferred for using the Rocket Core.

Independently of any further work on the firmware of the FPGA, Rust already includes initial support for the `riscv32gc-unknown-linux-gnu` target, with 32-bit RISC-V, Linux kernel version 5.4, and GNU C library version 2.33. The Rust standard library is currently not provided for this target, but if support for std can be added, the *Linux-on-LiteX-VexRiscv* project could become relevant again.
