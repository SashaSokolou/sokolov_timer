use rand::distr::{Alphanumeric, SampleString};
use std::{thread, time::Duration, sync::mpsc};
//use serde::{Serialize, Deserialize};

fn main() {  

/*/    struct CustomMessage {
    id: u128,
    content: String,
}*/

    let thread2 =  thread::spawn(|| {
        for i in 1..31{
            let string = Alphanumeric.sample_string(&mut rand::rng(), 255);
            println!("{i}) Random 255-chars string: {}", string);
            thread::sleep(Duration::from_secs(2));
        }
    });

    thread2.join().unwrap();

}



