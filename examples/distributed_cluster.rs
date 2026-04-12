use ptfs_distributed::cluster::{Cluster, Node};
use ptfs_distributed::replication::ReplicationManager;
use ptfs_distributed::quorum::Quorum;

fn main() {
    println!("=== PTFS Distributed Cluster Demo ===");

    // Create cluster
    let mut cluster = Cluster::new(1);

    cluster.add_node(Node {
        id: 1,
        address: "127.0.0.1:8001".into(),
    });

    cluster.add_node(Node {
        id: 2,
        address: "127.0.0.1:8002".into(),
    });

    cluster.add_node(Node {
        id: 3,
        address: "127.0.0.1:8003".into(),
    });

    println!("Cluster nodes: {:?}", cluster.all_nodes());

    // Replication
    let mut replication = ReplicationManager::new(2);

    let object_id = 42;
    replication.register_replica(object_id, 1);
    replication.register_replica(object_id, 2);

    println!(
        "Object {} replicas: {:?}",
        object_id,
        replication.get_replicas(&object_id)
    );

    // Quorum
    let quorum = Quorum::new(cluster.all_nodes().len());
    println!("Majority quorum: {}", quorum.majority());

    println!("=== Done ===");
}
