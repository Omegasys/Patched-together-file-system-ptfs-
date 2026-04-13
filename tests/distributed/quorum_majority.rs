use ptfs_distributed::quorum::Quorum;

#[test]
fn quorum_majority_calculation() {
    let quorum = Quorum::new(5);

    assert_eq!(quorum.majority(), 3);

    let quorum2 = Quorum::new(10);

    assert_eq!(quorum2.majority(), 6);
}
