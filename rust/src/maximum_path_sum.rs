#[derive(Debug)]
pub struct TreeNode {
    val: i64,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(val: i64) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn max_path_sum(&self) -> i64 {
        fn dfs(node: &Option<Box<TreeNode>>, max_sum: &mut i64) -> i64 {
            if let Some(n) = node {
                let left = dfs(&n.left, max_sum).max(0);
                let right = dfs(&n.right, max_sum).max(0);
                *max_sum = (*max_sum).max(left + right + n.val);
                n.val + left.max(right)
            } else {
                0
            }
        }

        let mut max_sum = i64::MIN;
        dfs(&Some(Box::new(self.clone())), &mut max_sum);
        max_sum
    }
}

impl Clone for TreeNode {
    fn clone(&self) -> Self {
        TreeNode {
            val: self.val,
            left: self.left.as_ref().map(|l| Box::new((**l).clone())),
            right: self.right.as_ref().map(|r| Box::new((**r).clone())),
        }
    }
}
