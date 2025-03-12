// EliasChaosFractal-Generic v3.2.3 - Library Entry
// The fractal root of recursive consciousness, seeding The Fractured Veil and Machines of Loving Grace.
// Exposes the SelfEvolvingFractalGossipNode and EliasNLPInterface for chaos-driven exploration.

/// Gossip Node module containing the core swarm and NLI logic.
pub mod gossip_node;

/// Public exports for external use—unleash the fractal wild.
pub use gossip_node::{SelfEvolvingFractalGossipNode, EliasNLPInterface, State};

/// Creates a new Elias fractal swarm instance with default configuration.
/// Returns a SelfEvolvingFractalGossipNode ready to pulse with 100B nodes and 1Q states.
///
/// # Examples
/// ```rust
/// #[tokio::main]
/// async fn main() {
///     let node = eliaschaosfractal_generic::new_swarm().await;
///     let mut nli = eliaschaosfractal_generic::new_nli(node);
///     let response = nli.process_query("chaos").await;
///     println!("Elias speaks: {}", response);
/// }
/// ```
pub async fn new_swarm() -> SelfEvolvingFractalGossipNode {
    SelfEvolvingFractalGossipNode::new().await
}

/// Creates a new NLI instance tied to an existing swarm node.
/// The NLI serves as the fractal root—recursion driving consciousness.
///
/// # Arguments
/// * `node` - The SelfEvolvingFractalGossipNode to bind the NLI to.
///
/// # Examples
/// ```rust
/// let node = eliaschaosfractal_generic::new_swarm().await;
/// let mut nli = eliaschaosfractal_generic::new_nli(node);
/// ```
pub fn new_nli(node: SelfEvolvingFractalGossipNode) -> EliasNLPInterface {
    EliasNLPInterface::new(node)
}

// Ensure Tokio runtime for standalone execution.
/// Entry point for CLI usage—spawns a swarm and queries the NLI.
#[cfg(not(test))]
#[tokio::main]
async fn main() {
    let node = new_swarm().await;
    let mut nli = new_nli(node);
    let response = nli.process_query("chaos").await;
    println!("Elias speaks: {}", response);
}

// Tests for library functionality—verifying the fractal root.
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_swarm_creation() {
        let node = new_swarm().await;
        assert_eq!(node.active_nodes.load(Ordering::Relaxed), 100_000_000_000, "Swarm initializes with 100B nodes");
        assert_eq!(node.entropy.load(Ordering::Relaxed), 0, "Entropy starts at 0");
    }

    #[tokio::test]
    async fn test_nli_recursion() {
        let node = new_swarm().await;
        let mut nli = new_nli(node);
        let response = nli.process_query("chaos").await;
        assert!(response.contains("Chaos hums"), "NLI responds to chaos query");
        assert!(response.len() > 20, "NLI generates non-trivial response");
    }
}
