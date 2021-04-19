
 

## Scala
- https://docs.scala-lang.org/overviews/scala-book/prelude-taste-of-scala.html
- high level programming language
- statically typed; supports OOP paradigm and functional paradigm
- result in .class files that run on JVM 


## Spinal HDL
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
--- 
Charles Papon - SpinalHDL Software generated hardware (https://www.youtube.com/watch?v=Ee7mQDVSHW8)

- SpinalHDL not a language, but a HDAPI in Scala
- similar to Chisel, FHDL(used for Migen)
- no logic overhead in the RTL

- peripheral implementation:`Req(Bits(32 bits)))` for variables
- use abstract Scala Classes for acessing peripherals
  - e.g. define abstract Memory Mapping with Read/Write methods
  - define other classes that use this abstract memory mapping -> bus agnostic
  - implement and use actual Memory Mapping (e.g. Apb3MemoryMapper) by extending abstract Memory Mapping class 
  -> creates a bridge between memoryBus and peripheral signals
    ```
    abstract class MemoryMapper {
        val spec = HashMap[Int, ArrayBuffer[Any]]()
        
        def write(target: Bits, address: Int) = ...
        def read(source: Bits, address: Int) = ...

        def build() = ??? // Unimplemented
    }

    class Apb3MemoryMapper(bus: Abp3) extends MemoryMapper {
        override def build() = {...}
    }

    class Something extends Bundle {
        val a, b = UInt(32 bits)

        def readFrom(factory: MemoryMapper) = {
            factory.read(a, address = 0x00)
            factory.read(b, address = 0x044)
        }
    }
    ```

## VexRiscV

Charles Papon - SpinalHDL Software generated hardware (https://www.youtube.com/watch?v=Ee7mQDVSHW8)
- VexRiscv / Plugins -> Pipeling framwork -> Scala/ SpinalHDL
- Modular CPU framework
    ```
    val config = VexRiscvConfig()
    config.plugins ++=List(
        new IBusSimplePlugin(resetVector = 0x80000000l),
        new DBusSimplePlugin,
        new CsrPlugin(CsrPluginConfig.smallest),
        new DescorderSimplePlugin,
        new RegFilePlugin(regFileReadyKind = plugin.SYNC),
        new SrcPlugin,
        new IntAluPlugin,
        new BranchPlugin(earlyBranch = false),
        new MulDivInterativePlugin(
            mulUnrollFactor = 4,
            divUnrollFactor =1
        ),
        new FullBarrelShifterPlugin,
        new HazardSimplePlugin,
        new YamlPlugin("cpu0.yaml)
    )

    new VexRiscv(config)
    ```
- custom plugins can be added :
  ```
  class my Plugin extends Plugin[Vexrisc] {
      override def setup(pipeline: VexRiscv): Unit = {...}
      override def build(pipeline: VexRiscv): Unit = {...}
  }
  ```
