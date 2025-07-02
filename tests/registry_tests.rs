use ecoblock_mesh::registry::NodeRegistry;


#[test]
fn test_node_registry_operations() {
    let mut registry = NodeRegistry::new();

    registry.register_node("node1", "Weather Sensor A");
    registry.register_node("node2", "Weather Sensor B");

    assert_eq!(registry.count(), 2);
    assert!(registry.contains("node1"));
    assert_eq!(registry.get_node("node1").unwrap().label, "Weather Sensor A");

    registry.unregister_node("node1");
    assert!(!registry.contains("node1"));
    assert_eq!(registry.count(), 1);
}
