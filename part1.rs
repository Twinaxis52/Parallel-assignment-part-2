use std::thread;
use std::sync::{Arc,Mutex};
/*Part 1 of the assignment Kariel Sanchez Ruiz */
fn main() {
    let guest = 100; //initialize count to 0
    let mut handles = vec![]; //mutable vector store all of our threads
    let cupcake = Arc::new(Mutex::new(true));
    let count = Arc::new(Mutex::new(0)); //initialize count to 0
    let guest_ate = Arc::new(Mutex::new(vec![false; guest])); //makes a boolean list of size of guest and all values will be false

    //the threads will run at the same time so it is random as it wont go in order
    for index in 0..guest{ //this for loop decides how many threads for example 0..8 gives us 8 threads
        let count = Arc::clone(&count); //reference to count to use in threads
        let cupcake = Arc::clone(&cupcake);
        let guest_ate = Arc::clone(&guest_ate);
        

        let h = thread::spawn(move||{ 
            let mut count = count.lock().unwrap(); //locking variables so they are used in each thread respectively
            let mut cupcake = cupcake.lock().unwrap();
            let mut guest_ate = guest_ate.lock().unwrap();
                let tracker = *count; //tracks the count before it increases

                if *cupcake == false{ //if the cupcake has been eaten the guest will request one
                    *cupcake = true
                }

                *count = travel(index, *cupcake, *count, &guest_ate); 

                //println!("count {} and tracker, {}", *count, tracker);

                if tracker < *count{ //if count increases a guest has eaten the cupcake so the guest ate index should be true and cupcake will be false since it has been eaten
                    guest_ate[index] = true;
                    *cupcake = false
                }
            
        });
        handles.push(h);
    }
    for h in handles.into_iter(){ //join all the threads before main thread to let them run before the main thread
        //println!("Got: {:?}", h);
        h.join().unwrap();
    }

    println!("count: {}", *count.lock().unwrap()); //prints how many times a cupcake was eaten
    println!("guest_list: {:?}", *guest_ate.lock().unwrap()); //shows us if all guest ate a cupcake
    
    if *count.lock().unwrap() == guest as i32{ // if count is same as guest all guest ate a cupcake to let the minotaur know
        println!("All guest ate a cupcake");
    }

}

fn travel(index: usize, cupcake: bool, mut count: i32, ate: &Vec<bool>)->i32{
    if ate[index] == false && cupcake == true{ //if guest hasn't eaten a cupcake and the cupcake is there raise the count since he will eat the cupcake
        count = count + 1;
        return count
    }
    return count
}