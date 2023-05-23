use std::io::Error;
use std::{collections::VecDeque, fs, str::FromStr};

pub mod instruction;
pub mod interpreter;
pub mod parser;

use instruction::Instruction;
use interpreter::{run_interpreter, InterpreterError};
use parser::{parse, ParserError};

pub enum FrontendError {
    UnrecognizedArgument(String),
    MissingArgumentParameter,
    InvalidBackend(String),
    InvalidInputFile(String, Error),
    NotImplemented,
}

enum Backend {
    Interpreter,
    Python,
    X86_64,
}

impl FromStr for Backend {
    type Err = FrontendError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Interpreter" => Ok(Backend::Interpreter),
            "Python" => Ok(Backend::Python),
            "X86_64" => Ok(Backend::X86_64),
            _ => Err(FrontendError::InvalidBackend(String::from(input))),
        }
    }
}
struct SimpleParserConfig {
    input: String,
    selected_backend: Backend,
}

fn handle_argument(
    config: &mut SimpleParserConfig,
    arguments: &mut VecDeque<String>,
    current_argument: String,
) -> Result<(), FrontendError> {
    // FIXME: Il messaggio di help dovrebbe impedire l'esecuzione normale
    if current_argument == "--help" || current_argument == "-h" {
        show_help_menu();
        Ok(())
    } else if current_argument == "--source" || current_argument == "-s" {
        // Il prossimo parametro è il nome del file di input
        let next_parameter = arguments.pop_front();

        match next_parameter {
            Some(file_name) => {
                let file_read_result = fs::read_to_string(&file_name);

                match file_read_result {
                    Ok(file_contents) => {
                        config.input = file_contents;
                        Ok(())
                    }
                    Err(error) => Err(FrontendError::InvalidInputFile(file_name, error)),
                }
            }
            None => Err(FrontendError::MissingArgumentParameter),
        }
    } else if current_argument == "--output" || current_argument == "-o" {
        // TODO: Implementare file di output
        Err(FrontendError::NotImplemented)

        // Il prossimo parametro è il nome del file di output
    } else if current_argument == "--backend" || current_argument == "-b" {
        // Il prossimo parametro indica il formato da generare
        let next_parameter = arguments.pop_front();

        let result = match next_parameter {
            Some(selected_backend) => Backend::from_str(selected_backend.as_str()),
            None => Err(FrontendError::MissingArgumentParameter),
        };

        match result {
            Ok(backend) => {
                config.selected_backend = backend;
                Ok(())
            }
            Err(error) => Err(error),
        }
    } else {
        Err(FrontendError::UnrecognizedArgument(current_argument))
    }
}

fn parse_arguments(mut arguments: VecDeque<String>) -> Result<SimpleParserConfig, FrontendError> {
    // Salta il primo parametro, non ci interessa il percorso usato per chiamare l'eseguibile
    arguments.pop_front();

    let mut config = SimpleParserConfig {
        input: String::new(),
        selected_backend: Backend::Interpreter,
    };

    loop {
        let next_argument = arguments.pop_front();

        match next_argument {
            Some(next_argument) => {
                if let Err(argument_error) =
                    handle_argument(&mut config, &mut arguments, next_argument)
                {
                    return Err(argument_error);
                }
            }
            None => return Ok(config),
        }
    }
}

fn run(config: &SimpleParserConfig, instructions: Vec<Instruction>) {
    match config.selected_backend {
        Backend::Interpreter => {
            let result = run_interpreter(&instructions);

            if let Err(error) = result {
                print_interpreter_error(error);
            }
        }
        Backend::Python => {
            // TODO: Implementa backend Python
            print_frontend_error(FrontendError::NotImplemented)
        }
        Backend::X86_64 => {
            // TODO: Implementa backend X86_64
            print_frontend_error(FrontendError::NotImplemented)
        }
    }
}

fn parse_and_run(config: &SimpleParserConfig) {
    let parsing_result = parse(&config.input);

    match parsing_result {
        Ok(instructions) => run(config, instructions),
        Err(parsing_error) => print_parser_error(parsing_error),
    }
}

fn main() {
    let arguments: VecDeque<String> = std::env::args().collect();

    let argument_parsing_result = parse_arguments(arguments);

    match argument_parsing_result {
        Ok(config) => parse_and_run(&config),
        Err(error) => print_frontend_error(error),
    }
}

fn show_help_menu() {
    // TODO: Generare il menù di help automaticamente
    println!("SimpleParser");

    println!("Argomenti:");

    println!("--help / -h : Stampa questo menù di aiuto");

    println!("--source <file> / -s <file> : Il file di input");
    println!("--output <file> / -o <file> : Il file di output");

    println!("--backend <backend> / -b <backend> : Il backend da utilizzare");
    println!("\tLista dei backend:");
    println!("\t\tInterpreter : Esegui il programma nell'interprete");
    println!("\t\tPython : Genera uno script Python equivalente al programma in input");
    println!("\t\tX86_64 : Compila il programma in assembly x86_64");
}

fn print_parser_error(error: ParserError) {
    // TODO: Aggiungere un modo per capire dove, nel file sorgente, si è verificato l'errore di sintassi

    match error {
        ParserError::InvalidInstruction(token) => {
            eprintln!("Token non valido: {token}")
        }
        ParserError::MissingParameter => {
            eprintln!("Manca un parametro")
        }
    }
}

fn print_interpreter_error(error: InterpreterError) {
    // TODO: Aggiungere un modo per capire dove, nel programma in esecuzione, si è verificato l'errore
    match error {
        InterpreterError::RedefinedVariable(variable_name) => {
            eprintln!("La variabile {variable_name} è già definita!")
        }
        InterpreterError::VariableDoesNotExist(variable_name) => {
            eprintln!("La variabile {variable_name} non esiste!")
        }
    }
}

fn print_frontend_error(error: FrontendError) {
    // TODO: Aggiungere un modo per capire dove, nella lista dei parametri, si è verificato l'errore
    match error {
        FrontendError::InvalidBackend(backend) => {
            eprintln!("Backend non valida: {backend}")
        }
        FrontendError::MissingArgumentParameter => {
            eprintln!("Inserito un argomento senza il relativo parametro!")
        }
        FrontendError::UnrecognizedArgument(argument) => {
            eprintln!("Argomento non riconosciuto: {argument}")
        }
        FrontendError::InvalidInputFile(file, reason) => {
            eprintln!("File {file} non valido! Errore: {reason}")
        }
        FrontendError::NotImplemented => {
            eprintln!("La funzionalità richiesta non è ancora stata implementata!")
        }
    }
}
