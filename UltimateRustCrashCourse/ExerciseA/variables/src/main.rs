const STARTING_MISSILES :i32 = 8;
const READY_AMOUNT :i32 = 2;

use variables::greet;

fn main() {

    //let mut missiles :i32 = STARTING_MISSILES;
    //let ready :i32 = READY_AMOUNT;
    let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;

    println!("{} missiles left!", missiles);

    // use allow us to not have to write: variables::greet();
    greet();

}