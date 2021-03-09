Charles Papon - SpinalHDL Software generated hardware (https://www.youtube.com/watch?v=Ee7mQDVSHW8)

## Spinal HDL

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
