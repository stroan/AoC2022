use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

const INPUT: &str = include_str!("real_input.txt");

fn main() {
    let fs = FileSystem::load();

    let mut dir_sizes: HashMap<NodeId, usize> = Default::default();

    // part 1
    {
        fs.visit(|fs, node_id, path| match &fs[node_id] {
            Node::File { size } => {
                for (_, dir_id) in path.iter() {
                    let current_size = dir_sizes.get(dir_id).map(|v| v.clone()).unwrap_or_default();
                    dir_sizes.insert(*dir_id, current_size + size);
                }
            }
            _ => {}
        });

        let part1: usize = dir_sizes
            .iter()
            .filter(|(_, size)| **size <= 100000)
            .map(|(_, size)| *size)
            .sum();
        dbg!(part1);
    }

    // part 2
    {
        const TOTAL_SIZE: usize = 70000000;
        const REQUIRED_SIZE: usize = 30000000;

        let free_space = TOTAL_SIZE - *dir_sizes.get(&fs.root_id()).unwrap();
        let (_, part2) = dir_sizes
            .iter()
            .filter(|(_, size)| free_space + **size >= REQUIRED_SIZE)
            .min_by_key(|(_, size)| Some(*size))
            .unwrap();
        dbg!(*part2);
    }
}

type NodeId = usize;

#[derive(Debug)]
struct FileSystem {
    nodes: Vec<Node>,
}

impl Default for FileSystem {
    fn default() -> Self {
        Self {
            nodes: vec![Node::Dir {
                children: Default::default(),
            }],
        }
    }
}

impl Index<NodeId> for FileSystem {
    type Output = Node;

    fn index(&self, index: NodeId) -> &Self::Output {
        &self.nodes[index]
    }
}

impl IndexMut<NodeId> for FileSystem {
    fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

impl FileSystem {
    fn root_id(&self) -> NodeId {
        0
    }

    fn insert(&mut self, node: Node) -> NodeId {
        let node_id = self.nodes.len();
        self.nodes.push(node);
        node_id
    }

    fn visit<F: FnMut(&Self, NodeId, &[(&'static str, NodeId)])>(&self, mut f: F) {
        let mut boundary = vec![(self.root_id(), "/", vec![])];

        while let Some((node_id, node_name, path)) = boundary.pop() {
            f(self, node_id, &path);
            match &self[node_id] {
                Node::Dir { children } => {
                    for (child_name, child_id) in children.iter() {
                        let mut child_path = path.clone();
                        child_path.push((node_name, node_id));
                        boundary.push((*child_id, child_name, child_path))
                    }
                }
                _ => {}
            }
        }
    }

    fn load() -> Self {
        let mut fs = FileSystem::default();

        let mut dir_stack: Vec<usize> = vec![];
        let mut cwd = fs.root_id();

        for line in INPUT.lines() {
            if line.starts_with("$ ls") {
                // ignore
            } else if line.starts_with("$ cd ") {
                match &line[5..] {
                    "/" => {
                        cwd = fs.root_id();
                        dir_stack.clear();
                    }
                    ".." => {
                        cwd = dir_stack.pop().unwrap();
                    }
                    dir => {
                        dir_stack.push(cwd);
                        cwd = fs[cwd].lookup(dir);
                    }
                }
            } else if line.starts_with("dir") {
                let new_dir = fs.insert(Node::Dir {
                    children: Default::default(),
                });
                fs[cwd].add_child(&line[4..], new_dir);
            } else {
                let (size_str, name) = line.split_once(" ").unwrap();
                let size = size_str.parse().unwrap();
                let new_file = fs.insert(Node::File { size });
                fs[cwd].add_child(name, new_file);
            }
        }

        fs
    }
}

#[derive(Debug)]
enum Node {
    Dir {
        children: HashMap<&'static str, NodeId>,
    },
    File {
        size: usize,
    },
}

impl Node {
    fn lookup(&self, name: &str) -> NodeId {
        match self {
            Node::Dir { children } => *children.get(name).unwrap(),
            _ => panic!("File has no children"),
        }
    }

    fn add_child(&mut self, name: &'static str, node_id: NodeId) {
        match self {
            Node::Dir { children } => {
                children.insert(name, node_id);
            }
            _ => panic!("File can't have children"),
        }
    }
}
