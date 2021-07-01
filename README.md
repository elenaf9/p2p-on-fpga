# Implementation of a Peer-to-Peer Network on a Field Programmable Gate Array with RISC-V CPU

Field Programmable Gate Arrays (FPGAs) are integrated circuits that do not have any application specific functionality, but instead consist of a large amount of logical elements that can be re-programmed after manufacturing. This elementary design allows to use Hardware Description Languages (HDLs) to implement entire processors in software, that mirror the behaviour of a hardwired implementation.  
The goal for this project is the implementation of a RISC-V soft-core on an FPGA, as platform to run a Peer-to-Peer network.
The application for the Peer-to-Peer network is written in the Rust programming language and uses the protocols and libraries of the Libp2p framework. At the current point in time, the Rust Standard Library on a RISC-V platform is only supported for the `riscv64gc-unknown-linux-gnu` target. Hence the requirement for the system on the FPGA is a 64-bit soft-cpu with Linux kernel version 4.20 and glibc version 2.29.  
For the testing of the software on actual hardware, the IOTA Crypto Core FPGA (ICCFPGA) and the Xilinx Arty A7-35T development board are used.

This repository contains a baremetal Rust program for the [IOTA Crypto Core FPGA](https://medium.com/@punpck/iota-crypto-core-fpga-final-report-77cc6a4aec9a), a Proof-of-Concept for a Peer-to-Peer network, and documentation that was written during research.

***Note:** It was not yet successful to run the written P2P-Network on an actual FPGA, since so far the focus has been more on research. For the tested boards, so far no system could be implemented that fulfills all requirements.*

## Content

Folder / File | Description
-|-
[p2p-network](./p2p-network) | The Proof-of-Concept of the Peer-to-Peer network.
[hello-baremetal-world](./hello-baremetal-world) | A baremetal Rust application for the ICCFPGA.
[rust-libp2p](https://github.com/elenaf9/rust-libp2p/tree/cross-compile/riscv64-linux) | Submodule: fork of the libp2p project with minor removals to support the `riscv64gc-unknown-linux-gnu` target.
[documentation](./documentation) | Documentation and notes that were collected during research.
