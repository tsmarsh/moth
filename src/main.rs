use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};
use moth::cmd;
use std::io;
use std::process;

#[derive(Parser)]
#[command(name = "moth")]
#[command(about = "A simple file-based issue tracker", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// List all story IDs (for completion)
    #[arg(long, hide = true)]
    list_ids: bool,

    /// List all status names (for completion)
    #[arg(long, hide = true)]
    list_statuses: bool,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Initialize .moth/ directory")]
    Init,

    #[command(about = "Create a new issue")]
    New {
        #[arg(help = "Issue title")]
        title: String,

        #[arg(short, long, help = "Severity (crit, high, med, low)")]
        severity: Option<String>,

        #[arg(long, help = "Skip opening editor")]
        no_edit: bool,

        #[arg(long, help = "Start the issue immediately (move to 'doing' status)")]
        start: bool,
    },

    #[command(about = "List issues")]
    Ls {
        #[arg(short = 't', long, help = "Filter by status")]
        status: Option<String>,

        #[arg(short, long, help = "Show all including done")]
        all: bool,

        #[arg(short = 's', long, help = "Filter by severity (crit, high, med, low)")]
        severity: Option<String>,
    },

    #[command(about = "Show issue details")]
    Show {
        #[arg(help = "Issue ID (full or partial)")]
        id: Option<String>,
    },

    #[command(about = "Move issue to 'doing' status")]
    Start {
        #[arg(help = "Issue ID (full or partial)")]
        id: String,
    },

    #[command(about = "Move issue to 'done' status")]
    Done {
        #[arg(help = "Issue ID (full or partial)")]
        id: Option<String>,
    },

    #[command(about = "Move issue to specific status")]
    Mv {
        #[arg(help = "Issue ID (full or partial)")]
        id: String,

        #[arg(help = "Target status")]
        status: String,
    },

    #[command(about = "Edit issue in configured editor")]
    Edit {
        #[arg(help = "Issue ID (full or partial)")]
        id: String,
    },

    #[command(about = "Delete an issue")]
    Rm {
        #[arg(help = "Issue ID (full or partial)")]
        id: String,
    },

    #[command(about = "Extract story change history from git commits as CSV")]
    Report {
        #[arg(long, help = "Start from this commit (optional)")]
        since: Option<String>,

        #[arg(long, help = "End at this commit (optional)")]
        until: Option<String>,
    },

    #[command(about = "Set priority order for a story")]
    Priority {
        #[arg(help = "Issue ID (full or partial)")]
        id: String,

        #[arg(help = "Position: top, bottom, above, below, or number")]
        position: String,

        #[arg(help = "Other issue ID (required for above/below)")]
        other_id: Option<String>,

        #[arg(long, help = "Compact after repositioning")]
        compact: bool,

        #[arg(long, help = "Don't compact after repositioning")]
        no_compact: bool,
    },

    #[command(about = "Compact priority numbering in status")]
    Compact {
        #[arg(help = "Status to compact (optional, defaults to all prioritized)")]
        status: Option<String>,
    },

    #[command(about = "Change issue severity")]
    Severity {
        #[arg(help = "Issue ID (full or partial)")]
        id: String,

        #[arg(help = "Severity level (crit, high, med, low)")]
        level: String,
    },

    #[command(about = "Manage git commit hooks")]
    Hook {
        #[command(subcommand)]
        command: HookCommands,
    },

    #[command(about = "Generate shell completions")]
    Completions {
        #[arg(help = "Shell type: bash, zsh, or fish")]
        shell: String,
    },
}

#[derive(Subcommand)]
enum HookCommands {
    #[command(about = "Install prepare-commit-msg hook")]
    Install {
        #[arg(long, help = "Overwrite existing hook")]
        force: bool,

        #[arg(long, help = "Append to existing hook")]
        append: bool,
    },

    #[command(about = "Uninstall prepare-commit-msg hook")]
    Uninstall,
}

