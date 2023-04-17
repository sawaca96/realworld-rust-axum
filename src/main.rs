mod application;
mod domain;
mod infrastructure;
mod schema;

#[cfg(test)]
pub mod test_utils;
#[cfg(test)]
mod tests {
    mod model_tests;
    mod repository_tests;
}

fn main() {
    println!("Hello, world!");
}
