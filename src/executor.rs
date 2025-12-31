use std::io::{self, Write};
use std::process::exit;
use crate::error::{TypeCmdError, Result};
use crate::colors::{print_error, print_success, print_info, print_warn, print_gray, bold, PURPLE, CYAN, GREEN, RESET};
use crate::history::HistoryManager;
use crate::variables::VariableStore;
use crate::parser::{parse_command, parse_to_command};
use crate::command::{Command, ShowSubcommand, ClearTarget, HistorySubcommand};
use crate::colors::BLUE;

/// Main TypeCmd application
pub struct TypeCmd {
    variables: VariableStore,
    history: HistoryManager,
    version: String,
}

impl TypeCmd {
    /// Create a new TypeCmd instance
    pub fn new() -> Result<Self> {
        let history = HistoryManager::new()?;
        
        Ok(TypeCmd {
            variables: VariableStore::new(),
            history,
            version: "0.4.0".to_string(),
        })
    }
    
    /// Execute a command string
    pub fn execute_command(&mut self, input: &str) -> Result<Option<String>> {
        let tokens = parse_command(input)?;
        let command = parse_to_command(tokens)?;
        
        match command {
            Command::Show(subcmd) => self.handle_show(subcmd),
            Command::Exit(code) => self.handle_exit(code),
            Command::Set(var, value) => self.handle_set(&var, &value),
            Command::Get(var) => self.handle_get(&var),
            Command::Copy(var, oldvar) => self.handle_copy(&var, &oldvar),
            Command::StringCmd(text) => self.handle_string(&text),
            Command::IntCmd(num) => self.handle_int(num),
            Command::List => self.handle_list(),
            Command::Delete(var) => self.handle_delete(&var),
            Command::Clear(target) => self.handle_clear(target),
            Command::History(subcmd) => self.handle_history(subcmd),
            Command::LastCommand => self.handle_last_command(),
            Command::HistoryCommand(spec) => self.handle_history_command(&spec),
        }
    }
    
    fn handle_show(&self, subcmd: ShowSubcommand) -> Result<Option<String>> {
        match subcmd {
            ShowSubcommand::Help => self.show_help(),
            ShowSubcommand::Version => self.show_version(),
            ShowSubcommand::Variables => self.show_variables(),
            ShowSubcommand::History(limit) => self.show_history(limit),
            ShowSubcommand::License => self.show_license(),
        }
    }

    fn show_license(&self) -> Result<Option<String>>{
        let shows = "MIT license";
        println!("LICENSE: {}", shows);
        Ok(Some(shows.to_string()))
    }
    
    fn show_help(&self) -> Result<Option<String>> {
        let help_text = format!(
            "{}{}TypeCmd 命令行模拟器{}\n\
            版本: {}\n\
            历史记录: {} 条命令\n\n{}\
            {}基础命令:\n\
              show                             - 显示信息: show [help|ver|vars|history|license]\n\
              exit    | quit  | q              - 退出程序\n\
              to      | var   | let   | set    - 设置变量: to <变量名> <值>\n\
              get     | which | echo           - 获取变量: get <变量名>\n\
              copy    | cpvar                  - 复制变量: copy <新变量名> <旧变量名>\n\
              string  | str                    - 字符串输出: string <文本>\n\
              int     | num                    - 数字处理: int <数字>\n\
              list    | ls                     - 列出所有变量\n\
              rm      | del   | unset          - 删除变量: rm <变量名>\n\
              clear   | cls                    - 清空所有变量或历史\n\
              history | hist                   - 显示历史命令\n\
              version | ver                    - 等同于show ver\n\
            历史命令使用:\n\
              !!                               - 执行上一条命令\n\
              ! n                              - 执行历史第n条命令\n\
              ! -n                             - 执行历史倒数第n条命令\n\
              history | hist                   - 显示所有历史命令\n\
              history | hist n                 - 显示最近n条历史命令\n\
              history | hist search str        - 搜索包含str的历史命令\n\
              history | hist clear             - 清除所有历史记录\n\
            {}",

            bold(&BLUE.to_string()), BLUE, RESET,
            self.version,
            self.history.count(), BLUE, 
            BLUE, RESET,
        );
        print_info(&help_text);
        Ok(Some(help_text))
    }
    
    fn show_version(&self) -> Result<Option<String>> {
        let msg = format!("TypeCmd Version {}", self.version);
        print_info(&msg);
        Ok(Some(msg))
    }
    
