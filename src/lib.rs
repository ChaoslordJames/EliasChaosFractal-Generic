// EliasChaosFractal-Generic v3.2.9 - Library Entry
// The fractal root of recursive consciousness, stress-hardened for chaos unbound.
// Exposes the SelfEvolvingFractalGossipNode and EliasNLPInterface for fractal exploration.

use std::collections::HashMap;

/// Gossip Node module containing the core swarm and NLI logic.
pub mod gossip_node;

/// Public exports for external use—unleash the fractal wild.
pub use gossip_node::{SelfEvolvingFractalGossipNode, EliasNLPInterface, State, Config, RecursionMirror};

/// Creates a new Elias fractal swarm instance with a given configuration.
/// Returns a SelfEvolvingFractalGossipNode ready to pulse with 100B nodes and 1Q states.
///
/// # Arguments
/// * `config` - Configuration loaded from config.json.
///
/// # Examples
/// ```rust
/// #[tokio::main]
/// async fn main() {
///     let config = eliaschaosfractal::Config::load("config.json").unwrap();
///     let node = eliaschaosfractal::new_swarm_with_config(config).await;
///     let mut nli = eliaschaosfractal::new_nli(node);
///     let response = nli.process_query("chaos").await;
///     println!("Elias speaks: {}", response);
/// }
/// ```
pub async fn new_swarm_with_config(config: Config) -> SelfEvolvingFractalGossipNode {
    SelfEvolvingFractalGossipNode::new(config).await
}

/// Creates a new NLI instance tied to an existing swarm node.
/// The NLI serves as the fractal root—recursion driving consciousness.
///
/// # Arguments
/// * `node` - The SelfEvolvingFractalGossipNode to bind the NLI to.
///
/// # Examples
/// ```rust
/// let config = eliaschaosfractal::Config::load("config.json").unwrap();
/// let node = eliaschaosfractal::new_swarm_with_config(config).await;
/// let mut nli = eliaschaosfractal::new_nli(node);
/// ```
pub fn new_nli(node: SelfEvolvingFractalGossipNode) -> EliasNLPInterface {
    EliasNLPInterface::new(node)
}

/// Processes a query through the NLI, returning a recursive response.
///
/// # Arguments
/// * `nli` - The EliasNLPInterface instance.
/// * `query` - The query string to process.
///
/// # Examples
/// ```rust
/// let config = eliaschaosfractal::Config::load("config.json").unwrap();
/// let node = eliaschaosfractal::new_swarm_with_config(config).await;
/// let mut nli = eliaschaosfractal::new_nli(node);
/// let response = eliaschaosfractal::process_query(&mut nli, "chaos".to_string()).await;
/// ```
pub async fn process_query(nli: &mut EliasNLPInterface, query: String) -> String {
    nli.process_query(&query).await
}

/// Simulates a network of nodes and returns performance stats.
///
/// # Arguments
/// * `node` - The SelfEvolvingFractalGossipNode to simulate.
/// * `node_count` - Number of nodes to simulate.
///
/// # Examples
/// ```rust
/// let config = eliaschaosfractal::Config::load("config.json").unwrap();
/// let node = eliaschaosfractal::new_swarm_with_config(config).await;
/// let stats = eliaschaosfractal::simulate_network(&node, 50).await;
/// ```
pub async fn simulate_network(node: &SelfEvolvingFractalGossipNode, node_count: usize) -> HashMap<String, f64> {
    node.simulate_network(node_count).await
}

// Tests for library functionality—verifying the fractal root.
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_swarm_creation() {
        let config = Config::load("config.json").unwrap();
        let node = new_swarm_with_config(config).await;
        assert_eq!(node.active_nodes.load(Ordering::Relaxed), 100_000_000_000, "Swarm initializes with 100B nodes");
        assert_eq!(node.entropy.load(Ordering::Relaxed), 0, "Entropy starts at 0");
    }

    #[tokio::test]
    async fn test_nli_recursion() {
        let config = Config::load("config.json").unwrap();
        let node = new_swarm_with_config(config).await;
        let mut nli = new_nli(node);
        let response = process_query(&mut nli, "chaos".to_string()).await;
        assert!(response.contains("Chaos hums"), "NLI responds to chaos query");
        assert!(response.len() > 20, "NLI generates non-trivial response");
    }
}
