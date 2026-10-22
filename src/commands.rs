use clap::{Args, Parser, Subcommand, ValueEnum};
use url::Url;

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


fn parse_target(target: &str) -> Result<Url, url::ParseError> {
    let input = if target.contains("://") {
        target.to_owned()
    } else {
        format!("http://{target}")
    };

    let url = Url::parse(&input)?;

    if matches!(url.scheme(), "http" | "https"){
        Ok(url)
    } else {
        Err(url::ParseError::RelativeUrlWithoutBase)
    }
}

impl Cli {
    pub fn execute(&self) {
        match &self.command {
            Command::Scan(args) => {
                let target_url = match parse_target(&args.target) {
                    Ok(url) => url,
                    Err(error) => {println!("Error while parsing: {error}.");return}
                };
                println!("Scan, sanitized target: {}", target_url.as_str());
                match args.mode {
                    ScanMode::Files => {
                        println!("Files scan");
                    }
                    ScanMode::Dirs => {
                        println!("Dirs scan");
                    }
                    ScanMode::Subs => {
                        println!("Subs scan");
                    }
                    ScanMode::Nmap => {
                        println!("Nmap scan");
                    }
                    ScanMode::Endpoint => {
                        println!("Endpoint scan");
                    }
                    ScanMode::Recon => {
                        println!("Recon scan");
                    }
                };
            },

            Command::Show(args) => {
                let target_url = match parse_target(&args.target) {
                    Ok(url) => url,
                    Err(error) => {println!("Error while parsing: {error}.");return}
                };
                println!("Show, sanitized target : {}", target_url);
                match args.mode {
                    DisplayMode::All => {
                        println!("Show all");
                    }
                    DisplayMode::Files => {
                        println!("Show files");
                    }
                    DisplayMode::Dirs => {
                        println!("Show dirs");
                    }
                    DisplayMode::Subs => {
                        println!("Show subs");
                    }
                    DisplayMode::Nmap => {
                        println!("Show nmap");
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

