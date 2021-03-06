# Linux on Litex with VexRiscv Core

GitHub: [litex-hub/linux-on-litex-vexriscv](https://github.com/litex-hub/linux-on-litex-vexriscv)

For the LiteX SoC with a VexRiscv core, the necessary platform information and configuration has been added for OpenSBI (described in [OpenSBI](./OpenSBI.md)), which facilitates the *Linux-on-LiteX-VexRiscv* project that among other boards also supports the Xilinx Arty A7-35T. Linux-on-LitX-VexRiscv provides pre-compiled files, the necessary instructions and a Makefile to easily build and document the FPGA bitstream of a [LiteX](../2_RISCV-on-FPGA/2-3_LiteX_Soc-Builder.md) SoC with [Vexriscv core](../2_RISCV-on-FPGA/2-2_VexRiscv_Soft-CPU.md), and to load it and Linux and OpenSBI images over serial to the hardware.
The Linux image also embeds [BusyBox](https://git.busybox.net/busybox), which is a set of programs that provide the common UNIX utilities for small or embedded systems, with the claim of being "The Swiss Army Knife of Embedded Linux" [2].

## (Additional) Resources

[1] Linux-on-LiteX-VexRiscv. <https://github.com/litex-hub/linux-on-litex-vexriscv>  
[2] BusyBox. <https://git.busybox.net/busybox>
