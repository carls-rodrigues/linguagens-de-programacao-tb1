mod maximum_path_sum;

use maximum_path_sum::TreeNode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut root = TreeNode::new(-10);
    let node2 = TreeNode::new(9);
    let node3 = TreeNode::new(20);
    let node4 = TreeNode::new(15);
    let node5 = TreeNode::new(7);

    root.left = Some(Box::new(node2));
    root.right = Some(Box::new(node3));
    root.right.as_mut().unwrap().left = Some(Box::new(node4));
    root.right.as_mut().unwrap().right = Some(Box::new(node5));

    println!("Max path sum: {}", root.max_path_sum());
    Ok(())
}
