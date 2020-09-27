use rand::Rng;
use std::thread;
static N: usize = 10;

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
                *counter == *total - 1
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
        let mut guest_list: Vec<Guest> = vec![Guest::Secondary{visited: false}; count-1];
        guest_list.push(Guest::Primary{counter:0,total:count as i32});
        guest_list
    }
}

fn start_birthday_party_thread(number_of_guests: usize){

    let mut guest_list = Guest::gen_guest_list(number_of_guests);

    let mut cupcake = true;
    let mut all_have_visited = false;

    let mut rng = rand::thread_rng();

    while !all_have_visited {
        let pick = rng.gen_range(0, number_of_guests);
        let mut guest_pick = guest_list[pick].clone();

        let child = thread::spawn( move|| {
            let res = guest_pick.visit(&mut cupcake);
            (guest_pick, res, cupcake)

        });
        let val = child.join().unwrap();
        guest_list[pick] = val.0;
        all_have_visited = val.1;
        cupcake = val.2;

    };

    println!("{:?}", guest_list);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequential_visit_all(){
        let number_of_guests = N;
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

fn main() {

    start_birthday_party_thread(N);

}
