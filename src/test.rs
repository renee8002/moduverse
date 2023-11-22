
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `test::private_function()`");
}

pub fn indirect_access() {
    print!("called `test::indirect_access()`, that\n> ");

    private_function();
}