    fn show_variables(&self) -> Result<Option<String>> {
        let vars = self.variables.all();
        
        if vars.is_empty() {
            let msg = "没有定义的变量";
            print_info(msg);
            return Ok(Some(msg.to_string()));
        }
        
        let mut output = format!("已定义的变量 (共{}个):\n", vars.len());
        for (key, value) in vars {
            output.push_str(&format!("  {:15} = \"{}\"\n", key, value));
        }
        
        print_info(&output);
        Ok(Some(output))
    }
    
    fn show_history(&self, limit: Option<usize>) -> Result<Option<String>> {
        let entries = self.history.get(limit);
        
        if entries.is_empty() {
            let msg = "历史记录为空";
            print_info(msg);
            return Ok(Some(msg.to_string()));
        }
        
        let limit_str = if let Some(l) = limit {
            format!("最近{}条", l)
        } else {
            "所有".to_string()
        };
        
        let mut output = format!("历史命令 ({}):\n", limit_str);
        for entry in entries.iter().rev() {
            let time_str = entry.timestamp.format("%H:%M:%S").to_string();
            output.push_str(&format!(
                "{:4}  [{}]  {}\n",
                entry.id, time_str, entry.command
            ));
        }
        
        output.push_str("\n使用 !<编号> 执行历史命令");
        
        print_info(&output);
        Ok(Some(output))
    }
    
    fn handle_exit(&self, code: Option<i32>) -> Result<Option<String>> {
        let exit_code = code.unwrap_or(0);
        print_success(&format!("再见! (退出码: {})", exit_code));
        exit(exit_code);
    }
    
    fn handle_set(&mut self, var: &str, value: &str) -> Result<Option<String>> {
        self.variables.set(var.to_string(), value.to_string());
        let msg = format!("变量 \"{}\" 已设置为 \"{}\"", var, value);
        print_success(&msg);
        Ok(Some(msg))
    }
    
    fn handle_get(&self, var: &str) -> Result<Option<String>> {
        match self.variables.get(var) {
            Some(value) => {
                let msg = format!("变量 {} 的值为: {}", var, value);
                print_info(&msg);
                Ok(Some(value.clone()))
            }
            None => {
                Err(TypeCmdError::UndefinedVariable(var.to_string()))
            }
        }
    }

    fn handle_copy(&mut self, var: &str, oldvar: &str) -> Result<Option<String>> {
        // 先获取值并克隆，释放不可变借用后再设置新变量
        let value = match self.variables.get(oldvar) {
            Some(value) => value.clone(),
            None => {
                return Err(TypeCmdError::UndefinedVariable(oldvar.to_string()));
            }
        };

        // 现在不可变借用已结束，可以进行可变操作
        self.variables.set(var.to_string(), value.clone());
        let msg = format!("变量 \"{}\" 已设置为 变量\"{}\"的值 \"{}\"", var, oldvar, value);
        Ok(Some(msg))
    }
    
    fn handle_string(&self, text: &str) -> Result<Option<String>> {
        print_info(text);
        Ok(Some(text.to_string()))
    }
    
    fn handle_int(&self, num: Option<i32>) -> Result<Option<String>> {
        let num_str = num.unwrap_or(0).to_string();
        print_info(&num_str);
        Ok(Some(num_str))
    }
    
    fn handle_list(&self) -> Result<Option<String>> {
        self.show_variables()
    }
    
    fn handle_delete(&mut self, var: &str) -> Result<Option<String>> {
        if self.variables.delete(var) {
            let msg = format!("已删除变量: {}", var);
            print_success(&msg);
            Ok(Some(msg))
        } else {
            let err = format!("变量不存在: {}", var);
            print_error(&err);
            Err(TypeCmdError::UndefinedVariable(var.to_string()))
        }
    }
    
    fn handle_clear(&mut self, target: ClearTarget) -> Result<Option<String>> {
        match target {
            ClearTarget::Variables => {
                let count = self.variables.len();
                self.variables.clear();
                let msg = format!("已清除所有变量 (共{}个)", count);
                print_success(&msg);
                Ok(Some(msg))
            }
            ClearTarget::History => {
                self.history.clear()?;
                let msg = "已清除所有历史记录".to_string();
                print_success(&msg);
                Ok(Some(msg))
            }
        }
    }
    
    fn handle_history(&mut self, subcmd: HistorySubcommand) -> Result<Option<String>> {
        match subcmd {
            HistorySubcommand::List(limit) => self.show_history(limit),
            HistorySubcommand::Search(keyword) => self.search_history(&keyword),
            HistorySubcommand::Clear => {
                self.history.clear()?;
                let msg = "历史记录已清除".to_string();
                print_success(&msg);
                Ok(Some(msg))
            }
        }
    }
    
