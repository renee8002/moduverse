mod machine;
mod repository;
mod user;
mod test;

fn function() {
    println!("called `function()`");
}
fn main() {
    println!("Hello, world!");
    test::function();
    function();
    test::indirect_access();
    test::nested::function();
}
