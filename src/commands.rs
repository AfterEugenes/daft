use clap::{Args, Parser, Subcommand, ValueEnum};

// define Parser
#[derive(Debug, Parser)]
#[command(
    no_binary_name = true,
    bin_name = ""
)]

// define the cli
pub struct Cli {
    #[command(subcommand)]
    command: Command,
    last_target: String,
}

// base commands
#[derive(Debug, Subcommand)]
enum Command {
    Scan(ScanArgs),
    Show(ShowArgs),

    Set {
        #[command(subcommand)]
        command: SetCommand,
    },
}

// args for scan
#[derive(Debug, Args)]struct ScanArgs {
    target: String,
    mode: ScanMode,
}

#[derive(Debug, Clone, ValueEnum)]
enum ScanMode {
    Files,
    Dirs,
    Subs,
    Nmap,
    Endpoint,
    Recon,
}

// args for show
#[derive(Debug, Args)]
struct ShowArgs {
    target: String,
    mode: DisplayMode,
}

#[derive(Debug, Clone, ValueEnum)]
enum DisplayMode {
    Files,
    Dirs,
    Subs,
    Nmap,
    All,
}

// subcommands for set
#[derive(Debug, Subcommand)]
enum SetCommand {
    New {
        #[command(subcommand)]
        command: SetNewCommand,
    },
}

#[derive(Debug, Subcommand)]
enum SetNewCommand {
    Scan {
        scan: String,
    },
}

fn sanitize_target(&target: String) {
    match &target:

}

impl Cli {
    pub fn execute(&self) {
        match &self.command {
            Command::Scan(args) => {
                println!("Complete target: {}", args.target);
                match args.mode {
                    ScanMode::Files => {
                        println!("Files scan: {}", args.target);
                    }
                    ScanMode::Dirs => {
                        println!("Dirs scan: {}", args.target);
                    }
                    ScanMode::Subs => {
                        println!("Subs scan: {}", args.target);
                    }
                    ScanMode::Nmap => {
                        println!("Nmap scan: {}", args.target);
                    }
                    ScanMode::Endpoint => {
                        println!("Endpoint scan: {}", args.target);
                    }
                    ScanMode::Recon => {
                        println!("Recon scan: {}", args.target);
                    }
                };
            },

            Command::Show(args) => {
                println!("Complete target: {}", args.target);
                match args.mode {
                    DisplayMode::All => {
                        println!("Displaying all: {}", args.target);
                    }
                    DisplayMode::Files => {
                        println!("Displaying Files: {}", args.target);
                    }
                    DisplayMode::Dirs => {
                        println!("Displaying Dirs: {}", args.target);
                    }
                    DisplayMode::Subs => {
                        println!("Displaying Subs: {}", args.target);
                    }
                    DisplayMode::Nmap => {
                        println!("Displaying Nmap: {}", args.target);
                    }
                };
            },

            Command::Set { command } => match command {
                SetCommand::New { command } => match command {
                    SetNewCommand::Scan { scan } => {
                        println!("Creating scan: {scan}");
                    }
                },
            },
        }
    }
}

