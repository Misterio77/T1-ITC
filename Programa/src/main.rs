use scc050_t1::Automaton;
use scc050_t1::Result;

use std::io::{stdin, BufRead};

fn main() -> Result<()> {
    // Inicializar stdin
    let stdin = stdin();
    // Travar stdin e ler input
    let input = Input::from_reader(&mut stdin.lock())?;

    // Criar automato com os dados passados
    let automaton = Automaton::new(
        &input.states,
        &input.symbols,
        &input.initial_states,
        &input.accepted_states,
        &input.transitions,
    )?;

    // Para cada cadeia
    for chain in input.chains {
        // Verificar se aceita ou rejeita
        if automaton.verify_chain(&chain) {
            println!("aceita")
        } else {
            println!("rejeita")
        }
    }

    Ok(())
}

/// Representa a entrada do programa
/// Inclui tudo nescessário para iniciar e alimentar o autômato
struct Input {
    states: Vec<u16>,
    symbols: Vec<char>,
    initial_states: Vec<u16>,
    accepted_states: Vec<u16>,
    transitions: Vec<(u16, char, u16)>,
    chains: Vec<Vec<char>>,
}
impl Input {
    /// Partindo de um BufRead (por exemplo, entrada padrão), lê e retorna a estrutura Input
    fn from_reader(reader: &mut (dyn BufRead)) -> Result<Input> {
        // Linhas do input
        let mut lines = reader.lines();

        // Quantidade de estados
        let state_qty: u16 = lines
            .next()
            .expect("Digite um número de estados válido")?
            .parse()?;

        // Símbolos
        let symbols: Vec<char> = lines
            .next()
            .expect("Digite um número de símbolos válido")?
            .split_whitespace()
            .skip(1)
            .map(|word| word.chars().next().expect("Digite um símbolo válido"))
            .collect();

        // Qtde de estados iniciais
        let initial_states_qty: u16 = lines
            .next()
            .expect("Digite um número de estados iniciais válido")?
            .parse()?;

        // Estados aceitáveis
        let accepted_states: Vec<u16> = lines
            .next()
            .expect("Digite um número de estados aceitáveis válido")?
            .split_whitespace()
            .skip(1)
            .map(|word| word.parse().expect("O estado deve ser numérico"))
            .collect();

        // Qtde de transições
        let transitions_qty: u16 = lines
            .next()
            .expect("Digite um número de transições válido")?
            .parse()?;

        // Transições
        let transitions: Vec<(u16, char, u16)> = lines
            .by_ref()
            .take(transitions_qty as usize)
            .map(|line| {
                let line = line?;
                let mut words = line.split_whitespace();
                let source = words
                    .next()
                    .expect("Digite o ponto inicial da transição")
                    .parse()?;
                let symbol = words
                    .next()
                    .expect("Digite o símbolo da transição")
                    .chars()
                    .next()
                    .expect("Digite um símbolo válido");
                let target = words
                    .next()
                    .expect("Digite o ponto final da transição")
                    .parse()?;
                Ok((source, symbol, target))
            })
            .collect::<Result<Vec<(u16, char, u16)>>>()?;

        // Qtde de cadeias
        let chains_qty: u16 = lines
            .next()
            .expect("Digite um número de cadeias válido")?
            .parse()?;

        // Cadeias
        let chains: Vec<Vec<char>> = lines
            .by_ref()
            .take(chains_qty as usize)
            .map(|line| Ok(line?.chars().collect::<Vec<char>>()))
            .collect::<Result<Vec<Vec<char>>>>()?;

        Ok(Input {
            states: (0..state_qty).collect(),
            symbols,
            accepted_states,
            initial_states: (0..initial_states_qty).collect(),
            transitions,
            chains,
        })
    }
}
