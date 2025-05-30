use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Money(i64); // centavos

impl Money {
    pub const ZERO: Money = Money(0);

    pub fn from_reais(reais: i64) -> Self {
        Money(reais * 100)
    }

    pub fn from_centavos(centavos: i64) -> Self {
        Money(centavos)
    }

    pub fn to_centavos(self) -> i64 {
        self.0
    }

    pub fn parse(s: &str) -> Result<Self, String> {
        let s = s.trim();
        if s.is_empty() {
            return Ok(Money::ZERO);
        }

        let is_negative = s.starts_with('-');
        let s = if is_negative { &s[1..] } else { s };

        if let Some(dot_pos) = s.find('.') {
            let (reais_str, cents_str) = s.split_at(dot_pos);
            let cents_str = &cents_str[1..]; // remove the dot

            let reais: i64 = reais_str.parse().map_err(|_| "Invalid reais part")?;

            let cents = if cents_str.is_empty() {
                0
            } else if cents_str.len() == 1 {
                let digit: i64 = cents_str.parse().map_err(|_| "Invalid cents part")?;
                digit * 10
            } else if cents_str.len() == 2 {
                cents_str.parse().map_err(|_| "Invalid cents part")?
            } else {
                return Err("Too many decimal places".to_string());
            };

            let total_centavos = reais * 100 + cents;
            Ok(Money(if is_negative {
                -total_centavos
            } else {
                total_centavos
            }))
        } else {
            let reais: i64 = s.parse().map_err(|_| "Invalid number")?;
            let total_centavos = reais * 100;
            Ok(Money(if is_negative {
                -total_centavos
            } else {
                total_centavos
            }))
        }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reais = self.0 / 100;
        let centavos = (self.0 % 100).abs();
        write!(f, "{}.{:02}", reais, centavos)
    }
}

impl std::ops::Add for Money {
    type Output = Money;

    fn add(self, other: Money) -> Money {
        Money(self.0 + other.0)
    }
}

impl std::ops::Sub for Money {
    type Output = Money;

    fn sub(self, other: Money) -> Money {
        Money(self.0 - other.0)
    }
}

impl std::ops::Mul<i32> for Money {
    type Output = Money;

    fn mul(self, other: i32) -> Money {
        Money(self.0 * other as i64)
    }
}

#[derive(Debug, Clone)]
pub struct ResultadoCalculo {
    pub taxa_fixa: Money,
    pub taxa_transporte: Money,
    pub dias_trabalhados: i32,
    pub custo_transporte: Money,
    pub deducoes_total: Money,
    pub pagamento_final: Money,
}

pub fn calcular_valores(
    taxa_fixa: Money,
    taxa_transporte: Money,
    dias_trabalhados: i32,
    deducoes_total: Money,
) -> ResultadoCalculo {
    let custo_transporte = taxa_transporte * dias_trabalhados * 2;
    let pagamento_final = taxa_fixa + custo_transporte - deducoes_total;

    ResultadoCalculo {
        taxa_fixa,
        taxa_transporte,
        dias_trabalhados,
        custo_transporte,
        deducoes_total,
        pagamento_final,
    }
}

pub fn obter_valor_numerico(prompt: &str) -> Money {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler entrada");

        match Money::parse(&input) {
            Ok(valor) => return valor,
            Err(_) => println!("Erro: Por favor, digite um valor numérico válido."),
        }
    }
}

pub fn obter_inteiro(prompt: &str) -> i32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler entrada");

        match input.trim().parse::<i32>() {
            Ok(valor) => return valor,
            Err(_) => println!("Erro: Por favor, digite um número inteiro válido."),
        }
    }
}

