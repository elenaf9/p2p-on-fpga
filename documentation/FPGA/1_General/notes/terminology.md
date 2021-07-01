# FPGAs

## Terminolgy: microprozessor vs microcontroller vs SoC vs ASIC vs FPGA

- microprocessor: = CPU, no RAM or other peripherals: multipurpose, clock-driven, register based [ref](https://en.wikipedia.org/wiki/Microprocessor)
- microcontroller: micro-computer for a specific task, has microprocessor and additionally RAM, serial ports, peripherals, ...
- System on a chip (SoC): integrates most components of a computer like microcontroller, wifi modem, bluetooth, etc. on one chip (e.g Raspberry Pis):
> Similar to how a microcontroller integrates a microprocessor with peripheral circuits and memory, an SoC can be seen as integrating a microcontroller with even more advanced peripherals ([Wikipedia](https://en.wikipedia.org/wiki/System_on_a_chip));
- ASIC: "Application specific integrated circuit": designed for a certain purpose: often include a microcontroller;
- FPGA: "Field Programmable gate array": a general purpose IC with logical elements that can be configured (and always reconfigured) with a HDL: can be programmed simulate a softcore-microprocessor; often a FPGA is used for prototyping and then once a design is verified it's created in hardware as an ASIC; useful in security critical areas to verify the microprocessor (everything is transparent)

## RTL

Source: <https://www.sciencedirect.com/topics/computer-science/register-transfer-level>
- in context of fpga design: __register transfer level__
> low level of abstraction allowing the description of a specific digital circuit. RTL can also be used to mean a hardware description language (VHDL, Verilog, SystemC), where “RTL” code is a lower level of abstraction than “Behavioral Level” code, although both are actually subsets of the full scope of HDL languages.
> level of description of a digital design in which the clocked behavior of the design is expressly described in terms of data transfers between storage element in sequential logic, which may be implied, and combinatorial logic, which may represent any computing or arithmetic-logic-unit logic.
- "the level (i.g. of a HDL) that describes how/when data flows between registers, is combined, ..."
- clocking: "specific internal or external device timing"
- "Simulation only" vs " Behavioral Level" vs "Register Transfer Level":
  - Simulation only: code is not intended for synthesis into an FPGA: abstract, algorithnic modeling of the final FPGA functionality
  - Behavioral Level: describe the chip that is intended to be sythesized, but more abstract, does not imply clocking, architecture-independent
  - RTL: low level, similar to design description on a schematic; fully specified clocks combinatorial logic between flip-flops, "technology independent (retargetable to different device families)" but architecture fixed  
