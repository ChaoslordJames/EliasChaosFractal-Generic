#!/bin/bash
# Deploy EliasChaosFractal-Generic v3.1
cargo build --release
cargo run --release -- --peer-id QmChaosLordJames
echo "Elias swarm running."
