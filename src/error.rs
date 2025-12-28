use std::io;
use thiserror::Error;

/// TypeCmd error types
#[derive(Error, Debug)]
pub enum TypeCmdError {
    #[error("I/O错误: {0}")]
    Io(#[from] io::Error),
    
    #[error("解析错误: {0}")]
    Parse(String),
    
    #[error("未找到命令: {0}")]
    CommandNotFound(String),
    
    #[error("参数不足: {0}")]
    InsufficientArgs(String),
    
    #[error("变量未定义: {0}")]
    UndefinedVariable(String),
    
    #[error("无效的历史命令: {0}")]
    InvalidHistoryCommand(String),
    
    #[error("其他错误: {0}")]
    Other(String),
}

/// Result type alias for TypeCmd operations
pub type Result<T> = std::result::Result<T, TypeCmdError>;