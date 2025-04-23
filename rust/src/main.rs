mod args;
mod binary_tree_maximum_path_sum;
use binary_tree_maximum_path_sum::TreeNode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let (root) = get_args()?;

    let mut root = TreeNode::new(-10);
    let node2 = TreeNode::new(9);
    let node3 = TreeNode::new(20);
    let node4 = TreeNode::new(15);
    let node5 = TreeNode::new(7);

    root.left = Some(Box::new(node2));
    root.right = Some(Box::new(node3));
    root.right.as_mut().unwrap().left = Some(Box::new(node4));
    root.right.as_mut().unwrap().right = Some(Box::new(node5));

    println!("TreeNode Root: {:#?}", root);
    println!("Max path sum: {}", root.max_path_sum());
    Ok(())
}
