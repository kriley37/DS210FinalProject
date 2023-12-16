// necessary imports
// I copied and pasted the code from main.rs and added a few new functions for testing purposes
use std::collections::{HashSet, VecDeque, HashMap};
use std::fs::File;
use std::io::prelude::*;
use csv::ReaderBuilder;

// A lot of this code was inspired by lecture 28.
type Vertex = usize;
type ListOfEdges = Vec<(Vertex, Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
struct Graph {
    outedges: AdjacencyLists,
}

impl Graph {
    fn add_undirected_edges(&mut self, edges: &ListOfEdges) {
        for (u, v) in edges {
            let u_idx = u - 1;
            let v_idx = v - 1;

            if u_idx >= self.outedges.len() {
                self.outedges.resize(u_idx + 1, vec![]);
            }

            if v_idx >= self.outedges.len() {
                self.outedges.resize(v_idx + 1, vec![]);
            }

            self.outedges[u_idx].push(v_idx);
            self.outedges[v_idx].push(u_idx);
        }
    }

    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }

    fn create_undirected(edges: &ListOfEdges) -> Graph {
        let mut g = Graph { outedges: vec![] };
        g.add_undirected_edges(edges);
        g.sort_graph_lists();
        g
    }

    fn bfs_shortest_path(&self, start: Vertex, end: Vertex) -> Option<Vec<Vertex>> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent_map = HashMap::new();

        queue.push_back(start - 1);
        visited.insert(start - 1);

        while let Some(current) = queue.pop_front() {
            if current == end - 1 {
                let mut path = Vec::new();
                let mut node = end - 1;
                while let Some(&parent) = parent_map.get(&node) {
                    path.push(node);
                    node = parent;
                }
                path.push(start - 1);
                path.reverse();
                return Some(path);
            }

            for &neighbor in &self.outedges[current] {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                    parent_map.insert(neighbor, current);
                }
            }
        }

        None
    }

    fn calculate_degrees(&self) -> Vec<usize> {
        self.outedges.iter().map(|neighbors| neighbors.len()).collect()
    }

    fn calculate_local_bridges(&self) -> usize {
        let mut count = 0;
        for (node, neighbors) in self.outedges.iter().enumerate() {
            for &neighbor in neighbors {
                if self.outedges[neighbor].iter().all(|&n| n != node) {
                    count += 1;
                }
            }
        }
        count
    }

    fn calculate_centrality_measures(&self) -> (f64, f64) {
        let degrees = self.calculate_degrees();

        let average_degree: f64 = degrees.iter().sum::<usize>() as f64 / degrees.len() as f64;

        let mut closeness_sum = 0.0;
        for node in 0..self.outedges.len() {
            if let Some(path) = self.bfs_shortest_path(node + 1, 1) {
                closeness_sum += 1.0 / path.len() as f64;
            }
        }
        let average_closeness = closeness_sum / self.outedges.len() as f64;

        (average_degree, average_closeness)
    }

    fn calculate_network_diameter(&self) -> Option<usize> {
        let mut max_diameter = 0;
        for i in 0..self.outedges.len() {
            for j in (i + 1)..self.outedges.len() {
                if let Some(path) = self.bfs_shortest_path(i + 1, j + 1) {
                    max_diameter = max_diameter.max(path.len());
                } else {
                    return None;
                }
            }
        }
        Some(max_diameter)
    }

    // NEW FUNCTION FOR TESTING PURPOSES
    pub fn example_function_for_testing(&self) -> usize {
        // Example logic to test
        self.outedges.len()
    }
}

fn read_csv(file_path: &str) -> ListOfEdges {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(contents.as_bytes());

    reader
        .records()
        .map(|record| {
            let record = record.expect("Error reading CSV record");
            let u: Vertex = record[0].parse().expect("Error parsing CSV record");
            let v: Vertex = record[1].parse().expect("Error parsing CSV record");
            (u, v)
        })
        .collect()
}

fn main() {
    let file_path = r"C:\Users\Karrington Riley\Desktop\DS210_FinalProject\euroroad.csv";
    let edges = read_csv(file_path);

    let mut edges = edges;
    edges.sort();

    let graph = Graph::create_undirected(&edges);

    let max_vertex_label = edges.iter().flat_map(|(u, v)| vec![*u, *v]).max().unwrap_or(0);
    println!("Max Vertex Label: {}", max_vertex_label);

    let num_edges = edges.len();
    let num_vertices = graph.outedges.len();
    println!("Number of Edges: {}", num_edges);
    println!("Number of Vertices: {}", num_vertices);

    let start: Vertex = read_user_input("Enter the start node: ");
    let end: Vertex = read_user_input("Enter the end node: ");

    if start <= max_vertex_label && end <= max_vertex_label {
        if let Some(shortest_path) = graph.bfs_shortest_path(start, end) {
            println!("Shortest Path from {} to {}: {:?}", start, end, shortest_path);
        } else {
            println!("No path found from {} to {}", start, end);
        }

        let (average_degree, average_closeness) = graph.calculate_centrality_measures();
        let network_diameter = graph.calculate_network_diameter();
        match network_diameter {
            Some(diameter) => println!("Network Diameter: {}", diameter),
            None => println!("The graph is not connected, so the network diameter is undefined."),
        }

        println!("Average Degree: {:.2}", average_degree);
        println!("Average Closeness: {:.2}", average_closeness);

        let degrees = graph.calculate_degrees();
        let mut sorted_indices: Vec<_> = (0..degrees.len()).collect();
        sorted_indices.sort_by_key(|&i| degrees[i]);

        println!("Top 5 Cities with Highest Degree of Distribution:");
        for &index in sorted_indices.iter().rev().take(5) {
            println!("City {}: Degree {}", index + 1, degrees[index]);
        }

        println!("Bottom 5 Cities with Lowest Degree of Distribution:");
        for &index in sorted_indices.iter().take(5) {
            println!("City {}: Degree {}", index + 1, degrees[index]);
        }

        let num_local_bridges = graph.calculate_local_bridges();
        println!("Number of Local Bridges: {}", num_local_bridges);
    } else {
        println!("Invalid node labels. Nodes must be within the range of the graph.");
    }
}

fn read_user_input(prompt: &str) -> Vertex {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid integer."),
        }
    }
}


