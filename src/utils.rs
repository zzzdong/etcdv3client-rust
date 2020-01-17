pub fn build_prefix_end(prefix: impl AsRef<[u8]>) -> Vec<u8> {
    let no_prefix_end = Vec::new();

    if prefix.as_ref().is_empty() {
        return no_prefix_end;
    }

    let mut end = prefix.as_ref().to_vec();

    for i in (0..end.len()).rev() {
        if end[i] < 0xff {
            end[i] += 1;
            return end[0..=i].to_vec();
        }
    }

    no_prefix_end
}
