# Instructions to route the network traffic from the linux on litex over ethernet cable

Prerequesites: linux image was loaded onto the FPGA, booted and the lxterm shell is open ([instructions](https://github.com/litex-hub/linux-on-litex-vexriscv)).
The default local ip of the fpga is 192.168.1.50 and the host system 192.168.1.100

__On the lxterm shell of board:__
- Configure ethernet (according to the [HOWTO](https://github.com/litex-hub/linux-on-litex-vexriscv/blob/master/HOWTO.md)):

  ```sh
  ifconfig eth0 192.168.1.50
  ```

- configure routing table:

  ```sh
  # add local/ loopback network
  $ route add -net 127.0.0.0 netmask 255.0.0.0 lo

  # set default gateway to ethernet/ host pc
  $ route add default gw 192.168.1.100

  # the following output should be printed
  $ route
  Destination     Gateway         Genmask         Flags Metric Ref    Use Iface
  default         192.168.1.100   0.0.0.0         UG    0      0        0 eth0
  127.0.0.0       *               255.0.0.0       U     0      0        0 lo 
  192.168.1.0     *               255.255.255.0   U     0      0        0 eth0
  ```

- set DNS:

  ```sh
  # add name server
  $ echo "nameserver 8.8.8.8" > /etc/resolv.conf

  ```

__On the host pc:__
- The iptables should be configured to forward the traffic to the wlan. On Ubuntu, this can be done with (replace eth0 and wlan0 with the correct identifiers if necessary):

```sh
# set ip address; after this step the buildroot and host should be able to ping each other
ifconfig eth0 192.168.1.100

# enable masquerading for outgoing connections towards wireless interface
iptables -t nat -A POSTROUTING -s 192.168.1.50 -o wlan0 -j MASQUERADE

# enable IP forwarding; after this step ping 8.8.8.8 should be successful
echo 1 | sudo tee /proc/sys/net/ipv4/ip_forward
```

- allow forwarded dns traffic

```sh
iptables -A FORWARD -s 192.168.1.50 -j ACCEPT
```

- `ping google.com` should now work from the board
