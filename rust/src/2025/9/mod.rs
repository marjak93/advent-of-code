use crate::util::get_input;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

#[derive(Clone, Debug)]
struct Polygon {
    bounding_box: (i32, i32, i32, i32),
    edges: Vec<Line>,
}

#[derive(Clone, Debug)]
struct Rect {
    p1: Point,
    p2: Point,
}

impl Line {
    /// Creates a new line from two points
    fn new(p1: Point, p2: Point) -> Self {
        Line { p1, p2 }
    }

    /// Checks if a point is exactly on this line segment
    fn contains_point(&self, p: Point) -> bool {
        let min_x = self.p1.x.min(self.p2.x);
        let max_x = self.p1.x.max(self.p2.x);
        let min_y = self.p1.y.min(self.p2.y);
        let max_y = self.p1.y.max(self.p2.y);

        // Check if point is collinear and within bounds
        (self.p2.x - self.p1.x) * (p.y - self.p1.y) == (self.p2.y - self.p1.y) * (p.x - self.p1.x)
            && p.x >= min_x
            && p.x <= max_x
            && p.y >= min_y
            && p.y <= max_y
    }

    /// Checks if point p is to the left of the intersection of this line with horizontal line at p.y
    /// Uses cross-product to avoid division errors
    fn point_left_of_intersection_at_y(&self, p: Point) -> bool {
        // We want to check if p.x < intersection_x
        // intersection_x = p1.x + (p.y - p1.y) * (p2.x - p1.x) / (p2.y - p1.y)
        // Rearranging to avoid division:
        // p.x < p1.x + (p.y - p1.y) * (p2.x - p1.x) / (p2.y - p1.y)
        // (p.x - p1.x) * (p2.y - p1.y) < (p.y - p1.y) * (p2.x - p1.x)
        let dx = p.x - self.p1.x;
        let dy = p.y - self.p1.y;
        let edge_dx = self.p2.x - self.p1.x;
        let edge_dy = self.p2.y - self.p1.y;

        // Use i64 to avoid overflow
        let left = (dx as i64) * (edge_dy as i64);
        let right = (dy as i64) * (edge_dx as i64);

        // Need to flip comparison if edge_dy is negative
        if edge_dy > 0 {
            left < right
        } else {
            left > right
        }
    }

    #[inline]
    fn crosses_horizontal_at_y(&self, y: i32) -> bool {
        (self.p1.y > y) != (self.p2.y > y)
    }
}

impl Rect {
    #[inline]
    fn area(&self) -> u64 {
        ((self.p2.x - self.p1.x).abs() as u64 + 1) * ((self.p2.y - self.p1.y).abs() as u64 + 1)
    }
}

impl Polygon {
    /// Creates a new polygon from vertices, computing bounding box and edges
    fn new(vertices: Vec<Point>) -> Self {
        let min_x = vertices.iter().map(|p| p.x).min().unwrap_or(0);
        let max_x = vertices.iter().map(|p| p.x).max().unwrap_or(0);
        let min_y = vertices.iter().map(|p| p.y).min().unwrap_or(0);
        let max_y = vertices.iter().map(|p| p.y).max().unwrap_or(0);

        // Pre-compute all edges
        let n = vertices.len();
        let mut edges = Vec::with_capacity(n);
        for i in 0..n {
            edges.push(Line::new(vertices[i], vertices[(i + 1) % n]));
        }

        Polygon {
            bounding_box: (min_x, max_x, min_y, max_y),
            edges,
        }
    }

    #[inline]
    fn bounding_box(&self) -> (i32, i32, i32, i32) {
        self.bounding_box
    }

