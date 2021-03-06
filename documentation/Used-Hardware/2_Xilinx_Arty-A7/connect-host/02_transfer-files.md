# Transfer files to board with tftp

Prerequisites:
- Loaded Linux image to dev-board
- Enabled ethernet connection between host computer and board

## Setup tftp server on host computer

Instructions for Ubuntu [source](https://www.addictivetips.com/ubuntu-linux-tips/set-up-a-tftp-server-on-ubuntu-server/)

```sh
# install prerequisites
$ apt install xinetd tftpd tftp

# configure tftpd
$ mkdir -p /etc/xinetd.d/
$ touch /etc/xinetd.d/tftp
```

- Insert the following configuration in `/etc/xinetd.d/tftp`:

```sh
service tftp
{
    protocol = udp
    port = 69
    socket_type = dgram
    wait = yes
    user = nobody
    server = /usr/sbin/in.tftpd
    server_args = /tftpboot
    disable = no
}
```

- Create the directory for the uploaded / downloaded files:

```sh
$ mkdir /tftpboot

# allow access 
$ chmod -R 777 /tftpboot

# change ownership so that every user has the same rights
$ chown -R nobody /tftpboot

```

- Restart TFTP server software

```sh
systemctl restart xinetd.service
```

### download a file to the board

- Use the tftp command from BusyBox

```sh
tftp [OPTIONS] HOST [PORT]
Transfer a file from/to tftp server
Options:
        -l FILE Local FILE
        -r FILE Remote FILE
        -g      Get file
        -p      Put file
        -b SIZE Transfer blocks of SIZE octets
```

- To download a file to the board, run from the buildroot terminal:

```sh
tftp -g -r <file-name> 192.168.1.100:69 
```
