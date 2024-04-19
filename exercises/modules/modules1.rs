// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.

mod sausage_factory {
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        let secret_ingredient = get_secret_recipe();
        println!("Making a sausage with secret ingredient: {}!", secret_ingredient);
        println!("Sausage made!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}