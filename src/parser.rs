use std::collections::VecDeque;

use crate::instruction::Instruction;

// Parte pubblica

pub enum ParserError {
    InvalidInstruction(String),
    MissingParameter,
}

pub fn parse(input: &String) -> Result<Vec<Instruction>, ParserError> {
    parse_into_instructions(parse_into_tokens(input))
}

// Parte privata

fn parse_into_tokens(input: &String) -> VecDeque<&str> {
    // TODO: Da rivedere, non sono troppo convinto nel convertire tutto in &str
    let token_list: VecDeque<&str> = input.split_whitespace().collect();

    token_list
}

fn parse_into_instructions(
    mut token_list: VecDeque<&str>,
) -> Result<Vec<Instruction>, ParserError> {
    let mut output: Vec<Instruction> = Vec::new();

    loop {
        let current_token = match token_list.pop_front() {
            Some(token) => token,
            None => {
                // Interrompi il loop quando arriviamo all'ultima parola
                break;
            }
        };

        let parsing_result = parse_instruction(&mut token_list, current_token);

        // Se l'istruzione è stata letta correttamente, aggiungila alla lista
        // Altrimenti, ritorna immediatamente l'errore
        match parsing_result {
            Ok(parsed_instruction) => output.push(parsed_instruction),
            Err(parser_error) => {
                return Err(parser_error);
            }
        }
    }

    Ok(output)
}

fn parse_instruction(
    token_list: &mut VecDeque<&str>,
    token: &str,
) -> Result<Instruction, ParserError> {
    // TODO: Trovare un modo migliore per confrontare un enum con delle stringhe
    if token == "stampa" {
        parse_print_instruction(token_list)
    } else if token == "crea_variabile" {
        parse_create_variable_instruction(token_list)
    } else if token == "stampa_variabile" {
        parse_print_variable_instruction(token_list)
    } else {
        Err(ParserError::InvalidInstruction(String::from(token)))
    }
}

// TODO: Queste funzioni non sono molto DRY
fn parse_create_variable_instruction(
    token_list: &mut VecDeque<&str>,
) -> Result<Instruction, ParserError> {
    let variable_name = token_list.pop_front();
    let variable_value = token_list.pop_front();

    match variable_name {
        Some(variable_name) => match variable_value {
            Some(variable_value) => Ok(Instruction::CreateVariable(
                String::from(variable_name),
                String::from(variable_value),
            )),
            None => Err(ParserError::MissingParameter),
        },
        None => Err(ParserError::MissingParameter),
    }
}

fn parse_print_instruction(token_list: &mut VecDeque<&str>) -> Result<Instruction, ParserError> {
    match token_list.pop_front() {
        Some(to_print) => Ok(Instruction::Print(String::from(to_print))),
        None => Err(ParserError::MissingParameter),
    }
}

fn parse_print_variable_instruction(
    token_list: &mut VecDeque<&str>,
) -> Result<Instruction, ParserError> {
    match token_list.pop_front() {
        Some(variable_name) => Ok(Instruction::PrintVariable(String::from(variable_name))),
        None => Err(ParserError::MissingParameter),
    }
}
