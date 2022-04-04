/// replaces the char `find` with the str `subst` in `string`
pub fn replace_char_with_str(string: &str, find: char, subst: &str) -> String {
    let mut result = String::new();
    for c in string.chars() {
        if c == find {
            result.push_str(subst);
        } else {
            result.push(c);
        }
    }
    result
}

/// replaces the str `find` with the str `subst` in `string`
pub fn replace_str_with_str(string: &str, find: &str, subst: &str) -> String {
    let mut result = String::new();
    let mut find_pos = 0;
    for c in string.chars() {
        if c == find.chars().nth(find_pos).unwrap() {
            find_pos += 1;
            if find_pos == find.len() {
                result.push_str(subst);
                find_pos = 0;
            }
        } else {
            result.push(c);
            find_pos = 0;
        }
    }
    result
}

/// replaces the char `find` with the char `subst` in `string`
pub fn replace_char_with_char(string: &str, find: char, subst: char) -> String {
    let mut result = String::new();
    for c in string.chars() {
        if c == find {
            result.push(subst);
        } else {
            result.push(c);
        }
    }
    result
}

/// replaces the str `find` with the char `subst` in `string`
pub fn replace_str_with_char(string: &str, find: &str, subst: char) -> String {
    let mut result = String::new();
    let mut find_pos = 0;
    for c in string.chars() {
        if c == find.chars().nth(find_pos).unwrap() {
            find_pos += 1;
            if find_pos == find.len() {
                result.push(subst);
                find_pos = 0;
            }
        } else {
            result.push(c);
            find_pos = 0;
        }
    }
    result
}

/// makes the string `string` lowercase
pub fn make_string_lowercase(string: &str) -> String {
    let mut result = String::new();
    for c in string.chars() {
        result.push(c.to_lowercase().next().unwrap());
    }
    result
}

/// makes the string `string` uppercase
pub fn make_string_uppercase(string: &str) -> String {
    let mut result = String::new();
    for c in string.chars() {
        result.push(c.to_uppercase().next().unwrap());
    }
    result
}
