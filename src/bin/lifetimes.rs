fn do_something_weird <'a> (v1 : &'a str, num : i32, v2 : &'a str) -> &'a str {
       if num == 1 {
           v2
       } else {
           v1
    }
}

fn main() {
    let my_v1 = "some string "; //my_v1 starts living here
    let weird; //starts living here

    {
        let my_v2 = "something other string"; //my_v2 starts living here
        weird = do_something_weird(&my_v1, 3, my_v2);
    } //my_v2 dies here
}
