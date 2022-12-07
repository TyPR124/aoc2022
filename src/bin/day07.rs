use aoc2022::{Day, Solution};

fn main() -> anyhow::Result<()> {
    aoc2022::run_solution::<Day07>()
}

#[test]
fn test_day07_solution() {
    aoc2022::test_solution::<Day07>()
}

struct Day07;
impl Solution for Day07 {
    const DAY: aoc2022::Day = match Day::number(7) {
        Ok(day) => day,
        _ => unreachable!(),
    };
    type Output1 = usize;
    type Output2 = usize;
    fn solve(input: &str) -> anyhow::Result<(Self::Output1, Self::Output2)> {
        let mut fs = FileSystem::new();
        let mut pwd = Index(0);
        let mut lines = input.lines().skip(2);
        while let Some(line) = lines.next() {
            if let Some(dirname) = line.strip_prefix("dir ") {
                let new = fs.add_item(Node::Dir(DirNode {
                    name: dirname.to_owned(),
                    children: vec![],
                    parent: Some(pwd),
                }));
                fs.get_dir_mut(pwd).unwrap().children.push(new)
            } else if let Some(newdir) = line.strip_prefix("$ cd ") {
                match newdir {
                    "." => (),
                    ".." => pwd = fs.get_dir(pwd).unwrap().parent.unwrap(),
                    _ => {
                        pwd = fs
                            .get_dir(pwd)
                            .unwrap()
                            .children
                            .iter()
                            .copied()
                            .find(|&child| match fs.get_dir(child) {
                                Some(DirNode { name, .. }) if name == newdir => true,
                                _ => false,
                            })
                            .unwrap()
                    }
                }
            } else if line == "$ ls" {
            } else if let Some((size, _name)) = line.split_once(' ') {
                let size = size.parse().unwrap();
                // let name = name.to_owned();
                let new = fs.add_item(Node::File(FileNode { size }));
                fs.get_dir_mut(pwd).unwrap().children.push(new)
            } else {
                unreachable!("invalid input data")
            }
        }
        let used_space = calculdate_dir_total_size(&fs, Index(0));
        const FS_TOTAL_SPACE: usize = 70000000;
        const FS_REQUIRED_SPACE: usize = 30000000;
        let free_space = FS_TOTAL_SPACE - used_space;
        let min_to_delete = FS_REQUIRED_SPACE - free_space;
        let mut deleting = usize::MAX;
        let mut part1_sum = 0;
        for (i, item) in fs.nodes.iter().enumerate() {
            if matches!(item, Node::Dir(_)) {
                let dsize = calculdate_dir_total_size(&fs, Index(i));
                if dsize <= 100_000 {
                    part1_sum += dsize
                }
                if dsize >= min_to_delete && dsize < deleting {
                    deleting = dsize;
                }
            }
        }
        Ok((part1_sum, deleting))
    }
}

#[derive(Debug)]
struct FileSystem {
    nodes: Vec<Node>,
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node::Dir(DirNode {
                name: "/".to_owned(),
                children: vec![],
                parent: None,
            })],
        }
    }
    pub fn get_dir(&self, index: Index) -> Option<&DirNode> {
        match &self.nodes[index.0] {
            Node::File(_) => None,
            Node::Dir(dir) => Some(dir),
        }
    }
    pub fn get_dir_mut(&mut self, index: Index) -> Option<&mut DirNode> {
        match &mut self.nodes[index.0] {
            Node::File(_) => None,
            Node::Dir(dir) => Some(dir),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Index(usize);
#[derive(Debug)]
enum Node {
    File(FileNode),
    Dir(DirNode),
}
#[derive(Debug)]
struct FileNode {
    // name: String,
    size: usize,
}
#[derive(Debug)]
struct DirNode {
    name: String,
    children: Vec<Index>,
    parent: Option<Index>,
}

impl FileSystem {
    pub fn add_item(&mut self, node: Node) -> Index {
        self.nodes.push(node);
        Index(self.nodes.len() - 1)
    }
}

fn calculdate_dir_total_size(fs: &FileSystem, dir: Index) -> usize {
    let d = fs.get_dir(dir).unwrap();
    let mut sum = 0;
    for &child in &d.children {
        match &fs.nodes[child.0] {
            Node::File(file) => sum += file.size,
            Node::Dir(_) => sum += calculdate_dir_total_size(fs, child),
        }
    }
    sum
}
