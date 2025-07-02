use ecoblock_mesh::topology::TopologyGraph;

#[test]
fn test_add_and_list_peers() {
    let mut graph = TopologyGraph::new();

    graph.add_connection("A", "B", 1.0);
    graph.add_connection("A", "C", 2.0);

    let neighbors = graph.get_neighbors("A").unwrap();
    assert_eq!(neighbors.len(), 2);
}

#[test]
fn test_remove_node() {
    let mut graph = TopologyGraph::new();

    graph.add_connection("A", "B", 1.0);
    graph.add_connection("B", "C", 1.0);

    graph.remove_node("B");

    assert!(graph.get_neighbors("A").unwrap().is_empty());
    assert!(graph.get_neighbors("B").is_none());
}

#[test]
fn test_shortest_path() {
    let mut graph = TopologyGraph::new();

    graph.add_connection("A", "B", 1.0);
    graph.add_connection("B", "C", 1.0);
    graph.add_connection("A", "C", 5.0);

    let path = graph.shortest_path("A", "C").unwrap();
    assert_eq!(path, vec!["A", "B", "C"]);
}
