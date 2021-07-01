# Instructions for routing network access and transfering files to the Arty board after loading Linux-on-Litex-VexRiscv

This folder contains the instructions for configuring internet access on the board and transfering files from the host computer. The pre-requisite is that the [Linux on LiteX-VexRiscv](https://github.com/litex-hub/linux-on-litex-vexriscv) has been loaded to the board by following the instructions of their GitHub Readme, and that the board is connected to a host computer via Ethernet.
It was tested on the [Xilinx Arty A7-35T](https://store.digilentinc.com/arty-a7-artix-7-fpga-development-board/) with a host computer that runs Ubuntu 20.04.

File | Description
-|-
[route network](./01_route-network.md) | Configure the routing table to use internet over Ethernet to host computer.
[transfer](./02_transfer-files.md) | Transfer files from the host computer to the board via TFTF.
