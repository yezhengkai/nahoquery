use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get available variables
    GetVars {
        start_time: String,
        end_time: String,
        user: Option<String>,
    },
    /// Download data
    Download {
        start_time: String,
        end_time: String,
        data_vars: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::GetVars {
            start_time,
            end_time,
            user,
        } => {
            let mut _user = String::from("");
            match user {
                Some(u) => _user.push_str(u),
                None => println!("use default user name"),
            }

            let result = nahoquery::get_vars(&start_time, &end_time, &_user).await;
            match result {
                Ok(content) => {
                    println!("{}", content);
                }
                Err(error) => println!("{}", error),
            }
        }
        Commands::Download {
            start_time,
            end_time,
            data_vars,
        } => {
            // https://stackoverflow.com/questions/33216514/how-do-i-convert-a-vecstring-to-vecstr
            // https://stackoverflow.com/questions/57223508/how-to-accept-both-vecstring-and-vecstr-as-function-arg-in-rust
            let _data_vars: Vec<&str> = data_vars.iter().map(|s| &**s).collect();

            let result = nahoquery::download(&start_time, &end_time, _data_vars).await;
            match result {
                Ok(content) => {
                    println!("{}", content);
                }
                Err(error) => println!("{}", error),
            }
        }
    }
    Ok(())
}
