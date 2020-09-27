use crossbeam_channel;
use std::thread;

pub fn run(){

    static N: usize = 10;

    let mut channel_list = Vec::new();
    let mut handles = vec![];
    for _ in 0..N{
       channel_list.push(crossbeam_channel::unbounded()); 
    }

    channel_list[0].0.send(true).unwrap();
    
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
                        Ok(sign) => {
                            if sign && i != N-1{
                                send_list[i+1].send(true).unwrap();
                                println!("Thread {}", i);
                            }
                        }
                        Err(err) => {
                            if err == crossbeam_channel::TryRecvError::Disconnected{
                                println!("{}",err);
                            }
                            break;
                        }
                    }
                }
            }
        ));
    }

    for i in channel_list{
        let s = i.0;
        let r = i.1;
        drop(s);
        drop(r);
        // println!("Drop");
    }
    for j in handles{
        j.join().unwrap();
        // println!("Drop!");
    }


}
