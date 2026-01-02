use crate::colors::print_warn;
use crate::error::{TypeCmdError, Result};
use crate::command::{Command, ShowSubcommand, ClearTarget, HistorySubcommand};

/// Parse a command string into tokens
pub fn parse_command(input: &str) -> Result<Vec<String>> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_quotes = false;
    let mut in_single_quotes = false;
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(next_c) = chars.next() {
                    current_token.push(next_c);
                }
            }
            '"' if !in_single_quotes => {
                in_quotes = !in_quotes;
            }
            '\'' if !in_quotes => {
                in_single_quotes = !in_single_quotes;
            }
            ' ' if !in_quotes && !in_single_quotes => {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            _ => {
                current_token.push(c);
            }
        }
    }
    
    if !current_token.is_empty() {
        tokens.push(current_token);
    }
    
    if in_quotes || in_single_quotes {
        return Err(TypeCmdError::Parse("未闭合的引号".to_string()));
    }
    
    Ok(tokens)
}

/// Parse tokens into a Command enum
pub fn parse_to_command(tokens: Vec<String>) -> Result<Command> {
    if tokens.is_empty() {
        return Err(TypeCmdError::Parse("空命令".to_string()));
    }
    
    let cmd = tokens[0].to_lowercase();
    let args = &tokens[1..];
    
    match cmd.as_str() {
        "show" => parse_show_command(args),
        "exit" | "quit" | "q" => parse_exit_command(args),
        "to" | "var" | "let" | "set" => parse_set_command(args),
        "ito" | "ivar" | "ilet" | "iset" => parse_iset_command(args),
        "get" | "which" | "echo" => parse_get_command(args),
        "iget" | "iwhich" | "iecho" => parse_iget_command(args),
        "add" | "iadd" => parse_iadd_command(args),
        "string" | "str" | "sprint" => parse_string_command(args),
        "int" | "num" => parse_int_command(args),
        "ls" | "list" => Ok(Command::List),
        "rm" | "del" | "unset" => parse_delete_command(args),
        "clear" | "cls" => parse_clear_command(args),
        "history" | "hist" => parse_history_command(args),
        "copy" | "cpvar" => parse_copy_command(args),
        "ver" | "version" => {
            let vstr = "ver";
            let ccc = vec![vstr.to_string()];
            let args = &ccc[0..];
            parse_show_command(args)
        }, 
        "!!" => Ok(Command::LastCommand),
        "!" => {
            if args.is_empty() {
                Err(TypeCmdError::InsufficientArgs("历史命令需要参数".to_string()))
            } else {
                Ok(Command::HistoryCommand(args[0].clone()))
            }
        }
        _ => Err(TypeCmdError::CommandNotFound(cmd)),
    }
}

fn parse_copy_command(args: &[String]) -> Result<Command> {
    if args.len() < 2 {
        return Err(TypeCmdError::InsufficientArgs(
            "copy命令需要至少2个参数".to_string(),
        ));
    } else if args.len() > 2 {
        print_warn("copy命令参数过多, 忽略剩余参数");
    }
    Ok(Command::Copy(args[0].clone(), args[1].clone()))
}
fn parse_show_command(args: &[String]) -> Result<Command> {
    if args.is_empty() {
        return Ok(Command::Show(ShowSubcommand::Help));
    }
    
    match args[0].to_lowercase().as_str() {
        "help" => Ok(Command::Show(ShowSubcommand::Help)),
        "ver" | "version" => Ok(Command::Show(ShowSubcommand::Version)),
        "vars" => Ok(Command::Show(ShowSubcommand::Variables)),
        "history" => {
            if args.len() > 1 {
                if let Ok(limit) = args[1].parse::<usize>() {
                    Ok(Command::Show(ShowSubcommand::History(Some(limit))))
                } else {
                    Err(TypeCmdError::Parse("无效的历史记录限制".to_string()))
                }
            } else {
                Ok(Command::Show(ShowSubcommand::History(None)))
            }
        }
        "lic" | "license" => Ok(Command::Show(ShowSubcommand::License)),
        _ => Err(TypeCmdError::Parse(format!("未知的show子命令: {}", args[0]))),
    }
}

