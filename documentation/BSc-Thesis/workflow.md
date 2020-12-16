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
- set environment variables LC_ALL and LANGUAGE 
- research the TDO error:
  - according to [this](https://www.allaboutcircuits.com/technical-articles/introduction-to-jtag-test-access-port-tap/) it is a problem with the JTAG Test data output (TDO)
  - try use another iccfpga v1.1, same problem
  - manual refers to the iccfpga v1.2, I am using v1.1, so maybe that's the problem? Wrote Thomas to ask him about this.
- downloading XILINX Software Vivado, XSDK and etc. with free/ experiment licence to start working on the FPGA from bottom up

__2020-12-15__:
- help from Thomas Pototschnig: 
  - TDO missmatch apparently a know problem because of how Raspberry Pi 4 is initialising it's GPIO pins.
  - Plan B: use Xilinx USB programmer. Problem: I dont have a USB-B cable that is needed for that, will have to order one
  - attempt to give him ssh access to my raspberry, fails because router firewall doesn't allow it  

__2020-12-16__:
- Configured wireguard VPN server on a publicly available address -> remote access to the RPi + ICCFPGA finally works
- Warning: ICCFPGA can cause Raspberry Pi 4s to break, "probably the pmod-pinheaders shorted with the usb / ethernet" 
  - Going to but a Raspberry Pi 3 since apparantly the problem hardly happends there
  - Consider using the dev 2.1 board
    - Benefits: 
       - SMT pin headers that prevent the above mentioned problem
       - have multiple v2.1 board but only 2 v2.0 boards and some of the v2.0 boards are damaged
    - Disadvantage:
       - female headers between Pi and iccfpga-dev-board-v2.1 are attached to the wrong side, I will have to buy new special headers, will take some time
       - maybe other differences that can cause error in the development?? Not really tested yet
