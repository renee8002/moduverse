    use std::path::{Path, PathBuf};
    use regex::Regex;
    use std::env;
    use std::fs::OpenOptions;
    use std::process;
    use std::io::{self, Write};

    // CommandInterpreter is responsible for interpreting and processing user commands.
    pub struct CommandInterpreter;

    impl CommandInterpreter {

        

        // Method 1: Guide the user on correct input format
        // Input: None
        // Output: None (Prints to stdout)
        pub fn guide_user_input() {
            println!("Currently supporting the following commands (Format: Command - Description):
                    1. init - Create an empty repository
                    2. clone <repo> - Copy an existing repository
                    3. add <file name> - Add specific files to track
                    4. remove <file name> - Remove specific files from tracking
                    5. status - Check the current status of the repository
                    6. heads - Show the current heads
                    7. diff <revision1> <revision2> - Check changes between revisions
                    8. cat <file name> <revision> - Inspect a file of a given revision
                    9. checkout <branch-name or commit-hash> - Check out a specific revision
                    10. commit -m '<message>' - Commit changes and create a new revision
                    11. log - View the change log
                    12. merge <source-branch> <target-branch> - Merge two revisions
                    13. pull <remote-name> <branch-name> - Pull changes from another repository
                    14. push <remote-name> <branch-name> - Push changes into another repository
                    
                    Example command: add main.txt");
        }


        pub fn get_user_input() -> String {
            let mut input = String::new();
    
            print!("Enter command: ");
            io::stdout().flush().unwrap();
    
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
    
            input.trim().to_string()
        }
        
        // Method 2: Interpret the user's command
        // Input: input - String (user input)
        // Output: Result<ExecutableCommand, InterpretationError>
        // This function will eventually parse and process user input.
        pub fn interpret_command(input: String) -> Result<ExecutableCommand, InterpretationError> {
            if input.trim() == "exit" {
                process::exit(0); // 直接退出程序
            }
              // 首先验证用户输入
            Self::validate_user_input(&input)?;

            // 解析命令
            let parts: Vec<&str> = input.trim().split_whitespace().collect();
            let command = match parts[0] {
                "init" => ExecutableCommand::Init,
                "clone" => ExecutableCommand::Clone(parts[1].to_string()),
                "add" => ExecutableCommand::Add(parts[1].to_string()),
                "remove" => ExecutableCommand::Remove(parts[1].to_string()),
                "cat" => ExecutableCommand::Cat(parts[1].to_string(), parts[2].to_string()),
                "checkout" => ExecutableCommand::Checkout(parts[1].to_string()),                   
                "commit" => ExecutableCommand::Commit(parts[2..].join(" ")
                .trim_matches(|c| c == '\'' || c == '"')
                .to_string()),                 
                "diff" => ExecutableCommand::Diff(parts[1].to_string(), parts[2].to_string()),
                "merge" => ExecutableCommand::Merge(parts[1].to_string(), parts[2].to_string()),
                "pull" => ExecutableCommand::Pull(parts[1].to_string(), parts[2].to_string()),       
                "push" => ExecutableCommand::Push(parts[1].to_string(), parts[2].to_string()),
                "status" => ExecutableCommand::Status,
                "heads" => ExecutableCommand::Heads,
                "log" => ExecutableCommand::Log,
                _ => return Err(InterpretationError::new("Unsupported command."))
            };

    

            Ok(command) // Placeholder for future implementation
        }

        // Method 3: Validate the input format
        // Input: input - &str (user input)
        // Output: bool (true if format is valid, false otherwise)
        // This function will check if the input format is correct.
        // Validate the user input format
        pub fn validate_user_input(input: &str) -> Result<(), InterpretationError> {
            let parts: Vec<&str> = input.trim().split_whitespace().collect();
            if parts.is_empty() {
                return Err(InterpretationError::new("Input is empty. Please enter a command."));
            }

            match parts[0] {
                "init" | "status" | "heads" | "log" => Self::validate_no_arguments(&parts)?,
                "add" => Self::validate_add_command(&input)?,
                "clone" | "remove" | "cat" => Self::validate_file_path(&parts)?,
                "checkout" => Self::validate_checkout_command(&parts)?,
                "commit" => Self::validate_commit_command(&parts)?,
                "diff" => Self::validate_diff_command(&parts)?,
                "merge" => Self::validate_merge_command(&parts)?,
                "pull" | "push" => Self::validate_pull_push_command(&parts, parts[0])?,
                _ => return Err(InterpretationError::new("Invalid command. Please enter a valid command."))
            }

            Ok(())
        }

        // Private helper methods
        pub fn validate_no_arguments(parts: &[&str]) -> Result<(), InterpretationError> {
            if parts.len() != 1 {
                return Err(InterpretationError::new("This command does not require any arguments."));
            }
            Ok(())
        }

        // Validates the 'add' command input
        pub fn validate_add_command(input: &str) -> Result<(), InterpretationError> {
            // Split the input into parts for analysis
            let parts: Vec<&str> = input.split_whitespace().collect();

            // Check if the input format is correct (e.g., "add filename")
            if parts.len() != 2 || parts[0] != "add" {
                return Err(InterpretationError::new("Invalid input format. Expected: add <filename>"));
            }

            // Resolve the file name to a full path
            let file_name = parts[1];
            let full_path = resolve_working_directory(file_name)?;

            // Check if the file already exists
            if full_path.exists() {
                return Err(InterpretationError::new(&format!("File {} already exists. Please use a different name.", file_name)));
            }

            Self::validate_file_name_format(file_name)?;

            Ok(())
        }


        pub fn validate_diff_command(parts: &[&str]) -> Result<(), InterpretationError> {
            if parts.len() != 3 {
                return Err(InterpretationError::new("Invalid diff command format. Expected: diff <revision1> <revision2>"));
            }
            if !Self::is_valid_revision(parts[1]) || !Self::is_valid_revision(parts[2]) {
                return Err(InterpretationError::new("Invalid revision format."));
            }
            Ok(())
        }
        
        pub fn validate_merge_command(parts: &[&str]) -> Result<(), InterpretationError> {
            if parts.len() != 3 {
                return Err(InterpretationError::new("Invalid merge command format. Expected: merge <source-branch> <target-branch>"));
            }
            if !Self::is_valid_branch(parts[1]) || !Self::is_valid_branch(parts[2]) {
                return Err(InterpretationError::new("Invalid branch name."));
            }
            Ok(())
        }
        
        pub fn validate_pull_push_command(parts: &[&str], command: &str) -> Result<(), InterpretationError> {
            if parts.len() != 3 {
                return Err(InterpretationError::new(
                    &format!("Invalid {} command format. Expected: {} <remote-name> <branch-name>", command, command)
                ));
            }
            if !Self::is_valid_remote(parts[1]) || !Self::is_valid_branch(parts[2]) {
                return Err(InterpretationError::new(
                    &format!("Invalid parameters for {} command.", command)
                ));
            }
            Ok(())
        }
        

        pub fn validate_file_path(parts: &[&str]) -> Result<(), InterpretationError> {
            // 确保parts数组中有足够的元素
            if parts.len() != 2 {
                return Err(InterpretationError::new("Invalid command format. Expected two parts."));
            }
        
            // 从parts中获取文件名
            let file_name = parts[1];
        
            // 使用 resolve_working_directory 函数来获取完整路径
            let full_path = resolve_working_directory(file_name)?;
        
            // 检查文件是否存在
            if !full_path.exists() {
                return Err(InterpretationError::new(&format!("Invalid file path. File {} does not exist.", file_name)));
            }
            
            //Self::validate_file_name_format(file_name)?;
            
            Ok(())
        }
        
        


        pub fn validate_file_name_format(file_name: &str) -> Result<(), InterpretationError> {
            // 正则表达式：允许字母、数字、下划线、横线和点
            let re = Regex::new(r"^[a-zA-Z0-9_.-]+$").expect("Invalid regex pattern");
        
            // 检查是否匹配正则表达式
            if !re.is_match(file_name) {
                return Err(InterpretationError::new("File name contains invalid characters."));
            }
        
            // 检查文件扩展名是否合法
            let extension = Path::new(file_name).extension().and_then(|s| s.to_str());
            match extension {
                Some("txt") | Some("md") | Some("json") | Some("rs") | Some("cpp") => Ok(()),
                _ => Err(InterpretationError::new("Invalid file extension. Supported extensions: .txt, .md, .json, .rs, .cpp"))
            }
        }
        

        fn is_valid_branch_or_commit(name: &str) -> bool {
            let re = Regex::new(r"^[a-zA-Z0-9\-_/.]+$").expect("Invalid regex pattern");
            re.is_match(name)
        }

        fn is_valid_revision(revision: &str) -> bool {
            let re = Regex::new(r"^[a-zA-Z0-9\-_]+$").expect("Invalid regex pattern");
            re.is_match(revision)
        }

        fn is_valid_branch(branch: &str) -> bool {
            let re = Regex::new(r"^[a-zA-Z0-9\-_/]+$").expect("Invalid regex pattern");
            re.is_match(branch)
        }
        
        fn is_valid_remote(remote: &str) -> bool {
            let re = Regex::new(r"^[a-zA-Z0-9]+$").expect("Invalid regex pattern");
            re.is_match(remote)
        }
        
       pub fn validate_checkout_command(parts: &[&str]) -> Result<(), InterpretationError> {
            if parts.len() != 2 {
                return Err(InterpretationError::new("Invalid checkout command format. Expected: checkout <branch-name or commit-hash>"));
            }

            let branch_or_commit = parts[1];
            if !Self::is_valid_branch_or_commit(branch_or_commit) {
                return Err(InterpretationError::new("Invalid branch name or commit hash."));
            }

            Ok(())
        }


        fn validate_commit_command(parts: &[&str]) -> Result<(), InterpretationError> {
            // 验证命令格式是否正确
            if parts.len() < 3 || parts[1] != "-m" {
                return Err(InterpretationError::new("Invalid commit command format. Expected: commit -m '<message>'"));
            }
        
            // 提取并处理提交信息
            // 这里假设提交信息是从第三个元素开始的所有元素组成的
            let message = parts[2..].join(" ");
        
            // 去除可能的单引号或双引号
            let trimmed_message = message.trim_matches(|c| c == '\'' || c == '"');
        
            // 检查提交信息是否为空
            if trimmed_message.is_empty() {
                return Err(InterpretationError::new("Commit message cannot be empty."));
            }
        
            Ok(())
        }
        

        
 
    }
    
    pub struct InterpretationError {
        message: String,
    }
    
    impl InterpretationError {
        pub fn new(message: &str) -> Self {
            InterpretationError {
                message: message.to_string(),
            }
        }
    }      
    
    // Method 4: Resolve the working directory from the input
    // Input: input - &str (file path)
    // Output: Result<PathBuf, InterpretationError> (resolved path or error)
    // This function will extract and validate the file path from the input.
    fn resolve_working_directory(file_name: &str) -> Result<PathBuf, InterpretationError> {
        let current_dir = env::current_dir().map_err(|_| InterpretationError::new("Failed to get current directory."))?;
        let absolute_path = current_dir.join(file_name);
        Ok(absolute_path)
    }
    
    pub enum ExecutableCommand {
        Init,
        Clone(String), 
        Add(String), 
        Remove(String),
        Cat(String, String), 
        Checkout(String),
        Commit(String), 
        Diff(String, String),
        Merge(String, String),
        Pull(String, String),
        Push(String, String),
        Status,
        Heads,
        Log,
    }

    #[derive(Debug)]
    pub enum ErrorType {
        Io,
        Network,
        Parse,
        Command,
    }
    // Method 5: Handle errors in command execution
    // Input: error - ExecutionError
    // Output: None (side effects may include logging or user notification)
    // This function will handle errors that occur during command execution.
    pub fn handle_error(error: ExecutionError) {
        match error.error_type {
            ErrorType::Io => {
                println!("I/O error occurred: {}", error);
                // 特定的I/O错误处理逻辑
            },
            ErrorType::Network => {
                println!("Network error occurred: {}", error);
                // 特定的网络错误处理逻辑
            },
            ErrorType::Parse => {
                println!("Parse error occurred: {}", error);
                // 解析错误的处理逻辑
            },
            ErrorType::Command => {
                println!("Command execution error: {}", error);
                // 命令执行错误的处理逻辑
            },
        }
    
        // 通用的错误处理，比如记录日志
        log_error(&error);
    }
    
    fn log_error(error: &ExecutionError) {
        let log_file_path = Path::new("error.log");
    
        let mut file = match OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&log_file_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Failed to open log file: {}", e);
                    return;
                }
        };
    
        if let Err(e) = writeln!(file, "Error: {}", error) {
            eprintln!("Failed to write to log file: {}", e);
        }
    }

    // // Structure representing the result of an action.
    // pub struct ActionResult {
    //     // Details of the action result (to be implemented)
    // }



    // Error type for execution errors.
    pub struct ExecutionError {
        error_type: ErrorType,
        code: Option<u32>,
        message: String,
    }
    
    impl ExecutionError {
        pub fn new(error_type: ErrorType, code: Option<u32>, message: &str) -> ExecutionError {
            ExecutionError {
                error_type,
                code,
                message: message.to_string(),
            }
        }
    }
    
    
    impl std::fmt::Display for ExecutionError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "[{:?}] Error {}: {}", self.error_type, self.code.unwrap_or(0), self.message)
        }
    }
    
    impl std::fmt::Debug for ExecutionError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{{ type: {:?}, code: {}, message: {} }}", self.error_type, self.code.unwrap_or(0), self.message)
        }
    }



    