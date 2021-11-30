#!/bin/bash
cargo build --release
sudo setcap cap_net_admin=eip $CARGO_TARGET_DIR/release/mtcp
sudo $CARGO_TARGET_DIR/release/mtcp &
pid=$!
echo "pid is : $pid"
sudo ip link set tun0 up
sudo ip addr add 192.168.42.100/24 dev tun0
sudo iptables -t nat -A POSTROUTING -s 192.168.42.0/24 -j MASQUERADE
sudo sysctl net.ipv4.ip_forward=1
trap "sudo kill -9 $pid" INT TERM
wait $pid