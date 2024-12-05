#!/bin/bash

BASE_PATH="/local_target/tmp/local/builder"
MODE=$1
case $ACTION in
    compile)
        # Prepare dir structure
        BUILD_PATH="$BASE_PATH"
        mkdir -p "$BUILD_PATH"
        cp -r res/* "$BUILD_PATH"
        rm -rf "$BUILD_PATH/userscrate/"
        rsync -av --progress "code/" "$BUILD_PATH/userscrate/" --exclude tmp > /dev/null 2>&1
        TOML_FILE="$BUILD_PATH/userscrate/Cargo.toml"
        KEY="name"
        NEW_VALUE="userscrate"
        sed -i "/^\[package\]$/,/^\[.*\]$/ s/^$KEY = .*/$KEY = \"$NEW_VALUE\"/" "$TOML_FILE"

        # Handle t-rust dependency on user crate
        T_RUST_LINE="t-rust = { path = \"$TRUST_DOCKER_ABS_PATH/t-rust/\" }"
        if grep -q '^\[dependencies\]' "$TOML_FILE"; then
            if grep -q 't-rust' "$TOML_FILE"; then
                sed -i "s|t-rust.*|$T_RUST_LINE|" "$TOML_FILE"
            else
                sed -i "/^\[dependencies\]/a $T_RUST_LINE" "$TOML_FILE"
            fi
        else
            echo -e "\n[dependencies]\n$T_RUST_LINE" >> "$TOML_FILE"
        fi

        # Update Cargo tomls
        cd "$BUILD_PATH"
        USERSCRATE_TOML_FILE="$BUILD_PATH/userscrate/Cargo.toml"
        RES_TOML_FILE="$BUILD_PATH/Cargo.toml"
        FILES=("$USERSCRATE_TOML_FILE" "$RES_TOML_FILE")
        for FILE in "${FILES[@]}"; do
                if [ -f "$FILE" ]; then
                        sed -i -E \
                            -e "s|(\.\./)+args|$TRUST_DOCKER_ABS_PATH/args|g" \
                            -e "s|(\.\./)+t-rust|$TRUST_DOCKER_ABS_PATH/t-rust|g" \
                            -e "s|(\.\./)+userscrate|\.\/userscrate|g" \
                            "$FILE"
                fi
        done

        # Build
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
