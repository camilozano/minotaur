static N: usize = 10;

mod cupcake;
mod vase;

fn main() {

   println!("Running Problem 1: Minataur Cupcake\n");
   cupcake::start_birthday_party_thread(N);

   println!("\nRunning Problem 2: Minataur Vase\n");
   vase::run(N);


}
