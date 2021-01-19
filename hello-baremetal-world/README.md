### Hello baremetal world
A `no_std` rust programm for the [iccfpga-core](https://gitlab.com/iccfpga-rv/iccfpga-core) that configures one LED to blink.   
Load it to the fpga with the following steps:
- Follow the Quickstart Guide of the iccfpga-manual to set up the environment on a Raspberry Pi that is connected to the iccfpga dev-board, and upload the core. 
  If this was successfull, three LEDs should be turned on.
- Run `start_debugger.sh` on raspberry pi (from the iccfpga-utils/raspberry-scrips)
- With rust nightly: Add `riscv32imc-unknown-none-elf` target with rustup, compile `hello-baremetal-world` and copy the output to the home directory of the pi.
- `telnet <Raspberri-pi-ip> 444` from host laptop
- In telnet terminal run the following commands sequentially:   
  \> `halt` > `reset` > `load_image /home/pi/hello-baremetal-world 0x00000000` > `resume`   
One of the 3 LEDs should start blinking now.
