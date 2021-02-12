# Field programmable gate array
https://www.youtube.com/watch?v=gUsHwi4M4xE
- programmable chip to design chips from scratch
- no intended function
- flexible, can do any digital functionality (not analog)
- contains many Logical Elements that can be configured
- sit between interconects that can be joined up by the functions that are implemented in the LE
- Gate Arrays: Consist of configurable NAND gates that can be programmed into anything
- FPGA are more complex, and the interconnects are limited
  - Logical Elements:
    - LUT
    - flipflops
    - Clock line, Reset line
    - Inputs, Outputs
  - I0-Block:
    - can be inputs/outputs
    - voltage standards
    - can contains flipflops
    - connected to the interconnects
- FPGAs are volatile -> can not store their internal configuration
  - but has configuration element that is connected to an external config flash memory that contains the information for the fpga configuration. On boot up the config is loaded into the fpga
- advantages:
  - do anything, field programmable
  - fast and massively parallel, which is not possible with a processor
  - high I/O count
  - can do specific function directly on I/O; no bottleneck pro cessor that limits the speed
- disadvantages
  - expensive, high power,
  - volatile / boot time
  - high pin count/ bga
  - complicated, many traps, complex tools,
  - hard to chose / compare; not common terminology/ architecture

https://www.xilinx.com/products/silicon-devices/fpga/what-is-an-fpga.html
- matrix of configurable blocks connected via programmable intercoonects
- can be reprogrammaed after maufacturing
- most typs are SRAM based which can be reprogrammed as the design eveolves

https://www.digikey.com/en/blog/fpgas-101-a-beginners-guide
- LUT implements an arbitrary logic function
- "building blocks for creating digital circuits"
- main benefits: flexiblity and speed
- can use soft processor

https://commons.wikimedia.org/wiki/File:FPGA_cell_example.png

https://www.youtube.com/watch?v=lrXjuotxqzE
FlipFlops:
- "register"
- 2 In: Clock, Data-In, 1 Out Q(and stuff like Reset-Input)
- D-FlipFlop mostly used
- flipflops keep the state of an fpga
Clock in FPGA: (_____|^^^^^|_____|^^^^^)
- squarewave
- runs though FPGA at a certain frequency, e.g. 1 MHz
- input for all flipflops, drives the fpga
- flipflops are triggered at the edge of the clock (most of the time the risin of the clock from 0 to 1)
Example 1:
DATA:   ___|^^^^^^^^|______________|^^^^^^^
CLOCK:  _____|^^^^^|_____|^^^^^|_____|^^^^^
Q:      _____|^^^^^^^^^^^|___________|^^^^^^
-> Q only goes high on clock trigger ("Only sees Data on rising edge of clock"), clock aligned
Example 2:
DATA:   _______|^^^^^^^|_____________
CLOCK:  _____|^^^^^|_____|^^^^^|_____
Q:           ________________________

https://www.eetimes.com/all-about-fpgas/
- former purpose of fpgas: prototyping asics, achive time-to-marked but will be replaced with asics asap -> can be programmed within a  minute
-> not always replaced with asics anymore
- Configurable logic blocks (CLB): create a small state machine
  - Flipflops: clocked storage elements
  - LUTs: RAM for creating arbitrary cominatorial logic functions
  - multiplexers to route the logic from and to external ressources, allow polarity selections and reset & clear input selection
- Configurable I/O Block
  - bring signal to the chip and send them back off again
  - consists of input- and outputbuffer with thress-state and open collector output controls
  - pull-up and pull-down resistors

- Programmable interconnect
  - transistors turn on / off connection between different lines
  - programmable swtich matrices
  - three-state buffers,
  - global clokc lines: low impedance and fast propagation times: connectioned to the block buffers and each clocked elemetns in each CLB
  - responsible for most delay
- clock circuit
- SRAM vs Antiguse vs Flash
  - SRAM:
    - small static ram for each programming element: 0/1-> turns off/on a swtich
    - chip fabrication plants are optimizing for better performance
    - reprogrammable any number of times
    - but volatile, large routing delay, slower consume more power, less secure
      - programming bitstream has to be loaded into the fpga on each powerup and could be observed
      - bit errors can happen or corruption by a power glitch
  - Antifuse:
    - microscopic structure that normally makes no connection
    - large amount of current during the programming of the device causes the two sides of the antifuse to connect
    - non-static, dont need external device to programm them on powerup
    - small routing delays -> faster, lower power
    - but: complex fabrication process, can not be changed after they were programmed
  - Flash EPROM bits for eacb programming element
    - non-volatile but reprogrammable