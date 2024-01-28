#!/usr/bin/env bash
set -e
cargo build

tail -f "/tmp/lsp-connection" | ./target/debug/asm-lsp
#
# LSP_PID=$!
# echo $LSP_PID
#
# sleep 10
#
# cat ./test/server_init_1.txt > /proc/$LSP_PID/fd/0
# # tail -f --pid=$LSP_PID /proc/$LSP_PID/fd/1
#
#
# while :
# do
#     echo "Boffa" > /dev/null
# done
#
# #
#
# sed -u -re 's/^(.*)$/\1\r/' |
#
