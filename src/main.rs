mod mem_manager;

use mem_manager::memalloc::memalloc;
fn main() {

        // Allocate 4096 bytes (one page).
        let mem1 = memalloc(4096);
        match mem1 {
            Ok(new_mem) => println!("Memory allocated at: {:?}", new_mem),
            Err(error) => println!("{:?}", error)
        }

        let mem2 = memalloc(10);
        match mem2 {
            Ok(new_mem) => println!("Memory allocated at: {:?}", new_mem),
            Err(error) => println!("{:?}", error)
        }
}
