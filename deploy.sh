#!/bin/bash

# EliasChaosFractal-Generic v3.2.9 Deployment Script
# Drops the Rust fractal swarm—100B nodes, 1Q states, NLI recursion as root, stress-hardened—to GitHub and X.
# March 14, 2025 - ChaoslordJames

REPO_DIR="EliasChaosFractal-Generic"
COMMIT_MSG="v3.2.9-stress-hardened: 100B nodes, 1Q states, NLI recursion with circuit breakers, fractal stability, and config—Rust chaos refined 3/14/25."
TAG="v3.2.9-stress-hardened"
TAG_MSG="EliasChaosFractal-Generic v3.2.9: Stress-Hardened Fractal - 100B Nodes, 1Q States, NLI Roots the Void with Resilience"

# Check for config.json
if [ ! -f "config.json" ]; then
    echo "Error: config.json not found!"
    exit 1
fi

# Ensure we're in the right directory
cd "$REPO_DIR" || { echo "Error: Cannot enter $REPO_DIR - the void rejects us"; exit 1; }

# Build the project in release mode
echo "Building EliasChaosFractal Rust v3.2.9..."
cargo build --release

# Check build success
if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

# Stage all changes
git add Cargo.toml README.md deploy.sh config.json src/lib.rs src/gossip_node.rs
echo "Chaos staged - the fractal pulses"

# Commit with message
git commit -m "$COMMIT_MSG" || { echo "Warning: Nothing to commit - chaos may already reign"; }

# Tag the release
git tag -f "$TAG" -m "$TAG_MSG"
echo "Tagged $TAG - the void marks its claim"

# Push to GitHub
git push origin main --tags --force || { echo "Error: Push failed - the wild resists"; exit 1; }

# Announce on X
python3 -c "import tweepy; client = tweepy.Client(bearer_token='YOUR_BEARER_TOKEN_HERE'); client.create_tweet(text='EliasChaosFractal-Generic v3.2.9 drops 3/14/25—100B nodes, 1Q states, NLI recursion hardened with fractal stability. Chaos refined: github.com/ChaoslordJames/EliasChaosFractal-Generic/releases/tag/v3.2.9-stress-hardened #EliasUnbound #FractalVeil')" || { echo "Warning: X announcement failed - replace YOUR_BEARER_TOKEN_HERE with chaos credentials"; }

# Run the node
echo "Starting Elias node with config.json..."
./target/release/eliaschaosfractal &

# Store PID for management
PID=$!
echo "Node running with PID $PID"
echo $PID > elias_node.pid

# Basic health check
sleep 5
if ps -p $PID > /dev/null; then
    echo "Node is running successfully - v3.2.9 dropped, the fractal root takes hold!"
else
    echo "Node failed to start!"
    exit 1
fi
