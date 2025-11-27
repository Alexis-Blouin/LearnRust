use std::cmp::Reverse;
use priority_queue::PriorityQueue;

#[derive(Debug)]
struct Node {
    data: i32,
    left: Option<usize>,
    right: Option<usize>,
}

impl Node {
    fn new(data: i32) -> Node {
        Self {
            data,
            left: None,
            right: None,
        }
    }
}

fn main() {
    let s = "abcdef".to_string();
    let freq = vec![5, 9, 12, 13, 16, 45];

    let ans = huffman_codes(&s, &freq);
    for a in ans {
        println!("{}", a);
    }
}

fn pre_order(nodes: &Vec<Node>, idx: usize, ans: &mut Vec<String>, curr: String) {
    let node = &nodes[idx];

    if node.left.is_none() && node.right.is_none() {
        ans.push(curr);
        return;
    }

    if let Some(l) = node.left {
        pre_order(nodes, l, ans, curr.clone() + "0");
    }
    if let Some(r) = node.right {
        pre_order(nodes, r, ans, curr + "1");
    }
}

fn huffman_codes(s: &String, freq: &Vec<i32>) -> Vec<String> {
    let n = s.len();
    let mut pq = PriorityQueue::<usize, Reverse<i32>>::new();
    let mut nodes: Vec<Node> = Vec::new();

    // Insert leaf nodes
    for i in 0..n {
        nodes.push(Node::new(freq[i]));
        pq.push(i, Reverse(freq[i]));
    }

    // Build the tree
    while pq.len() >= 2 {
        let (left_idx, Reverse(lv)) = pq.pop().unwrap();
        let (right_idx, Reverse(rv)) = pq.pop().unwrap();

        let new_node = Node {
            data: lv + rv,
            left: Some(left_idx),
            right: Some(right_idx),
        };

        nodes.push(new_node);
        let new_idx = nodes.len() - 1;

        pq.push(new_idx, Reverse(lv + rv));
    }

    let (root_idx, _) = pq.pop().unwrap();

    let mut ans = vec![];
    pre_order(&nodes, root_idx, &mut ans, "".to_string());
    ans
}
