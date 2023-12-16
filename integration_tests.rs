

use project::Graph;

#[test]
fn test_example_function_for_testing() {
    // Arrange
    let mut g = Graph { outedges: vec![] };
    g.add_undirected_edges(&[(1, 2), (2, 3)]);

    // Act
    let result = g.example_function_for_testing();

    // Assert
    assert_eq!(result, 3);  // Update this with the expected result
}

// Add more test functions as needed
