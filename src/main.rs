// The main entry point of your application. 
// This is where you initialize and run your liquidator bot.
mod liquidator;
mod api;
mod models;
mod utils;


fn main() {
    println!("Hello, world!");
    start_bot();
    update_positions();
    watch_positions();
    liquidator::liquidate_position();
}

fn start_bot() {
    println!("Bot is starting up");
    liquidator::create_contract();
}

fn update_positions() {
    println!("Found old open positions");
}

fn watch_positions() {
    println!("Starting process to watch incoming positions");
}
