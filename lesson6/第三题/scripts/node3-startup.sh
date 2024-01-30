#!/usr/bin/env bash
./target/release/node-template \
  --base-path /tmp/node3 \
  --chain ./ubuntu-staging-raw.json \
  --port 30335 \
  --ws-port 9947 \
  --rpc-port 9935 \
  --validator \
  --name node3
