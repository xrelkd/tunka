#!/bin/sh

CONFIG_FILE="/config.ovpn"
AUTH_FILE="/auth.txt"

echo "Starting OpenVPN client as a daemon"
if [ -e $AUTH_FILE ]; then
  openvpn --config $CONFIG_FILE \
    --auth-user-pass $AUTH_FILE \
    --auth-nocache \
    --daemon
else
  openvpn --config $CONFIG_FILE \
    --auth-nocache \
    --daemon
fi

echo "Starting Tunelo $(tunelo version)"
exec tunelo socks-server --ip 0.0.0.0 --port 8118
