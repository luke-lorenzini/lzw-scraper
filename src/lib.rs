use std::collections::HashMap;

pub struct LZW {
    // compression_map: HashMap<String, u32>,
    // decompression_map: HashMap<u32, String>,
}

impl Default for LZW {
    fn default() -> Self {
        // let mut compression_map = HashMap::new();
        // let mut decompression_map = HashMap::new();

        // (0..255).for_each(|i| {
        //     compression_map.insert((char::from(i)).to_string(), i as u32);
        //     decompression_map.insert(i as u32, (char::from(i)).to_string());
        // });

        LZW {
            // compression_map,
            // decompression_map,
        }
    }
}

impl LZW {
    pub fn new(
        // compression_map: HashMap<String, u32>,
        // decompression_map: HashMap<u32, String>,
    ) -> Self {
        LZW {
            // compression_map,
            // decompression_map,
        }
    }

    pub fn decompress(&mut self, message: Vec<u32>, decompression_map: &mut HashMap<u32, String>) -> String {
        let mut last_entry = decompression_map.len() as u32;
        let mut ocode = message[0];
        let mut s = String::default();
        let mut res: String = format!("{}", char::from_u32(ocode).unwrap());

        let mut c = char::default();

        message
        .into_iter()
        .skip(1)
        .for_each(|ncode| {
            match decompression_map.get(&ncode) {
                Some(v) => {
                    s = v.to_string();
                }
                None => {
                    s = char::from_u32(ocode).unwrap().to_string();
                    s = format!("{}{}", s, c);
                }
            };
            res = format!("{}{}", res, s);
            c = s.chars().nth(0).unwrap();
            last_entry += 1;
            decompression_map.insert(
                last_entry,
                format!(
                    "{}{}",
                    decompression_map.get(&ocode).unwrap(),
                    c.to_owned()
                ),
            );
            ocode = ncode;
        });

        res
    }

    pub fn compress(&mut self, message: Vec<u8>, compression_map: &mut HashMap<String, u32>) -> Vec<u32> {
        let mut last_entry = compression_map.len() as u32;
        let mut s = (message[0] as char).to_string();
        let mut res: Vec<u32> = Vec::default();

        message.into_iter().skip(1).for_each(|c| {
            let sc = format!("{}{}", s, c as char);
            // let sc = s.to_owned() + &(c as char).to_string();

            match compression_map.get(&sc) {
                Some(_) => {
                    s = format!("{}{}", s, c as char);
                    // s = s.to_owned() + &(c as char).to_string();
                }
                None => {
                    res.push(
                            *compression_map
                            .get(s.as_str())
                            .expect("Table pre-allocated"),
                    );
                    last_entry += 1;
                    compression_map.insert(sc, last_entry);
                    s = (c as char).to_string();
                }
            };
        });

        res.push(
            *compression_map
                .get(s.as_str())
                .expect("Table pre-allocated"),
        );

        res
    }

    pub fn calculate_compression_ratio(&self, uncompressed: u32, compressed: u32) -> f64 {
        1. - (compressed as f64 / uncompressed as f64)
    }
}

#[inline]
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::LZW;

    fn new_maps() -> (HashMap<String, u32>, HashMap<u32, String>) {
        let mut compression_map = HashMap::new();
        let mut decompression_map = HashMap::new();
    
        (0..255).for_each(|i| {
            compression_map.insert((char::from(i)).to_string(), i as u32);
            decompression_map.insert(i as u32, (char::from(i)).to_string());
        });
    
        (compression_map,decompression_map)
    }

    #[test]
    fn compress_wikipedia_example() {
        let message = Vec::from("TOBEORNOTTOBEORTOBEORNOT");
        let (mut comp, _) = new_maps();

        let mut thing = LZW::default();
        let result = thing.compress(message, &mut comp);

        let expected_result = vec![
            84, 79, 66, 69, 79, 82, 78, 79, 84, 256, 258, 260, 265, 259, 261, 263,
        ];

        assert_eq!(expected_result, result);
    }

    #[test]
    fn decompress_wikipedia_example() {
        let message = vec![
            84, 79, 66, 69, 79, 82, 78, 79, 84, 256, 258, 260, 265, 259, 261, 263,
        ];

        let (_, mut decomp) = new_maps();

        let mut thing = LZW::default();
        let result = thing.decompress(message, &mut decomp);

        let expected_result = String::from("TOBEORNOTTOBEORTOBEORNOT");

        assert_eq!(expected_result, result);
    }

    #[test]
    fn compress_single_char() {
        let message = Vec::from("a");

        let (mut comp, _) = new_maps();

        let mut thing = LZW::default();
        let result = thing.compress(message, &mut comp);

        let expected_result = vec![97];

        assert_eq!(expected_result, result);
    }

    #[test]
    fn decompress_single_char() {
        let message = vec![97];

        let (_, mut decomp) = new_maps();

        let mut thing = LZW::default();
        let result = thing.decompress(message, &mut decomp);

        let expected_result = String::from("a");

        assert_eq!(expected_result, result);
    }
}
