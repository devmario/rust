pub fn main() {
    let mut x = 0;
    for 4096.times {
        x += 1;
    }
    fail_unless!(x == 4096);
    io::println(fmt!("x = %u", x));
}
