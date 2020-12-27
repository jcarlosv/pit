//use FrameworksAndDrivers/repositories/patient/FileRepo as repo
//use vim UseCasesAndEntities/patient/usecase/implementation as patientUseCase
//
//fn main() {
//   //INIT MODULES
//   repo = repo.new()
//   usecase = patientUseCase.new()
//
//   Presentation.init();   //RUN APP
//   runApp(MyApp());
//}
//

use clap::Clap;
mod subcommands;
use crate::subcommands::user;

#[derive(Clap)]
#[clap(version = "1.0", author = "The pit Authors")] struct Opts {
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: Subcommands,
}

#[derive(Clap)]
enum Subcommands {
    User(user::UserSubCmd),
}


fn handle_root_subcmds(s: Subcommands) {
    match s {
        Subcommands::User(s) => {
            user::handle_user_subcmd(s)
        }
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.verbose {
        0 => {},
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }
    handle_root_subcmds(opts.subcmd)
}
