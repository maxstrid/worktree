mod args;
use std::error::Error;
use std::fmt;
use std::path::Path;
use std::process::Command;

pub struct Worktree;

impl Worktree {
    pub fn remove_worktree(
        filename: String,
        branch_name: Option<String>,
    ) -> Result<(), Box<dyn Error>> {
        let file_path = Path::new(&filename);

        if !file_path.exists() {
            return Err(Box::new(WorktreeError(format!(
                "File not found: {}",
                filename
            ))));
        }

        println!("Removing directory {filename}");

        std::fs::remove_dir_all(file_path)?;

        Self::spawn_cmd("git worktree prune".to_string())?;

        if let Some(branch) = branch_name {
            Self::spawn_cmd(format!("git branch -D {branch}"))?;
        }

        return Ok(());
    }

    pub fn add_from_gerrit(
        gerrit_command: String,
        branch_name: String,
    ) -> Result<(), Box<dyn Error>> {
        Self::spawn_cmd(String::from(
            *gerrit_command
                .split("&&")
                .collect::<Vec<&str>>()
                .get(0)
                .unwrap(),
        ))?;

        Self::spawn_cmd(format!("git worktree add {branch_name} FETCH_HEAD"))?;

        Self::spawn_cmd(format!("cd {branch_name} && git switch -c {branch_name}"))
    }

    fn spawn_cmd(command_name: String) -> Result<(), Box<dyn Error>> {
        println!("Running: {command_name}");

        let mut command = Command::new("sh").arg("-c").arg(&command_name).spawn()?;

        let status = command.wait()?;

        if !status.success() {
            return Err(Box::new(WorktreeError(format!(
                "Failed to run command '{}' with exit code {}",
                command_name, status
            ))));
        }

        return Ok(());
    }
}

#[derive(Debug, Clone)]
pub struct WorktreeError(String);

impl fmt::Display for WorktreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Worktree error: {}", self.0)
    }
}

impl std::error::Error for WorktreeError {}