pub fn calcular_pagamento() {
    println!("=== CALCULADORA DE PAGAMENTO ===\n");

    // Coleta de dados
    let taxa_fixa = obter_valor_numerico("Digite a taxa fixa (R$):");
    let taxa_transporte = obter_valor_numerico("Digite a taxa de transporte por viagem (R$):");
    let dias_trabalhados = obter_inteiro("Digite o número de dias trabalhados:");
    let deducoes_total = obter_valor_numerico("Digite o valor das deduções (R$):");

    // Cálculo
    let resultado = calcular_valores(taxa_fixa, taxa_transporte, dias_trabalhados, deducoes_total);

    // Exibição dos resultados
    println!("\n{}", "=".repeat(40));
    println!("RESUMO DO PAGAMENTO");
    println!("{}", "=".repeat(40));
    println!("Taxa fixa: R$ {}", resultado.taxa_fixa);
    println!("Dias trabalhados: {}", resultado.dias_trabalhados);
    println!(
        "Taxa de transporte por viagem: R$ {}",
        resultado.taxa_transporte
    );
    println!(
        "Custo total do transporte: R$ {}",
        resultado.custo_transporte
    );
    println!(
        "  ({} dias × R$ {} × 2 viagens)",
        resultado.dias_trabalhados, resultado.taxa_transporte
    );

    if resultado.deducoes_total > Money::ZERO {
        println!("Deduções: R$ {}", resultado.deducoes_total);
    }

    println!("{}", "-".repeat(40));
    println!("PAGAMENTO FINAL: R$ {}", resultado.pagamento_final);
    println!("{}", "=".repeat(40));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_parsing() {
        assert_eq!(Money::parse("123.45").unwrap(), Money::from_centavos(12345));
        assert_eq!(Money::parse("7.50").unwrap(), Money::from_centavos(750));
        assert_eq!(Money::parse("7.5").unwrap(), Money::from_centavos(750));
        assert_eq!(Money::parse("100").unwrap(), Money::from_reais(100));
        assert_eq!(Money::parse("100.").unwrap(), Money::from_reais(100));
        assert_eq!(Money::parse("").unwrap(), Money::ZERO);
        assert_eq!(Money::parse("  ").unwrap(), Money::ZERO);
        assert_eq!(Money::parse("-20.50").unwrap(), Money::from_centavos(-2050));
        assert_eq!(Money::parse("-100").unwrap(), Money::from_centavos(-10000));
        assert_eq!(Money::parse("-7.5").unwrap(), Money::from_centavos(-750));
    }

    #[test]
    fn test_calculo_basico() {
        let resultado = calcular_valores(
            Money::from_reais(100),
            Money::from_reais(5),
            10,
            Money::ZERO,
        );

        assert_eq!(resultado.custo_transporte, Money::from_reais(100)); // 5 * 10 * 2
        assert_eq!(resultado.pagamento_final, Money::from_reais(200)); // 100 + 100 - 0
    }

    #[test]
    fn test_calculo_com_deducoes() {
        let resultado = calcular_valores(
            Money::from_reais(150),
            Money::parse("7.50").unwrap(),
            8,
            Money::from_reais(25),
        );

        assert_eq!(resultado.custo_transporte, Money::from_reais(120)); // 7.50 * 8 * 2
        assert_eq!(resultado.pagamento_final, Money::from_reais(245)); // 150 + 120 - 25
    }

    #[test]
    fn test_valores_zero() {
        let resultado = calcular_valores(Money::ZERO, Money::ZERO, 0, Money::ZERO);

        assert_eq!(resultado.custo_transporte, Money::ZERO);
        assert_eq!(resultado.pagamento_final, Money::ZERO);
    }

    #[test]
    fn test_apenas_taxa_fixa() {
        let resultado = calcular_valores(Money::from_reais(200), Money::ZERO, 5, Money::ZERO);

        assert_eq!(resultado.custo_transporte, Money::ZERO);
        assert_eq!(resultado.pagamento_final, Money::from_reais(200));
    }

    #[test]
    fn test_apenas_transporte() {
        let resultado = calcular_valores(Money::ZERO, Money::from_reais(10), 6, Money::ZERO);

        assert_eq!(resultado.custo_transporte, Money::from_reais(120)); // 10 * 6 * 2
        assert_eq!(resultado.pagamento_final, Money::from_reais(120));
    }

    #[test]
    fn test_deducoes_maiores_que_pagamento() {
        let resultado = calcular_valores(
            Money::from_reais(50),
            Money::from_reais(5),
            3,
            Money::from_reais(100),
        );

        assert_eq!(resultado.custo_transporte, Money::from_reais(30)); // 5 * 3 * 2
        assert_eq!(resultado.pagamento_final, Money::from_centavos(-2000)); // 50 + 30 - 100 = -20
    }

    #[test]
    fn test_valores_decimais() {
        let resultado = calcular_valores(
            Money::parse("123.45").unwrap(),
            Money::parse("3.75").unwrap(),
            4,
            Money::parse("15.50").unwrap(),
        );

        assert_eq!(resultado.custo_transporte, Money::from_reais(30)); // 3.75 * 4 * 2
        assert_eq!(resultado.pagamento_final, Money::parse("137.95").unwrap()); // 123.45 + 30 - 15.50
    }

    #[test]
    fn test_money_display() {
        assert_eq!(format!("{}", Money::from_centavos(12345)), "123.45");
        assert_eq!(format!("{}", Money::from_centavos(750)), "7.50");
        assert_eq!(format!("{}", Money::from_reais(100)), "100.00");
        assert_eq!(format!("{}", Money::from_centavos(-2050)), "-20.50");
        assert_eq!(format!("{}", Money::from_centavos(-750)), "-7.50");
    }

    #[test]
    fn test_estrutura_retorno() {
        let resultado = calcular_valores(
            Money::from_reais(100),
            Money::from_reais(5),
            2,
            Money::from_reais(10),
        );

        // Verifica se os valores de entrada são preservados
        assert_eq!(resultado.taxa_fixa, Money::from_reais(100));
        assert_eq!(resultado.taxa_transporte, Money::from_reais(5));
        assert_eq!(resultado.dias_trabalhados, 2);
        assert_eq!(resultado.deducoes_total, Money::from_reais(10));
    }
}
