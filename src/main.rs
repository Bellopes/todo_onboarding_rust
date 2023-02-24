use std::io::{Stdin, Stdout, Write};
fn main() {
    writeln!(std::io::stdout(), "👏 Seja bem vindo(a)!!! 👏").ok();
    let mut terminal = Terminal::new();
    if let Err(error) = terminal.show_message() {
        println!("{}", error.show_erro());
}
}
#[derive(Debug, Clone)]
struct Todo {
    message: String,
}
impl Todo {
    pub fn new(message: String) -> Todo {
        Todo { message }
    }
}
struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}
enum TerminalError {
    Stdout(std::io::Error),
    Stdin(std::io::Error),
}
impl TerminalError {
    fn show_erro(&self) -> String {
        match self {
            TerminalError::Stdin(error) => format!("Entrada inválida: {}", error),
            TerminalError::Stdout(error) => format!("Saída inválida: {}", error),
}}}

impl Terminal {
    fn new() -> Terminal {
        Terminal {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }
    
    fn ask_for_new_todo(&mut self) -> Result<Option<Todo>, TerminalError> {
        if !self.start()? {
            return Ok(None);
        }
        writeln!(self.stdout,"Qual o nome do arquivo TODO você deseja criar?").map_err(|error| TerminalError::Stdout(error))?;
        let name_todo = self.input()?;
        Ok(Some(Todo::new(name_todo)))

    }
    fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        writeln!(self.stdout, "TODO criado: {}", todo.message).map_err(|error| TerminalError::Stdout(error))
    }

    fn show_message (&mut self) -> Result<(), TerminalError> {
    loop {
        let messege = self.ask_for_new_todo();
        if let Ok(Some(messege)) = messege {
            self.show_todo(&messege);
        } else {
            break;
    }
    }
    Ok(())
    }
    fn start(&mut self) -> Result<bool, TerminalError> {
        loop {
            writeln!(self.stdout, "Você deseja criar um novo TODO? (s/n)").map_err(|error| TerminalError::Stdout(error))?;
            let input = self.input()?;
                match input.as_str() {
                    "s" => return Ok(true),
                    "n" => return Ok(false),
                    &_ => 
                    writeln!(self.stdout, "Entrada inválida. Se deseja criar um novo TODO, insira 's', senão, insira 'n'.").map_err(|error| TerminalError::Stdout(error))?,
            }
        }
}
    fn input(&mut self) -> Result<String, TerminalError> {
        let mut buf = String::new();
        self.stdin.read_line(&mut buf).map_err(|error| TerminalError::Stdin(error))?;
        return Ok(buf.trim().to_string());
    }
}


