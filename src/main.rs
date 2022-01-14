use dexter_coin_lib::*;
fn main() {
    let block = block::Block::init(0, 0, vec![0; 32], 0, "Genesis block".to_string());

    println!("{:?}", block);
}
