# VexRiscv

VexRiscv is a 32-bit Soft-CPU that implements the RISC-V instruction set architecture.
It is written in [SpinalHDL](https://spinalhdl.github.io/SpinalDoc-RTD), which implements a hardware description API in the general purpose programming language Scala.  
SpinalHDL provides abstract Scala classes that can then be extended for the desired peripherals, one example for such a class is the abstract *IMasterSlave* class that can be extended to implement e.g. a custom bus, or an existing one like the APB3 bus can be configured and used.  
The basic VexRiscv core implements a RV32I ISA with 5 pipeline stages, everything else is added via plugins. It provides fully implemented plugins for e.g. MMU, MUL/DIV, instruction and data caches, interrupt and exception handling, shift instructions, bypass or interlock hazard logic and debugging with via JTAG. The implementer only has to add the desired plugins into the VexRiscvConfig and can add custom Plugins following the VexRiscv Plugin API.

Example of a VexRiscv configuration:

```py
val plugins = ArrayBuffer[Plugin[VexRiscv]]()
plugins ++= List(
    new IBusCachedPlugin(...),
    new DBusCachedPlugin(...),
    new MmuPlugin(...),
    new DecoderSimplePlugin(..),
    new RegFilePlugin(...),
    new IntAluPlugin,
    new SrcPlugin(...),
    new FullBarrelShifterPlugin,
    new HazardSimplePlugin(...),
    new BranchPlugin(...),
    new CsrPlugin(...),
    new YamlPlugin(...)
    new MulPlugin,
    new DivPlugin,
    new ExternalInterruptArrayPlugin(...),
    new DebugPlugin(..)
)
val cpuConfig = VexRiscvConfig(plugins.toList)
val cpu = new VexRiscv(cpuConfig)
```

(*Source: Implementation of the VexRiscv core within [LiteX](https://github.com/enjoy-digital/litex)*)

## (Additional) Resources

[1] VexRiscv. <https://github.com/SpinalHDL/VexRiscv>  
[2] SpinalHDL. <https://spinalhdl.github.io/SpinalDoc-RTD>  
[3] Charles Papon. *SpinalHDL: Software generated hardware*. <https://www.youtube.com/watch?v=Ee7mQDVSHW8>  
