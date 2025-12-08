use crate::util::get_input;

type JunctionBox = (f64, f64, f64);

#[inline]
fn calculate_distance(a: &JunctionBox, b: &JunctionBox) -> f64 {
    let (x1, y1, z1) = a;
    let (x2, y2, z2) = b;

    ((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2)).sqrt()
}

// Union-Find (Disjoint Set Union) data structure for efficient set merging
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // Already in same set
        }

        // Union by rank
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
            self.rank[root_x] += 1;
        }

        true
    }

    fn get_component_sizes(&mut self) -> Vec<usize> {
        let mut components = std::collections::HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            *components.entry(root).or_insert(0) += 1;
        }
        components.values().copied().collect()
    }
}

fn create_circuits(
    junction_boxes: &[JunctionBox],
    max_iterations: usize,
) -> (Vec<usize>, Option<(usize, usize)>) {
    let n = junction_boxes.len();
    let mut last: Option<(usize, usize)> = None;

    // Pre-allocate with exact capacity
    let total_pairs = n * (n - 1) / 2;
    let mut distances = Vec::with_capacity(total_pairs);

    // Calculate all pairwise distances
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = calculate_distance(&junction_boxes[i], &junction_boxes[j]);
            distances.push((i, j, dist));
        }
    }

    // Sort by distance (ascending) - could use partial_sort for first k elements
    distances.sort_unstable_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut uf = UnionFind::new(n);
    let mut components = n;

    for (idx, &(a, b, _)) in distances.iter().enumerate() {
        if idx >= max_iterations {
            break;
        }

        if uf.union(a, b) {
            components -= 1;
            last = Some((a, b));

            if components == 1 {
                break;
            }
        }
    }

    (uf.get_component_sizes(), last)
}

pub fn part1() {
    let input = get_input(2025, 8);

    let junction_boxes: Vec<JunctionBox> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut coords = line.split(',').map(|s| s.trim().parse::<f64>().unwrap());
            (
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect();

    let (component_sizes, _) = create_circuits(&junction_boxes, 1000);

    // Find three largest components
    let mut sizes = component_sizes;
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    let result: usize = sizes.iter().take(3).product();
    println!("{}", result);
}

pub fn part2() {
    let input = get_input(2025, 8);

    let junction_boxes: Vec<JunctionBox> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut coords = line.split(',').map(|s| s.trim().parse::<f64>().unwrap());
            (
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect();

    let (_, last) = create_circuits(&junction_boxes, usize::MAX);

    if let Some((a, b)) = last {
        let result = (junction_boxes[a].0 as i64) * (junction_boxes[b].0 as i64);
        println!("{}", result);
    }
}
