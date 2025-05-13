# Trabalho de Algoritmos - 1º Bimestre

Este repositório contém o desenvolvimento do trabalho referente ao primeiro bimestre da disciplina de Algoritmos. A atividade consiste na implementação do algoritmo **Binary Tree Maximum Path Sum** em **três linguagens de programação diferentes**.

## 👨‍💻 Equipe

- João Amoris
- Alisson Dutra
- Carlos Fernandes - 140650

## 📌 Descrição do Trabalho

A proposta do trabalho consiste em:

1. **Escolha do algoritmo clássico**: Implementação da **Binary Tree Maximum Path Sum**.
2. **Implementação do algoritmo em no mínimo 3 linguagens** escolhidas pela equipe.
3. **Análise comparativa** entre as linguagens quanto a:
   - Facilidade de **escrita** (medida em número de linhas de código);
   - Facilidade de **leitura** (avaliada por meio de um questionário aplicado a colegas programadores).

## 🔢 Algoritmo Escolhido
Foi utilizado um algoritmo **Binary Tree Maximum Path Sum**. Este algoritmo tem como objetivo encontrar o maior caminho de soma em uma árvore binária, onde o caminho pode começar e terminar em qualquer nó. A ideia principal é calcular, para cada nó, o maior ganho possível vindo de seus filhos e atualizar um valor global com a maior soma encontrada.

### Casos de Borda
- **Árvore vazia**: Se a árvore não contém nós, a soma máxima do caminho é geralmente definida como 0 ou um valor mínimo, dependendo da implementação.
- **Árvore com um único nó**: O maior caminho será o valor do próprio nó.
- **Valores negativos**: O algoritmo considera que caminhos com soma negativa não contribuem para o resultado, e por isso os ignora ao calcular a soma máxima.

### Complexidade
- **Tempo**: O algoritmo percorre cada nó da árvore exatamente uma vez, resultando em uma complexidade de tempo de `O(n)`, onde `n` é o número de nós na árvore.
- **Espaço**: A complexidade de espaço é `O(h)`, onde `h` é a altura da árvore, devido à pilha de recursão utilizada na travessia em profundidade.
## 💻 Linguagens Utilizadas

- C
- Python
- Rust

Cada linguagem foi utilizada para implementar a mesma lógica da **Binary Tree Maximum Path Sum**, respeitando as boas práticas de identação exigidas, conforme o exemplo abaixo:

### C
```c
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
```
### Python
```python
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
```

### Rust
```rust
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
```