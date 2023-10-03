use fpt::lr35902::LR35902;

fn main() {
    let mut lr = LR35902::new();

    loop {
        lr.step();
    }
}
