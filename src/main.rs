mod liquidator;
use crate::liquidator::liquidator_contract;

fn main() {
    println!("Hello, world!");
    start_bot();
    update_positions();
    watch_positions();
    liquidator_contract::liquidate_position();
}

fn start_bot() {
    println!("Bot is starting up");
    liquidator_contract::create_contract();
}

fn update_positions() {
    println!("Found old open positions");
}

fn watch_positions() {
    println!("Starting process to watch incoming positions");
}
