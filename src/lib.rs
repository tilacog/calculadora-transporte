use chrono::{Datelike, NaiveDate, Weekday};
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

pub fn contar_dias_uteis(mes: u32, ano: i32) -> Result<i32, String> {
    if mes < 1 || mes > 12 {
        return Err("Mês deve estar entre 1 e 12".to_string());
    }

    if ano < 1900 || ano > 2100 {
        return Err("Ano deve estar entre 1900 e 2100".to_string());
    }

    let primeiro_dia = match NaiveDate::from_ymd_opt(ano, mes, 1) {
        Some(date) => date,
        None => return Err("Data inválida".to_string()),
    };

    let proximo_mes = if mes == 12 { 1 } else { mes + 1 };
    let proximo_ano = if mes == 12 { ano + 1 } else { ano };

    let ultimo_dia = match NaiveDate::from_ymd_opt(proximo_ano, proximo_mes, 1) {
        Some(date) => date.pred_opt().unwrap(),
        None => return Err("Data inválida".to_string()),
    };

    let mut dias_uteis = 0;
    let mut data_atual = primeiro_dia;

    while data_atual <= ultimo_dia {
        let dia_semana = data_atual.weekday();
        if dia_semana != Weekday::Sat && dia_semana != Weekday::Sun {
            dias_uteis += 1;
        }
        data_atual = data_atual.succ_opt().unwrap();
    }

    Ok(dias_uteis)
}

pub fn obter_nome_mes(mes: u32) -> &'static str {
    match mes {
        1 => "Janeiro",
        2 => "Fevereiro",
        3 => "Março",
        4 => "Abril",
        5 => "Maio",
        6 => "Junho",
        7 => "Julho",
        8 => "Agosto",
        9 => "Setembro",
        10 => "Outubro",
        11 => "Novembro",
        12 => "Dezembro",
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone)]
pub struct InformacaoCalendario {
    pub mes: u32,
    pub ano: i32,
    pub nome_mes: &'static str,
    pub dias_uteis_mes: i32,
    pub feriados_deduzidos: i32,
    pub dias_trabalhados: i32,
}

#[derive(Debug, Clone)]
pub struct ResultadoCalculo {
    pub taxa_fixa: Money,
    pub taxa_transporte: Money,
    pub dias_trabalhados: i32,
    pub custo_transporte: Money,
    pub deducoes_total: Money,
    pub pagamento_final: Money,
    pub calendario: Option<InformacaoCalendario>,
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
        calendario: None,
    }
}

