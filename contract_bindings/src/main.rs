use ethers::prelude::Abigen;

fn main() {
    println!("Generating contract bindings..");

    Abigen::new("Comptroller", "abi/comptroller.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("contract_bindings/src/comptroller_bindings.rs")
        .unwrap();

    Abigen::new("Unitroller", "abi/unitroller.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("contract_bindings/src/unitroller_bindings.rs")
        .unwrap();

    Abigen::new("Ctoken", "abi/cerc20.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("contract_bindings/src/ctoken_bindings.rs")
        .unwrap();
}
