# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Build and Run
- `cargo run` - Run the payment calculator CLI
- `cargo build` - Build the project
- `cargo build --release` - Build optimized release version
- `cargo test` - Run all tests

### Testing
- `cargo test` - Run complete test suite
- `cargo test test_money_parsing` - Run specific test for money parsing
- `cargo test test_calculo_basico` - Run basic calculation tests

## Architecture Overview

This is a Rust-based payment calculator (`calculadora_pagamento`) that implements fixed-precision monetary arithmetic.

### Core Structure
- **Binary (`src/main.rs`)**: Simple entry point that calls the main calculation function
- **Library (`src/lib.rs`)**: Contains all core logic including the `Money` type and calculation functions

### Key Components

#### Money Type
- Custom `Money` struct wrapping `i64` to store centavos (cents)  
- Avoids floating-point arithmetic errors for monetary calculations
- Implements parsing from decimal strings without using floats
- Supports arithmetic operations (+, -, multiplication by i32)
- Always displays with 2 decimal places

#### Core Functions
- `calcular_valores()` - Basic calculation logic for payment computation
- `calcular_valores_com_calendario()` - Calendar-based calculation with automatic workday counting
- `calcular_pagamento()` - CLI interface function (now uses calendar-based input)
- `contar_dias_uteis()` - Counts weekdays (Mon-Fri) in a given month/year
- `obter_valor_numerico()` / `obter_inteiro()` - Input helpers
- `obter_mes()` / `obter_ano()` / `obter_feriados()` - Calendar-specific input helpers
- `obter_deducoes()` - Iterative deduction collection with descriptions

### Payment Formula
```
Transportation Cost = Transport Rate × Working Days × 2
Final Payment = Fixed Rate + Transportation Cost - Deductions
```

### Calendar Feature
The application now automatically calculates working days based on month/year input:
- Counts weekdays (Monday-Friday) in the specified month
- Excludes weekends (Saturday-Sunday) automatically
- Allows user to specify additional holidays/non-work days to deduct
- Supports years from 1900 to 2100
- Handles leap years correctly

#### Calendar Input Flow
1. Month (1-12)
2. Year (1900-2100) 
3. Number of holidays/non-work days to deduct
4. Other standard inputs (rates, deductions)

#### Data Structures
- `InformacaoCalendario` - Stores calendar calculation details
- `ResultadoCalculo.calendario` - Optional calendar information in results
- `Deducao` - Stores individual deduction with value and description
- `ResultadoCalculo.deducoes` - Vector of itemized deductions

### Supported Money Formats
- `100` → R$ 100.00
- `123.45` → R$ 123.45
- `7.5` → R$ 7.50
- `-20.50` → R$ -20.50
- Empty/whitespace → R$ 0.00

### Deduction Feature
Deductions are collected iteratively with descriptions:
- User enters deduction value (0 or empty to finish)
- If value != 0, user provides a description
- Process repeats until user enters 0 or blank
- Report shows itemized deductions with descriptions and total

### Test Coverage
Comprehensive tests cover:
- Money parsing and display formatting
- Basic calculations with various scenarios
- Edge cases (zero values, negative results)
- Calendar functionality (weekday counting, month/year validation)
- Calendar-based calculations with holidays
- Multiple deductions with descriptions
- Error handling for invalid dates