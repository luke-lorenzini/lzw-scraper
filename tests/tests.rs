#[cfg(test)]
mod tests {
    use demo::{lzw::LZW, new_maps};

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
