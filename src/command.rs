/// Command enum representing all possible commands
#[derive(Debug)]
pub enum Command {
    Show(ShowSubcommand),
    Exit(Option<i32>),
    Set(String, String),
    Get(String),
    Copy(String, String),
    ISet(String, i64),
    StringCmd(String),
    IntCmd(Option<i32>),
    List,
    Delete(String),
    Clear(ClearTarget),
    History(HistorySubcommand),
    LastCommand,
    HistoryCommand(String),
}

/// Show command subcommands
#[derive(Debug)]
pub enum ShowSubcommand {
    Help,
    Version,
    Variables,
    History(Option<usize>),
    License, 
}

/// Clear command targets
#[derive(Debug)]
pub enum ClearTarget {
    Variables,
    History,
}

/// History command subcommands
#[derive(Debug)]
pub enum HistorySubcommand {
    List(Option<usize>),
    Search(String),
    Clear,
}