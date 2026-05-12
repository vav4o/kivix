
use clap::{Parser, Subcommand};

mod commands {
    pub mod init;
    pub mod hash_object;
    pub mod cat_file;
    pub mod write_tree;
    pub mod ls_tree;
    pub mod commit_tree;
    pub mod inner_diff;
    pub mod stage;
    pub mod diff;
    pub mod commit;
    pub mod add;
    pub mod remove;
    pub mod delete;
    pub mod apply_diff;
    pub mod reverse_diff;
}

mod tools {
    pub mod hash_object;
    pub mod stage_to_tree;
    pub mod normalize_format;
}

/// A simple git-like version control system written in Rust.
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

/// The available subcommands for the git-like system.
#[derive(Debug, Subcommand)]
enum Command {
    /// Initialize a new, empty repository.
    Init,
    HashObject {
        #[clap(short = 'w')]
        write: bool,

        file: String,
    },
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,

        object_hash: String,
    },
    WriteTree {
        #[clap(short = 'w')]
        write: bool,
    },
    LsTree {
        object_hash: String,
    },
    CommitTree {
        #[clap(short = 'w')]
        write: bool,

        tree_hash: String,

        #[clap(short = 'p')]
        parent_hash: Option<String>,

        #[clap(short = 'm')]
        message: String,
    },
    Diff {
        file1: String,
        file2: String,
    },
    Stage,
    InnerDiff {
        file1: String,
        file2: String,
    },
    Commit {
        #[clap(short = 'm')]
        message: String,
    },
    Add {
        file: String,
    },
    Remove {
        file: String,
    },
    Delete {
        file: String,
    },
    ApplyDiff {
        diff_file: String,
        target_file: String,
    },
    ReverseDiff {
        diff_file: String,
    },
}

fn main() { 
    let args = Args::parse();
    match args.command {
        Command::Init => {
            commands::init::run();
        }
        Command::HashObject { write, file } => {
            commands::hash_object::run(write, file);
        }
        Command::CatFile { pretty_print, object_hash } => {
            commands::cat_file::run(pretty_print, object_hash);
        }
        Command::WriteTree { write } => {
            commands::write_tree::run(write);
        }
        Command::LsTree { object_hash } => {
            commands::ls_tree::run(object_hash);
        }
        Command::CommitTree { write, tree_hash, parent_hash, message } => {
            commands::commit_tree::run(write, tree_hash, parent_hash, message);
        }
        Command::Diff{file1, file2} => {
            commands::diff::run(file1, file2);
        }
        Command::Stage => {
            commands::stage::run();
        }
        Command::InnerDiff { file1, file2 } => {
            commands::inner_diff::run(file1, file2);
        }
        Command::Commit { message } => {
            commands::commit::run(message);
        }
        Command::Add { file } => {
            commands::add::run(file);
        }
        Command::Remove { file } => {
            commands::remove::run(file);
        }
        Command::Delete { file } => {
            commands::delete::run(file);
        }
        Command::ApplyDiff { diff_file, target_file } => {
            commands::apply_diff::run(diff_file, target_file);
        }
        Command::ReverseDiff { diff_file } => {
            commands::reverse_diff::run(diff_file);
        }
    }

}
