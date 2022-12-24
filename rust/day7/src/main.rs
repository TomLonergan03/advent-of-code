use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = Path::new("../../inputs/day7.txt");
    let raw_text: Vec<String> = read_lines(path)
        .expect("Line buffer failure")
        .map(|line| Result::expect(line, "Line read failure"))
        .collect::<String>()
        .replace("\n", ",")
        .split("$")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let mut tree: Tree = Tree::new();
    tree.build_tree(raw_text);

    let part_1_total_size = 0;

    let part_2_first_marker = 0;

    println!("Part 1 first marker at = {part_1_total_size}");
    println!(
        "Part 2 start of message marker at = {}",
        part_2_first_marker
    );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone)]
pub struct Node {
    name: String,
    parent: usize,
    children: Option<Vec<usize>>,
    size: Option<i32>,
}

impl Node {
    pub fn new(name: String, children: Option<Vec<usize>>, value: Option<i32>) -> Node {
        Node {
            name: (name),
            parent: (usize::MAX),
            children: (children),
            size: (value),
        }
    }
    pub fn add_child(mut self, new_node: usize, parent: usize) {
        self.children.unwrap_or(vec![]).push(new_node);
        self.parent = parent;
    }

    pub fn get_children(self) -> Option<Vec<usize>> {
        return self.children;
    }

    pub fn get_value(self) -> i32 {
        return self.size.expect("Node has no value");
    }
}

#[derive(Clone)]
pub struct Tree {
    root: usize,
    tree: Vec<Node>,
    current_node: usize,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            root: (0),
            tree: (vec![]),
            current_node: (0),
        }
    }

    fn add_node(mut self, new_node: Node) {
        let next_node_id = self.tree.len();
        self.tree.push(new_node);
    }

    pub fn set_current_node(mut self, target_node: usize) {
        self.current_node = target_node;
    }

    fn move_to_child(mut self, id: usize) {
        self.current_node = id;
    }

    fn move_to_parent(mut self) {
        self.current_node = self.tree[self.current_node].parent;
    }

    pub fn build_tree(self, commands: Vec<String>) {
        self.add_node(Node::new("/".to_string(), None, None));
        for line in commands {
            if line.contains("$ cd") {
                self.add_node(Node::new(line.replace("$ cd ", ""), None, None));
                continue;
            }
            if line.contains("$ ls") {
                continue;
            }
            if line.contains("dir ") {
                self.add_node(Node::new(line.replace("dir ", ""), None, None));
                continue;
            }
            self.add_node(Node::new(
                line.split(' ').next().expect("No file size").to_string(),
                None,
                None,
            ));
        }
    }
}
