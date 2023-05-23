use std::collections::HashMap;

use crate::instruction::Instruction;

// Parte pubblica

pub enum InterpreterError {
    RedefinedVariable(String),
    VariableDoesNotExist(String),
}

pub fn run_interpreter(instructions: &Vec<Instruction>) -> Result<(), InterpreterError> {
    let mut context = InterpreterContext {
        variables: HashMap::new(),
    };

    for instruction in instructions {
        if let Err(error) = execute_instruction(&mut context, instruction) {
            // TODO: Aggiungere altre informazioni in modo che si possa capire quale istruzione è fallita
            return Err(error);
        }
    }
    // TODO: Maybe implementare un exit-code? O ritornare il context?
    Ok(())
}

// Parte privata

// La "memoria" o "stato" dell'interprete, usato per tenere traccia dello stato dell'esecuzione del programma in esecuzione
struct InterpreterContext {
    variables: HashMap<String, String>,
    // TODO: Tenere traccia dell'istruzione correntemente in esecuzione?
}

fn execute_instruction(
    context: &mut InterpreterContext,
    instruction: &Instruction,
) -> Result<(), InterpreterError> {
    match instruction {
        Instruction::Print(to_print) => {
            println!("{}", to_print);
            Ok(())
        }
        Instruction::CreateVariable(variable_name, variable_value) => {
            create_variable(context, variable_name, variable_value)
        }
        Instruction::PrintVariable(variable_name) => print_variable(context, variable_name),
    }
}

fn create_variable(
    context: &mut InterpreterContext,
    variable_name: &String,
    variable_value: &String,
) -> Result<(), InterpreterError> {
    // Per ora non è permesso creare due variabili con lo stesso nome
    if context.variables.contains_key(variable_name) {
        return Err(InterpreterError::RedefinedVariable(variable_name.clone()));
    }

    // Aggiungi la nuova variabile al "contesto" dell'interprete
    context
        .variables
        .insert(variable_name.clone(), variable_value.clone());

    Ok(())
}

fn print_variable(
    context: &mut InterpreterContext,
    variable_name: &String,
) -> Result<(), InterpreterError> {
    let variable_value = context.variables.get(variable_name);

    match variable_value {
        // La variabile esiste, stampa il valore
        Some(value) => {
            println!("{value}");
            Ok(())
        }
        // La variabile richiesta non esiste
        None => Err(InterpreterError::VariableDoesNotExist(
            variable_name.clone(),
        )),
    }
}
