#[macro_use]
extern crate itertools;

use rand::{seq::SliceRandom, thread_rng, Rng};
use regex::Regex;
use std::{
    collections::HashMap, error::Error, fs::OpenOptions, io::Read, path::PathBuf, str::FromStr,
};
use structopt::StructOpt;
//use rust_twitter_bot_lib::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "markov")]
struct Opt {
    #[structopt(short = "i", long = "input")]
    input: Option<PathBuf>,
    #[structopt(short = "l", long = "length")]
    length: Option<u32>,
}

fn read_file(filename: PathBuf) -> Result<String, Box<dyn Error>> {
    let mut file = OpenOptions::new().read(true).open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn split_words(w: &str) -> Vec<&str> {
    let spaces_re = Regex::new(r" +").unwrap();
    spaces_re.split(w).collect::<Vec<&str>>()
}

fn build_table(words: Vec<&str>) -> HashMap<(&str, &str), Vec<&str>> {
    let mut ret = HashMap::new();
    for (w0, w1, w2) in izip!(&words, &words[1..], &words[2..]) {
        let current = ret.entry((*w0, *w1)).or_insert_with(Vec::new);
        current.push(*w2);
    }
    ret
}

fn run(input: PathBuf, length: u32) -> Result<(), Box<dyn Error>> {
    let file_str = read_file(input)?;
    let words = split_words(&file_str);

    let mut rng = thread_rng();
    let i = rng.gen_range(0, words.len() - 3);

    let mut w0 = words[i];
    let mut w1 = words[i + 1];
    let mut w2 = words[i + 2];
    let mut tweet = String::from("");

    let lookup = build_table(words);

    for _ in 0..length {
        tweet.push_str(" ");
        tweet.push_str(w2);

        w2 = &lookup[&(w0, w1)].choose(&mut rng).unwrap_or(&"NONE");
        w0 = w1;
        w1 = w2;
    }

    //my attempt at making a sentence regex

    let tweet_cleaner = Regex::new(r"[A-Z]+\s+[^.!?]*[.!?]").unwrap();
    let cleaned_tweet = tweet_cleaner.find_iter(&tweet).map(|mat| mat.as_str()).collect::<String>();

    print!("{}", cleaned_tweet);
    //print!("{}", tweet);

    //     let twitter_bot = TwitterBot::new()
    //     .consumer_key(YOUR_CONSUMER_KEY)
    //     .consumer_secret_key(YOUR_CONSUMER_SECRET_KEY)
    //     .access_token(YOUR_ACCESS_TOKEN)
    //     .secret_access_token(YOUR_SECRET_ACCESS_TOKEN);

    //   let res = twitter_bot.tweet("{} ", w2).unwrap();

    //   println!("{:?}", res);

    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    let filename = opt
        .input
        .unwrap_or_else(|| PathBuf::from_str("tweets.txt").unwrap());
    let length = opt.length.unwrap_or(25);

    if let Err(e) = run(filename, length) {
        eprintln!("Error: {}", e);
        ::std::process::exit(1);
    };
}