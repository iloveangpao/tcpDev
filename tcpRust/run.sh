#!/bin/sh

cargo b --release
sudo setcap cap_net_admin=eip /home/iloveangpao/tcpDev/tcpRust/target/release/tcpRust
/home/iloveangpao/tcpDev/tcpRust/target/release/tcpRust &
pid = $!
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
wait $pid