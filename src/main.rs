mod arraylist;
use arraylist::ArrayList;

fn main() {
    let my_list: ArrayList<&str> = ArrayList::default();
    println!("{:?}", my_list);

    
}
