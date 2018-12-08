use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    let assigned = kosaraju(&vec![vec![1, 2], vec![0], vec![0], vec![0, 1, 2]]);
    println!("{:?}", assigned);
}

fn kosaraju(adjacent: &Vec<Vec<usize>>) -> HashMap<usize, usize> {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut l: VecDeque<usize> = VecDeque::new();
    for u in 0..adjacent.len() {
        visit(u, adjacent, &mut visited, &mut l);
    }
    let adjacent = &transpose(adjacent);
    let mut assigned = HashMap::new();
    for u in l {
        assign(u, u, adjacent, &mut assigned);
    }
    assigned
}

fn visit(
    u: usize,
    adjacent: &Vec<Vec<usize>>,
    visited: &mut HashSet<usize>,
    l: &mut VecDeque<usize>,
) {
    if !visited.contains(&u) {
        visited.insert(u);
        for &v in &adjacent[u] {
            visit(v, adjacent, visited, l);
        }
        l.push_front(u);
    }
}

fn assign(
    u: usize,
    root: usize,
    adjacent: &Vec<Vec<usize>>,
    assigned: &mut HashMap<usize, usize>,
) {
    if !assigned.contains_key(&u) {
        assigned.insert(u, root);
        for &v in &adjacent[u] {
            assign(v, root, adjacent, assigned);
        }
    }
}

fn transpose(adjacent: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut inverted: Vec<Vec<usize>> = vec![Vec::new(); adjacent.len()];
    for u in 0..adjacent.len() {
        for &v in &adjacent[u] {
            inverted[v].push(u);
        }
    }
    inverted
}
