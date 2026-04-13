use ptfs_distributed::cluster::{Cluster, Node};

#[test]
fn network_partition_behavior() {
    let mut cluster = Cluster::new(1);

    for i in 1..=4 {
        cluster.add_node(Node {
            id: i,
            address: format!("10.0.0.{}", i),
        });
    }

    let reachable = cluster.reachable_nodes(vec![1, 2]);
    let unreachable = cluster.unreachable_nodes(vec![1, 2]);

    assert!(reachable.len() > 0);
    assert!(unreachable.len() > 0);
}
