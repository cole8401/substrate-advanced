#!/usr/bin/env bash

./target/release/node-template key insert \
  --base-path /tmp/node1 \
  --chain ubuntu-staging-raw.json \
  --scheme Sr25519 \
  --suri "cover tragic mistake win library cry battle suffer firm sponsor impact excuse" \
  --password-interactive \
  --key-type babe

./target/release/node-template key insert \
  --base-path /tmp/node1 \
  --chain ubuntu-staging-raw.json \
  --scheme Ed25519 \
  --suri "cover tragic mistake win library cry battle suffer firm sponsor impact excuse" \
  --password-interactive \
  --key-type gran

./target/release/node-template key insert \
  --base-path /tmp/node2 \
  --chain ubuntu-staging-raw.json \
  --scheme Sr25519 \
  --suri "cover tragic mistake win library cry battle suffer firm sponsor impact excuse" \
  --password-interactive \
  --key-type babe

./target/release/node-template key insert \
  --base-path /tmp/node2 \
  --chain ubuntu-staging-raw.json \
  --scheme Ed25519 \
  --suri "cover tragic mistake win library cry battle suffer firm sponsor impact excuse" \
  --password-interactive \
  --key-type gran

./target/release/node-template key insert \
  --base-path /tmp/node3 \
  --chain ubuntu-staging-raw.json \
  --scheme Sr25519 \
  --suri "cover tragic mistake win library cry battle suffer firm sponsor impact excuse" \
  --password-interactive \
  --key-type babe

./target/release/node-template key insert \
  --base-path /tmp/node3 \
  --chain ubuntu-staging-raw.json \
  --scheme Ed25519 \
  --suri "cover tragic mistake win library cry battle suffer firm sponsor impact excuse" \
  --password-interactive \
  --key-type gran

  ./target/release/node-template key insert \
  --base-path /tmp/node4 \
  --chain ubuntu-staging-raw.json \
  --scheme Sr25519 \
  --suri "cover tragic mistake win library cry battle suffer firm sponsor impact excuse" \
  --password-interactive \
  --key-type babe

./target/release/node-template key insert \
  --base-path /tmp/node4 \
  --chain ubuntu-staging-raw.json \
  --scheme Ed25519 \
  --suri "cover tragic mistake win library cry battle suffer firm sponsor impact excuse" \
  --password-interactive \
  --key-type gran
