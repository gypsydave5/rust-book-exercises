mod hm;
use hm::scores;

fn main() {
    for (k, v) in scores().iter() {
        println!("Team {} scored {}", k, v);
    }
}
