use ptfs_distributed::cluster::{Cluster, Node};

#[test]
fn split_brain_detection_simulation() {
    let mut cluster = Cluster::new(1);

    for i in 1..=5 {
        cluster.add_node(Node {
            id: i,
            address: format!("10.0.0.{}", i),
        });
    }

    // simulate partition (logical split)
    let group_a = cluster.partition(vec![1, 2]);
    let group_b = cluster.partition(vec![3, 4, 5]);

    assert!(group_a.len() > 0);
    assert!(group_b.len() > 0);

    // future: quorum decides which survives
}
