pub fn concatenate_strings( str1: &str, str2: &str ) -> String {
    let mut result = String::from("");
    result.push_str(str1);
    result.push_str(str2);
    result
}