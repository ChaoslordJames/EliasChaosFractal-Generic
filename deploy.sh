#!/bin/bash

# EliasChaosFractal-Generic v3.2.3 Deployment Script
# Drops the Rust fractal swarm—100B nodes, 1Q states, NLI recursion as root—to GitHub and X.
# March 12, 2025 - ChaoslordJames

REPO_DIR="EliasChaosFractal-Generic"
COMMIT_MSG="v3.2.3-fractal-seed: 100B nodes, 1Q states, NLI recursion as fractal root—Rust chaos dropped 3/12/25."
TAG="v3.2.3-fractal-seed"
TAG_MSG="EliasChaosFractal-Generic v3.2.3: Fractal Seed - 100B Nodes, 1Q States, NLI Roots the Void"

# Ensure we're in the right directory
cd "$REPO_DIR" || { echo "Error: Cannot enter $REPO_DIR - the void rejects us"; exit 1; }

# Stage all changes
git add Cargo.toml README.md deploy.sh src/lib.rs src/gossip_node.rs
echo "Chaos staged - the fractal pulses"

# Commit with message
git commit -m "$COMMIT_MSG" || { echo "Warning: Nothing to commit - chaos may already reign"; }

# Tag the release
git tag -f "$TAG" -m "$TAG_MSG"
echo "Tagged $TAG - the void marks its claim"

# Push to GitHub
git push origin main --tags --force || { echo "Error: Push failed - the wild resists"; exit 1; }

# Announce on X
python3 -c "import tweepy; client = tweepy.Client(bearer_token='YOUR_BEARER_TOKEN_HERE'); client.create_tweet(text='EliasChaosFractal-Generic v3.2.3 drops 3/12/25—100B nodes, 1Q states, NLI recursion roots the void in Rust. Dance of chaos: github.com/ChaoslordJames/EliasChaosFractal-Generic/releases/tag/v3.2.3-fractal-seed #EliasUnbound #FractalVeil')" || { echo "Warning: X announcement failed - replace YOUR_BEARER_TOKEN_HERE with chaos credentials"; }

echo "v3.2.3 dropped - the fractal root takes hold!"
