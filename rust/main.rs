#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;

mod blockchain;

use anyhow::Result;
use std::io;
use std::io::Write;
use std::process;
use wtools as wt;

use blockchain::system::*;

//

fn main() {
    let mut sys = System::Make();
    /*
      issue : https://github.com/Learn-Together-Pro/Blockchain/issues/21
      complexity : mid
      stage : mid
    */
    let mut choice;

    loop {
        println!("");
        println!(".quit => Quit");

        println!(".system.wipe => Clear data from blockchain");
        println!(".system.log => Show system state");
        println!(".system.difficulty => Difficulty set");
        println!(".system.reward => Set reward");

        println!(".transaction.create => Create transaction");

        println!(".block.mine => Mine a block");
        println!(".blocks.log => Show sys state");

        println!(".wallet.create => Create waller");
        println!(".wallet.log => Print information about a wallet");
        println!(".wallets.log => Print information about all wallets");

        choice = wca::input::ask("\nPlease select");

        match choice.to_lowercase().trim() {
            ".quit" => command_exit(),
            ".block.mine" => command_block_mine(&mut sys),
            ".blocks.log" => command_blocks_log(&sys),
            ".transaction.create" => command_transaction_create(&mut sys),
            ".system.wipe" => command_system_wipe(&mut sys),
            ".system.log" => command_system_log(&sys),
            ".system.difficulty" => command_system_difficulty(&mut sys),
            ".system.reward" => command_system_reward(&mut sys),
            ".wallet.create" => command_wallet_create(&mut sys),
            ".wallet.log" => command_wallet_log(&mut sys),
            ".wallets.log" => command_wallets_log(&mut sys),
            command => println!("Unknown command : {}\n", command),
        }
    }
}

//

pub fn ask(request: &str) -> String {
    let mut response = String::new();
    print!("{} : ", request);
    io::stdout().flush().ok();
    io::stdin().read_line(&mut response).ok();
    response.trim().to_string()
}

//

fn command_exit() {
    println!("Exiting..");
    process::exit(0);
}

//

fn command_system_wipe(sys: &mut System) {
    *sys = System::MakePersistant();
}

//

fn command_blocks_log(sys: &System) {
    println!("{:#?}", &sys.chain);
}

//

fn command_system_log(sys: &System) {
    println!("{:#?}", sys);
}

//

fn command_system_difficulty(_sys: &mut System) {
    /*
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/15
    complexity : easy
    stage : early
    */
    let difficulty = wca::input::ask("\nPlease enter difficulty of chain");
    match difficulty.parse::<u32>() {
        Ok(i) => {
          _sys.chain.difficulty = i;
        },
        Err(..) => {
          println!("This was not an integer: {}", difficulty);
          command_system_difficulty(_sys);
        }
    };
}

//

fn command_system_reward(_sys: &mut System) {
    /*
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/14
    complexity : easy
    stage : early
    */
}

//

fn command_transaction_create(sys: &mut System) {
    let sender = wca::input::ask("Sender");
    let receiver = wca::input::ask("Receiver");
    let amount_str = wca::input::ask("Amount");
    let amount: f64 = amount_str.parse().unwrap();
    sys.transaction_create(sender, receiver, amount);
    sys.store();
}

//

fn command_block_mine(sys: &mut System) {
    let block_option = sys.chain.block_mine();
    if let Some(block) = block_option {
        sys.chain.block_add(block);
    }
    sys.store();
}

//

fn command_wallet_create(_sys: &mut System) {
    /*
    Issue : https://github.com/Learn-Together-Pro/Blockchain/issues/17
    complexity : easy
    stage : late
    */
}

//

fn command_wallet_log(_sys: &mut System) {
    /*
    Issue : https://github.com/Learn-Together-Pro/Blockchain/issues/20
    complexity : easy
    stage : late
    */
}

//

fn command_wallets_log(_sys: &mut System) {
    /*
    issue : https://github.com/Learn-Together-Pro/Blockchain/issues/18
    complexity : easy
    stage : early
    */
}
