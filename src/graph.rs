use std::usize;

#[derive(Debug)]
pub struct Graph {
    num_vertices: usize,
    adjacent: Vec<Vec<(usize, usize)>>,
}

impl Graph {
    pub fn new(num_vertices: usize) -> Self {
        Self {
            num_vertices,
            adjacent: vec![vec![]; num_vertices],
        }
    }

    pub fn connect(&mut self, vertex1: usize, vertex2: usize, edge_value: usize) {
        self.adjacent[vertex1].push((vertex2, edge_value));
        self.adjacent[vertex2].push((vertex1, edge_value));
    }

    pub fn assign_vertex_values(&self) -> Option<Vec<usize>> {
        let mut vertex_values = vec![usize::MAX; self.num_vertices];
        let mut visited = vec![false; self.num_vertices];

        for root in 0..self.num_vertices {
            if visited[root] {
                continue;
            }

            vertex_values[root] = 0;

            let mut tovisit = vec![(None, root)];
            while let Some((parent, vertex)) = tovisit.pop() {
                visited[vertex] = true;

                let mut skip = true;
                for &(neighbor, edge_value) in &self.adjacent[vertex] {
                    if skip && Some(neighbor) == parent {
                        skip = false;
                        continue;
                    }

                    if visited[neighbor] {
                        return None;
                    }

                    tovisit.push((Some(vertex), neighbor));
                    vertex_values[neighbor] = (self.num_vertices + edge_value
                        - vertex_values[vertex])
                        % self.num_vertices;
                }
            }
        }

        debug_assert!(vertex_values.iter().all(|&x| x < usize::MAX));
        Some(vertex_values)
    }
}
