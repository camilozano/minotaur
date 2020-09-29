
use rand::Rng;
use std::thread;
use multiqueue;

#[derive(Debug, Clone, Copy)]
enum Guest {
    Primary {counter: i32, total: i32},
    Secondary {visited: bool}
}

impl Guest {
    fn visit(&mut self, cupcake: &mut bool) -> bool {
        match self{
            Guest::Primary { counter, total } => {
                if !*cupcake {
                    *cupcake = true;
                    *counter += 1;
                }
                *counter == *total 
            }
            Guest::Secondary { visited } => {
                if !*visited && *cupcake{
                    *cupcake = false;
                    *visited = true;
                }
                false
            }
        }
    }
    fn gen_guest_list(count: usize) -> Vec<Guest>{
        let guest_list: Vec<Guest> = vec![Guest::Secondary{visited: false}; count-1];
        //guest_list.push(Guest::Primary{counter:1,total:count as i32});
        guest_list
    }
}

pub fn start_birthday_party_thread(number_of_guests: usize){

    let mut primary_guest = Guest::Primary{counter:1, total:number_of_guests as i32};
    let primary_guest_idx = number_of_guests-1;
    let guest_list = Guest::gen_guest_list(number_of_guests);

    let mut cupcake = true;
    let mut handles = Vec::with_capacity(number_of_guests);

    let (send,recv) = multiqueue::broadcast_queue(100);
    let (send_done,recv_done) = multiqueue::broadcast_queue(0);


    let primary_recv = recv.clone();
    let primary_send_done = send_done.clone();
    let primary_guest_handle = thread::spawn( move || {
        for msg in primary_recv{
            if msg == primary_guest_idx {
                println!("{}",msg);
                primary_guest.visit(&mut cupcake);
                primary_send_done.try_send(true).unwrap();
            }
        } 
    });

    for i in 0..number_of_guests-1{
        let guest_recv = recv.clone();
        let guest_send_done = send_done.clone();
        handles.push(
            thread::spawn(
                move || {
                    for msg in guest_recv {
                        println!("Here {}", i);
                        if msg == i {
                            println!("{}",msg);
                            primary_guest.visit(&mut cupcake);
                            guest_send_done.try_send(true).unwrap();
                        }
                    }
                }
            )
        );
    }

    let minataur_send = send.clone();
    let minataur_recv_done = recv_done.clone();
    thread::spawn(
        move || {
            for msg in minataur_recv_done{
                if msg {
                    let mut rng = rand::thread_rng();
                    let pick = rng.gen_range(0, number_of_guests);
                    println!("Minataur Send {}", pick);
                    minataur_send.try_send(pick).unwrap();
                }
            }
        }
    );
    send_done.try_send(true).unwrap();
    

    primary_guest_handle.join().unwrap();


    println!("{:?}", guest_list);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequential_visit_all(){
        let number_of_guests = 10;
        let mut guest_list = Guest::gen_guest_list(number_of_guests);
        let mut cupcake = true;
        for i in 0..number_of_guests{
            guest_list[i].visit(&mut cupcake);
            guest_list[number_of_guests-1].visit(&mut cupcake);
        }

        let mut num_visited = 0;
        for g in &guest_list{
            match &g {
                Guest::Primary { counter: _, total: _ } => {num_visited += 1}
                Guest::Secondary { visited } => { if *visited { num_visited +=1 } else {}}
            }
        }
        assert_eq!(num_visited, number_of_guests as i32);
    }
}


