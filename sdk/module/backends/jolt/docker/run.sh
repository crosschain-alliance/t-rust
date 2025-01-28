#!/bin/bash

BASE_PATH="/jolt_target/tmp"
MODE="release"
case $ACTION in
   compile)
        rm -rf "/jolt_target/t-rust/target"
        BUILD_PATH="$BASE_PATH"
        mkdir -p "$BUILD_PATH"
        cp -r res/* "$BUILD_PATH"
        rm -rf "$BUILD_PATH/userscrate/"

        rsync -av --progress "code/" "$BUILD_PATH/userscrate/" --exclude tmp > /dev/null 2>&1
        BUILDER_TOML_FILE="$BUILD_PATH/Cargo.toml"
        sed -i 's|\.\.\/\.\./\.\./\.\./\.\./t-rust/|../../../t-rust/|g' "$BUILDER_TOML_FILE"
        sed -i 's|\.\.\/\.\./\.\./\.\./\.\./userscrate|./userscrate|g' "$BUILDER_TOML_FILE"

        KEY="name"
        NEW_VALUE="userscrate"
        USERSCRATE_TOML_FILE="$BUILD_PATH/userscrate/Cargo.toml"
        sed -i "/^\[package\]$/,/^\[.*\]$/ s/^$KEY = .*/$KEY = \"$NEW_VALUE\"/" "$USERSCRATE_TOML_FILE"
        T_RUST_LINE="t-rust = { path = \"$TRUST_DOCKER_ABS_PATH/t-rust/\" }"
        if grep -q '^\[dependencies\]' "$USERSCRATE_TOML_FILE"; then
            if grep -q 't-rust' "$USERSCRATE_TOML_FILE"; then
                sed -i "s|t-rust.*|$T_RUST_LINE|" "$USERSCRATE_TOML_FILE"
            else
                sed -i "/^\[dependencies\]/a $T_RUST_LINE" "$USERSCRATE_TOML_FILE"
            fi
        else
            echo -e "\n[dependencies]\n$T_RUST_LINE" >> "$USERSCRATE_TOML_FILE"
        fi
		echo $(ls)

        cd "$BUILD_PATH"
        sed -i 's|\.\./\.\./t-rust/|../../../../t-rust/|g' "$USERSCRATE_TOML_FILE"
        BIN_FOLDER_PATH="/jolt_target/tmp"
        RUNNER_TOML_FILE="$BIN_FOLDER_PATH/Cargo.toml"
        sed -i 's|\.\.\/\.\./\.\./\.\./\.\./args/|../../../args/|g' "$RUNNER_TOML_FILE"
	    cd guest
        sed -i 's|\.\.\/\.\./userscrate|../userscrate|g' Cargo.toml
      
        if [ "$MODE" == "release" ]; then
            VER_KEY=$(cd "$BIN_FOLDER_PATH" && RUST_LOG=info cargo build --release --verbose)
            echo "$VER_KEY" | tr -d '\n'
        else
            echo "[!] Mode not valid!"
        fi
        exit
        ;;
    run)
        BIN_FOLDER_PATH="${BASE_PATH}/target/release"
        if [ -d "$BIN_FOLDER_PATH" ]; then
            if [ "$(ls -A $BIN_FOLDER_PATH)" ]; then
                if [ "$MODE" == "release" ]; then
                    cd "$BIN_FOLDER_PATH" && RUST_BACKTRACE=1 RUST_LOG=info ./host
                else
                    echo "[!] Mode not valid!"
                fi
            else
                echo "[!] please build the target first"
            fi
        else
            echo "[!] please build the target first"
        fi
        exit
        ;;
    benchmark)
        BIN_FOLDER_PATH="${BASE_PATH}/target/release"
        if [ -d "$BIN_FOLDER_PATH" ]; then
            if [ "$(ls -A $BIN_FOLDER_PATH)" ]; then
                if [ "$MODE" == "release" ]; then
                    START_TIME=$(date +%s%N)
                    cd "$BIN_FOLDER_PATH" && RUST_BACKTRACE=1 RUST_LOG=info ./host
                    END_TIME=$(date +%s%N)
                    EXECUTION_TIME=$((END_TIME - START_TIME))
                    echo
                    echo
                    echo "Execution took ${EXECUTION_TIME} ns" | tr -d '\n'
                else
                    echo "[!] Mode not valid!"
                fi
            else
                echo "[!] please build the target first"
            fi
        else
            echo "[!] please build the target first"
        fi
        exit
        ;;
    codehash)
        BIN_FOLDER_PATH="${BASE_PATH}/runner"
        if [ -d "$BIN_FOLDER_PATH" ]; then
            if [ "$(ls -A $BIN_FOLDER_PATH)" ]; then
                if [ "$MODE" == "release" ]; then
                    cd "$BIN_FOLDER_PATH" && RUST_LOG=info cargo run --package sp1-script --bin vkey --release
                else
                    echo "[!] Mode not valid!"
                fi
            else
                echo "[!] please build the target first"
            fi
        else
            echo "[!] please build the target first"
        fi
        exit
        ;;
    *)
        echo "[!] unknown action"
        ;;
esac
