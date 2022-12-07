use libaoc::Timer;

#[derive(Debug, Clone)]
struct TreeNode {
    parent_idx: usize,
    size: usize,
    name: String,
    children: Vec<usize>,
}

pub fn solve(timer: &mut Timer, input: &str) -> () {
    let mut tree = vec![TreeNode {
        parent_idx: 0,
        size: 0,
        name: String::from("/"),
        children: vec![],
    }];
    let mut current_dir = 0;
    let mut iter = input.lines().skip(1).peekable();

    while let Some(line) = iter.next() {
        // skip `cd /`
        if line == "$ ls" {
            while let Some(line) = iter.peek() {
                if line.starts_with('$') {
                    break;
                }

                let (size, name) = line.split_once(' ').unwrap();
                let size = size.parse().unwrap_or(0);
                tree.push(TreeNode {
                    parent_idx: current_dir,
                    size,
                    name: name.to_string(),
                    children: vec![],
                });
                let id = tree.len() - 1;
                tree[current_dir].children.push(id);
                iter.next();
            }
        } else if line == "$ cd .." {
            current_dir = tree[current_dir].parent_idx;
        } else {
            let (_, dir) = line.rsplit_once(' ').unwrap();

            for &child in &tree[current_dir].children {
                if tree[child].name == dir {
                    current_dir = child;
                }
            }
        }
    }
    set_sizes(&mut tree, 0);
    timer.lap("Parse");

    let mut part_1 = 0;
    for node in &tree {
        if !node.children.is_empty() && node.size <= 100000 {
            part_1 += node.size;
        }
    }
    timer.lap("Part 1");

    let required = tree[0].size - (70000000 - 30000000);
    let mut part_2 = usize::MAX;
    for node in &tree {
        if !node.children.is_empty() && node.size >= required {
            part_2 = part_2.min(node.size);
        }
    }
    timer.lap("Part 2");

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}

fn set_sizes(tree: &mut [TreeNode], root: usize) -> usize {
    let mut size = tree[root].size;

    for node in tree[root].children.clone() {
        size += set_sizes(tree, node);
    }

    tree[root].size = size;

    size
}
