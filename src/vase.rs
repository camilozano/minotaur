use crossbeam_channel;
use std::{thread,time};
use rand::Rng;


fn generate_guest_queue(number_of_guests: usize) -> Vec<usize>{
    let mut rng = rand::thread_rng();
    let mut guest = Vec::with_capacity(number_of_guests);
    for _ in 0..number_of_guests{
        guest.push(rng.gen_range(0, number_of_guests))
    }
    guest
}

pub fn run(){

    static N: usize = 10;
    assert!(N>0);

    let mut guest_queue = generate_guest_queue(N);

    let mut channel_list = Vec::new();
    let mut handles = vec![];
    for _ in 0..N{
       channel_list.push(crossbeam_channel::unbounded()); 
    }

    let first_pick = guest_queue.pop().unwrap();
    let last_in_line = guest_queue[0];
    channel_list[first_pick].0.send(guest_queue).unwrap();
    
    for i in 0..N{
        let recv = channel_list[i].1.clone();
        let send_list = {
            let mut x = Vec::with_capacity(N);
            for j in 0..N{
                x.push(channel_list[j].0.clone())
            }
            x 
        };
       handles.push(thread::spawn(
            move || {
                loop {
                    match recv.try_recv(){
                        Ok(guest_list) => {
                            let mut guest_list = guest_list;
                            if !guest_list.is_empty(){
                                let next_pick = guest_list.pop().unwrap();
                                println!("Thread: {}\tNext: {}\tList: {:?} ", i, next_pick,guest_list);
                                send_list[next_pick].send(guest_list).unwrap();
                            }
                            else{
                                println!("Done");
                                break;
                            }
                        }
                        Err(err) => {
                            if err == crossbeam_channel::TryRecvError::Disconnected{
                                println!("{}",err);
                                break;
                            }
                        }
                    }
                }
            }
        ));
    }
    // thread::sleep(time::Duration::from_secs(5));

    // for i in channel_list{
    //     let s = i.0;
    //     let r = i.1;
    //     drop(s);
    //     drop(r);
    //     // println!("Drop");
    // }
    for j in handles{
        j.join().unwrap();
        println!("Drop!");
    }




}
