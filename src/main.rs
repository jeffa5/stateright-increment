use stateright::*;

mod double;
mod double_lock;
mod single;

fn main() {
    let address = "localhost:3000";
    println!("Serving on http://{}", address);
    double_lock::State::default().checker().serve(address);
}
