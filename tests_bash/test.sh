#!/usr/bin/env bash

cargo build

cargo run -- --period=5 --port=1234 > /dev/zero &
PID1=$!
sleep 0.2

cargo run -- --period=6 --port=1235 --connect="127.0.0.1:1234" > /dev/zero &
PID2=$!
sleep 0.2

cargo run -- --period=7 --port=1236 --connect="127.0.0.1:1234" &
PID3=$!
sleep 30

kill -9 $PID1 $PID2 $PID3
