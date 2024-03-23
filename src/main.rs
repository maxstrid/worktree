use std::env;

use worktree::Worktree;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.get(1).unwrap() == "remove" {
        let filename = args.get(2).unwrap();
        let branch = args.get(3);

        Worktree::remove_worktree(filename.to_string(), branch.cloned()).unwrap();
    } else if args.get(1).unwrap() == "gerrit" {
        let command = args.get(2).unwrap();
        let branch = args.get(3).unwrap();

        Worktree::add_from_gerrit(command.to_string(), branch.to_string()).unwrap();
    }
}
