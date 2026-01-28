use crate::changeset::{Change, ChangeSet};
use crate::paths::PrismctlHome;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaudeMcpScope {
    Local,
    Project,
    User,
}

impl ClaudeMcpScope {
    pub fn as_flag_value(&self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Project => "project",
            Self::User => "user",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeminiMcpScope {
    User,
    Project,
}

impl GeminiMcpScope {
    pub fn as_flag_value(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Project => "project",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum McpTransport {
    Http,
    Sse,
    StreamableHttp,
    Stdio,
}

impl McpTransport {
    pub fn as_flag_value(&self) -> &'static str {
        match self {
            Self::Http => "http",
            Self::Sse => "sse",
            Self::StreamableHttp => "streamable-http",
            Self::Stdio => "stdio",
        }
    }
}

#[derive(Debug, Clone)]
pub struct McpServerDefinition {
    /// Stable identifier used in `--name` selection.
    pub id: &'static str,
    pub transport: McpTransport,

    /// Remote servers.
    pub url: Option<&'static str>,
    pub headers: &'static [(&'static str, &'static str)],

    /// Local stdio servers.
    pub command: Option<&'static str>,
    pub args: &'static [&'static str],
    pub env: &'static [(&'static str, &'static str)],
}

pub fn list_builtin_mcp_servers() -> &'static [McpServerDefinition] {
    list_builtin_claude_mcp_servers()
}

pub fn list_builtin_claude_mcp_servers() -> &'static [McpServerDefinition] {
    // Keep this list small and curated; it can be expanded as we gain confidence.
    // Values can reference environment variables (e.g. "${API_KEY}") which Claude Code expands.
    static SERVERS: &[McpServerDefinition] = &[
        McpServerDefinition {
            id: "context7",
            transport: McpTransport::Http,
            url: Some("https://mcp.context7.com/mcp"),
            headers: &[("CONTEXT7_API_KEY", "${CONTEXT7_API_KEY}")],
            command: None,
            args: &[],
            env: &[],
        },
        McpServerDefinition {
            id: "mcp-deepwiki",
            transport: McpTransport::Stdio,
            url: None,
            headers: &[],
            command: Some("npx"),
            args: &["-y", "mcp-deepwiki@latest"],
            env: &[],
        },
        McpServerDefinition {
            id: "playwright",
            transport: McpTransport::Stdio,
            url: None,
            headers: &[],
            command: Some("npx"),
            args: &["-y", "@playwright/mcp@latest"],
            env: &[],
        },
        McpServerDefinition {
            id: "spec-workflow",
            transport: McpTransport::Stdio,
            url: None,
            headers: &[],
            command: Some("npx"),
            args: &["-y", "@pimzino/spec-workflow-mcp@latest"],
            env: &[],
        },
        McpServerDefinition {
            id: "open-websearch",
            transport: McpTransport::Stdio,
            url: None,
            headers: &[],
            command: Some("npx"),
            args: &["-y", "open-websearch@latest"],
            env: &[
                ("MODE", "stdio"),
                ("DEFAULT_SEARCH_ENGINE", "duckduckgo"),
                ("ALLOWED_SEARCH_ENGINES", "duckduckgo,bing,brave"),
            ],
        },
    ];
    SERVERS
}

pub fn builtin_mcp_server(id: &str) -> Option<&'static McpServerDefinition> {
    list_builtin_mcp_servers().iter().find(|s| s.id == id)
}

pub fn builtin_claude_mcp_server(id: &str) -> Option<&'static McpServerDefinition> {
    builtin_mcp_server(id)
}

/// Plan to add a built-in MCP server via Claude Code CLI (`claude mcp add`).
///
/// We set `HOME` (and `USERPROFILE` on Windows) to Prismctl's resolved home to preserve the
/// `--home` sandbox semantics.
pub fn plan_claude_mcp_add(
    home: &PrismctlHome,
    scope: ClaudeMcpScope,
    server_id: &str,
    project_cwd: Option<PathBuf>,
) -> Result<ChangeSet, String> {
    let def = builtin_mcp_server(server_id)
        .ok_or_else(|| format!("未知 MCP server: {}（内置）", server_id))?;

    let mut args: Vec<String> = Vec::new();
    args.push("mcp".to_string());
    args.push("add".to_string());
    args.push("--transport".to_string());
    args.push(def.transport.as_flag_value().to_string());
    args.push("--scope".to_string());
    args.push(scope.as_flag_value().to_string());

    // Options must come before the server name.
    for (k, v) in def.headers.iter() {
        args.push("--header".to_string());
        args.push(format!("{}: {}", k, v));
    }
    for (k, v) in def.env.iter() {
        args.push("--env".to_string());
        args.push(format!("{}={}", k, v));
    }

    args.push(def.id.to_string());

    match def.transport {
        McpTransport::Http | McpTransport::Sse | McpTransport::StreamableHttp => {
            let url = def
                .url
                .ok_or_else(|| format!("MCP server {} 缺少 url", def.id))?;
            args.push(url.to_string());
        }
        McpTransport::Stdio => {
            let cmd = def
                .command
                .ok_or_else(|| format!("MCP server {} 缺少 command", def.id))?;
            args.push("--".to_string());

            // Windows native requires `cmd /c` wrapper for `npx` (Claude Code docs).
            let is_windows = env::consts::OS == "windows";
            if is_windows && cmd == "npx" {
                args.push("cmd".to_string());
                args.push("/c".to_string());
            }

            args.push(cmd.to_string());
            args.extend(def.args.iter().map(|s| (*s).to_string()));
        }
    }

    let mut cs = ChangeSet::new();
    cs.push(Change::RunCommand {
        program: "claude".to_string(),
        args,
        cwd: project_cwd,
        env: claude_cli_env_for_home(home),
    });
    Ok(cs)
}

