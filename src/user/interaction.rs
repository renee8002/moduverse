// Interaction Module
pub mod interaction {
    use crate::user_interaction::ExecutableCommand;
    use crate::repository::repository::Repository;
    use std::env;
    use std::error::Error;
    use std::path::{Path, PathBuf};
    // InteractionManager is responsible for managing interactions with the Repository Module.
    pub struct InteractionManager;

    impl InteractionManager {
        // Method 1: Send a validated command to the Repository Module
        // Input: command - ExecutableCommand
        // Output: Result<(), SendError> (success or send error)
        // This function will handle the logic of interacting with the Repository Module.
        pub fn send_command_to_repository(command: ExecutableCommand) -> Result<String, SendError> {
            // Current working directory is assumed to be the repository path
            let repo_path = env::current_dir().map_err(|e| SendError::RepositoryError(e.to_string()))?.to_str().unwrap().to_owned();
            let repository = Repository::new(&repo_path);
            let repo_root = Self::find_repo_root(Path::new(&repo_path))
                            .map_err(|e| SendError::RepositoryError(e))?;

            match command {
                ExecutableCommand::Init => {
                    Repository::init(&repository).map_err(|e| SendError::RepositoryError(e))
                },
                ExecutableCommand::Clone(path) => {
                    Repository::clone(&repository, &repo_path).map_err(|e| SendError::RepositoryError(e))
                },
                ExecutableCommand::Add(file_name) => {
                    Repository::add(&repo_path, &file_name).map_err(|e| SendError::RepositoryError(e))
                },
                ExecutableCommand::Remove(file_name) => {
                    Repository::remove(&repo_path, &file_name).map_err(|e| SendError::RepositoryError(e))
                },
                ExecutableCommand::Cat(file_name, revision) => {
                    Repository::cat(&repo_path, &revision, &file_name).map_err(|e| SendError::RepositoryError(e))
                },
                ExecutableCommand::Checkout(branch_or_commit) => {
                    Repository::checkout(&branch_or_commit).map_err(|e| SendError::RepositoryError(e))
                },
                ExecutableCommand::Commit(message) => {
                    Repository::commit(&message).map_err(|e| SendError::RepositoryError(e))
                },
                ExecutableCommand::Diff(rev1, rev2) => {
                    Repository::diff(&rev1, &rev2).map_err(|e| SendError::RepositoryError(e))
                },
                ExecutableCommand::Merge(source, target) => {
                    Repository::merge(&source, &target).map_err(|e| SendError::RepositoryError(e))
                },
                ExecutableCommand::Pull(remote, branch) => {
                    Repository::pull(&repository, &branch).map_err(|e| SendError::RepositoryError(e))
                },
                ExecutableCommand::Push(remote, branch) => {
                    Repository::push(&repository, &branch).map_err(|e| SendError::RepositoryError(e))
                },
                ExecutableCommand::Status => {
                    Repository::status().map_err(|e| SendError::RepositoryError(e))
                },
                ExecutableCommand::Heads => {
                    Repository::heads(&repo_root).map_err(|e| SendError::RepositoryError(e))
                },
                ExecutableCommand::Log => {
                    Repository::log().map_err(|e| SendError::RepositoryError(e))
                },
                _ => Err(SendError::UnsupportedCommand),
            }
        }

        fn find_repo_root(starting_path: &Path) -> Result<PathBuf, String> {
            let mut current_path = starting_path.to_path_buf();
        
            loop {
                if current_path.join(".mdv").exists() {
                    return Ok(current_path);
                }
        
                if !current_path.pop() {
                    return Err("Not inside a repository.".to_string());
                }
            }
        }
        
        // Method 2: Receive and handle the response from the Repository Module
        // Input: None
        // Output: Result<Response, ReceiveError> (response or receive error)
        // This function will process the responses received from the Repository Module.
        pub fn receive_response(result: RepositoryResult) -> Result<String, ReceiveError> {
            match result.result {
                Ok(message) => Ok(message),
                Err(err_msg) => Err(ReceiveError::RepositoryError(err_msg)),
            }
        }
    }

    // Error types for sending operations
    pub enum SendError {
        RepositoryError(String),  // Error from repository operations
        UnsupportedCommand,       // Command is not supported
    }

    // Error types for receiving responses
    pub enum ReceiveError {
        RepositoryError(String),  // Error in receiving response
    }

    // Structure representing the response from the repository module
    pub struct RepositoryResult {
        result: Result<String, String>,
    }

    impl RepositoryResult {
        pub fn success(message: String) -> Self {
            RepositoryResult {
                result: Ok(message),
            }
        }

        pub fn error(err_msg: String) -> Self {
            RepositoryResult {
                result: Err(err_msg),
            }
        }

        pub fn is_success(&self) -> bool {
            self.result.is_ok()
        }

        pub fn message(&self) -> String {
            match &self.result {
                Ok(msg) => msg.clone(),
                Err(err_msg) => err_msg.clone(),
            }
        }
    }
    // Test cases for the Interaction Module.
    // #[cfg(test)]
    // mod tests {
    //     use super::*;

    //     // Test 1: Testing sending a command to the repository
    //     #[test]
    //     fn test_send_command_to_repository() {
    //         let command = ExecutableCommand::new();
    //         assert!(matches!(InteractionManager::send_command_to_repository(command), Ok(_)));
    //     }

    //     // Test 2: Testing receiving a response from the repository
    //     #[test]
    //     fn test_receive_response() {
    //         assert!(matches!(InteractionManager::receive_response(), Ok(_)));
    //     }

    //     // Test 3: Testing sending an invalid command to the repository
    //     #[test]
    //     fn test_send_command_with_invalid_command() {
    //         let command = ExecutableCommand::new(); // Assuming this command is invalid
    //         assert!(matches!(InteractionManager::send_command_to_repository(command), Err(_)));
    //     }

    //     // Test 4: Testing receiving an error response from the repository
    //     #[test]
    //     fn test_receive_error_response() {
    //         // Assuming a scenario that leads to an error response
    //         assert!(matches!(InteractionManager::receive_response(), Err(_)));
    //     }

    //     // Test 5: Testing handling of invalid response format from the repository
    //     #[test]
    //     fn test_receive_invalid_response_format() {
    //         // Assuming a scenario with an invalid response format
    //         assert!(matches!(InteractionManager::receive_response(), Err(_)));
    //     }

    //     // Test 6: Testing timeout error when receiving a response
    //     #[test]
    //     fn test_receive_timeout_error() {
    //         // Assuming a scenario that leads to a timeout error
    //         assert!(matches!(InteractionManager::receive_response(), Err(_)));
    //     }
    // }
}
