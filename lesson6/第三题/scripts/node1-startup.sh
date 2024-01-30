#!/usr/bin/env bash
./target/release/node-template \
  --node-key 621dd4de79d33148eb49ac314b250eb254dda73779d45413239268d9a9575f6f \
  --base-path /tmp/node1 \
  --chain ./ubuntu-staging-raw.json \
  --port 30333 \
  --ws-port 9945 \
  --rpc-port 9933 \
  --validator \
  --name node1
