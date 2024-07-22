use std::collections::HashMap;
use std::fs;

fn main() {
    // let message = fs::read_to_string("./input_text").expect("Unable to read file");
    let message = fs::read("./input_text").expect("Unable to read file");

    let mut my_map = HashMap::new();

    (0..255).for_each(|i| {
        my_map.insert((char::from(i)).to_string(), i as u32);
    });
    // println!("{:?}", my_map);

    let mut last_entry = 256;
    let mut s = (message[0] as char).to_string();
    // println!("{:?}", s);

    let mut res: Vec<u32> = Vec::default();

    message
        .into_iter()
        .skip(1)
        // .zip(257..4096)
        .for_each(|c| {
            let sc = format!("{}{}", s, c as char);
            // println!("{:?}", sc);

            match my_map.get(&sc) {
                Some(_) => {
                    // println!("{}\t{}\t{}\tY\tx", s, c, sc);
                    s = format!("{}{}", s, c as char);
                }
                None => {
                    res.push(*my_map.get(s.as_str()).expect("Table pre-allocated"));
                    // println!("{}\t{}\t{}\tN\t{}", s, c, sc, my_map.get(s.as_str()).unwrap());
                    last_entry += 1;
                    my_map.insert(sc, last_entry);
                    s = (c as char).to_string();
                }
            };
        });

    res.push(*my_map.get(s.as_str()).expect("Table pre-allocated"));

    let expected_result = vec![
        84, 79, 66, 69, 79, 82, 78, 79, 84, 257, 259, 261, 266, 260, 262, 264,
    ];
    println!("{:?}", res);

    assert_eq!(expected_result, res);
}
