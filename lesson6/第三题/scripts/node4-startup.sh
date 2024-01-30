#!/usr/bin/env bash
./target/release/node-template \
  --base-path /tmp/node4 \
  --chain ./ubuntu-staging-raw.json \
  --port 30336 \
  --ws-port 9948 \
  --rpc-port 9936 \
  --validator \
  --name node4
