#[macro_use]
extern crate derive_builder;

mod client;
mod tuitools;



use clap::{Parser, Subcommand, Args};
use client::gdenums::Authority;
use tuitools::boxtools::simplebox;



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

#[derive(Args)]
struct WeelkyInfoCmd {
    /// Gets the User alongside the level..
    #[arg(long, short)]
    user_info:bool,
}

#[derive(Args)]
struct LevelCommentsCmd{

    /// The id of the level to get comments from 
    #[arg(long, short)]
    level_id:i64,
    /// The Page to get comments from
    #[arg(long, short)]
    page:Option<u32>,
}   

#[derive(Args)]
struct DailyCommentsCmd{
    /// The Page to Request for comments
    #[arg(long, short)]
    page:Option<u32>,
}

#[derive(Args)]
struct WeeklyCommentsCmd{
    /// The Page to Request for comments
    #[arg(long, short)]
    page:Option<u32>,
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
    /// Requests for a Geometry Dash Weekly Demon
    WeeklyInfo(WeelkyInfoCmd),
    
    /// Requests for level Comments from Geometry Dash
    LevelCommentInfo(LevelCommentsCmd),

    /// Requests for Daily Comments from Geometry Dash
    DailyCommentInfo(DailyCommentsCmd),

    /// Requests for weekly Comments from Geometry Dash
    WeeklyCommentInfo(WeeklyCommentsCmd),
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

async fn level_comments_command(id:i64, page:u32, cli:&Cli){
    let comments = client::get_level_comments(id, page, cli.timeout, &cli.proxy).await;
    for comment in comments{
        match comment.get_authority_level(){
            Authority::Eldermod => println!("{}", simplebox(format!("{} (ELDERMOD)", comment.user.username).as_str(), comment.comment(), 60)),
            Authority::Mod => println!("{}", simplebox(format!("{} (MOD)", comment.user.username).as_str(), comment.comment(), 60)),
            Authority::User => println!("{}", simplebox(format!("{}", comment.user.username).as_str(), comment.comment(), 60)),
        }
    }
}


#[tokio::main]
async fn main() {
    let cli = Cli::parse();

 

    match &cli.command {
        /* Fetching commands... */

        Commands::User(cmd) => {
            println!("{}", simplebox("Loading", "Fetching User Info...".to_string(), 40));
            let user = client::get_user(&cmd.name, cli.timeout, &cli.proxy).await.unwrap();
            // Share Results (If Any...)
            user.report_info();
        },

        Commands::LevelInfo(cmd) => {
            println!("{}", simplebox("Loading", "Fetching Level Info...".to_string(), 40));
            level_command(cmd.id,cmd.user_info, &cli).await;
        },
        
        Commands::DailyInfo(cmd) => {
            println!("{}", simplebox("Loading", "Fetching Daily Level Info...".to_string(), 40));
            level_command(-1, cmd.user_info, &cli).await;
        },

        Commands::WeeklyInfo(cmd) => {
            println!("{}", simplebox("Loading...", "Fetching Weekly Level Info...".to_string(), 40));        
            level_command(-2, cmd.user_info, &cli).await;
        }

        Commands::LevelCommentInfo(cmd) => {
            level_comments_command(cmd.level_id, cmd.page.unwrap_or(0), &cli).await;
        }

        Commands::WeeklyCommentInfo(cmd) => {
            level_comments_command(-2, cmd.page.unwrap_or(0), &cli).await;
        }

        Commands::DailyCommentInfo(cmd) => {
            level_comments_command(-1, cmd.page.unwrap_or(0), &cli).await;
        }

        /* TODO: Logins, Level Comments, Daily-Chat & Etc... */
    }

}

