use std::{env};
use chrono::{NaiveDate};
use clap::Parser;

use dotenv::dotenv;
use fake::{faker::name::{zh_tw::Name}, Fake};
use rand::Rng;
use vprikol::{Prikol, PrikolMember};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    server_id: i32,
    #[arg(short, long)]
    fraction_id: i32,
    
    #[arg(long)]
    test: bool,

    #[arg(long)]
    test_date: Option<String>,

    #[arg(long)]
    test_count: Option<i32>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args = Args::parse();
    
    let token = env::var("PRIKOL_TOKEN").unwrap();
    let prikol = Prikol::new(token.to_string());

    let members: Vec<PrikolMember>;
    let date: NaiveDate;
    
    match args.test {
        false => {
            members = prikol.members(args.server_id, args.fraction_id).await.unwrap();
            date = chrono::Local::now().naive_local().date();
        },
        true => {
            members = test_mode(args.test_count).unwrap();
            date = NaiveDate::parse_from_str(args.test_date.unwrap().as_str(), "%Y-%m-%d").unwrap();
        }
    }

    println!("{:?}", date.format("%d.%m.%Y").to_string());

    for member in members {
        println!("{:?}", member);
    }
}

fn test_mode(count: Option<i32>) -> anyhow::Result<Vec<PrikolMember>> {
    if count.is_none() {
        return Err(anyhow::anyhow!("No count"));
    }

    let mut rng = rand::thread_rng();
    
    let mut members = Vec::new();

    for _ in 0..count.unwrap() {
        let name = Name().fake();

        members.push(PrikolMember {
            id: rng.gen_range(0..100000),
            name,
            rank: rng.gen_range(0..10),
            is_online: rng.gen_bool(0.5),
            is_leader: false,
            rank_label: "Test Rank".to_string(), 
        })
    }
    
    Ok(members)
}