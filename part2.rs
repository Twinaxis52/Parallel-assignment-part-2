use std::thread;
use std::sync::{Arc,Mutex};
/*Part 2 of assignment Kariel Sanchez Ruiz */
fn main() {
    let guest = 100; //initialize guest to 100
    let mut handles = vec![]; //mutable vector store all of our threads
    let Busy = Arc::new(Mutex::new(0)); //initialize busy to 0
    let count = Arc::new(Mutex::new(0)); //initialize count to 0

    for index in 0..guest{ //this for loop decides how many threads for example 0..8 gives us 8 threads
        let mut Busy = Arc::clone(&Busy); //reference to busy to use in threads
        let mut count = Arc::clone(&count);
        

        let h = thread::spawn(move||{ 
            let mut Busy = Busy.lock().unwrap(); //this will lock busy for each thread
            let mut count = count.lock().unwrap();

                if *Busy == 0{ //if room isnt busy u can enter
                    *Busy = 1;   // a person is in room so it is busy
                    println!("Guest {} is in the room. Room is Busy", index);
                    *count = *count +1; //count the guest that have seen the room
                    *Busy = 0; // person is gonna leave room so it is available
                    println!("Guest {} is leaving room. Room is Available", index);
                }
            
        });
        handles.push(h);
    }
    for h in handles.into_iter(){
        //println!("Got: {:?}", h);
        h.join().unwrap();
    }
    println!("{} guest have seen the vase", *count.lock().unwrap());

}
