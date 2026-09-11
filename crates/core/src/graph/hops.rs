//! Shortest-path hop distance resolver using Breadth-First Search (BFS).

use crate::model::{NodeStatus, NoteNode};
use std::collections::{HashMap, HashSet, VecDeque};

/// Assigns hop distances and status transitions from designated Hub.
/// Handles cycles gracefully via visited set.
pub fn assign_bfs_hops(
    hub_id: &str,
    nodes: &mut HashMap<String, NoteNode>,
    adj: &HashMap<String, Vec<String>>,
) {
    // Reset all nodes to orphan (-1)
    for node in nodes.values_mut() {
        node.hop_distance = -1;
        node.status = NodeStatus::Orphan;
    }

    if !nodes.contains_key(hub_id) {
        return;
    }

    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<(String, i32)> = VecDeque::new();

    queue.push_back((hub_id.to_string(), 0));
    visited.insert(hub_id.to_string());

    while let Some((curr_id, dist)) = queue.pop_front() {
        if let Some(node) = nodes.get_mut(&curr_id) {
            node.hop_distance = dist;
            node.status = if dist == 0 {
                NodeStatus::Active
            } else {
                NodeStatus::ConnectedInactive
            };
        }

        if let Some(neighbors) = adj.get(&curr_id) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) && nodes.contains_key(neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push_back((neighbor.clone(), dist + 1));
                }
            }
        }
    }
}
