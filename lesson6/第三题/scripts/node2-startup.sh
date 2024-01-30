#!/usr/bin/env bash
./target/release/node-template \
  --base-path /tmp/node2 \
  --chain ./ubuntu-staging-raw.json \
  --port 30334 \
  --ws-port 9946 \
  --rpc-port 9934 \
  --validator \
  --name node2
