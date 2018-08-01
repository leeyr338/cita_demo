#!/usr/bin/env bash

current_dir=$(pwd)
echo "${currdir}"

protobuf_dependencies="
    protoc
"

proto_files="
    auth.proto
"

function check_dependencies {
    local l_dependencies="$1"
    for bin in ${l_dependencies}; do
        if [ ! -x "$(which ${bin})" ]; then
            echo -e "\033[31m[Error] Please check if you have installed \033[43m${bin}\033[0m in your \$PATH. \033[0m"
            echo "See dependense decription in README.md for more detail!"
            exit 1
        fi
    done
}

rust_dependencies="
    ${protobuf_dependencies}
    protoc-gen-rust
    protoc-gen-rust-grpc
"

function gen_rust {
    check_dependencies "${rust_dependencies}"
    echo -e "Generate\033[33m rust\033[0m files from protos:"

    # Support for rust sdk, needs only cita_node_intf_files for now.
    for protofile in ${proto_files}; do
        rm -f ../${protofile/%.proto/.rs}
        protoc --rust_out=${current_dir}/.. ${protofile}
        echo "  ${protofile} -> ${protofile/%.proto/.rs}"
    done

    # Generate grpc
    for protofile in ${proto_files}; do
        rm -f ../${protofile/%.proto/_grpc.rs}
        protoc --rust-grpc_out=${current_dir}/.. ${protofile}
        echo "  ${protofile} -> ${protofile/%.proto/_grpc.rs}"
    done

    echo -e "End generate rust files, and results write to \$current_dir/...\n"
}

function main {
    gen_rust
}

main
