#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

pub fn run(input: String) {
    let mut values = input.split(' ').collect::<Vec<_>>();

    let tree = read_node(&mut values);

//    println!("{:#?}", tree);

    part1(&tree);
    part2(&tree);
}

fn read_node<'str>(input: &mut Vec<&str>) -> Node {
    let child_count = input[0].parse::<u32>().unwrap();
    let metadata_count = input[1].parse::<u32>().unwrap();

    input.remove(0);
    input.remove(0);

    let mut children = Vec::new();

    for _ in 0..child_count {
        children.push(read_node(input));
    }

    let mut metadata = Vec::new();

    for _ in 0..metadata_count {
        metadata.push(input[0].parse::<u32>().unwrap());
        input.remove(0);
    }

    Node {
        children,
        metadata,
    }
}

fn part1(root: &Node) {
    fn rec(node: &Node) -> u32 {
        let mut sum = 0;

        for child in &node.children {
            sum += rec(child);
        }

        for data in &node.metadata {
            sum += data;
        }

        sum
    }

    let sum = rec(root);

    println!("{}", sum);
}

fn part2(root: &Node) {
    fn rec(node: &Node) -> u32 {
        if node.children.len() == 0 {
            return node.metadata.iter().sum();
        }

        let mut sum = 0;

        for data in &node.metadata {
            if ((*data as usize) - 1) < node.children.len() {
                sum += rec(&node.children[((*data as usize) - 1)]);
            }
        }

        sum
    }

    let sum = rec(root);

    println!("{}", sum);
}