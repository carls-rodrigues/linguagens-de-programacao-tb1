#include <limits.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct TreeNode {
    long long val;
    struct TreeNode* left;
    struct TreeNode* right;
} TreeNode;

TreeNode* new_node(long long val) {
    TreeNode* node = (TreeNode*)malloc(sizeof(TreeNode));
    node->val = val;
    node->left = NULL;
    node->right = NULL;
    return node;
}

long long dfs(TreeNode* node, long long* max_sum) {
    if (node == NULL) return 0;

    long long left = dfs(node->left, max_sum);
    long long right = dfs(node->right, max_sum);

    if (left < 0) left = 0;
    if (right < 0) right = 0;

    long long local_sum = left + right + node->val;
    if (local_sum > *max_sum) *max_sum = local_sum;

    return node->val + (left > right ? left : right);
}

long long max_path_sum(TreeNode* root) {
    long long max_sum = LLONG_MIN;
    dfs(root, &max_sum);
    return max_sum;
}

int main() {
    TreeNode* root = new_node(-10);
    root->left = new_node(9);
    root->right = new_node(20);
    root->right->left = new_node(15);
    root->right->right = new_node(7);

    printf("Max path sum: %lld\n", max_path_sum(root));

    // Liberação de memória omitida por simplicidade
    return 0;
}
