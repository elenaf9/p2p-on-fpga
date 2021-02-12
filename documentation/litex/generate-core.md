# Generating a core with litex
source: https://www.youtube.com/watch?v=nJ1rkUn-pUA
migen / nmigen (nmigen = new migen):
- litex still based on migen, not ported to nmigen yet
- python generator for HDL logic

generating the SoC:
- main script puts together base SoC, sets yosys config, builder, programm bitstream file to hardware
  - can generate the SoCs documentation
  - can generate headers for C and Rust by generating a svd file that then can be used by Rust / C
- uses migen platform definition that is FPGA specific (contains platform specific Pins etc.)
- can use different cores (e.g. vexriscv), default is lite core
- can add debug configuration to debug core with gdb
- add BIOs for fpgas with e.g. DDR3 memory, but can be disabled to jump to a fixed address on start up
- cpu is attached to perihperals with a wishbone bus, wishbone bus can be manipulated with wishbone bridge (if debug is enabled)
- instanciate core
- instanciate clock:
  - power-and-reset-clock-domain/signal required as minimum, feeds the system at 12MHz
  - can use higher frequency by automatically instanciating pll
  - could easily create multiple clock domains with migen
  - instanciate system clock (can be same as power-and-reset-clock)
- can travers the whole system tree / architecture
- can instanciate additional submodules like SRAM, SPI-Flash and attach them to a certain address
  - spi flash is used to load the bitstream file to the fpga, but can be used afterwards e.g. as memory
  - add rim linker region with the offset since the first part of SPI flash includes the bitstream, and the programm should be loaded behind it instead of overwriting that memeory (`self.mem_map["spiflash"] + flash_offset`)
  - use `CSRField` functionality for e.g. an LED to create an API for C / Rust to access it (CSR = "config and status registers")
- default 8bit addresses
  
running the SoC
- running yosys, placement and routing of the SoC, programming the hardware