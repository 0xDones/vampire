use clap::Parser;
use colored::Colorize;
use dotenvy;
use std::{io, path::Path};

mod ssm;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "FILE", required = true)]
    file: String,

    #[arg(short, long, value_name = "PREFIX", default_value = "")]
    prefix: String,

    #[arg(short, long, value_name = "REGION", required = true)]
    region: String,

    #[arg(short, long)]
    overwrite: bool,
}

#[tokio::main]
async fn main() {
    println!("Starting...");
    let args = Args::parse();

    println!("File: {}", args.file.white().bold());
    println!("Prefix: {}", args.prefix.white().bold());
    println!("Region: {}", args.region.white().bold());
    println!("Overwrite: {}", args.overwrite);

    let envfile_path = Path::new(&args.file);
    println!("The following variables will be created:");
    for item in dotenvy::from_path_iter(envfile_path).unwrap() {
        let (key, _) = item.unwrap();
        let s = format!("{}{}=***", args.prefix, key);
        println!("{}", s.green().bold());
    }

    println!("{}", "Would you like to proceed? (n/y)".yellow());

    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm).unwrap();

    if confirm.trim_end().ne("y") {
        println!("Exiting program.");
        std::process::exit(0);
    }

    let ssm_service = ssm::SSMService::new(args.region).await;
    for item in dotenvy::from_path_iter(envfile_path).unwrap() {
        let (key, val) = item.unwrap();
        let mut name = args.prefix.to_owned();
        name.push_str(&key);

        match ssm_service.put_parameter(&name, &val, args.overwrite).await {
            Err(err) => {
                println!("[error] parameter not created | name: {}", &name);
                println!("[error] {}", err);
            }
            _ => {}
        };
    }
}
