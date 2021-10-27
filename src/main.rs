use stateright::*;

mod double;
mod double_lock;
mod n;
mod n_lock;
mod single;

fn main() {
    let address = "localhost:3000";
    println!("Serving on http://{}", address);

    n_lock::State::new(5).checker().serve(address);
}
