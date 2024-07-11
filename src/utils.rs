pub(crate) const TOKEN_FIELD_NAME: &str = "token";

pub fn build_prefix_end(prefix: impl AsRef<[u8]>) -> Vec<u8> {
    const NO_PREFIX_END: Vec<u8> = Vec::new();

    let mut end = prefix.as_ref().to_vec();

    for (i, b) in end.iter_mut().rev().enumerate() {
        if *b < 0xFF {
            *b += 1;
            end.truncate(end.len() - i);
            return end;
        }
    }

    NO_PREFIX_END
}

#[cfg(test)]
mod test {

    #[test]
    fn test_build_prefix_end() {
        assert_eq!(super::build_prefix_end(b"abc"), b"abd".to_vec());
        assert_eq!(super::build_prefix_end(b"abc\xFF"), b"abd".to_vec());
        assert_eq!(super::build_prefix_end(b"\xFF\xFF"), b"".to_vec());
    }
}
