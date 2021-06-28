# Implementation of a Peer-to-Peer Network on an Field Programmable Gate Array

This repository contains the source code that was written as part of my Bachelor thesis "Implementation of a Peer-to-Peer Network on an Field Programmable Gate Array".
The inital goal for the bachelor thesis was the implementation of a RISC-V soft-core on an FPGA, as platform to run the Peer-to-Peer network.
This repository contains a baremetal Rust programm for the [IOTA Crypto Core FPGA](https://medium.com/@punpck/iota-crypto-core-fpga-final-report-77cc6a4aec9a), and the Proof-of-Concept of the Peer-to-Peer network.  
_Note: Executing the Rust programm requires that the Rust toolchain is installed, the instructions for this are availabe [here](https://rustup.rs/)_.  
Apart from that, the repository includes some collected notes and the document for tracking the work in this thesis. It has to be noted that moste of these documentation are more a collection of information, rather than well-formulated descriptions.
The comprehensive description of the relevant topics will be added in the form of chapters from the bachelor thesis paper, once the thesis was officially examined.

## Content

Folder / File | Description
-|-
[p2p-network](./p2p-network) | The Proof-of-Concept of the Peer-to-Peer network.
[hello-baremetal-world](./hello-baremetal-world) | The Rust embedded application that was written for the ICCFPGA.
[rust-libp2p](https://github.com/elenaf9/rust-libp2p/tree/cross-compile/riscv64-linux) | Submodule: fork of the libp2p project with minor removals to support the `riscv64gc-unknown-linux-gnu` target.
[workflow](./documentation/BSc-Thesis/workflow.md) | Log / Tracking of the practical work with the hardware.
[testing Linux-on-Litex-VexRiscv](./documentation/litex-vexriscv/testing-linux/) | Instructions for routing network access and transfering files to the Arty board after loading Linux-on-Litex-VexRiscv.
[documentation](./documentation) | Documentation and notes that were collected during research.

## Abstract

  *Field Programmable Gate Arrays (FPGAs) are integrated circuits that do not have any application specific functionality, but instead consist of a large amount of logical elements that can be re-programmed after manufacturing. This elementary design allows to use Hardware Description Languages (HDLs) to implement entire processors in software, that mirror the behaviour of a hardwired implementation. In this thesis, the two existing open-source cores VexRiscv and Rocket are described, that both implement the RISC-V Instruction Set Architecture on FGPAs. The cores are discussed together with projects that design System-on-Chips around them and facilitate running Linux, with the goal of adopting such an existing implementation as platform for a Peer-to-Peer network. The application for the Peer-to-Peer network is written in the Rust programming language and uses the protocols and libraries of the Libp2p framework. For the demonstration of the discussed designs and software on actual hardware, the IOTA Crypto Core FPGA (ICCFPGA) and the Xilinx Arty A7-35T development board are evaluated and tested.*
