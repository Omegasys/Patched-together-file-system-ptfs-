use ptfs_distributed::replication::ReplicationManager;

#[test]
fn failover_and_recovery_behavior() {
    let mut replication = ReplicationManager::new(3);

    let object_id = 99;

    replication.register_replica(object_id, 1);
    replication.register_replica(object_id, 2);

    // simulate node failure
    replication.mark_node_failed(1);

    let active = replication.active_replicas(&object_id);

    assert!(active.contains(&2));

    // recovery simulation
    replication.recover_node(1);

    let updated = replication.active_replicas(&object_id);

    assert!(updated.len() >= 1);
}
