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
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init => cmd::init::run(),
        Commands::New { title, priority, no_edit } => {
            cmd::new::run(&title, priority.as_deref(), no_edit)
        }
        Commands::Ls { status, all } => cmd::list::run(status.as_deref(), all),
        Commands::Show { id } => cmd::show::run(&id),
        Commands::Start { id } => cmd::start::run(&id),
        Commands::Done { id } => cmd::done::run(&id),
        Commands::Mv { id, status } => cmd::mv::run(&id, &status),
        Commands::Edit { id } => cmd::edit::run(&id),
        Commands::Rm { id } => cmd::rm::run(&id),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
