#[macro_use]
extern crate derive_builder;

mod client;






use clap::{Parser, Subcommand, Args};



#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Enables Proxy Support With Sending Requests to Geometry Dash
    #[arg(long, short)]
    proxy:Option<String>,

    /// allows for connection timeouts to take place...
    #[arg(short, long)]
    timeout: Option<u64>,

    #[command(subcommand)]
    command: Commands,

}



#[derive(Args)]
struct UserCmd
{
    /// Query , accountID or userID or Name of the geometry dash user to lookup
    #[arg(short, long)]
    name: String,
}

#[derive(Args)]
struct LevelInfoCmd {

    /// The Level ID That will be requested for you can use -1 if you want the daily -2 = weelky 
    #[arg()]
    id:i64,

    /// Gets the User alongside the level..
    #[arg(long, short)]
    user_info:bool,
}


#[derive(Args)]
struct DailyInfoCmd {
    /// Gets the User alongside the level..
    #[arg(long, short)]
    user_info:bool,
}



/// gdcli By Calloc, A Geometry Dash Commandline tool

#[derive(Subcommand)]
enum Commands {
    /// Requests for a GD User
    User(UserCmd),

    /// Requests for a Geometry Dash Level
    LevelInfo(LevelInfoCmd),
    /// Requests for a Geometry Dash Daily Level
    DailyInfo(DailyInfoCmd),
}


async fn level_command(id:i64, userinfo:bool, cli:&Cli){
    let level = client::get_level(id,cli.timeout, &cli.proxy).await; 
    level.report_info();
    if userinfo {
        println!("[...] Getting Level Author");
        let user = client::get_user(&level.player_id.to_string(), cli.timeout, &cli.proxy).await.unwrap();
        user.report_info();
    }
}


#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::User(cmd) => {
            let user = client::get_user(&cmd.name, cli.timeout, &cli.proxy).await.unwrap();
            // Share Results (If Any...)
            user.report_info();
        }

        Commands::LevelInfo(cmd) => {
            level_command(cmd.id,cmd.user_info, &cli).await;
        },
        
        Commands::DailyInfo(cmd) => {
            level_command(-1, cmd.user_info, &cli).await;
        }
    
    }
}

