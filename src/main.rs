//! # i - A Minimalist, High-Performance Programming Language
//! 
//! i is a Zen-inspired programming language that achieves massive functionality
//! within a 56MB footprint through aggressive optimization and zero-cost abstractions.

mod macros;
mod stdlib;

use clap::Parser;
use std::process;

#[derive(Parser)]
#[command(name = "i")]
#[command(about = "A minimalist, high-performance programming language")]
struct Args {
    /// The i source file to execute
    file: String,
    
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Number(f64),
    Text(String),
    Boolean(bool),
    List(Vec<Value>),
    Map(hashbrown::HashMap<String, Value>),
    Function(fn(&[Value]) -> Result<Value, String>),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::Text(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Map(m) => !m.is_empty(),
            Value::Function(_) => true, // Functions are always truthy
        }
    }
}

#[derive(Debug)]
pub struct Interpreter {
    globals: hashbrown::HashMap<String, Value>,
    stack: Vec<Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = hashbrown::HashMap::new();
        
        // Initialize with core Zen-inspired vocabulary
        globals.insert("zen".to_string(), Value::Number(0.0));
        globals.insert("flow".to_string(), Value::Boolean(true));
        globals.insert("void".to_string(), Value::Nil);
        
        // Register all standard library functions
        let stdlib_functions = stdlib::register_stdlib_functions();
        for (name, func) in stdlib_functions {
            globals.insert(name, Value::Function(func));
        }
        
