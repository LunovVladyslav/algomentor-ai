use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct McpClient {
    _child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    next_id: AtomicU64,
}

#[derive(Serialize)]
struct JsonRpcRequest {
    jsonrpc: &'static str,
    id: u64,
    method: &'static str,
    params: Value,
}

impl McpClient {
    pub async fn start(command: &str, args: &[String], envs: &HashMap<String, String>) -> Result<Self> {
        let mut cmd = Command::new(command);
        cmd.args(args);
        cmd.envs(envs);
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::inherit());
        cmd.kill_on_drop(true);

        let mut child = cmd.spawn().context("Failed to spawn MCP server")?;

        let stdin = child.stdin.take().context("Failed to open stdin")?;
        let stdout = BufReader::new(child.stdout.take().context("Failed to open stdout")?);

        let mut client = Self {
            _child: child,
            stdin,
            stdout,
            next_id: AtomicU64::new(1),
        };

        client.initialize().await?;
        Ok(client)
    }

    async fn send_request(&mut self, method: &'static str, params: Value) -> Result<Value> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let req = JsonRpcRequest {
            jsonrpc: "2.0",
            id,
            method,
            params,
        };

        let mut line = serde_json::to_string(&req)?;
        line.push('\n');

        self.stdin.write_all(line.as_bytes()).await?;
        self.stdin.flush().await?;

        // Read response
        let mut response_line = String::new();
        self.stdout.read_line(&mut response_line).await?;

        let response: Value = serde_json::from_str(&response_line)?;
        
        if let Some(err) = response.get("error") {
            anyhow::bail!("MCP Error: {}", err);
        }

        Ok(response["result"].clone())
    }

    pub async fn initialize(&mut self) -> Result<()> {
        let params = serde_json::json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "algomentor",
                "version": "0.1.0"
            }
        });

        self.send_request("initialize", params).await?;
        
        // Send notifications/initialized
        let notif = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        });
        let mut line = serde_json::to_string(&notif)?;
        line.push('\n');
        self.stdin.write_all(line.as_bytes()).await?;
        self.stdin.flush().await?;

        Ok(())
    }

    pub async fn list_tools(&mut self) -> Result<Vec<crate::llm::provider::Tool>> {
        let res = self.send_request("tools/list", serde_json::json!({})).await?;
        
        let mut tools = Vec::new();
        if let Some(tools_array) = res["tools"].as_array() {
            for t in tools_array {
                if let (Some(name), Some(desc), Some(schema)) = (
                    t["name"].as_str(),
                    t["description"].as_str(),
                    t.get("inputSchema")
                ) {
                    tools.push(crate::llm::provider::Tool {
                        type_: "function".to_string(),
                        function: crate::llm::provider::FunctionDefinition {
                            name: name.to_string(),
                            description: desc.to_string(),
                            parameters: schema.clone(),
                        }
                    });
                }
            }
        }

        Ok(tools)
    }

    pub async fn call_tool(&mut self, name: &str, arguments: Value) -> Result<String> {
        let params = serde_json::json!({
            "name": name,
            "arguments": arguments
        });

        let res = self.send_request("tools/call", params).await?;
        
        if let Some(content) = res["content"].as_array() {
            let mut output = String::new();
            for item in content {
                if let Some(text) = item["text"].as_str() {
                    output.push_str(text);
                }
            }
            Ok(output)
        } else {
            Ok(res.to_string())
        }
    }
}
