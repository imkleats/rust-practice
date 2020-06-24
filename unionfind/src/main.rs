pub mod qf {
    #[derive(Debug)]
    pub struct QuickFind {
        pub size: usize,
        pub graph: Vec<QuickFindNode>,
    }
    #[derive(Debug)]
    pub struct QuickFindNode {
        pub group: u32
    }
    
    impl QuickFind {
        pub fn new(n: usize) -> QuickFind {
            let mut graph = Vec::with_capacity(n);
            for i in 0..n as u32 {
                println!("Adding {}th node out of {} with group {}.", i, n, i);
                graph.push(QuickFindNode{
                    group: i
                });
            }
            QuickFind {
                size: n,
                graph: graph
            }
        }
    
        pub fn connected(&self, n: usize, m: usize) -> bool {
            match self.graph.get(n) {
                Some(k) => {
                    match self.graph.get(m) {
                        Some(j) => j.group == k.group,
                        None => false
                    }
                },
                None => false
            }
        }
    
        pub fn union(&mut self, n: usize, m: usize) {
            // bounds check the UF
            if n >= self.graph.len() || m >= self.graph.len() {
                println!("Triggered bounds check.");
                ()
            }
            let j: u32 = match self.graph.get(n) {
                Some(q) => {
                    println!("Storing q.group in j: {}", q.group);
                    q.group
                },
                None => return ()
            };
            let k: u32 = match self.graph.get(m) {
                Some(q) => {
                    println!("Storing q.group in k: {}", q.group);
                    q.group
                },
                None => return ()
            };
    
            if k == j {
                println!("K group ({}) is equal to J group ({})", k, j);
                ()
            }
    
            for i in 0..self.graph.len() {
                if let Some(node) = self.graph.get_mut(i) {
                    println!("Checking: Node {} in Graph is in Group {}, Comparison Group {}", i, node.group, k );
                    if node.group == k {
                        node.group = j;
                    }
                }
            }
        }
    }
}

pub mod qu {
    #[derive(Debug)]
    pub struct QuickUnion {
        size: usize,
        graph: Vec<usize>,
        children: Vec<usize>
    }
    impl QuickUnion {
        pub fn new(n: usize) -> QuickUnion {
            let mut graph = Vec::with_capacity(n);
            for i in 0..n {
                graph.push(i);
            }
            QuickUnion {
                size: n,
                graph: graph,
                children: vec![1; n]
            }
        }
        pub fn get_root(&mut self, a: usize) -> usize {
            let mut root = a;
            
            while root != self.graph[root] {
                self.graph[root] = self.graph[self.graph[root]];
                root = self.graph[root];
            }

            root
        }
        pub fn view_root(&self, a: usize) -> usize {
            let mut root = a;
            
            while root != self.graph[root] {
                root = self.graph[root];
            }

            root
        }
        pub fn connected(&mut self, a: usize, b: usize) -> bool {
            if a > self.graph.len() - 1 || 
               b > self.graph.len() - 1 { return false; }

            if a == b { return true }

            self.get_root(a) == self.get_root(b)
        }
        pub fn union(&mut self, a: usize, b: usize) {
            let a_root = self.get_root(a);
            let b_root = self.get_root(b);

            if a_root == b_root {
                ()
            }
            if self.children[a_root] < self.children[b_root] {
                self.graph[a_root] = b_root;
                self.children[b_root] += self.children[a_root];
            } else {
                self.graph[b_root] = a_root;
                self.children[a_root] += self.children[b_root];
            }
            
        }
        pub fn print_me(&self) {
            for i in 0..self.graph.len() {
                println!("Checking: Node {} in Graph has Parent {} and Root {}", i, &self.graph[i], self.view_root(i));
            }
        }
    }
}

pub mod percolation {
    use rand::Rng;
    pub struct Simulation {
        iterations: usize,
        rows: usize,
        columns: usize,
        area: f64,
        trials: Vec<u64>,
        results: Option<SimResult>
    }
    impl Simulation {
        pub fn new_simulation(iterations: usize, rows: usize, columns: usize) -> Simulation {
            Simulation {
                iterations,
                rows,
                columns,
                area: (rows * columns) as f64,
                trials: Vec::with_capacity(iterations),
                results: None,
            }
        }
        pub fn run_simulation(&mut self) {
            let mut i = 0;
            let mut random_col: usize;
            let mut random_row: usize;
            while i < self.iterations {
                let mut perc = Percolator::create_percolator(self.rows, self.columns);
                while !perc.percolates() {
                    random_col = rand::thread_rng().gen_range(1, self.columns+1);
                    random_row = rand::thread_rng().gen_range(1, self.rows+1);
                    perc.open(random_row, random_col);
                }
                self.trials.push(perc.total_open() as u64);
                i += 1;
            }
            let mean = self.calculate_mean();
            let stddev = self.calculate_stddev(mean);
            let confidence = self.calculate_conf(mean, stddev);
            self.results = Some(SimResult{
                mean,
                stddev,
                confidence
            });
        }
        pub fn calculate_mean(&self) -> f64 {
            let mut sum: f64 = 0.0;
            for i in &self.trials {
                sum += (*i as f64)/self.area;
            }
            sum / (self.iterations as f64)
        }
        pub fn calculate_stddev(&self, mean: f64) -> f64 {
            let mut sum: f64 = 0.0;
            for i in &self.trials {
                sum += ((((*i as f64))/self.area) - mean).powi(2);
            }
            sum / ((self.iterations - 1) as f64)
        }
        pub fn calculate_conf(&self, mean: f64, stddev: f64) -> (f64, f64) {
            (
                mean - 1.96*stddev/(self.iterations as f64).sqrt(),
                mean + 1.96*stddev/(self.iterations as f64).sqrt(),
            )
        }
        pub fn report_results(&self) {
            match &self.results {
                Some(result) => {
                    println!("Based on {} independent trials on an {}x{} Percolator...",
                        self.iterations, self.columns, self.rows);
                    result.print_me();
                },
                None => println!("Must run simulation before reporting results.")
            }
        }
    }
    pub struct SimResult {
        mean: f64,
        stddev: f64,
        confidence: (f64, f64)
    }
    impl SimResult {
        pub fn print_me(&self) {
            println!("mean\t\t= {}", self.mean);
            println!("stddev\t\t= {}", self.stddev);
            println!("confid\t\t= {:?}", self.confidence);
        }
    }
    pub struct Node {
        parent: usize,
        child_count: usize,
        open: bool
    }
    pub struct Percolator {
        rows: usize,
        columns: usize,
        field_size: usize,
        graph: Vec<Node>
    }