fn main() {
    let cli = Cli::parse();

    // Handle hidden completion helper flags
    if cli.list_ids {
        list_story_ids();
        return;
    }

    if cli.list_statuses {
        list_statuses();
        return;
    }

    let Some(command) = cli.command else {
        eprintln!("Error: No command specified. Use --help for usage information.");
        process::exit(1);
    };

    let result = match command {
        Commands::Init => cmd::init::run(),
        Commands::New {
            title,
            severity,
            no_edit,
            start,
        } => cmd::new::run(&title, severity.as_deref(), no_edit, start),
        Commands::Ls {
            status,
            all,
            severity,
        } => {
            let sev_filter = severity
                .as_deref()
                .map(|s| s.parse())
                .transpose()
                .unwrap_or_else(|e| {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                });
            cmd::list::run(status.as_deref(), all, sev_filter)
        }
        Commands::Show { id } => cmd::show::run(id.as_deref()),
        Commands::Start { id } => cmd::start::run(&id),
        Commands::Done { id } => cmd::done::run(id.as_deref()),
        Commands::Mv { id, status } => cmd::mv::run(&id, &status),
        Commands::Edit { id } => cmd::edit::run(&id),
        Commands::Rm { id } => cmd::rm::run(&id),
        Commands::Report { since, until } => cmd::report::run(since.as_deref(), until.as_deref()),
        Commands::Priority {
            id,
            position,
            other_id,
            compact,
            no_compact,
        } => {
            let compact_opt = if compact {
                Some(true)
            } else if no_compact {
                Some(false)
            } else {
                None
            };
            cmd::priority::run(&id, &position, other_id.as_deref(), compact_opt)
        }
        Commands::Compact { status } => cmd::priority::compact(status.as_deref()),
        Commands::Severity { id, level } => {
            let sev = level.parse().unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                process::exit(1);
            });
            cmd::severity::run(&id, sev)
        }
        Commands::Hook { command } => match command {
            HookCommands::Install { force, append } => cmd::hook::install(force, append),
            HookCommands::Uninstall => cmd::hook::uninstall(),
        },
        Commands::Completions { shell } => {
            generate_completions(&shell);
            return;
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn list_story_ids() {
    use std::fs;

    // Fast path: scan filesystem without full validation
    let Ok(mut current) = std::env::current_dir() else {
        return;
    };

    // Find .moth directory
    let moth_dir = loop {
        let moth = current.join(".moth");
        if moth.is_dir() {
            break moth;
        }
        if !current.pop() {
            return; // No .moth found, silently exit
        }
    };

    // Scan all status directories
    let Ok(entries) = fs::read_dir(&moth_dir) else {
        return;
    };

    let mut ids = Vec::new();
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if !path.is_dir() || path.file_name().unwrap().to_str().unwrap().starts_with('.') {
            continue;
        }

        // Scan files in this status directory
        let Ok(files) = fs::read_dir(&path) else {
            continue;
        };

        for file in files.filter_map(Result::ok) {
            let filename = file.file_name();
            let name = filename.to_string_lossy();
            if !name.ends_with(".md") {
                continue;
            }

            // Extract ID from filename: [NNN-]ID-priority-slug.md
            let parts: Vec<&str> = name.trim_end_matches(".md").split('-').collect();
            if parts.len() < 3 {
                continue;
            }

            // Check if first part is a number (priority order)
            let id_idx = if parts[0].parse::<u32>().is_ok() {
                1
            } else {
                0
            };
            if parts.len() > id_idx {
                ids.push(parts[id_idx].to_string());
            }
        }
    }

    // Print unique IDs
    ids.sort();
    ids.dedup();
    for id in ids {
        println!("{}", id);
    }
}

fn list_statuses() {
    use moth::config::Config;

    // Try to load config, fail silently if not available
    let Ok(config) = Config::load() else {
        return;
    };

    for status in &config.statuses {
        println!("{}", status.name);
    }
}

fn generate_completions(shell_name: &str) {
    let shell = match shell_name.to_lowercase().as_str() {
        "bash" => Shell::Bash,
        "zsh" => Shell::Zsh,
        "fish" => Shell::Fish,
        _ => {
            eprintln!(
                "Error: Unknown shell '{}'. Supported: bash, zsh, fish",
                shell_name
            );
            process::exit(1);
        }
    };

    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "moth", &mut io::stdout());
}