        Self {
            globals,
            stack: Vec::new(),
        }
    }
    
    pub fn execute(&mut self, source: &str) -> Result<Value, String> {
        let tokens = self.tokenize(source)?;
        let ast = self.parse(&tokens)?;
        self.eval(&ast)
    }
    
    fn tokenize(&self, source: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = source.chars().peekable();
        
        while let Some(&ch) = chars.peek() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => {
                    chars.next();
                }
                '(' => {
                    tokens.push(Token::LeftParen);
                    chars.next();
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    chars.next();
                }
                '"' => {
                    chars.next();
                    let mut text = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch == '"' {
                            chars.next();
                            break;
                        }
                        text.push(ch);
                        chars.next();
                    }
                    tokens.push(Token::Text(text));
                }
                _ if ch.is_ascii_digit() || ch == '-' => {
                    let mut number = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_ascii_digit() || ch == '.' || ch == '-' {
                            number.push(ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    let num: f64 = number.parse().map_err(|_| format!("Invalid number: {}", number))?;
                    tokens.push(Token::Number(num));
                }
                _ => {
                    let mut ident = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_ascii_alphanumeric() || ch == '_' {
                            ident.push(ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Identifier(ident));
                }
            }
        }
        
        Ok(tokens)
    }
    
    fn parse(&self, tokens: &[Token]) -> Result<Expr, String> {
        if tokens.is_empty() {
            return Ok(Expr::Literal(Value::Nil));
        }
        
        // Simple recursive descent parser
        self.parse_expr(&tokens, 0).map(|(expr, _)| expr)
    }
    
    fn parse_expr(&self, tokens: &[Token], pos: usize) -> Result<(Expr, usize), String> {
        if pos >= tokens.len() {
            return Err("Unexpected end of input".to_string());
        }
        
        match &tokens[pos] {
            Token::Number(n) => Ok((Expr::Literal(Value::Number(*n)), pos + 1)),
            Token::Text(s) => Ok((Expr::Literal(Value::Text(s.clone())), pos + 1)),
            Token::Identifier(id) => {
                match id.as_str() {
                    "zen" => Ok((Expr::Literal(Value::Number(0.0)), pos + 1)),
                    "void" => Ok((Expr::Literal(Value::Nil), pos + 1)),
                    "flow" => Ok((Expr::Literal(Value::Boolean(true)), pos + 1)),
                    _ => Ok((Expr::Variable(id.clone()), pos + 1)),
                }
            }
            Token::LeftParen => {
                if pos + 1 >= tokens.len() {
                    return Err("Expected expression after '('".to_string());
                }
                
                if let Token::Identifier(func) = &tokens[pos + 1] {
                    let mut args = Vec::new();
                    let mut current_pos = pos + 2;
                    
                    while current_pos < tokens.len() && !matches!(tokens[current_pos], Token::RightParen) {
                        let (arg, new_pos) = self.parse_expr(tokens, current_pos)?;
                        args.push(arg);
                        current_pos = new_pos;
                    }
                    
                    if current_pos >= tokens.len() {
                        return Err("Expected ')'".to_string());
                    }
                    
                    Ok((Expr::Call(func.clone(), args), current_pos + 1))
                } else {
                    Err("Expected function name after '('".to_string())
                }
            }
            _ => Err(format!("Unexpected token: {:?}", tokens[pos])),
        }
    }
    
    fn eval(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Variable(name) => {
                self.globals.get(name)
                    .cloned()
                    .ok_or_else(|| format!("Undefined variable: {}", name))
            }
            Expr::Call(func, args) => {
                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(self.eval(arg)?);
                }
                
                // Handle built-in functions with Zen-inspired names
                match func.as_str() {
                    "echo" => {
                        if let Some(arg) = evaluated_args.first() {
                            println!("{}", self.format_value(arg));
                        }
                        Ok(Value::Nil)
                    }
                    "add" => {
                        let result = self.arithmetic_op(&evaluated_args, |a, b| a + b)?;
                        Ok(Value::Number(result))
                    }
                    "sub" => {
                        let result = self.arithmetic_op(&evaluated_args, |a, b| a - b)?;
                        Ok(Value::Number(result))
                    }
                    "mul" => {
                        let result = self.arithmetic_op(&evaluated_args, |a, b| a * b)?;
                        Ok(Value::Number(result))
                    }
                    "div" => {
                        let result = self.arithmetic_op(&evaluated_args, |a, b| a / b)?;
                        Ok(Value::Number(result))
                    }
                    "if" => {
                        if evaluated_args.len() != 3 {
                            return Err("if requires exactly 3 arguments: condition, then, else".to_string());
                        }
                        let condition = &evaluated_args[0];
                        let then_expr = &evaluated_args[1];
                        let else_expr = &evaluated_args[2];
                        
                        if condition.is_truthy() {
                            Ok(then_expr.clone())
                        } else {
                            Ok(else_expr.clone())
                        }
                    }
                    _ => Err(format!("Unknown function: {}", func)),
                }
            }
        }
    }
    
    fn arithmetic_op<F>(&self, args: &[Value], op: F) -> Result<f64, String>
    where
        F: Fn(f64, f64) -> f64,
    {
        if args.len() < 2 {
            return Err("Arithmetic operation requires at least 2 arguments".to_string());
        }
        
        let mut result = match args[0] {
            Value::Number(n) => n,
            _ => return Err("Arithmetic operation requires numbers".to_string()),
        };
        
        for arg in &args[1..] {
            match arg {
                Value::Number(n) => result = op(result, *n),
                _ => return Err("Arithmetic operation requires numbers".to_string()),
            }
        }
        
        Ok(result)
    }
    
    fn format_value(&self, value: &Value) -> String {
        match value {
            Value::Nil => "void".to_string(),
            Value::Number(n) => n.to_string(),
            Value::Text(s) => format!("\"{}\"", s),
            Value::Boolean(b) => b.to_string(),
            Value::List(l) => {
                let items: Vec<String> = l.iter().map(|v| self.format_value(v)).collect();
                format!("[{}]", items.join(" "))
            }
            Value::Map(m) => {
                let items: Vec<String> = m.iter()
                    .map(|(k, v)| format!("{}: {}", k, self.format_value(v)))
                    .collect();
                format!("{{{}}}", items.join(" "))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),
    Text(String),
    Identifier(String),
    LeftParen,
    RightParen,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Variable(String),
    Call(String, Vec<Expr>),
}

fn main() {
    let args = Args::parse();
    
    // Validate file extension
    let file_extension = std::path::Path::new(&args.file)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    if !matches!(file_extension, "i" | "rag") {
        eprintln!("Error: i language only supports .i and .rag file extensions");
        eprintln!("Found extension: .{}", file_extension);
        process::exit(1);
    }
    
    if args.verbose {
        eprintln!("i interpreter v0.1.0 - Zen-inspired minimalism");
        eprintln!("Executing file: {} (.{} extension)", args.file, file_extension);
    }
    
    let mut interpreter = Interpreter::new();
    
    // Read the source file
    let source = match std::fs::read_to_string(&args.file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", args.file, err);
            process::exit(1);
        }
    };
    
    // Execute the program
    match interpreter.execute(&source) {
        Ok(result) => {
            if args.verbose {
                eprintln!("Program completed with result: {}", interpreter.format_value(&result));
            }
        }
        Err(err) => {
            eprintln!("Runtime error: {}", err);
            process::exit(1);
        }
    }
}
