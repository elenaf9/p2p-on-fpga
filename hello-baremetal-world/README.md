# Hello baremetal world

*Note: the programm requires to use the Rust nightly channel.*

A baremetal (`no_std`) Rust programm for the [iccfpga-core](https://gitlab.com/iccfpga-rv/iccfpga-core) that configures one LED to blink.
Load it to the fpga with the following steps:
- Follow the Quickstart Guide of the [iccfpga-manual](./documentation/../../documentation/iccfpga-rv/iccfpga.pdf) to set up the environment on a Raspberry Pi that is connected to the iccfpga dev-board, and upload the core.
  If this was successfull, three LEDs should be turned on.
- Run `start_debugger.sh` on raspberry pi (from the [iccfpga-utils/raspberry-scrips](https://gitlab.com/iccfpga-rv/iccfpga-utils/-/tree/master/raspberry_scripts))
- With rust **nightly**: Add `riscv32imc-unknown-none-elf` target with rustup, compile `hello-baremetal-world` and copy the output to the home directory of the pi:

```sh
$ rustup toolchain install nightly
$ rustup +nightly target add riscv32imc-unknown-none-elf
$ cargo +nightly build 
$ scp target/riscv32imc-unknown-none-elf/debug/hello-baremetal-world <RaspPi-ip-addr>:~/
```

- `telnet <RaspbPi-ip-addr> 444` from host laptop
- In telnet terminal run the following commands sequentially:
  \> `halt` > `reset` > `load_image /home/pi/hello-baremetal-world 0x00000000` > `resume`
One of the 3 LEDs should start blinking now.

**Sources for the Programm Code:**
- [riscv_rt crate](https://docs.rs/riscv-rt/0.8.0/riscv_rt/index.html)
- [Craig J Bishop. *Rust on Risc-V (VexRiscv) on SpinalHDL with SymbiFlow on the Hackaday Supercon Badge*](https://craigjb.com/2020/01/22/ecp5/)
- [Thomas Pototschnig](https://gitlab.com/microengineer18)
