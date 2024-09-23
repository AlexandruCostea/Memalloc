mod mem_manager;

use mem_manager::{forget::forget, memorize::memorize};
fn main() {

        let mem1 = memorize(30);
        if !mem1.is_null() {
            println!("Memory allocated at: {:?}", mem1);
        }

        let mem2 = memorize(10);
        if !mem2.is_null() {
            println!("Memory allocated at: {:?}", mem2);
        }

        forget(mem1);

        let mem3 = memorize(10);
        if !mem3.is_null() {
            println!("Memory allocated at: {:?}", mem3);
        }

}
