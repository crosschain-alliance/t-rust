#!/bin/bash

BASE_PATH="/local_target/tmp/local/builder"
MODE=$1
case $ACTION in
    compile)
        BUILD_PATH="$BASE_PATH"
        mkdir -p "$BUILD_PATH"
        cp -r t-rust/ "$BUILD_PATH"
        cp -r res/* "$BUILD_PATH"
        cp -r args/ "$BUILD_PATH"
        rm -rf "$BUILD_PATH/userscrate/"
        rsync -av --progress "code/" "$BUILD_PATH/userscrate/" --exclude tmp > /dev/null 2>&1
        TOML_FILE="$BUILD_PATH/userscrate/Cargo.toml"
        KEY="name"
        NEW_VALUE="userscrate"
        sed -i "/^\[package\]$/,/^\[.*\]$/ s/^$KEY = .*/$KEY = \"$NEW_VALUE\"/" "$TOML_FILE"
        cd "$BUILD_PATH"
        RES_TOM_FILE="Cargo.toml"
        sed -i 's|\.\./\.\./\.\./\.\./args/|./args/|g' "$RES_TOM_FILE"
        sed -i 's|\.\./\.\./\.\./\.\./t-rust/|./t-rust/|g' "$RES_TOM_FILE"
        sed -i 's|\.\./\.\./\.\./\.\./userscrate|./userscrate|g' "$RES_TOM_FILE"
        if [ "$MODE" == "release" ]; then
            cargo build --features=t-rust/local --release
        else
            cargo build --features=t-rust/local
        fi
        BIN_PATH="${BASE_PATH}/target/${MODE}/local"
        CODEHASH=$(sha256sum "$BIN_PATH" | cut -f1 -d' ')
        echo "$CODEHASH" | tr -d '\n'
        ;;
    run)
        BIN_PATH="${BASE_PATH}/target/${MODE}/local"
        if [ -x "$BIN_PATH" ]; then
            "$BIN_PATH" | tr -d '\n'
        else
            echo "[!] please build the target first"
        fi
        exit
        ;;
    codehash)
        BIN_PATH="${BASE_PATH}/target/${MODE}/local"
        if [ -x "$BIN_PATH" ]; then
            CODEHASH=$(sha256sum "$BIN_PATH" | cut -f1 -d' ')
            echo "$CODEHASH" | tr -d '\n'
        else
            echo "[!] please build the target first"
        fi
        exit
        ;;
    *)
        echo "[!] unknown action"
        ;;
esac
