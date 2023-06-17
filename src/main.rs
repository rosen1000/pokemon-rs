use term_inquiry;
pub mod pokemon;

#[allow(unreachable_code)]
fn main() {
    println!("Hello, world!");

    let rng = fastrand::u32(..);
    // println!("{:032b}", rng);
    // println!("{}, {}", (rng << 24) >> 24, rng % 256);
    let string = "2D 31 31 2D 41 41 0C 03 2D 40 00 01 00 00 00 00 1F 14 46 03 01 07 41 00 00 03 00 00";
    // println!("{:?}", string.split(" ").collect::<Vec<&str>>().len());
    // println!("{}", u8::MAX);

    // let file = File::open("poke.mon");
    let mut data: [u8; 28] = [0; 28];
    let mut i = 0;
    let string_array = string.split(" ");
    for line in string_array {
        data[i] = u8::from_str_radix(line.trim(), 16).unwrap();
        i+=1;
        // println!("{}", i);
        // println!("{}", u8::from_str_radix(line.trim(), 16).unwrap());
    }
    // println!("{}", u8::from_str_radix("2D", 16).unwrap());
    // println!("{:?}", data);

    let poke = pokemon::Pokemon::new(rng, data);// {personality_value: rng, data: data};
    println!("{:?}", poke);

    let ans = term_inquiry::CheckboxList::<&'static str>::new("Select option".to_string())
        .add_item("option 1", "value 1")
        .add_item("option 2", "value 2")
        .inquire();
    // println!("{:?}", ans);

    panic!("STOP");

    std::thread::sleep(std::time::Duration::from_millis(100));

    let cyan = console::Style::new().cyan();
    println!("{} world!", cyan.apply_to("Hello"));

    println!("Hello {}!", console::style("world").cyan());

    use console::Emoji;
    println!("[3/4] {}Downloading ...", Emoji("🚚 ", ""));
    println!("[4/4] {} Done!", Emoji("✨", ":-)"));

    let term = console::Term::stdout();
    term.write_str("lmao").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(1000));
    term.clear_line().unwrap();
    // let a = format!("{}");
    // term.write(a.as_bytes());
}
