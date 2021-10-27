use stateright::*;

mod double;
mod double_lock;
mod n;
mod single;

fn main() {
    let address = "localhost:3000";
    println!("Serving on http://{}", address);

    n::State::new(5).checker().serve(address);
}