    // Check if the rectangle is fully contained within the polygon
    fn can_contain_rect(&self, rect: &Rect) -> bool {
        let rect_min_x = rect.p1.x.min(rect.p2.x);
        let rect_max_x = rect.p1.x.max(rect.p2.x);
        let rect_min_y = rect.p1.y.min(rect.p2.y);
        let rect_max_y = rect.p1.y.max(rect.p2.y);

        // Fast bounding box check first
        let (poly_min_x, poly_max_x, poly_min_y, poly_max_y) = self.bounding_box();
        if rect_min_x < poly_min_x
            || rect_max_x > poly_max_x
            || rect_min_y < poly_min_y
            || rect_max_y > poly_max_y
        {
            return false;
        }

        // Check all four corners
        if !self.point_in_or_on_polygon(Point {
            x: rect_min_x,
            y: rect_min_y,
        }) {
            return false;
        }
        if !self.point_in_or_on_polygon(Point {
            x: rect_min_x,
            y: rect_max_y,
        }) {
            return false;
        }
        if !self.point_in_or_on_polygon(Point {
            x: rect_max_x,
            y: rect_min_y,
        }) {
            return false;
        }
        if !self.point_in_or_on_polygon(Point {
            x: rect_max_x,
            y: rect_max_y,
        }) {
            return false;
        }

        // For concave polygons, sample edges
        let width = rect_max_x - rect_min_x;
        let height = rect_max_y - rect_min_y;

        // Quick check with fewer samples first
        let quick_samples = 5;
        for i in 1..quick_samples {
            let x = rect_min_x + (width * i as i32) / quick_samples as i32;
            if !self.is_point_inside_by_ray_casting(Point { x, y: rect_min_y }) {
                return false;
            }
            if !self.is_point_inside_by_ray_casting(Point { x, y: rect_max_y }) {
                return false;
            }
        }
        for i in 1..quick_samples {
            let y = rect_min_y + (height * i as i32) / quick_samples as i32;
            if !self.is_point_inside_by_ray_casting(Point { x: rect_min_x, y }) {
                return false;
            }
            if !self.is_point_inside_by_ray_casting(Point { x: rect_max_x, y }) {
                return false;
            }
        }

        // If quick check passed, do thorough check for larger rectangles
        let max_dim = width.max(height);
        if max_dim > 10000 {
            let samples = (max_dim / 1000) as usize;
            for i in quick_samples..samples {
                let x = rect_min_x + (width * i as i32) / samples as i32;
                if !self.point_in_or_on_polygon(Point { x, y: rect_min_y }) {
                    return false;
                }
                if !self.point_in_or_on_polygon(Point { x, y: rect_max_y }) {
                    return false;
                }
            }
            for i in quick_samples..samples {
                let y = rect_min_y + (height * i as i32) / samples as i32;
                if !self.point_in_or_on_polygon(Point { x: rect_min_x, y }) {
                    return false;
                }
                if !self.point_in_or_on_polygon(Point { x: rect_max_x, y }) {
                    return false;
                }
            }
        }

        true
    }

    // Checks if a point is inside or on the edge of the polygon
    fn point_in_or_on_polygon(&self, p: Point) -> bool {
        if self.is_point_on_any_edge(p) {
            return true;
        }
        self.is_point_inside_by_ray_casting(p)
    }

    /// Returns true if the point is exactly on any edge of the polygon
    fn is_point_on_any_edge(&self, p: Point) -> bool {
        for edge in &self.edges {
            if edge.contains_point(p) {
                return true;
            }
        }
        false
    }

    /// Uses ray casting to determine if a point is inside the polygon
    fn is_point_inside_by_ray_casting(&self, p: Point) -> bool {
        let mut crossings = 0;
        for edge in &self.edges {
            if edge.crosses_horizontal_at_y(p.y) {
                if edge.point_left_of_intersection_at_y(p) {
                    crossings += 1;
                }
            }
        }
        crossings % 2 == 1
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');

            let x = parts.next().unwrap().parse::<i32>().unwrap();
            let y = parts.next().unwrap().parse::<i32>().unwrap();

            Point { x, y }
        })
        .collect()
}

#[allow(unused_variables)]
pub fn part1() {
    let input = get_input(2025, 9);

    let points = parse_input(&input);

    let mut max_area: u64 = 0;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let p1 = points[i];
            let p2 = points[j];

            let rect = Rect { p1, p2 };
            let area = rect.area();

            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("Max area: {}", max_area);
}

#[allow(unused_variables)]
pub fn part2() {
    let input = get_input(2025, 9);

    let points: Vec<Point> = parse_input(&input);
    let polygon = Polygon::new(points.clone());

    // Generate all candidate rectangles with their areas
    let mut candidates: Vec<(Rect, u64)> = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            let rect = Rect { p1, p2 };
            let area = rect.area();
            candidates.push((rect, area));
        }
    }

    // Sort by area descending - check largest first
    candidates.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    // Find the largest contained rectangle
    let mut max_area: u64 = 0;

    for (rect, area) in candidates {
        if polygon.can_contain_rect(&rect) {
            max_area = area;
            break; // Found the largest, exit early
        }
    }

    println!("Max contained area: {}", max_area);
}
