use crossbeam_channel;
use std::thread;
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
    let mut last_handle = thread::spawn(||{false});
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
       let handle = thread::spawn(
            move || {
                let mut done = false;
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
                                done = true;
                                return done;
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
                done
            }
        );

        if i != last_in_line{
            handles.push(handle);
        }
        else {
            last_handle = handle;
        }


    }

    assert_eq!(last_handle.join().unwrap(),true);

}
