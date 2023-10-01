use fpt::lr35902::LR35902;

fn main() {
    let mut lr35902 = LR35902::new();

    loop {
        lr35902.step();
    }
}
