pub fn split_string_to_i32_vec(data: String) -> Vec<i32> {
    let data: Vec<_> = data.split(',').collect();
    let mut resp = Vec::new();
    for str in data {
        resp.push(std::str::FromStr::from_str(str).unwrap_or(0));
    }
    resp
}