# FPGAs
__Terminolgy: microprozessor vs microcontroller vs SoC vs ASIC vs FPGA__
- microprocessor: = CPU, no RAM or other peripherals: multipurpose, clock-driven, register based [ref](https://en.wikipedia.org/wiki/Microprocessor)
- microcontroller: micro-computer for a specific task, has microprocessor and additionally RAM, serial ports, peripherals, ...
- System on a chip (SoC): integrates most components of a computer like microcontroller, wifi modem, bluetooth, etc. on one chip (e.g Raspberry Pis): 
>  Similar to how a microcontroller integrates a microprocessor with peripheral circuits and memory, an SoC can be seen as integrating a microcontroller with even more advanced peripherals ([Wikipedia](https://en.wikipedia.org/wiki/System_on_a_chip));
- ASIC: "Application specific integrated circuit": designed for a certain purpose: often include a microcontroller; 
- FPGA: "Field Programmable gate array": a general purpose IC with logical elements that can be configured (and always reconfigured) with a HDL: can be programmed simulate a softcore-microprocessor; often a FPGA is used for prototyping and then once a design is verified it's created in hardware as an ASIC; useful in security critical areas to verify the microprocessor (everything is transparent)
__FPGA Clock__
https://hardwarebee.com/ultimate-guide-fpga-clock/
- signal inside any digital circuit that determines how fast a flip flop (or a group of flip flops) runs
- clock signal is connected to all flip flops and RAM blocks and activates them according to the clock frequency
- FPGAs typically consist of several clock signals and thus allow different areas to operate in different speeds
- drives the FPGA design and determines how fast it can run and process data
- refer to PLL to use a single clock domain to generate multiple frequencies of waves and run different components
- PLL: "phase locked loop" takes the reference clock and spins it up to generate a very high frequency -> can manipulate the code to change how often your clock enabler gets pulsed 
- but: multiple clock domains can get difficult if you want to transfer data between them

__RTL__
- https://www.sciencedirect.com/topics/computer-science/register-transfer-level
  - in context of fpga design: __register transfer level__
  >  low level of abstraction allowing the description of a specific digital circuit. RTL can also be used to mean a hardware description language (VHDL, Verilog, SystemC), where “RTL” code is a lower level of abstraction than “Behavioral Level” code, although both are actually subsets of the full scope of HDL languages.
  > level of description of a digital design in which the clocked behavior of the design is expressly described in terms of data transfers between storage element in sequential logic, which may be implied, and combinatorial logic, which may represent any computing or arithmetic-logic-unit logic.
  - "the level (i.g. of a HDL) that describes how/when data flows between registers, is combined, ..."
  - clocking: "specific internal or external device timing"
  - "Simulation only" vs " Behavioral Level" vs "Register Transfer Level":
     - Simulation only: code is not intended for synthesis into an FPGA: abstract, algorithnic modeling of the final FPGA functionality
     - Behavioral Level: describe the chip that is intended to be sythesized, but more abstract, does not imply clocking, architecture-independent
     - RTL: low level, similar to design description on a schematic; fully specified clocks combinatorial logic between flip-flops, "technology independent (retargetable to different device families)" but architecture fixed  

__Scala__
- https://docs.scala-lang.org/overviews/scala-book/prelude-taste-of-scala.html
- high level programming language
- statically typed; supports OOP paradigm and functional paradigm
- result in .class files that run on JVM 


__SpinalDHL__
- [is a set of Scala libraries](https://index.scala-lang.org/spinalhdl/spinalhdl/spinalhdl-core/1.2.1)
- https://www.youtube.com/watch?v=XyDiz3SRogY:
  - embedded into general purpose language
  - generates HDL like Verilog from SpinalHDL classes, can do the same things as VHDL/Verilog, claims that there is [no overhead](https://spinalhdl.github.io/SpinalDoc-RTD/SpinalHDL/About%20SpinalHDL/faq.html)
  e.g.: 
  ```
  import spinal.core._

  object Main extends App{
    SpinalVerilog(new MyToplevel)
  }

  class MyToplevel extends Component {
    val a,b = in UIntU(8 bits)
    val result = out UInt(8 bits)
    result := a + b
  }
  ```
  - [SpinalHDL vs Chisel:](https://github.com/SpinalHDL/SpinalHDL/issues/202): more from an IC hardware engineer PoW: clear module devision and conceptual design in line with hardware development; SpinalHDL is strongly typed

__RISC V__
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

__VexRiscV__
- https://tomverbeure.github.io/rtl/2018/12/06/The-VexRiscV-CPU-A-New-Way-To-Design.html
- https://risc-v-getting-started-guide.readthedocs.io/_/downloads/en/latest/pdf/
- https://github.com/litex-hub/linux-on-litex-vexriscv
