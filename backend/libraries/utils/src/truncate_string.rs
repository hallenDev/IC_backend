
pub fn truncate_string(data: String, length: usize) -> String {
    let mut short = data;
    if short.len() > length {
        short.truncate(length - 3);
        short += "...";
    }
    short
}