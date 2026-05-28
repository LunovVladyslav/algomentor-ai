use anyhow::{Context, Result};
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

/// The maximum time allowed for code execution to prevent infinite loops
const EXECUTION_TIMEOUT: Duration = Duration::from_secs(5);

pub struct RunCodeTool;

impl RunCodeTool {
    pub fn name() -> &'static str {
        "run_code"
    }

    pub fn description() -> &'static str {
        "Compiles and runs the provided solution file. Returns the stdout and stderr output. Use this to verify if the user's code works."
    }

    pub fn parameters() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "file_path": {
                    "type": "string",
                    "description": "The relative or absolute path to the solution file to run (e.g. 'solution.py', 'solution.java')."
                }
            },
            "required": ["file_path"]
        })
    }

    pub async fn execute(file_path: &str) -> String {
        match Self::run_internal(file_path).await {
            Ok(output) => output,
            Err(e) => format!("Error executing code: {}", e),
        }
    }

    async fn run_internal(file_path: &str) -> Result<String> {
        let path = Path::new(file_path);
        if !path.exists() {
            anyhow::bail!("File not found: {}", file_path);
        }

        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        
        match extension {
            "py" => Self::run_python(path).await,
            "java" => Self::run_java(path).await,
            "js" => Self::run_node(path).await,
            "rs" => Self::run_rust(path).await,
            _ => anyhow::bail!("Unsupported file extension: .{}", extension),
        }
    }

    async fn execute_command_with_timeout(child: tokio::process::Child) -> Result<String> {
        match timeout(EXECUTION_TIMEOUT, child.wait_with_output()).await {
            Ok(Ok(output)) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                
                let mut result = String::new();
                if !stdout.is_empty() {
                    result.push_str("=== STDOUT ===\n");
                    result.push_str(&stdout);
                    result.push_str("\n");
                }
                if !stderr.is_empty() {
                    result.push_str("=== STDERR ===\n");
                    result.push_str(&stderr);
                    result.push_str("\n");
                }
                
                if result.is_empty() {
                    result.push_str("[Execution finished with no output]");
                }
                
                Ok(result)
            }
            Ok(Err(e)) => anyhow::bail!("Failed to read process output: {}", e),
            Err(_) => {
                anyhow::bail!("Execution timed out after {} seconds (possible infinite loop)", EXECUTION_TIMEOUT.as_secs())
            }
        }
    }

    async fn run_python(path: &Path) -> Result<String> {
        let child = Command::new("python3")
            .arg(path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .context("Failed to start python3")?;
            
        Self::execute_command_with_timeout(child).await
    }

    async fn run_node(path: &Path) -> Result<String> {
        let child = Command::new("node")
            .arg(path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .context("Failed to start node")?;
            
        Self::execute_command_with_timeout(child).await
    }

    async fn run_java(path: &Path) -> Result<String> {
        // Compile first
        let compile_output = Command::new("javac")
            .arg(path)
            .output()
            .await
            .context("Failed to run javac")?;

        if !compile_output.status.success() {
            let stderr = String::from_utf8_lossy(&compile_output.stderr);
            return Ok(format!("=== COMPILATION ERROR ===\n{}", stderr));
        }

        // Run
        let parent_dir = path.parent().unwrap_or(Path::new(""));
        let class_name = path.file_stem().unwrap_or_default();
        
        let child = Command::new("java")
            .current_dir(parent_dir)
            .arg(class_name)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .context("Failed to start java")?;

        Self::execute_command_with_timeout(child).await
    }

    async fn run_rust(path: &Path) -> Result<String> {
        // Use rustc for single files
        let parent_dir = path.parent().unwrap_or(Path::new(""));
        let file_stem = path.file_stem().unwrap_or_default();
        let out_file = parent_dir.join(file_stem);

        let compile_output = Command::new("rustc")
            .arg(path)
            .arg("-o")
            .arg(&out_file)
            .output()
            .await
            .context("Failed to run rustc")?;

        if !compile_output.status.success() {
            let stderr = String::from_utf8_lossy(&compile_output.stderr);
            return Ok(format!("=== COMPILATION ERROR ===\n{}", stderr));
        }

        let child = Command::new(&out_file)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .context("Failed to run compiled rust binary")?;

        let res = Self::execute_command_with_timeout(child).await;
        
        // Cleanup binary
        let _ = tokio::fs::remove_file(out_file).await;
        
        res
    }
}
