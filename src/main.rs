
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
    pub mod hybrid_distributor;
    pub mod restore;
    pub mod set_name;
    pub mod set_diff;
    pub mod current;
    pub mod create_branch;
    pub mod branches;
    pub mod set_branch;
    pub mod set_commit;
}

mod tools {
    pub mod hash_object;
    pub mod stage_to_tree;
    pub mod normalize_format;
    pub mod decoding;
    pub mod read_file;
    pub mod config_tools;
}

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Init,
    HashObject {
        #[clap(short = 'w')]        write: bool,
                                    file: String,
    },
    CatFile {
        #[clap(short = 'p')]        pretty_print: bool,
                                    object_hash: String,
    },
    WriteTree {
        #[clap(short = 'w')]        write: bool,
    },
    LsTree {
                                    object_hash: String,
    },
    CommitTree {
        #[clap(short = 'w')]        write: bool,
                                    tree_hash: String,
        #[clap(short = 'p')]        parent_hash: Option<String>,
        #[clap(short = 'm')]        message: String,
    },
    Diff {
                                    file1: String,
                                    file2: String,
    },
    Stage {
        #[clap(short = 'f')]        full: bool,
        #[clap(short = 't')]        time: bool,
    },
    InnerDiff {
                                    file1: String,
                                    file2: String,
    },
    Commit {
        #[clap(short = 'm')]        message: Option<String>,
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
    HybridDistributor {
                                    accumulated_diff_size: u64,
                                    file_path: String,
                                    object_hash: String,
    },
    Restore {
                                    file_hash: String,
        #[clap(short = 'n')]        name: Option<String>,
    },
    SetName {
                                    name: String,
    },
    SetDiff {
        #[clap(short = 'p')]        diff_size_p: Option<u64>,
        #[clap(short = 's')]        max_size: Option<u64>,
        #[clap(short = 'P')]        max_size_p: Option<u64>,
    },
    Current,
    CreateBranch {
                                    branch_name: String,
                                    commit_hash: Option<String>,
    },
    Branches {
        #[clap(short = 'c')]        commits: bool,
    },
    SetBranch {
                                    branch_name: String,
    },
    SetCommit {
                                    commit_hash: String,
    }
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
        Command::Stage { full, time } => {
            commands::stage::run(full, time);
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
        Command::HybridDistributor { accumulated_diff_size, file_path , object_hash } => {
            commands::hybrid_distributor::run(accumulated_diff_size, &file_path, &object_hash);
        }
        Command::Restore { file_hash, name } => {
            commands::restore::run(file_hash, name);
        }
        Command::SetName { name } => {
            commands::set_name::run(name);
        }
        Command::SetDiff { diff_size_p, max_size, max_size_p } => {
            commands::set_diff::run(diff_size_p, max_size, max_size_p);
        }
        Command::Current => {
            commands::current::run();
        }
        Command::CreateBranch { branch_name, commit_hash } => {
            commands::create_branch::run(&branch_name, commit_hash);
        }
        Command::Branches { commits }=> {
            commands::branches::run(commits);
        }
        Command::SetBranch { branch_name } => {
            commands::set_branch::run(&branch_name);
        }
        Command::SetCommit { commit_hash } => {
            commands::set_commit::run(&commit_hash);
        }
    }

}