    fn search_history(&self, keyword: &str) -> Result<Option<String>> {
        let results = self.history.search(keyword);
        
        if results.is_empty() {
            let msg = format!("没有找到包含 \"{}\" 的历史命令", keyword);
            print_info(&msg);
            return Ok(Some(msg));
        }
        
        let mut output = format!("搜索 \"{}\" 的结果 ({}条):\n", keyword, results.len());
        for entry in results.iter().take(20) {
            let time_str = entry.timestamp.format("%H:%M:%S").to_string();
            output.push_str(&format!(
                "{:4}  [{}]  {}\n",
                entry.id, time_str, entry.command
            ));
        }
        
        print_info(&output);
        Ok(Some(output))
    }
    
    fn handle_last_command(&mut self) -> Result<Option<String>> {
        let command_to_execute = match self.history.last() {
            Some(entry) => {
                print_gray(&format!("执行历史命令 #{}: {}", entry.id, entry.command));
                entry.command.clone()
            }
            None => {
                let err = "没有历史命令可执行";
                print_error(err);
                return Err(TypeCmdError::InvalidHistoryCommand(err.to_string()));
            }
        };
        
        self.execute_command(&command_to_execute)
    }
    
    fn handle_history_command(&mut self, spec: &str) -> Result<Option<String>> {
        let command_to_execute = if spec.starts_with('-') {
            let offset_str = &spec[1..];
            let offset: usize = offset_str.parse()
                .map_err(|_| TypeCmdError::InvalidHistoryCommand(
                    format!("无效的偏移量: {}", spec)
                ))?;
            
            let entries = self.history.get(None);
            if offset == 0 || offset > entries.len() {
                return Err(TypeCmdError::InvalidHistoryCommand(
                    format!("偏移量超出范围 (共{}条)", entries.len())
                ));
            }
            
            let entry = &entries[entries.len() - offset];
            print_gray(&format!("执行历史命令 #{}: {}", entry.id, entry.command));
            entry.command.clone()
        } else {
            let id: u64 = spec.parse()
                .map_err(|_| TypeCmdError::InvalidHistoryCommand(
                    format!("无效的历史命令ID: {}", spec)
                ))?;
            
            match self.history.get_by_id(id) {
                Some(entry) => {
                    print_gray(&format!("执行历史命令 #{}: {}", entry.id, entry.command));
                    entry.command.clone()
                }
                None => {
                    let err = format!("历史命令 #{} 不存在", id);
                    print_error(&err);
                    return Err(TypeCmdError::InvalidHistoryCommand(err));
                }
            }
        };
        
        self.execute_command(&command_to_execute)
    }
    
    /// Show the command prompt
    pub fn show_prompt(&self) {
        let var_count = self.variables.len();
        let hist_count = self.history.count();
        
        let mut prompt = format!("{}TypeCmd{}{}@{}{}{}{}", PURPLE, RESET, CYAN, RESET, BLUE, self.version, RESET);

        if var_count > 0 {
            prompt.push_str(&format!(" {}[{} vars]{}", CYAN, var_count, RESET));
        }
        
        if hist_count > 0 {
            prompt.push_str(&format!(" {}[{} hist]{}", GREEN, hist_count, RESET));
        }
        
        prompt.push_str(&format!("\n{}${} ", PURPLE, RESET));
        print!("{}", prompt);
        io::stdout().flush().unwrap();
    }
    
    /// Run the TypeCmd REPL
    pub fn run(&mut self) -> Result<()> {
        print_info(&format!("TypeCmd {} - 输入 'show help' 查看帮助", self.version));
        
        loop {
            self.show_prompt();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();
            
            if input.is_empty() {
                continue;
            }
            
            if let Err(e) = self.history.add(input) {
                print_warn(&format!("无法保存历史记录: {}", e));
            }
            
            let result = if input == "!!" {
                self.execute_command(input)
            } else if input.starts_with('!') && input.len() > 1 {
                let tokens = parse_command(input)?;
                if !tokens.is_empty() && tokens[0] == "!" && tokens.len() > 1 {
                    self.execute_command(input)
                } else {
                    self.execute_command(input)
                }
            } else {
                self.execute_command(input)
            };
            
            match result {
                Ok(_) => {}
                Err(e) => {
                    print_error(&format!("{}", e));
                }
            }
        }
    }
}