pub fn calcular_valores_com_calendario(
    taxa_fixa: Money,
    taxa_transporte: Money,
    mes: u32,
    ano: i32,
    feriados_deduzidos: i32,
    deducoes_total: Money,
) -> Result<ResultadoCalculo, String> {
    let dias_uteis_mes = contar_dias_uteis(mes, ano)?;
    let dias_trabalhados = (dias_uteis_mes - feriados_deduzidos).max(0);

    let custo_transporte = taxa_transporte * dias_trabalhados * 2;
    let pagamento_final = taxa_fixa + custo_transporte - deducoes_total;

    let calendario = InformacaoCalendario {
        mes,
        ano,
        nome_mes: obter_nome_mes(mes),
        dias_uteis_mes,
        feriados_deduzidos,
        dias_trabalhados,
    };

    Ok(ResultadoCalculo {
        taxa_fixa,
        taxa_transporte,
        dias_trabalhados,
        custo_transporte,
        deducoes_total,
        pagamento_final,
        calendario: Some(calendario),
    })
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

pub fn obter_mes() -> u32 {
    loop {
        println!("Digite o mês (1-12):");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler entrada");

        match input.trim().parse::<u32>() {
            Ok(mes) if mes >= 1 && mes <= 12 => return mes,
            Ok(_) => println!("Erro: Mês deve estar entre 1 e 12."),
            Err(_) => println!("Erro: Por favor, digite um número válido."),
        }
    }
}

pub fn obter_ano() -> i32 {
    loop {
        println!("Digite o ano (ex: 2024):");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler entrada");

        match input.trim().parse::<i32>() {
            Ok(ano) if ano >= 1900 && ano <= 2100 => return ano,
            Ok(_) => println!("Erro: Ano deve estar entre 1900 e 2100."),
            Err(_) => println!("Erro: Por favor, digite um ano válido."),
        }
    }
}

pub fn obter_feriados() -> i32 {
    loop {
        println!("Digite o número de feriados/dias não trabalhados no mês:");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler entrada");

        match input.trim().parse::<i32>() {
            Ok(feriados) if feriados >= 0 => return feriados,
            Ok(_) => println!("Erro: Número de feriados não pode ser negativo."),
            Err(_) => println!("Erro: Por favor, digite um número válido."),
        }
    }
}

pub fn calcular_pagamento() {
    println!("=== CALCULADORA DE PAGAMENTO ===\n");

    // Coleta de dados
    let taxa_fixa = obter_valor_numerico("Digite a taxa fixa (R$):");
    let taxa_transporte = obter_valor_numerico("Digite a taxa de transporte por viagem (R$):");

    // Nova funcionalidade: cálculo baseado em calendário
    let mes = obter_mes();
    let ano = obter_ano();
    let feriados = obter_feriados();
    let deducoes_total = obter_valor_numerico("Digite o valor das deduções (R$):");

    // Cálculo com calendário
    let resultado = match calcular_valores_com_calendario(
        taxa_fixa,
        taxa_transporte,
        mes,
        ano,
        feriados,
        deducoes_total,
    ) {
        Ok(resultado) => resultado,
        Err(erro) => {
            println!("Erro no cálculo: {}", erro);
            return;
        }
    };

    // Exibição dos resultados
    println!("\n{}", "=".repeat(40));
    println!("RESUMO DO PAGAMENTO");
    println!("{}", "=".repeat(40));
    println!("Taxa fixa: R$ {}", resultado.taxa_fixa);

    // Informações do calendário
    if let Some(calendario) = &resultado.calendario {
        println!("Mês/Ano: {} {}", calendario.nome_mes, calendario.ano);
        println!("Dias úteis no mês: {}", calendario.dias_uteis_mes);
        if calendario.feriados_deduzidos > 0 {
            println!(
                "Feriados/dias não trabalhados: {}",
                calendario.feriados_deduzidos
            );
        }
        println!("Dias trabalhados: {}", calendario.dias_trabalhados);
    }

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

    // Testes para funcionalidade de calendário

    #[test]
    fn test_contar_dias_uteis_novembro_2024() {
        // Novembro 2024: tem 21 dias úteis (1-30, excluindo sábados e domingos)
        let dias_uteis = contar_dias_uteis(11, 2024).unwrap();
        assert_eq!(dias_uteis, 21);
    }

    #[test]
    fn test_contar_dias_uteis_fevereiro_2024() {
        // Fevereiro 2024 (ano bissexto): tem 21 dias úteis
        let dias_uteis = contar_dias_uteis(2, 2024).unwrap();
        assert_eq!(dias_uteis, 21);
    }

    #[test]
    fn test_contar_dias_uteis_mes_invalido() {
        assert!(contar_dias_uteis(0, 2024).is_err());
        assert!(contar_dias_uteis(13, 2024).is_err());
    }

    #[test]
    fn test_contar_dias_uteis_ano_invalido() {
        assert!(contar_dias_uteis(1, 1899).is_err());
        assert!(contar_dias_uteis(1, 2101).is_err());
    }

    #[test]
    fn test_obter_nome_mes() {
        assert_eq!(obter_nome_mes(1), "Janeiro");
        assert_eq!(obter_nome_mes(2), "Fevereiro");
        assert_eq!(obter_nome_mes(11), "Novembro");
        assert_eq!(obter_nome_mes(12), "Dezembro");
    }

    #[test]
    #[should_panic]
    fn test_obter_nome_mes_invalido() {
        obter_nome_mes(13);
    }

    #[test]
    fn test_calcular_valores_com_calendario() {
        let resultado = calcular_valores_com_calendario(
            Money::from_reais(150),
            Money::parse("7.50").unwrap(),
            11, // Novembro
            2024,
            2, // 2 feriados
            Money::from_reais(25),
        )
        .unwrap();

        assert_eq!(resultado.taxa_fixa, Money::from_reais(150));
        assert_eq!(resultado.taxa_transporte, Money::parse("7.50").unwrap());
        assert_eq!(resultado.dias_trabalhados, 19); // 21 dias úteis - 2 feriados

        // Custo transporte: 7.50 * 19 * 2 = 285.00
        assert_eq!(resultado.custo_transporte, Money::from_centavos(28500));

        // Pagamento final: 150 + 285 - 25 = 410
        assert_eq!(resultado.pagamento_final, Money::from_reais(410));

        let calendario = resultado.calendario.unwrap();
        assert_eq!(calendario.mes, 11);
        assert_eq!(calendario.ano, 2024);
        assert_eq!(calendario.nome_mes, "Novembro");
        assert_eq!(calendario.dias_uteis_mes, 21);
        assert_eq!(calendario.feriados_deduzidos, 2);
        assert_eq!(calendario.dias_trabalhados, 19);
    }

    #[test]
    fn test_calcular_valores_com_feriados_excessivos() {
        let resultado = calcular_valores_com_calendario(
            Money::from_reais(100),
            Money::from_reais(10),
            11, // Novembro
            2024,
            25, // Mais feriados que dias úteis
            Money::ZERO,
        )
        .unwrap();

        // Dias trabalhados deve ser 0 (não pode ser negativo)
        assert_eq!(resultado.dias_trabalhados, 0);
        assert_eq!(resultado.custo_transporte, Money::ZERO);
        assert_eq!(resultado.pagamento_final, Money::from_reais(100));
    }

    #[test]
    fn test_calcular_valores_com_calendario_sem_feriados() {
        let resultado = calcular_valores_com_calendario(
            Money::from_reais(200),
            Money::from_reais(5),
            1, // Janeiro
            2024,
            0, // Sem feriados
            Money::ZERO,
        )
        .unwrap();

        let calendario = resultado.calendario.unwrap();
        assert_eq!(calendario.feriados_deduzidos, 0);
        assert_eq!(calendario.dias_trabalhados, calendario.dias_uteis_mes);
    }
}
