use ptfs_distributed::cluster::{Cluster, Node};

#[test]
fn node_join_and_leave_behavior() {
    let mut cluster = Cluster::new(1);

    cluster.add_node(Node {
        id: 1,
        address: "10.0.0.1".into(),
    });

    cluster.add_node(Node {
        id: 2,
        address: "10.0.0.2".into(),
    });

    assert_eq!(cluster.all_nodes().len(), 2);

    cluster.remove_node(2);

    assert_eq!(cluster.all_nodes().len(), 1);
}
