"RISC-V Vietnam 2020: 1040 RISC-V: Instruction Sets Want to be Free (Krste Asanović)" (www.youtube.com/watch?v=fCzvkP890KM)

- Why ISA matters: ISA is most important interface in computer system -> where software meets hardware
- ISAs often come and go, if they are propretary, their existence is tied to the companies fortune and will
- one chip has many ISAs for different specialised areas -> each block has its own propetary ISA

RISC-V started in 2010, was released 2014
-"RISC-V International" non-profit organisation with the mission "to accelerate RISC-V adoption with shared benefit to the entire community of stakeholders"
    - manages the RISC-V standard
    - promote RISC-V
- NVIDIA uses RISC-V in all of their GPUs
- "RISC-V is not an open processor; RISC-V is an open specification"
  - many open source cores for RISC-V

DIfferent about RISC-V:
- smaller than other ISAs, probides minimal \
- clean-slate design:  
  - clean separation between user and privileged ISA
  - avoids micro-architecture or technology-dependent ISA
- Modular ISA designed for extensibility as well as specialization
  - small standard base ISA ("skeleton") with multiple standard extensions
  - Parse & variable-length instruction encoding for vast opcode space
  - domain-specific extensions
- Stable
  - base and first standard extensions are frozen
  - additions via optional extensions, not new versions
- Community designed with leading industry/academic experts and software developers
- Continously being extended

New business model
- pick ISA first the pick vendor or build own core
- add own extensions without getting permissions

Goal: "Become the industry standard ISA for all computing devices"

RISC-V coding spaces:
- Standard: defined by the foundation
- Reserved: reserved for futures standard extension
- Custom: for implementer-specific extendsions

David Patterson RISC-V Lecture: https://www.youtube.com/watch?v=mD-njD2QKN0

- Three base integer ISAs, one peer addres width RV32I, RV64I, RV128I; minimal: <50 hardware instructions needed
- standard extensions
  - multiply M
  - atomics A
  - single precision floating-point F
  - Double precision floating-point D
  - G = IMAFD "general pupose ISA"
  - Quad-precosopm floating point Q
  - compressed instruciton encoding 16b and 32b C
- privileged architecture (also hypervised, but not listed below due to not relevant for my thesis), clean split between layers of the software stack
  - embedded device running single application: 
    - Application > Applicaiton Binary Interface (ABI: user ISA + calls to AEE) > Application Execution Environment (AEE)
  - classion OS multiprogramm
    - Application > ABI > OS > System Binary Interface (user ISA + privileged ISA + calls to SEE) > Supervisor Execution Environment (SEE)

- Hardware Abstraction Layer to map device drivers
  - virtual device drivers: one per generic type
  - separtate features for HW platform from EE in HAL ((A/S)EE > HAL > Hardware)
  - execution environments communicate with HW plaforms via HAL
  - details of execution env and hardware platform isolated from OS ports
- 4 supervisore architectures e.g. Mbare for baremetal (no tranlstion or protection), Mbb for base and bounds protection, ..

Krste Asanovis RISC-V Lecture https://www.youtube.com/watch?v=KxuQW8HWBXI
- suits all implementation technologies and all sizes
- native hardware ISA

- privileged modes:
    - User (U)
    - Supervisor (S)
    - machine (M)
  - combinations:
    - M (simple embedded systems) (Mbare)
      - no address translation / protection
      - all code inherently trusted
      - low implementation cost
    - M, U (embeddded systems with protection)
      - M-mode runs secure boot and runtime monitor
      - embedded code runs in U-mode
      - physical memory protcetion (PmP) on U-mode accesses
      - arbitrary number of isolated subsystems
    - M, S, U (systems running Unix-like OS)
      - M mode runs secure boot andmonitor
      - S runs OS
      - U runs applicaiton on top of OS or M-mode

Rocket Core Generator
- 64-bit 5-stage single-issue in order pipeline (IF, ID, EX, MEM, WB)
- MMU supports page-base virutal memory

https://www.youtube.com/watch?v=PNcX3MCAIjo

- SBI (System Binary Interface)
  - calling conventions between Supervisor (S-mode OS) and Supervisor Executing enviroment (SEE)
- OpenSBI: open source SPI implementations
  - Layers: Platform Specific Reference Firmware > Platform Specific Library > SBI Library
  - Provides run-time in M-mode
    - typically used inboot stage following ROM/Loader
    - Provides support for reference platforms
    - Generic simple drivers included for M-mode to operate

https://www.youtube.com/watch?v=mD-njD2QKN0
- RISC: "Reduces Instruction Set"
  - in general, instructions are usually implemented in hardware, reduced means that more instructions are necessary ("more lines of assembly code") since the instruction perform rather basic operations, and more RAM is used, but the hardware is less complex -> emphasis on software ([RISC vs CISC](https://cs.stanford.edu/people/eroberts/courses/soco/projects/risc/risccisc/)); today, only Intel x86 use CISC
  - executes more instructions but fewer clock cycles per instructions -> fast than CISC
  - use fast RAM to build fast instruction cache of user visible instructions, not fixed hardware microroutines
  - IEE Milestone: "The reduced instructions of RISC-I reduced the hardware for instruction decode and control, which enabled a flat 32-bit address space , a large set of registers, operating system, RISC-I influenced instruction sets widely used today (...)"
- open source instruction set architecture (the most common ISAs like ARM and x86 are patented) which is great for SoCs in out "PostPC era"
- OpenRISC: didn't separate architecture and implementation; "too early, came up in the PC era"
- three base integer ISAs, one per address width (RV32I, RV64I, RV128I), <50 hardware instructions needed
- very modular: can optionally add extensions like privileged instructions, instructions for multiplication, compressed instructions, atomic instructions, etc    
https://www.youtube.com/watch?v=kNsWipR1MWM
- simple base, modular, stable, community designed
- clear separation between user and privileged IS; avoids architecture or technology-dependent features
- broad Ecosystem