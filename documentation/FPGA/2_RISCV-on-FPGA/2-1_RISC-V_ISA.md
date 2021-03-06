# RISC-V Instruction Set Architecture (ISA)

RISC-V is an open-source specification for an reduced instruction set architecture that provides ISAs for 32bit, 64bit and 128bit architectures, with the goal of it becoming "the industry standard ISA for all computing devices"[ 1].  
Its base integer ISA includes only a minimal amount of instructions, hence the name **Reduced** Instruction Set Computer V, with the intention of it being only a skeleton that can then be extended with standard and custom extensions. The standard extensions are extensions for e.g. multiplying (*M*), atomics (*A*), single/double precision floating-point (*F/D*), or compressed instruction encoding (*C*) for 16-bit instruction formats.
The combination of the *IMAFFD* extensions is often labeled as general-purpose (*G*).  
The architecture implements a clean separation between user and privileged ISAs, and specifies three privileged modes for **U**ser, **S**upervisor, and **M**achine that can be used to implement embedded system with (**M, U**) or without protection (**M**), as well as systems that run Unix-like Operating Systems (**U, S, M**), and even systems with a hypervisior.  
In case of an operating system, a System Binary Interface (SBI) is leveraged that implements the calling convention between the Supervisor/ Operating System and the Supervisor Executing Environment (SEE).
This layer is further discussed in the [Linux-on-RISCV](../3_Linux-on-RISCV) docs.  
Related to this architecture, another important concept behind RISC-V is the abstraction between the Supervisor Execution Environment and the actual hardware that is used, by specifying a Hardware Abstraction Layer (HAL) with virtual device drivers. The Executing Environment uses the HAL to communicate with the Hardware platform, which isolates the details of the platform and allows the Operating System (OS) and applications to be oblivious to it.

The general flexibility that the RISC-V architecture provides concerning the underlying hardware as well as the ISA itself and its extensions, together with the major factor of it being free and open-source, resulted in it becoming a popular architecture for soft-cores. Preserving the generic way of how the RISC-V ISA can be used and extended, the two mentioned cores VexRiscv and the Rocket Chip both follow a Plugin-based approach, but while the goal of VexRiscv is a RISC-V CPU implementation itself, the Rocket Chip Generator aims to create not only the Rocket Core, but also a whole SoC around it.

## (Additional) Resources

[1] Krste Asanovic. *Instruction Sets Want to be Free*. RISC-V Vietnam, 2020. <https://www.youtube.com/watch?v=fCzvkP890KM>
[2] David Patterson. *Instruction Sets Want To Be Free: A Case for RISC-V*. Lecture at University of Washington, 2015. <www.youtube.com/watch?v=mD-njD2QKN0>
[3] The RISC-V Instruction Set Manual, Volume I: User-Level ISA. <http://www2.eecs.berkeley.edu/Pubs/TechRpts/2016/EECS-2016-118.html>
[4] The RISC-V Instruction Set Manual Volume II: Privileged Architecture. <https://www2.eecs.berkeley.edu/Pubs/TechRpts/2016/EECS-2016-161.pdf>
[5] RISC-V Supervisor Binary Interface Specification. <https://github.com/riscv/riscv-sbi-doc>