    impl Percolator {
        pub fn create_percolator(
            rows: usize, columns: usize
        ) -> Percolator {
            let field_size = rows * columns;
            let mut graph = Vec::with_capacity(field_size + 2);

            // Initialize the graph vector
            for i in 0..(field_size + 2) {
                graph.push(Node{
                    parent: i,
                    child_count: 1,
                    open: false,

                })
            }

            let mut perc = Percolator {
                rows,
                columns,
                field_size: rows * columns,
                graph
            };
            perc.initialize_anchors();
            perc
        }
        pub fn initialize_anchors(&mut self) {
            // Top synthetic point at index of 0 
            // Bottom synthetic point at index of field_size + 1
            // Connect those points to top/bottom rows
            for i in 0..self.columns {
                self.union(0, i+1);
                self.union(self.field_size+1, self.field_size - i);
            }
        }
        pub fn get_root(&mut self, a: usize) -> usize {
            let mut root = a;
            
            while root != self.graph[root].parent {
                self.graph[root].parent = self.graph[self.graph[root].parent].parent;
                root = self.graph[root].parent;
            }

            root
        }
        pub fn view_root(&self, a: usize) -> usize {
            let mut root = a;
            
            while root != self.graph[root].parent {
                root = self.graph[root].parent;
            }

            root
        }
        pub fn connected(&self, a: usize, b: usize) -> bool {
            if a > self.graph.len() - 1 || 
               b > self.graph.len() - 1 { return false; }

            if a == b { return true }

            self.view_root(a) == self.view_root(b)
        }
        pub fn union(&mut self, a: usize, b: usize) {
            let a_root = self.get_root(a);
            let b_root = self.get_root(b);

            if a_root == b_root {
                ()
            }
            if self.graph[a_root].child_count < self.graph[b_root].child_count {
                self.graph[a_root].parent = b_root;
                self.graph[b_root].child_count = self.graph[b_root].child_count.saturating_add(self.graph[a_root].child_count);
            } else {
                self.graph[b_root].parent = a_root;
                self.graph[a_root].child_count = self.graph[a_root].child_count.saturating_add(self.graph[b_root].child_count);
            }
            
        }
        pub fn union_adj(&mut self, row: usize, col: usize) {
            let cell = (row - 1)*self.columns + col;
            for i in self.adjacent_indices(row, col) {
                if self.graph[i].open {
                    self.union(cell, i);
                }
            }
        }
        pub fn open(&mut self, row: usize, col: usize) {
            if !self.graph[(row - 1)*self.columns + col].open {
                self.graph[(row - 1)*self.columns + col].open = true;
                self.union_adj(row, col);
            }
        }
        pub fn is_open(&self, row: usize, col: usize) -> bool {
            match self.graph.get((row - 1)*self.columns + col) {
                Some(node) => node.open,
                None => false
            }
        }
        pub fn total_open(&self) -> usize {
            let mut sum: usize = 0;
            for i in &self.graph {
                if i.open {
                    sum += 1;
                }
            }
            sum
        }
        pub fn is_full(&self, row: usize, col: usize) -> bool {
            self.connected(0, (row - 1)*self.columns + col)
        }
        pub fn percolates(&self) -> bool {
            self.connected(0, self.field_size + 1)
        }
        pub fn adjacent_indices(&self, row: usize, col: usize) -> Vec<usize> {
            let mut adj = Vec::new();
            let row_above: usize = if row == self.rows { row } else { row + 1 };
            let row_below: usize = if row == 1 { row } else { row - 1 };
            let col_right = if col == self.columns { col } else { col + 1 };
            let col_left = if col == self.columns { col } else { col + 1 };
            adj.push((row - 1)*self.columns + col_right);
            adj.push((row - 1)*self.columns + col_left);
            adj.push((row_above - 1)*self.columns + col);
            adj.push((row_below - 1)*self.columns + col);
            adj
        }
    }
}
fn main() {
    let mut my_sim = percolation::Simulation::new_simulation(50, 200, 50);
    my_sim.report_results();
    my_sim.run_simulation();
    my_sim.report_results();
    //println!("This is my_graph: {:?}", my_graph);
    println!("Hello, world!");
}
