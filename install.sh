#!/bin/bash

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    SHELL_RC="$HOME/.bashrc"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    SHELL_RC="$HOME/.zshrc"
else
    echo "[!] unsupported os: $OSTYPE"
    exit 1
fi
if ! command -v python3 &> /dev/null; then
    echo "[!] please install python3"
    exit 1
fi
if ! command -v pip3 &> /dev/null; then
    echo "[!] please install pip3"
    exit 1
fi
python3 setup.py sdist && pip3 install . --break-system-packages
echo "[+] please run 'source $SHELL_RC'"
echo "[+] t-rust installed successfully!"
