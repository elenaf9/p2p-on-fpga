## Log workflow and current status

__2020-12-12__:
- Connect to raspberry pi via ssh:
  - write `Raspberry Pi OS (32-BIT)` to 16GB SD Card with Raspberry Pi Imager v1.4
  - two partitions on SD card: /rootfs and /boot (there is also a /rootfs/boot directory)
  - add empty ssh file to /boot to enable ssh
  - edit /rootfs/wpa_supplicant/wpa_supplicant.conf according to [this](https://www.raspberrypi-spy.co.uk/2017/04/manually-setting-up-pi-wifi-using-wpa_supplicant-conf/):
  ```
    ctrl_interface=DIR=/var/run/wpa_supplicant GROUP=netdev
    update_config=1
    country=DE

    network={
     ssid="<wlan_ssid>"
     psk="<wlan-psk>"
     scan_ssid=1
    }

  ```
  - copy that wpa_supplicant.conf to /boot to connect pi upon boot
  - scan local network with `nmap -p 192.168.178.0/24` for raspberry pi Ip address
  - connect via ssh to raspberry pi, add my public key to /.ssh/authorized_keys on raspberry   
    -> SSH works most of the time, but somehow not on every start, I often either have to restart the PI or add the ssh and wpa_supplicant.conf again to /boot
- Follow instructions from ICCFPGA-RV _Manual 2.1 Raspberry Pi Quickstart_, but problem with step 10 because the `./download_bin.sh` does not exist.
  - Wrote to author Thomas Pototschnig, his instructions:
   ```
   cd iccfpga-bin
   git checkout master
   git pull
   ```
   And then follow the remaining instructions.
  - Problem with running `./upload_core.sh `:   
  `'../iccfpga-bin/iccfpga_flash.xsvf': No such file or directory`  
   -> changed file to run similar named existing files `/../iccfpga-bin/iccfpga_fpga.xsvf` or `/../iccfpga-bin/iccfpga_spi_flash.xsvf` instead
  - upon flashing file to the fpga get error: `[xsvf.c:224] TDO mismatch` 

__2020-12-14__:
- noticed that I overread the following info in the above link about the wpa_supplicant.conf:
> NOTE: This method to setup WiFi must be completed before you boot this card for the first time. This is the point at which the system checks for the wpa_supplicant.conf file. If you have already booted the card you will need to re-write with a fresh image and continue  
-> Probably the reason why it sometimes didnt connect to the wifi, since I overwrote the image quite often and didn't always immediatly added the necessary files to /boot
- rewrote image to SD card, this time using `RASPBERRY PI OS LITE (32-BIT)` because no need for desktop applications anyway, and adding files to /boot before booting for the first time
- follow the setup steps from __2020-12-12__ again and set environment variable LC_ALL by adding the line `export  LC_ALL=en_US.UTF-8` at the bottom of the `.bashrc` 
- research the TDO error:
  - according to [this](https://www.allaboutcircuits.com/technical-articles/introduction-to-jtag-test-access-port-tap/) it is a problem with the JTAG Test data output (TDO)
  - try use another iccfpga v1.1, same problem
  - manual refers to the iccfpga v1.2, I am using v1.1, so maybe that's the problem? Wrote Thomas to ask him about this.
- downloading XILINX Software Vivado, XSDK and etc. with free/ experiment licence to start working on the FPGA from bottom up

__2020-12-15__:
- help from Thomas Pototschnig: 
  - TDO missmatch apparently a know problem because of how Raspberry Pi 4 is initializing it's GPIO pins.
  - Plan B: use Xilinx USB programmer. Problem: I dont have a USB-B cable that is needed for that, will have to order one
  - attempt to give him ssh access to my raspberry, fails because router firewall doesn't allow it  

__2020-12-16__:
- Configured wireguard VPN server on a publicly available address -> remote access to the RPi + ICCFPGA works
- Gave remote access to Thomas, but unfortunately he was also not able to find the source of the TDO error. The above mentioned problem about how the Raspberry Pi 4 initializes it's GPIO pins is not the reason in my case.
- Warning: ICCFPGA can cause Raspberry Pi 4s to break, "probably the pmod-pinheaders shorted with the usb / ethernet" 
  - Going to but a Raspberry Pi 3 since apparantly the problem hardly happends there
  - Consider using the dev 2.1 board
    - Benefits: 
       - SMT pin headers that prevent the above mentioned problem
       - have multiple v2.1 board but only 2 v2.0 boards and some of the v2.0 boards are damaged
    - Disadvantage:
       - female headers between Pi and iccfpga-dev-board-v2.1 are attached to the wrong side, I will have to buy new special headers, will take some time
       - maybe other differences that can cause error in the development?? Not really tested yet

__2020-12-18__:
- TDO-missmatch error was actually a mechanical error, the iccfpga-module was not exactly horizontal in the socket because the screws were to tight, now solved, flashing the core firmware was sucessfull
- `./start_serial` not successful, error `cannot open /dev/ttyS0: no fuch file or directory`

__2020-12-24__:
- Setup to use Raspberry Pi 3 Model B+ instead of Raspberry to prevent the above problem that a Pi 4 can break. Flashing works as expected.  
- starting serial still not successful

__2020-12-25__:
- Enabled serial interface in the raspi-config file, which I already did at some point before on the PI 4, but seems like I didn't do it correctly/ flashed a new image afterwards and forgot to do it again/ ... 
   - `sudo raspi-config`
   - \> 3 Interface Options \> P6 Serial Port
   - disable shell over serial
   - enable serial port hardware 
- Starting serial picocom terminal (`/iccfpga-rv/iccfpga-utils/raspberry_scripts/start_serial.sh`) now successful, commands described in the quickstart work:
   ```sh
   { "command":"version"}
   # Response
   {"version":"0.07rv","command":"version","duration":0,"code":200}
   ```
- Next step figure out how to run a binary on the soft-riscv-cpu
-> Decide to start from the bottom and get started with Vivado to hopefully eventually understand the whole VexRiscV blackbox

__2020-12-27__:
- try to open the iccfpga-rv project in Vivado (Xilinx Software for development on their FPGAs), but need to update version etc, and right now too complex because I am not familiar with Vivado
- try to connect to virtual cable server on the raspberry pi with Vivado (iccfpga-utils provide a script for that) -> VCS on raspberry works but Vivado somehow doesn't detect it at the Raspberries IP address + port (*Update from 2021-01-05: the problem was that I tried to connect to a "remote server", but the server actually is a local server that connects to a remote Xilinx Virtual Server Cable. The correct steps are `Hardware Manager -> Open Target -> Local Server -> Add Xilinx Virtual Cable -> <host_name>, port: 2542`*))
- Discovered Linux-on-litex-vexriscv, going to look into that further since it is compatible with Spartan 7 (*Update from 2021-1-09: it is compartible with the Arty s7 uses the same Spartan 7 FPGA like the iccfpa, but this doesn't mean that it is compartible with the iccfpga, since the Arty s7 uses different port, and has peripherals like ddram that the iccfpga doesn't have*)

__2020-12-28__:
- follow steps from linux-on-litex-vexriscv on the raspberry pi, but the riscv toolchain is for 64bit and not the 32bit raspberry: `/bin/sh: 1: riscv64-unknown-elf-gcc: Exec format error`.

__2020-12-29__:
- compiling the riscv multilib toolchain myself from the [source](https://github.com/riscv/riscv-gnu-toolchain) for riscv32 on my host Ubuntu laptop
```sh
git clone https://github.com/riscv/riscv-gnu-toolchain.git
cd riscv-gnu-toolchain
# install prerequisites
sudo apt-get install autoconf automake autotools-dev curl python3 libmpc-dev libmpfr-dev libgmp-dev gawk build-essential bison flex texinfo gperf libtool patchutils bc zlib1g-dev libexpat-dev
./configure --prefix=/opt/riscv --with-arch=rv32gc --with-abi=ilp32d
make linux
```
- follow steps from linux-on-litex-vexriscv on my Ubuntu laptop and run LiteX simulation
  - error: No such file or directory: 'images/Image': solved by copying the [precompiled linux and openSBI images](https://github.com/litex-hub/linux-on-litex-vexriscv/issues/164) to the `images/` directory
  - error: `event2/listener.h: No such file or directory`: solved by installing `libevent-dev` 
  - error: `json-c/json.h: No such file or directory`: solve by installing `libjson-c-dev`
  - Simulation worked on my Ubuntu Laptop! Now I have to figure out how to do that on the raspberry pi / load it to the FPGA  

__2021-01-03__:
- downloaded [riscv32-unknown-elf-gcc](https://freebsd.pkgs.org/13/freebsd-armv7/riscv32-unknown-elf-gcc-8.4.0_2.txz.html) to raspberry, simulation from `linux-on-litex-vexriscv` starts, but crashes since it the command `riscv32-unknown-elf-gcc` is still not found
- tried build the bitstream `./make.py --board=arty_s7 --cpu-count=2 --build` but error: `litex.build.generic_platform.ConstraintError: Resource not found: spisdcard:None`
- double checked with iccfpga-rv: `.flash_core` also tries to flash the spi and fails with error `TDO mismatch`. This is not the same error that I was struggling with at the beginning (the other scripts like `upload_core` now work), but this error message also happens on Raspberry PI 4 (I am using RPi 3 though) according to Thomas Pototschnig. Apparently there the problem is that you can only flash as single time after a reset and afterwards this problem happens. Changed the Pi and tried with RPi 4, somehow flashing the iccfpga core works (at least the first time after a boot) but building the bitstream still doesn't work.  
*Update from 2020-09-01: the spisdcard constraint error has nothing to do with the TDO missmatch or anything, but happends because I was building for the arty_s7, which does have the same FPGA as the iccfpga, but completely different ports, therefore the spicard was not found at the port that was hardcoded for the arty s7*

__2021-01-05__
- after talking to Thomas I realized that linux will not run on the iccfpga hardware that I am using because it only has about 260kB of memory, which is not enough for linux. linux-on-litex-vexriscv is compartible with the ArtyS7 board that uses the same  Xilinx Spartan7 XC7S50 FPGA that the iccfpga does, but the ArtyS7 has 16-bits 256MB DDR3 RAM, which the iccfpga hardware does not. Therefore I will shift my focus away from running linux and think about alternative solutions:
   - run rust embedded on VexRiscv with no linux on top: this means that my code would have to support `no_std`, which causes some problems
   - shift my thesis topic and implement the p2p-network on the RaspberryPi, and only use the fpga for encryption by flashing the iccfpga-core without doing any own work with the fpga
   - buy a ArtS7 Board  
-> Right now I will look further into VexRiscv if I can at least get this to run; will talk with my Prof soon to talk about the above problem
- managed to run the virtual cable server with the script `./start_xvc_server.sh` within the raspberry scripts of the iccfpga project, and successfully connected to that with Vivado. This should enable generating and flashing bitstreams to the fpga from a project in vivado
- generated VexRiscv CPU following the VexRiscv README and opened the created VexRiscv.v file in Vivado, but could not run implementation for the selected target `xc7s50ftgb196-1` that should be used according the fpga that I have. The reason for this is that the `xc7s50ftgb196-1` only allows 100 I/O pins, but VexRiscv is using more than that (tried the full VexRiscv as well as the smallest gen, but same error on both). If I try to add the sources of the iccfpga-rv project (and disable the VexRiscv source that i tried before), the same error occurs.
- tried to open the iccfpa-rv in a seperate project in Vivado by opening the project file `iccfpga-core/iccfpga/iccfpga.xpr`, but this causes multiple errors:
  - error: ` [IP_Flow 19-993] Could not find IP file for IP 'mem_128k'.`
  - warning: ` [IP_Flow 19-5097] Unable to determine VLNV from IP file; verify it has the correct syntax:`
  - warning: ` [IP_Flow 19-3577] Failed to recreate IP instance 'design_iccfpga_riscvwrapper_0_0'. Error during subcore creation.`
  - When opening the block design: claims that some ip files are not up-to-date anymore; tried update them but this causes multiple different errors. Read section in the iccfpga-core wiki about the [setup](https://gitlab.com/iccfpga/iccfpga-core/-/wikis/setup.), there it states that Vivado version 2018-2 should be used (I am currently using 2020.2), so I decided to switch to that older version.
- tried to add mem_128k by adding `iccfpga-core/iccfpga/iccfpga.ip_user_files/mem_init_files/mem_128k_rom.mif` as a source, but unfortunately this doesn't fix the issue.
  
__2021-01-09__
- more research on alternative solutions:
  - could use the `no_std` crate [smoltcp](https://github.com/smoltcp-rs/smoltcp) for tcp/ip stack on bare-metal systems and do some hacky things to send the messages over uart to the pi and try to configure the pi to forward the messages between fpga and internet.
  - could buy a [bluetooth interface](http://store.digilentinc.com/pmod-bt2-bluetooth-interface/) for the iccfpga and implement the p2p communication with bluetooth. Would be a way lower range but theoretically possible according to [this](https://rust-embedded.github.io/discovery/11-usart/index.html)
   - buy the [Arty A7](https://store-7gavg.mybigcommerce.com/arty-a7-artix-7-fpga-development-board-for-makers-and-hobbyists/) and not the ArtyS7 that I considered before, since the Arty A7 has ethernet connection. This would mean that my work then would be completely independent of the iccfpa/ IOTA hardware and have the benefit that I can always buy new pmods to extend my project.
- checked again linux-on-litex-vexriscv just to try one last time if there is any chance to run it on the iccfpga: definitely not possible the way that it is right now because the ddram is essential and also the arty\_s7  ports don't match the one's of the iccfpga, so I would have to write a completely new configuration for my platform and ports.
  
__2020-01-10__
- write first `no_std` rust programm for bare metal riscv with instructions from the [riscv_rt crate](https://docs.rs/riscv-rt/0.8.0/riscv_rt/index.html) and another project I found that runs [Rust embedded on VexRiscV CPU](https://craigjb.com/2020/01/22/ecp5/)
  - used memory mapping from iccfpga manual, problem is that I dont know the exact address of led to get it to blink
  - compiled for target `riscv32imac-unknown-none-elf`
- try upload compiled rust program to the iccfpga core according to the manual 9.3 "Uploading RISC-V Firmware to the ICCFPGA module, using OpenOCD on a Raspberry Pi", 
  - no error, but also no LED blinking to verify that it was success
  - manual is quite unspecific what exactly can be loaded into the fpga with this script, could be that my attempt to just upload that compiled rust code is way too naive
- managed to start openOCD server on raspberry pi
  - managed dial it with gdb but openODC accepts and then immediately drops connection, gdb shows warning `warning: No executable has been specified and target does not support determining executable automatically`
  - managed to connect with telnet 

__2020-01-11__
- changed `no_std` rust programm `hello-baremetal-world`:
   - compile with `riscv32imc-unknown-none-elf` instead of `riscv32imac-unknown-none-elf`
   - LED on, on GPIO pin 19 with instruction `*(0xf1030000 as *mut u32) = 1 << 19)`
- don't follow the last steps from the manual, only do the openOCD instructions without following `Listing 5: Unreset RISC-V` via telnet
- LED blinked after following these steps:
  1. upload\_core.sh (on RPi)
  2. start\_debugger.sh (on RPi))
  3. compile rust program on host pc, scp to pi home
  4. `telnet <Raspberri-pi-ip> 444`
  5. `> halt` `> reset` `> load_image /home/pi/hello-baremetal-world 0x00000000` `> resume`

__2021-01-15__
- New Hardware: [Xilinx Arty A7](https://store.digilentinc.com/arty-a7-artix-7-fpga-development-board-for-makers-and-hobbyists/), which is one of the listed board that the linux-on-litex-vexriscv supports and that has an  ethernet socket
- Buidling and loading linux worked after following the steps on their [github repo](https://github.com/litex-hub/linux-on-litex-vexriscv) (for Ubuntu)
  ```
  # install prerequesites
  $ sudo apt install build-essential device-tree-compiler wget python3-setuptools

  # clone linux-on-litex-vexriscv
  $ git clone https://github.com/enjoy-digital/linux-on-litex-vexriscv && cd linux-on-litex-vexriscv

  # install LiteX
  $ wget https://raw.githubusercontent.com/enjoy-digital/litex/master/litex_setup.py
  $ chmod +x litex_setup.py
  $ ./litex_setup.py init install --user (--user to install to user directory)

  # install Risc-V toolchain
  $ wget https://static.dev.sifive.com/dev-tools/riscv64-unknown-elf-gcc-8.1.0-2019.01.0-x86_64-linux-ubuntu14.tar.gz
  $ tar -xvf riscv64-unknown-elf-gcc-8.1.0-2019.01.0-x86_64-linux-ubuntu14.tar.gz
  $ export PATH=$PATH:$PWD/riscv64-unknown-elf-gcc-8.1.0-2019.01.0-x86_64-linux-ubuntu14/bin/

  # build fpga bitstream
  $ sudo -E env "PATH=$PATH" ./make.py --board=XXYY --cpu-count=X --build

  # load fpga bitstream to board
  $ sudo -E env "PATH=$PATH" ./make.py --board=XXYY --cpu-count=X --load

  # Load the Linux images over Serial
  $ sudo -E env "PATH=$PATH" lxterm --images=images/boot.json /dev/ttyUSB1 --speed=1e6
  ```
__2021-1-19__
- Discovered that the rust support for riscv 32 bit linux is not even close to complete, and not ready to use. There are the following alternatives to consider:
  - `riscv32imac-unknown-none-elf` riscv 32 bit baremetal: would be the same issue as with the iccfpga that I can't use the `std` crate then, which is necessary for the p2p-network
  - `i686-unknown-linux-gnu` 32-bit Linux (kernel 2.6.32+, glibc 2.11+): might work and is worth a shot, but I don't count on that
  - `riscv64gc-unknown-linux-gnu` RISC-V Linux (kernel 4.20, glibc 2.29): 64bit architecture which I current don't have, but I found in the [list of projects with LiteX](https://github.com/enjoy-digital/litex/wiki/Projects#a-trustworthy-libre-self-hosting-64bit-risc-v-linux-computer) a project that implements a [64bit RISC-V CPU with Linux](http://www.contrib.andrew.cmu.edu/~somlo/BTCP/) by adding the [Rocket Chip SoC design](https://www2.eecs.berkeley.edu/Pubs/TechRpts/2016/EECS-2016-17.html) to Litex. The project uses the ecp5versa board, I don't know yet how much of an issue it would be to follow the steps on the Arty A7 
- My next steps are:
  1) Try to load baremetal rust code to the fpga together with the linux image according to [this guide](https://github.com/enjoy-digital/litex/wiki/Load-Application-Code-To-CPU)
  2) Test if rust code that is compiled for i686 runs on it
  3) If 2) doesn't work (or if I have some time left) try to implement 64bit linux on the Arty A7.
- Tried to load the compiled `hello-baremetal-world` binary to the fpga by copying it into the image directory and edit the /images/boot.json:
```
{
    "Image":       "0x40000000",
    "rv32.dtb":    "0x40ef0000",
    "rootfs.cpio": "0x41000000",
    "opensbi.bin": "0x40f00000",
    "hello-baremetal-world": "0x41100000"
}
```
-> It loads the binaries to the different addresses and "the last file/address tuple is used for the CPU jump" according to the wiki article.
Produced following output:

```
...
--============== Boot ==================--
Booting from serial...
Press Q or ESC to abort boot completely.
sL5DdSMmkekro
[LXTERM] Received firmware download request from the device.
[LXTERM] Uploading images/Image to 0x40000000 (7279384 bytes)...
[LXTERM] Upload complete (86.2KB/s).
[LXTERM] Uploading images/rv32.dtb to 0x40ef0000 (4747 bytes)...
[LXTERM] Upload complete (72.1KB/s).
[LXTERM] Uploading images/rootfs.cpio to 0x41000000 (4002304 bytes)...
[LXTERM] Upload complete (86.1KB/s).
[LXTERM] Uploading images/opensbi.bin to 0x40f00000 (53640 bytes)...
[LXTERM] Upload complete (84.0KB/s).
[LXTERM] Uploading images/hello-baremetal-world to 0x41100000 (604368 bytes)...
[LXTERM] Upload complete (86.0KB/s).
[LXTERM] Booting the device.
[LXTERM] Done.
Executing booted program at 0x41100000

--============= Liftoff! ===============--
```

->Seems to work but currently not way to verify. Will need to figure out the address of an LED and use that.

__2021-01-22__
- Since in my above approach the CPU directly "jumps" to the programm instad of executing the opensbi.bin before (which is necessary to access the hardware properly), the program did most likely not run at all. I will need to think of another way to load my app binary onto the system

__2021-01-25__
- managed to configure network access for the board (steps are described [here](/documentation/linux-on-litex/02_route-network.md), this could now be used to install software from source
- this linux is using [BusyBox](https://www.commandlinux.com/man-page/man1/busybox.1.html) which is something I will look further into and maybe configure the install packages.

Started programming the p2p-network in Rust by using the libp2p crate:
- combining the following protocols:
   - to upgrade the transport layer:
     - noise: encryption (Diffie-Hellmann encryption, XX Handshake) 
     - yamux: multiplexing
     - pnet:"private network with a preshared key that is read from a file
   - behaviour:
     - multicast DNS: peer discovery within a local network
     - kademlia Distributed Hash Table: Peer discovery and file sharing
     - Request Reponse Protocol: enables direct messages between peers
     - gossip sub: pub sub message layer
- init two tasks at begin that run in separate task and communicate over a channel:
   - one polling from the swarm for incoming message and to forward outgoing request 
   - one polling for user input (right now only stdin, eventually also from switches/ buttons from the dev-board)
- right now only the main structure, not detaile implementation
- not tested on hardware yet

__2020-01-30__
- configured TFTP server on my Ubuntu host computer according to [this](/documentation/linux-on-litex/03_transfer-files.md)
-> managed to load compiled rust project to board
- error executing a hello-world test programm that was compiled for `i686-unknown-linux-gnu"`: `line 1: syntax error: unexpected ")"`
- error executing a the baremetal programm that ran on the iccfpga: 
```
[ 3416.415380] hello-baremetal[137]: unhandled signal 11 code 0x2 at 0x00000000 in hello-baremetal-world[0+1000]
[ 3416.417074] CPU: 0 PID: 137 Comm: hello-baremetal Not tainted 5.10.0 #4                                       
[ 3416.417989] epc: 00000000 ra : 00053254 sp : 9d9d7de0                                                         
[ 3416.419118]  gp : 000e1800 tp : 95b3bef0 t0 : 000039d4                                                        
[ 3416.419851]  t1 : 95a8ddd0 t2 : 959ef934 s0 : 000e2418                                                        
[ 3416.420578]  s1 : 000e23f4 a0 : 00000000 a1 : 000e2418                                                        
[ 3416.421302]  a2 : 000e2424 a3 : 0000002f a4 : 0000002f                                                        
[ 3416.422031]  a5 : 000e23f8 a6 : 7efefeff a7 : 000000dd                                                        
[ 3416.423104]  s2 : 000dbb41 s3 : 000c1884 s4 : 000e2424                                                        
[ 3416.423835]  s5 : 95b3bef8 s6 : 00000008 s7 : 00000000                                                        
[ 3416.424565]  s8 : 00000000 s9 : 000e4610 s10: 000e2418                                                        
[ 3416.425294]  s11: 00000001 t3 : 0009fdd0 t4 : 000de384                                                        
[ 3416.426005]  t5 : 32333b00 t6 : 00000001                                                                      
[ 3416.426877] status: 00000020 badaddr: 00000000 cause: 0000000c                                                
Segmentation fault
```

