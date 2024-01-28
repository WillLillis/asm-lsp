#!/usr/bin/env bash

pipe="/tmp/lsp-connection"
if [ ! -p $pipe ]; then
    mkfifo $pipe
fi


source ./test/server_starts.sh &

sleep 2

cat ./test/server_init_1.txt > $pipe
# Wait until the pipe is set up or whatever
