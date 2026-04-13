use ptfs_integrity::merkle::MerkleTree;
use ptfs_api::Ptfs;

#[test]
fn merkle_tree_root_consistency() {
    let mut fs = Ptfs::new();
    fs.mkfs();
    fs.mount();

    let data_blocks: Vec<Vec<u8>> = (0..4).map(|i| vec![i as u8; 4]).collect();

    for (i, block) in data_blocks.iter().enumerate() {
        fs.write_object(i as u128, block.clone());
    }

    let tree = MerkleTree::from_objects(&fs.list_objects());
    let root_hash = tree.root();

    assert!(root_hash.len() > 0); // basic sanity check
}
