# Field Programmable Gate Arrays

Field Programmable Gate Arrays (FPGAs) are integrated circuits that fundamentally differ from most other semiconductor devices, because the predominant purpose of an FPGA is not to provide any hardwired design logic, but instead to allow the programming of such logic with software.
The topic of FGPAs is very broad, because this whole concept breaks the typically rather strict barrier between hardware and software.  
Usually, a system relies upon certain functionalities being implemented in hardware.
Mainstream computing mostly uses either high-performant Application Specific Integrated Circuits (ASICs) that are designed for only one specific purpose, or General Purpose Processors (GPP) that already include e.g. a Control Unit, an Arithmetic Logic Unit, and Registers, and also assure that these components are properly wired up in the first place.
An embedded software engineer can build on this hardware to provide a solid basis and to predetermine fundamental aspects like the Instruction Set Architecture (ISA), which in return also gives assurance about the underlying system and its reliability.  
Opposite to this, FPGAs are general-purposed chips that only consist of a matrix of configurable blocks, without any intended field of application.
Any desired functionality has to explicitly be programmed into them.
The implementer has to take the full responsibility for the whole system from the low level of combinatorial logic and wires, up to a potential CPU and its ISA, virtual memory, clock frequency, and much more.  
It has to be acknowledged beforehand, that working with FPGAs confronts the implementer with issues, that would already be solved when using an appropriate GPP. Therefore, in many cases existing hardwired CPUs should be preferred over soft-CPUs, especially there is no plan to ever change or update the low-level logic in the hindsight.
But while the use of FPGAs comes with an increased liability, this in return also grants a huge empowerment.  
First of all, using FPGAs offers great flexibility that would not be possible otherwise, since there are no predetermined decisions that narrow the amount of use cases. The potential application is primarily limited by the amount of blocks in the FPGA, which means that the same FPGA can be used to implement image processing, cryptocoin mining, or a System on Chip (SoC) with multiple CPUs.  
Additionally, the whole system in its implementation is completely transparent and the software can be published, reviewed and modified, which resulted in a whole ecosystem around FPGAs in the open-source community.
While with hardwired functionalities the user has to trust the vendors technical description and the actual detailed specification is often kept closed and hidden behind an Intellectual Property (IP), the nature of FPGAs allows to write sovereign implementations and to use existing project without being restricted by a licence.
These aspects make FPGAs especially suited for research purposes and prototyping.

## Content 

Folder / File | Description
-|-
[1_General](./1_General) | Background knowledge on the general architecture of FPGAs and how they allow to substitute hardwired logic with software.
[1_General: Symbiflow Toolchain](./1_General/Symbiflow-toolchain.md) | Open-source toolchain for the end-to-end synthesis process starting with a Hardware Description Language (HLD), up until the bitstream that is used to program the FPGA.  
[2_RISCV-on-FPGA](./2_RISCV-on-FPGA) | Two Open-Source RISC-V Soft-CPUs (VexRiscv and Rocket Chip) and how they are respectively embedded in a SoC.
[3_Linux-on-RISCV](./3_Linux-on-RISCV) | Projects that facilitate running Linux on an FPGA with RISC-V core.
