//! An implementation of the Hungarian algorithm to solve the Assignment Problem.
//! The worst case complexity is `O(n^3)`.

/// # Examples
///
/// ```
/// let weights = &vec!(
///     vec!(11,  6, 12),
///     vec!(12,  4,  6),
///     vec!( 8, 12, 11),
/// );
///
/// let mut problem = munkres::Problem::new(weights);
/// assert_eq!(problem.solve(), vec!((0,1), (1,2), (2,0)));
/// ```
#[derive(Debug)]
pub struct Problem<'a> {
    size: usize,
    weights: &'a Vec<Vec<u32>>,
    m_ij: Vec<Option<usize>>, // Edges of the matching indexed by the row
    m_ji: Vec<Option<usize>>, // Edges of the matching indexed by the column
    y_i: Vec<u32>, // Dual variables for the rows
    y_j: Vec<u32>, // Dual variables for the columns
    min_slack: Vec<MinSlack>, // Slack on the y_j variable
    tree_i: Vec<Option<usize>>, // Ancestors of the even vertices in the alternating tree
    tree_j: Vec<Option<usize>>, // Ancestors of the odd vertices in the alternating tree
    tight_edges: Vec<(usize, usize)>, // Candidate edges to be added to the tree
}

#[derive(Debug)]
struct MinSlack {
    value: u32,
    arg: usize,
}

impl Clone for MinSlack {
    fn clone(&self) -> MinSlack {
        MinSlack { value: self.value, arg: self.arg}
    }
}

impl<'a> Problem<'a> {
    pub fn new(weights: &Vec<Vec<u32>>) -> Problem {
        let size = weights.len();
        Problem {
            size,
            weights,
            y_i: vec![0; size],
            y_j: vec![0; size],
            m_ij: vec![None; size],
            m_ji: vec![None; size],
            tree_i: vec![None; size],
            tree_j: vec![None; size],
            min_slack: vec![MinSlack{ value: 0, arg: 0 } ;size],
            tight_edges: Vec::with_capacity(size),
        }
    }

    // Return the slack in the following inequality:
    //    weights[i][j] + y_j[j] >= y_i[i]
    fn slack(&self, i: usize, j: usize) -> u32 {
        self.weights[i][j] + self.y_j[j] - self.y_i[i]
    }

    // Return a row not covered by the matching.
    fn free_even_vertex(&self) -> Option<usize> {
        self.m_ij.iter().position(|&i| i.is_none())
    }

    // Given the root of the tree, set the min slack to the tree and the tight edges to the tree.
    fn initialize_tree(&mut self, r: usize) {
        for j in 0..self.size {
            self.min_slack[j].value = self.slack(r, j);
            self.min_slack[j].arg = r;
            if self.min_slack[j].value == 0 {
                self.tight_edges.push((r, j));
            }
            self.tree_j[j] = None;
            self.tree_i[j] = None;
        }
    }

    // Expands the alternating tree to an odd vertex. The search path consists of the tight edges.
    // If the expansion succeeds, it returns the odd vertex.
    fn add_odd_vertex(&mut self) -> Option<usize> {
        while let Some((i, j)) = self.tight_edges.pop() {
            // Add the edge to the tree if it expends it.
            if self.tree_j[j] == None {
                self.tree_j[j] = Some(i);
                return Some(j);
            }
        }
        None
    }

    // Expand the alternating tree to an even vertex. It updates the min_slacks of the odd
    // vertices not in tree and collects the newly created tight edges (if any).
    fn add_even_vertex(&mut self, i: usize, j: usize) {
        // Set j as i's ancestor in the tree
        self.tree_i[i] = Some(j);

        // update the min_slack of the odd vertices not in the tree
        for j in 0..self.size {
            if self.tree_j[j].is_none() {
                let slack = self.slack(i, j);
                if slack < self.min_slack[j].value {
                    if slack == 0 {
                        // collect the edge if tight
                        self.tight_edges.push((i, j));
                    } else {
                        // update the min_slack otherwise
                        self.min_slack[j].value = slack;
                        self.min_slack[j].arg = i;
                    }
                }
            }
        }
    }

    // Update the dual variables.
    fn update_dual(&mut self, root: usize) {
        // Odd vertices not in the tree
        let odd_out: Vec<usize> = (0..self.size)
            .filter(|&j| self.tree_j[j].is_none())
            .collect();

        // Find the smallest min_slack to the odd vertices not covered by the tree
        let delta: u32 = odd_out.iter().skip(1).fold(
            self.min_slack[odd_out[0]].value,
            |m, &j| {
                let s = self.min_slack[j].value;
                if s < m { s } else { m }
            },
        );

        // Update the dual variables
        for i in 0..self.size {
            if i == root || self.tree_i[i].is_some() {
                self.y_i[i] += delta;
            }
            if self.tree_j[i].is_some() {
                self.y_j[i] += delta;
            }
        }

        // Update the min_slacks
        for j in odd_out {
            let slack = &mut self.min_slack[j];
            if delta == slack.value {
                self.tight_edges.push((slack.arg, j));
            } else {
                slack.value -= delta;
            }
        }
    }

    fn alternate_path(&mut self, j: usize) {
        let mut odd = Some(j);
        while let Some(j) = odd {
            let i = self.tree_j[j].unwrap();
            odd = self.tree_i[i];
            self.m_ij[i] = Some(j);
            self.m_ji[j] = Some(i);
        }
    }

    // Greedily build a matching.
    fn greedy_algo(&mut self) {
        // Maximize the dual variables of the even vertices.
        for i in 0..self.size {
            self.y_i[i] = self.weights[i].iter().skip(1).fold(
                self.weights[i][0],
                |m, &x| if x < m { x } else { m },
            );
        }

        for i in 0..self.size {
            for j in 0..self.size {
                if self.slack(i, j) == 0 && self.m_ji[j] == None {
                    self.m_ij[i] = Some(j);
                    self.m_ji[j] = Some(i);
                    break;
                }
            }
        }
    }

    pub fn solve(&mut self) -> Vec<(usize, usize)> {
        self.greedy_algo();

        // As long as there is an even vertex not covered by the matching
        while let Some(r) = self.free_even_vertex() {
            // Start an alternating tree from that vertex
            self.initialize_tree(r);

            #[allow(while_true)]
            while true {
                // Expand the alternating tree with an odd vertex
                let j: usize;
                match self.add_odd_vertex() {
                    Some(k) => j = k,
                    None => {
                        self.update_dual(r);
                        j = self.add_odd_vertex().unwrap();
                    }
                }

                // Is the new odd vertex covered by the matching?
                if let Some(i) = self.m_ji[j] {
                    // Add the edge of the matching to the tree
                    self.add_even_vertex(i, j);
                } else {
                    // An augmenting path is found
                    self.alternate_path(j);
                    break;
                }

            }
        }

        self.m_ij
            .iter()
            .enumerate()
            .map(|(i, &j)| (i, j.unwrap()))
            .collect::<Vec<(usize, usize)>>()

    }
}
