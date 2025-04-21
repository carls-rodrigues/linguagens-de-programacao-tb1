# Trabalho de Algoritmos - 1º Bimestre

Este repositório contém o desenvolvimento do trabalho referente ao primeiro bimestre da disciplina de Algoritmos. A atividade consiste na implementação do algoritmo **Busca Binária** em **três linguagens de programação diferentes**.

## 👨‍💻 Equipe

- Nome do Integrante 1
- Nome do Integrante 2
- Carlos Fernandes - 140650

## 📌 Descrição do Trabalho

A proposta do trabalho consiste em:

1. **Escolha do algoritmo clássico**: Implementação da **Busca Binária**.
2. **Implementação do algoritmo em no mínimo 3 linguagens** escolhidas pela equipe.
3. **Análise comparativa** entre as linguagens quanto a:
   - Facilidade de **escrita** (medida em número de linhas de código);
   - Facilidade de **leitura** (avaliada por meio de um questionário aplicado a colegas programadores).

## 🔢 Algoritmo Escolhido

O algoritmo escolhido para este trabalho é a **Busca Binária**. Ele é utilizado para encontrar a posição de um elemento em um vetor ordenado. O processo consiste em dividir o vetor em duas metades, comparando o valor do meio com o valor alvo e ajustando os limites da busca (low e high) até encontrar o valor ou concluir que ele não existe no vetor.

## 💻 Linguagens Utilizadas

- Linguagem 1 (ex: C)
- Linguagem 2 (ex: Python)
- Rust

Cada linguagem foi utilizada para implementar a mesma lógica da **Busca Binária**, respeitando as boas práticas de identação exigidas, conforme o exemplo abaixo:

```c
while (low <= high) {
    mid = (low + high) / 2;
    if (arr[mid] == target) {
        return mid;
    } else if (arr[mid] < target) {
        low = mid + 1;
    } else {
        high = mid - 1;
    }
}
```

## 📊 Análise Comparativa

### Facilidade de Escrita

A contagem de linhas de código foi feita com base em um comando por linha e início/fim de bloco em linhas separadas.

| Linguagem | Nº de Linhas |
|-----------|--------------|
| C         | XX linhas    |
| Python    | XX linhas    |
| Rust      | 28 linhas    |

### Facilidade de Leitura

Um questionário foi aplicado a pelo menos 17 programadores (estudantes ou profissionais), que avaliaram a legibilidade de cada código com base nos seguintes critérios:

- Ranking de leitura (1 = mais legível, 3 = menos legível);
- Anos de experiência do respondente;
- Familiaridade com as linguagens apresentadas.

### Resultados

*Inserir aqui a tabela de respostas tabuladas e o gráfico com os resultados finais.*

## 🧪 Exemplo de Execução

O exemplo a seguir mostra a execução da Busca Binária nas três linguagens:

**Entrada:**
- Vetor: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
- Alvo: 7

**Saída:**
- Posição do número 7: 6 (índice 6, considerando a indexação zero)

## 📁 Estrutura do Repositório

```
.
├── linguagem1/
│   └── binary_search_l1.ext
├── linguagem2/
│   └── binary_search_l2.ext
├── rust/
│   └── binary_search.rs
├── resultados/
│   ├── questionario_respostas.xlsx
│   ├── grafico_resultado.png
│   └── tabela_comparativa.md
└── relatorio_final.pdf
```

## 📝 Relatório Final

O relatório final contém:

- Introdução ao problema;
- Explicação da Busca Binária;
- Códigos-fonte das três linguagens;
- Exemplo de uso do algoritmo;
- Análise de facilidade de escrita;
- Análise de facilidade de leitura com base no questionário;
- Gráficos e tabelas com os resultados.

---

Projeto acadêmico desenvolvido para comparação entre linguagens de programação e prática de escrita e leitura de algoritmos clássicos.