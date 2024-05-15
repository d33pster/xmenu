use xmenu::Xmenu;

fn main() {
    let mut xm = Xmenu::new();
    xm.add("https");
    xm.add("ssh");
    let result = xm.run();
    println!("{}", result);
}
