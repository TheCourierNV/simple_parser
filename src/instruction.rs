// La lista delle istruzioni nel linguaggio
pub enum Instruction {
    Print(String),
    PrintVariable(String),
    CreateVariable(String, String),
}
