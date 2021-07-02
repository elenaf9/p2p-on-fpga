# Running the P2P-Network on actual Hardware

The P2P Network uses the Rust Standard Library (std).
Std is required for the abstraction over the underlying platform and to use the core I/O functionality like the network primitives for TCP communication.
At the current point in time, Rust only supports std on RISC-V with the target `riscv64gc-unknown-linux-gnu`, which requires a 64-bit RISC-V system with Linux kernel version 4.20 and the GNU library C version 2.29.  
So far, two different boards with FPGAs have been used for the practical test of different open source projects for FPGA firmware (described in [2_RISCV-on-FPGA](../FPGA/2_RISCV-on-FPGA) and [3_Linux-on-RISCV](../FPGA/3_Linux-on-RISCV)), to actually run the P2P-Network on the target hardware.  
The first approach has been to use the IOTA Crypto Core FPGA (ICCFPGA), but during the work on the ICCFPGA, the implementation of a Linux kernel turned out to be impossible since the 128kb RAM of the ICCFPGA are not sufficient enough for it.
Rust also provides support for RISC-V baremetal targets. Such a program was written for test purposes, the source code and further information is described in [hello-baremetal-world](../../hello-baremetal-world).  
It would have been an option to continue with that baremetal application and use e.g. the UART for communication and a host system that would wrap and forward the messages into the network.
However, this would have meant that the main focus would be on the Rust application and the RISC-V CPU, rather than on the FPGA itself.  
Therefore the choice was made to instead switch to using the Xilinx Arty A7-35T development board and continue on the FPGA aspect of the work.
The Arty was selected because it is among the boards that are supported by [Linux-on-LiteX-VexRiscv](../FPGA/3_Linux-on-RISCV/README.md), and because it additionally includes an Ethernet MAC that is required for the network communication.
The Linux-on-LiteX-VexRiscv implementation was successfully tested, and it was possible to gain terminal access over the USB connection and configure network connection from it. The instructions for this are provided in [2_Xilinx_Arty-A7/connect-host](2_Xilinx_Arty-A7/connect-host).

It went unnoticed until this point, that Rust currently only supports using the Rust std library for 64-bit RISC-V with Linux kernel, thus even with a Linux kernel, the 32-bit VexRiscv implementation turned out to not be satisfactory.  
To fulfil the requirements for 64-bit RISC-V core, an subsequent attempt could be done to use an implementation of the [Rocket Chip](../FPGA/2_RISCV-on-FPGA/2-4_Rocket-Chip.md), either based on the Freedom E310, or by adding support for the Arty A7 to the [Linux-on-LiteX-Rocket](../FPGA/3_Linux-on-RISCV/Linux-on-Litex-Rocket.md) project that implements a Linux on a LiteX SoC using the Rocket Chip, but currently only supports other boards.  
However, so far this was not done, and instead an existing hardware emulation using [Renode](https://renode.io) was tested.
Renode provides the necessary platform support and script to simulation the [SiFive Highfive Unleashed](https://www.sifive.com/boards/hifive-unleashed) with the Rocket Chip and a Linux kernel implemented with the [Freedom U SDK](https://github.com/sifive/freedom-u-sd).
The emulation was successful and a test application that has been compiled for the `riscv64gc-unknown-linux-gnu` platform was loaded into the system, but due to the system using the GNU library C version 2.26 instead of the required version 2.29, the execution of the rust program failed in this case as well.

**TL;DR: So far no system on an FPGA that fulfills all requirements was accomplished.**

## Content

Folder / File | Description
-|-
[1_IOTA-Crypto-Core](./1_IOTA-Crypto-Core) | Initial work with the IOTA Crypto Core FPGA.
[2_Xilinx_Arty-A7](./2_Xilinx_Arty-A7) | Xilinx Arty A7-35T development Board.
[2_Xilinx_Arty-A7: connect host](./2_Xilinx_Arty-A7/connect-host) | Connecting the Arty with a host computer and transferring files, after the [Linux-on-LiteX-VexRiscv project](../FPGA/3_Linux-on-RISCV/README.md) was loaded to it.
