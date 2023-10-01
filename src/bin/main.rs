use fpt::lr35902::LR35902;

fn main() {
    let mut lr35902 = LR35902::default();

    loop {
        lr35902.step();
    }
}
