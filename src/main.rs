
use clap::{Parser, Subcommand};

mod commands {
    pub mod init;
    pub mod hash_object;
    pub mod cat_file;
    pub mod write_tree;
    pub mod ls_tree;
    pub mod commit_tree;
    pub mod diff;
}

mod tools {
    pub mod hash_object;
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
    Diff{},
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
        Command::Diff{} => {
            commands::diff::run();
        }
    }

}
