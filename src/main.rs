use clap::{Parser, Subcommand};
use moth::cmd;
use std::process;

#[derive(Parser)]
#[command(name = "moth")]
#[command(about = "A simple file-based issue tracker", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Initialize .moth/ directory")]
    Init,

    #[command(about = "Create a new issue")]
    New {
        #[arg(help = "Issue title")]
        title: String,

        #[arg(short, long, help = "Priority (crit, high, med, low)")]
        priority: Option<String>,

        #[arg(long, help = "Skip opening editor")]
        no_edit: bool,
    },

    #[command(about = "List issues")]
    Ls {
        #[arg(short, long, help = "Filter by status")]
        status: Option<String>,

        #[arg(short, long, help = "Show all including done")]
        all: bool,
    },

    #[command(about = "Show issue details")]
    Show {
        #[arg(help = "Issue ID (full or partial)")]
        id: String,
    },

    #[command(about = "Move issue to 'doing' status")]
    Start {
        #[arg(help = "Issue ID (full or partial)")]
        id: String,
    },

    #[command(about = "Move issue to 'done' status")]
    Done {
        #[arg(help = "Issue ID (full or partial)")]
        id: String,
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

    #[command(about = "Manage git commit hooks")]
    Hook {
        #[command(subcommand)]
        command: HookCommands,
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

    let result = match cli.command {
        Commands::Init => cmd::init::run(),
        Commands::New {
            title,
            priority,
            no_edit,
        } => cmd::new::run(&title, priority.as_deref(), no_edit),
        Commands::Ls { status, all } => cmd::list::run(status.as_deref(), all),
        Commands::Show { id } => cmd::show::run(&id),
        Commands::Start { id } => cmd::start::run(&id),
        Commands::Done { id } => cmd::done::run(&id),
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
        Commands::Hook { command } => match command {
            HookCommands::Install { force, append } => cmd::hook::install(force, append),
            HookCommands::Uninstall => cmd::hook::uninstall(),
        },
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
