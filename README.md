# Calculadora de Pagamento

Uma calculadora simples para calcular pagamentos considerando taxa fixa, custos de transporte e deduções, implementada em Rust com aritmética de precisão fixa para valores monetários.

## Características

- **Aritmética de precisão fixa**: Usa inteiros para representar valores monetários (centavos), evitando erros de ponto flutuante
- **Parsing sem floats**: Analisa valores decimais dividindo a string no ponto decimal
- **Interface de linha de comando**: Interface simples e intuitiva
- **Testes abrangentes**: Cobertura completa de casos de teste

## Estrutura do Projeto

```
calculadora_pagamento/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs          # Lógica principal e tipo Money
│   └── bin/
│       └── main.rs     # Executável principal
```

## Funcionalidades

A calculadora processa os seguintes dados:

- **Taxa fixa**: Valor base do pagamento
- **Taxa de transporte**: Custo por viagem (ida e volta são contadas separadamente)
- **Dias trabalhados**: Número de dias para calcular o transporte
- **Deduções**: Valores a serem subtraídos do pagamento total

### Fórmula de Cálculo

```
Custo do Transporte = Taxa de Transporte × Dias Trabalhados × 2
Pagamento Final = Taxa Fixa + Custo do Transporte - Deduções
```

## Como Usar

### Instalação

1. Certifique-se de ter o Rust instalado:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone o repositório:
   ```bash
   git clone <url-do-repositorio>
   cd calculadora_pagamento
   ```

### Executar

```bash
cargo run
```

### Exemplo de Uso

```
=== CALCULADORA DE PAGAMENTO ===

Digite a taxa fixa (R$):
150.00
Digite a taxa de transporte por viagem (R$):
7.50
Digite o número de dias trabalhados:
10
Digite o valor das deduções (R$):
25.00

========================================
RESUMO DO PAGAMENTO
========================================
Taxa fixa: R$ 150.00
Dias trabalhados: 10
Taxa de transporte por viagem: R$ 7.50
Custo total do transporte: R$ 150.00
  (10 dias × R$ 7.50 × 2 viagens)
Deduções: R$ 25.00
----------------------------------------
PAGAMENTO FINAL: R$ 275.00
========================================
```

## Testes

Execute os testes com:

```bash
cargo test
```

Os testes cobrem:
- Parsing de valores monetários (incluindo negativos)
- Cálculos básicos e com deduções
- Casos extremos (valores zero, deduções maiores que pagamento)
- Formatação de saída

## Implementação Técnica

### Tipo `Money`

O tipo `Money` representa valores monetários usando um `i64` interno que armazena centavos:

```rust
pub struct Money(i64); // centavos
```

### Parsing Sem Floats

O parsing de strings é feito dividindo no ponto decimal e processando cada parte como inteiros:

```rust
// "123.45" → 123 reais + 45 centavos → 12345 centavos
// "-20.50" → -(20 reais + 50 centavos) → -2050 centavos
```

### Operações Suportadas

- Adição e subtração entre valores `Money`
- Multiplicação de `Money` por `i32`
- Comparações (igualdade, ordenação)
- Formatação para exibição (sempre com 2 casas decimais)

## Formatos de Entrada Aceitos

- `100` → R$ 100.00
- `100.` → R$ 100.00
- `123.45` → R$ 123.45
- `7.5` → R$ 7.50
- `-20.50` → R$ -20.50
- ` ` (vazio) → R$ 0.00

## Limitações

- Máximo de 2 casas decimais
- Valores devem estar dentro do range de `i64` (±9,223,372,036,854,775,807 centavos)
- Interface apenas em português

## Compilação

Para compilar o projeto:

```bash
cargo build --release
```

O executável estará em `target/release/calculadora`.

## Licença

Este projeto é de código aberto. Consulte o arquivo LICENSE para detalhes.
