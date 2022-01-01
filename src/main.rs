use blockchainlib::*;
fn main() {
    let mut block = Block::new(
        0,
        now(),
        vec![0; 32],
        0,
        vec![0; 32],
        "Genesis Block".to_owned(),
        0x00ffffffffffffffffffffffffffffff,
    );
    block.hash = block.hash();
    println!("{:?}", &block);

    block.mine();
    println!("{:?}", &block);
}
