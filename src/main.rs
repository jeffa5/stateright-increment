use stateright::*;

mod double;
mod single;

fn main() {
    let address = "localhost:3000";
    println!("Serving on http://{}", address);
    double::State::default().checker().serve(address);
}