fn parse_exit_command(args: &[String]) -> Result<Command> {
    if args.is_empty() {
        Ok(Command::Exit(None))
    } else {
        match args[0].parse::<i32>() {
            Ok(code) => Ok(Command::Exit(Some(code))),
            Err(_) => Err(TypeCmdError::Parse("无效的退出码".to_string())),
        }
    }
}

fn parse_set_command(args: &[String]) -> Result<Command> {
    if args.len() < 2 {
        return Err(TypeCmdError::InsufficientArgs(
            "set命令需要至少2个参数".to_string(),
        ));
    }
    
    let var_name = args[0].clone();
    let value = args[1..].join(" ");
    Ok(Command::Set(var_name, value))
}

fn parse_iset_command(args: &[String]) -> Result<Command> {
    if args.len() < 2 {
        return Err(TypeCmdError::InsufficientArgs(
            "iset命令需要至少2个参数".to_string(),
        ));
    }
    let varname = args[0].clone();
    let value = args[1].clone();
    match value.parse::<i64>(){
        Ok(num) => Ok(Command::ISet(varname, num)),
        Err(_) => Err(TypeCmdError::Parse("无效的数字".to_string()))
    }
}

fn parse_get_command(args: &[String]) -> Result<Command> {
    if args.is_empty() {
        return Err(TypeCmdError::InsufficientArgs(
            "get命令需要变量名".to_string(),
        ));
    }
    
    Ok(Command::Get(args[0].clone()))
}

fn parse_iget_command(args: &[String]) -> Result<Command> {
    if args.is_empty() {
        return Err(TypeCmdError::InsufficientArgs(
            "iget命令需要变量名".to_string(),
        ))
    }

    Ok(Command::IGet(args[0].clone()))
}

fn parse_iadd_command(args: &[String]) -> Result<Command> {
    if args.len() < 2 {
        return Err(TypeCmdError::InsufficientArgs(
            "iadd命令参数不足".to_string(),
        ))
    }
    let var = args[0].clone();
    let val2 = args[1].clone();
    let val = match val2.parse::<i64>(){
        Ok(val) => val, 
        Err(_) => {
            return Err(TypeCmdError::Parse("无效数字".to_string()))
        }
    };
    Ok(Command::IAdd(var, val))
}

fn parse_string_command(args: &[String]) -> Result<Command> {
    let text = if args.is_empty() {
        String::new()
    } else {
        args.join(" ")
    };
    
    Ok(Command::StringCmd(text))
}

fn parse_int_command(args: &[String]) -> Result<Command> {
    if args.is_empty() {
        Ok(Command::IntCmd(None))
    } else {
        match args[0].parse::<i32>() {
            Ok(num) => Ok(Command::IntCmd(Some(num))),
            Err(_) => Err(TypeCmdError::Parse("无效的数字".to_string())),
        }
    }
}

fn parse_delete_command(args: &[String]) -> Result<Command> {
    if args.is_empty() {
        return Err(TypeCmdError::InsufficientArgs(
            "delete命令需要变量名".to_string(),
        ));
    }
    
    Ok(Command::Delete(args[0].clone()))
}

fn parse_clear_command(args: &[String]) -> Result<Command> {
    if args.is_empty() || args[0].to_lowercase() == "vars" {
        Ok(Command::Clear(ClearTarget::Variables))
    } else if args[0].to_lowercase() == "history" {
        Ok(Command::Clear(ClearTarget::History))
    } else {
        Err(TypeCmdError::Parse("clear命令参数应为: vars 或 history".to_string()))
    }
}

fn parse_history_command(args: &[String]) -> Result<Command> {
    if args.is_empty() {
        Ok(Command::History(HistorySubcommand::List(None)))
    } else {
        match args[0].to_lowercase().as_str() {
            "clear" => Ok(Command::History(HistorySubcommand::Clear)),
            "search" => {
                if args.len() < 2 {
                    Err(TypeCmdError::InsufficientArgs("搜索需要关键词".to_string()))
                } else {
                    Ok(Command::History(HistorySubcommand::Search(args[1].clone())))
                }
            }
            _ => {
                if let Ok(limit) = args[0].parse::<usize>() {
                    Ok(Command::History(HistorySubcommand::List(Some(limit))))
                } else {
                    Err(TypeCmdError::Parse("无效的历史命令参数".to_string()))
                }
            }
        }
    }
}