/// Plan to remove an MCP server via Claude Code CLI (`claude mcp remove`).
pub fn plan_claude_mcp_remove(
    home: &PrismctlHome,
    server_name: &str,
    project_cwd: Option<PathBuf>,
) -> ChangeSet {
    let args = vec![
        "mcp".to_string(),
        "remove".to_string(),
        server_name.to_string(),
    ];
    let mut cs = ChangeSet::new();
    cs.push(Change::RunCommand {
        program: "claude".to_string(),
        args,
        cwd: project_cwd,
        env: claude_cli_env_for_home(home),
    });
    cs
}

fn claude_cli_env_for_home(home: &PrismctlHome) -> Vec<(String, String)> {
    let mut envs = Vec::new();
    let home_dir = home.home_dir().to_string_lossy().to_string();
    envs.push(("HOME".to_string(), home_dir.clone()));
    // Windows tooling often relies on USERPROFILE.
    envs.push(("USERPROFILE".to_string(), home_dir));
    envs
}

/// Plan to add a built-in MCP server via Gemini CLI (`gemini mcp add`).
///
/// We set `HOME` (and `USERPROFILE` on Windows) to Prismctl's resolved home to preserve the
/// `--home` sandbox semantics.
pub fn plan_gemini_mcp_add(
    home: &PrismctlHome,
    scope: GeminiMcpScope,
    server_id: &str,
    project_cwd: Option<PathBuf>,
) -> Result<ChangeSet, String> {
    let def = builtin_mcp_server(server_id)
        .ok_or_else(|| format!("未知 MCP server: {}（内置）", server_id))?;

    let mut args: Vec<String> = Vec::new();
    args.push("mcp".to_string());
    args.push("add".to_string());

    // Gemini CLI flags.
    args.push("--transport".to_string());
    // Gemini CLI exposes `http` for streamable HTTP transport.
    let transport = match def.transport {
        McpTransport::StreamableHttp => McpTransport::Http,
        other => other,
    };
    args.push(transport.as_flag_value().to_string());
    args.push("--scope".to_string());
    args.push(scope.as_flag_value().to_string());

    for (k, v) in def.headers.iter() {
        args.push("--header".to_string());
        args.push(format!("{}: {}", k, v));
    }
    for (k, v) in def.env.iter() {
        args.push("--env".to_string());
        args.push(format!("{}={}", k, v));
    }

    // Positional args differ by transport.
    args.push(def.id.to_string());
    match def.transport {
        McpTransport::Http | McpTransport::Sse | McpTransport::StreamableHttp => {
            let url = def
                .url
                .ok_or_else(|| format!("MCP server {} 缺少 url", def.id))?;
            args.push(url.to_string());
        }
        McpTransport::Stdio => {
            let cmd = def
                .command
                .ok_or_else(|| format!("MCP server {} 缺少 command", def.id))?;
            args.push(cmd.to_string());
            args.extend(def.args.iter().map(|s| (*s).to_string()));
        }
    }

    let mut cs = ChangeSet::new();
    cs.push(Change::RunCommand {
        program: "gemini".to_string(),
        args,
        cwd: project_cwd,
        env: gemini_cli_env_for_home(home),
    });
    Ok(cs)
}

pub fn plan_gemini_mcp_remove(
    home: &PrismctlHome,
    scope: GeminiMcpScope,
    server_name: &str,
    project_cwd: Option<PathBuf>,
) -> ChangeSet {
    let args = vec![
        "mcp".to_string(),
        "remove".to_string(),
        "--scope".to_string(),
        scope.as_flag_value().to_string(),
        server_name.to_string(),
    ];
    let mut cs = ChangeSet::new();
    cs.push(Change::RunCommand {
        program: "gemini".to_string(),
        args,
        cwd: project_cwd,
        env: gemini_cli_env_for_home(home),
    });
    cs
}

pub fn plan_gemini_mcp_enable(
    home: &PrismctlHome,
    server_name: &str,
    project_cwd: Option<PathBuf>,
) -> ChangeSet {
    let args = vec![
        "mcp".to_string(),
        "enable".to_string(),
        server_name.to_string(),
    ];
    let mut cs = ChangeSet::new();
    cs.push(Change::RunCommand {
        program: "gemini".to_string(),
        args,
        cwd: project_cwd,
        env: gemini_cli_env_for_home(home),
    });
    cs
}

pub fn plan_gemini_mcp_disable(
    home: &PrismctlHome,
    server_name: &str,
    project_cwd: Option<PathBuf>,
) -> ChangeSet {
    let args = vec![
        "mcp".to_string(),
        "disable".to_string(),
        server_name.to_string(),
    ];
    let mut cs = ChangeSet::new();
    cs.push(Change::RunCommand {
        program: "gemini".to_string(),
        args,
        cwd: project_cwd,
        env: gemini_cli_env_for_home(home),
    });
    cs
}

fn gemini_cli_env_for_home(home: &PrismctlHome) -> Vec<(String, String)> {
    // Same env trick as Claude: redirect `~` resolution.
    let mut envs = Vec::new();
    let home_dir = home.home_dir().to_string_lossy().to_string();
    envs.push(("HOME".to_string(), home_dir.clone()));
    envs.push(("USERPROFILE".to_string(), home_dir));
    envs
}
