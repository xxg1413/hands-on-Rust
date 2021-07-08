fn main() {
   /*
    let MYLIST = [ "One", "Two", "Three" ];
    for i in 0..3 {
        println!("{}", MYLIST[i]);
    }
    */

    let my_list = [ "One", "Two", "Three"];

    for item in &my_list {
        println!("{}", item)
    }
}
