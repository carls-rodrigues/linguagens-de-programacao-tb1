import sys

class TreeNode:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None

def max_path_sum(root):
    max_sum = [-sys.maxsize]

    def dfs(node):
        if not node:
            return 0

        left = max(dfs(node.left), 0)
        right = max(dfs(node.right), 0)

        max_sum[0] = max(max_sum[0], left + right + node.val)

        return node.val + max(left, right)

    dfs(root)
    return max_sum[0]

if __name__ == "__main__":
    root = TreeNode(-10)
    root.left = TreeNode(9)
    root.right = TreeNode(20)
    root.right.left = TreeNode(15)
    root.right.right = TreeNode(7)

    print("Max path sum:", max_path_sum(root))
