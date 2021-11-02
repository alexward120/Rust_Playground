#![allow(non_snake_case)]

//mod print; //Similar to #include "filename" in c++
mod variables;

fn main() {  //use cargo run to compile an run
    //print::run();  //:: is the scope modifer so you know what file to get a certain function from
    variables::run();
}
