use clap::Clap;

#[derive(Clap)]
pub struct UserSubCmd {
    #[clap(subcommand)]
    subcmd: UserSubCmds,
}

#[derive(Clap)]
enum UserSubCmds {
    Name(UserNameSubCmd),
}

#[derive(Clap)]
struct UserNameSubCmd {
    #[clap(short, long)]
    set: Option<String>,
}

fn handle_username_subcmd(u: UserNameSubCmd) {
    match u.set {
        Some(n) => {
            println!("Set user name: {}", n);
        }
        None => {
            println!("Show current name");
        }
    }
}

pub fn handle_user_subcmd(u: UserSubCmd) {
    match u.subcmd {
        UserSubCmds::Name(un) => {
            handle_username_subcmd(un);
        }
    }
}
