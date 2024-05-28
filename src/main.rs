use bls_signatures::PrivateKey;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Transaction {
    from: String,
    to: String,
    amount: i128,
}

fn main() {
    let private = PrivateKey::generate(&mut thread_rng());
    let public = private.public_key();
    // println!("{private:?}, {public:?}");
    
    // let message = "Invest in Intel";

    let message = Transaction {
        amount: 100,
        from: String::from("Leon"),
        to: String::from("You"),
    };

    let serialize_message = serde_json::to_string(&message).unwrap();

    let sig = private.sign(&serialize_message);

    let verification = public.verify(sig, &serialize_message);
    println!("Is the message authentic? {}", verification);
}
