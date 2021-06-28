# LiteX: a Migen/MiSoc based Core/ SoC builder

LiteX is a Migen/MiSoc based Core/ SoC builder that provides the common components of a SoC
[Migen](https://github.com/m-labs/migen) (Milkymist generator) is "a Python toolbox for building complex digital hardware" ([ref](https://github.com/m-labs/migen)), that followed the development of Milkymist, which is a SoC written in Verilog.  
Its basis is its Fragmented Hardware Description Language (FHDL), which consist of a low level system to describe *Signals* and combinatorial and synchronous statements that operate on them.  
A Signal object is similarly used in Verilog or VHDL, and represents a value that is expected to change in the circuit.
**Migen Example:**

```py
from migen import *

led = Signal()

m = Module()
counter = Signal(26)
m.comb += led.eq(counter[25])
m.sync += counter.eq(counter+1)

print(verilog.convert(m, ios={led}))
```

(*Source: [Sébastien Bourdeauducq - Building system-on-chips with Migen and MiSoC](https://archive.fosdem.org/2016/schedule/event/migen_misoc/)*)

**Generated Verilog:**

```verilog
/*Machine-generated using Migen*/
module top(
    output led,
    input sys_clk,
    input sys_rst
);

reg [25:0] counter = 26'd0;

// synthesis translate_off
reg dummy_s;
initial dummy_s <= 1'd0;
// synthesis translate_on

assign led = counter[25];

always @(posedge sys_clk) begin
    counter <= (counter + 1'd1);
    if (sys_rst) begin
        counter <= 26'd0;
    end
end

endmodule
```

The system relies on Python algorithms to build complex structures from the FHDL elements, and already provides Python objects to assemble a design. The above example implements a blinking LED by assigning the LED pin to the last bit of the counter with the `eq` method, and adding this logic to the combinatorial logic of the module `m.comb`. The `m.sync` property on the other hand contains code that should be executed on each clock signal, in this case the increase of the counter.

Additionally, Migen includes modules for the different platforms to enable the access to peripherals, like accessing the user led pin of the Xilinx Arty A7:

```py
from migen.build.platforms import arty_a7

plat = arty_a7.Platform()
led = plat.request("user_led")

# ... some logic

plat.build()
```

The build command attempts to build the bitstream for the targeted platform, in case of the Arty A7 it automatically executes the necessary commands in the Xilinx Vivado software.
The Migen library provides a simulator that is written in Python and interprets the FHDL structure, and methods to generate the Verilog code of the written design and a build system.

Within LiteX, Migen allows to easily import and use the provided components like buses, RAM, UART and wrapper/integration of multiple CPUs. Additionally it facilitates the large ecosystem that evolved around LiteX, with cores for e.g. DRAM, Ethernet and SATA, as well as support for different boards and build backends for Symbiflow and vendor toolchains.
Apart from the VexRiscv core, LiteX also includes the Rocket Chip as a CPU, but at the point of writing this, there is no support for this core for the Arty board.
