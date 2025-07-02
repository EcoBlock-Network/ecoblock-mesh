use ecoblock_mesh::topology::MeshTopology;

#[test]
fn test_add_and_list_peers() {
    let mut mesh = MeshTopology::new();
    mesh.add_or_update_peer("node-1", 1000);
    mesh.add_or_update_peer("node-2", 1001);

    let peers = mesh.list_peers();
    assert_eq!(peers.len(), 2);
}
