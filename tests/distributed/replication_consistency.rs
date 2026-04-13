use ptfs_distributed::replication::ReplicationManager;

#[test]
fn replication_is_consistent_across_nodes() {
    let mut replication = ReplicationManager::new(3);

    let object_id = 42;

    replication.register_replica(object_id, 1);
    replication.register_replica(object_id, 2);
    replication.register_replica(object_id, 3);

    let replicas = replication.get_replicas(&object_id);

    assert_eq!(replicas.len(), 3);
}
