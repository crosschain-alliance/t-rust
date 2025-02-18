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

echo "Removing existing t-rust containers"
IMAGES=("trust_local_1" "trust_risc0_1" "trust_sp1_1" "trust_jolt_1")
for IMAGE in "${IMAGES[@]}"; do
  CONTAINERS=$(docker ps -a -q --filter "ancestor=$IMAGE")
  if [ ! -z "$CONTAINERS" ]; then
    docker rm -f $CONTAINERS
  fi
done
for IMAGE in "${IMAGES[@]}"; do
  IMAGE_ID=$(docker images -q $IMAGE)
  if [ -n "$IMAGE_ID" ]; then
    echo "Removing image: $IMAGE (ID: $IMAGE_ID)"
    docker rmi $IMAGE
  else
    echo "Image $IMAGE not found, skipping..."
  fi
done

DANGLING_IMAGES=$(docker images --filter "dangling=true" -q --no-trunc)
if [ ! -z "$DANGLING_IMAGES" ]; then
  docker rmi $DANGLING_IMAGES
fi

echo "[+] please run 'source $SHELL_RC'"
echo "[+] t-rust installed successfully!"
