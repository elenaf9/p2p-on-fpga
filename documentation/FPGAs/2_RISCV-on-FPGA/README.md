# RISC-V CPU and System-on-Chip on FPGA

Based on the in folger [1_General](../1_General) described FPGA architecture and how written HDL code can be realized on the hardware, the necessary building blocks for a soft-CPU are provided.
However, while writing the whole core entirely in VHDL or Verilog is possible, it is an elaborate task and within the recent year, some libraries for meta-programming RTL logic have evolved.
The basic idea behind this meta-programming is that "instead of writing multiple RTL instances, write one instance generator that uses meta-programming for powerful parameterization" ([src][chisel]).  
These so-called *Hardware Generators* are based on a higher-level programming language e.g. Python or Scala, and thus benefit from concepts like object-oriented programming, and features and libraries from the programming language itself.
The library behind it will then generate the respective Verilog or VHDL for it.

It has to be noted that there is neither one standard library for meta-programming FPGAs, nor in general one CPU implementation or project that servers all use-cases and architectures.
Instead, different projects support different core implementations, different boards, and often also not all revisions of one board.
The whole ecosystem is still evolving, which is why it can be rather difficult to immediately find a satisfactory project for a certain use-case.  
In this context the ability to re-program FPGAs has proven to be a major advantage for testing different SoCs and cores.
Taking into account that the Xilinx Arty A7-35T ("Arty") board was targeted in this work, the two relevant projects for this hardware at this point in time are the [LiteX](https://github.com/enjoy-digital/litex) project using the 32-bit [VexRiscv](https://github.com/SpinalHDL/VexRiscv) core, with the associated repository for running Linux on it [Linux-on-LiteX-VexRiscv](https://github.com/litex-hub/linux-on-litex-vexriscv) (this is described in [3_Linux-on-RISCV](../3_Linux-on-RISCV)), and the [SiFive Freedom E300 Platform](https://github.com/sifive/freedom) that implements a 64-bit version of the UC Berkely [Rocket Chip](rocket-chip/Ressources/EECS-2016-17.pdf) for the Arty board.  
VexRiscv and the Rocket Chip are both implementing the RISC-V specification.

## Content
