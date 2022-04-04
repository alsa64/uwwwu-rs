use std::{env, io};

use rand::Rng;
use substring::Substring;

/// module with various char related functions
#[allow(dead_code)]
mod char_tools;

/// module with various string related functions
#[allow(dead_code)]
mod string_tools;

/// Will replace all occurrences of a substring `find` in `str` with `sub`, but it will try to keep the characters signs.
/// Like (pay attention to the capitalization):.
/// ("Hello World", "hello", "hi") -> "Hi World".
/// ("hello World", "hello", "hi") -> "hi World".
/// ("HELLO World", "hello", "hi") -> "HI World".
/// You can also supply a callback to only perform a replacement if certain conditions apply.
fn conditional_replace_but_keep_signs(str: &str, find: &str, sub: &str, callback: fn(s1: &str, s2: &str, i: usize) -> bool) -> String {
    let mut ss = "".to_string();

    if str.len() == 0 {
        return "".to_string();
    }

    if find.len() == 0 {
        return "".to_string();
    }

    let find = find.to_lowercase();

    for (i, _) in str.chars().enumerate() {
        let found_in_text = str.substring(i, i + find.len());
        let found_in_text_lower = found_in_text.to_lowercase();
        if found_in_text_lower == find {
            if callback(str, &found_in_text, i) {
                // Here we've found our occurrence...
                // We have three possible cases:
                // 1: len(find) == len(sub), in this case we want to sync capitalization by index.
                // 2: len(find) < len(sub), in this case we sync by index, BUT...
                // 3: len(find) > len(sub): sync capitalization by index

                // We want to sync capitalization by index
                // This accounts for both cases: 1 and 3
                if found_in_text.len() >= sub.len() {
                    for (j, _) in sub.chars().enumerate() {
                        let cf = found_in_text.chars().nth(j).unwrap();
                        let cs = sub.chars().nth(j).unwrap();

                        ss.push(char_tools::copy_sign(cf, cs));
                    }
                } else if found_in_text.len() < sub.len() {
                    let mut following_chars_sign = ' ';
                    let mut do_have_following_char = false;
                    // Do we even have a following char?
                    if str.len() >= i + found_in_text.len() + 1 {
                        let following_char = str.chars().nth(i + found_in_text.len() + 1).unwrap();

                        // Is it a letter?
                        if char_tools::is_letter(following_char) {
                            // Copy its sign
                            following_chars_sign = following_char;
                            do_have_following_char = true;
                        }
                    }

                    let mut last_char_char_sign = ' ';
                    for (j, _) in sub.chars().enumerate() {
                        let cs = sub.chars().nth(j).unwrap();

                        // Do we still have chars of 'find' left?
                        if j < found_in_text.len() {
                            // yes: Just copy the sign as is, and update the last sign seen
                            let cf = found_in_text.chars().nth(j).unwrap();
                            last_char_char_sign = cf;
                            ss.push(char_tools::copy_sign(cf, cs));
                        } else {
                            // No: Use the last sign seen, or the sign of the following char (the following char within the same word-boundary) (Important for replacing vocals within a word)
                            let char_sign_to_use = if do_have_following_char {
                                following_chars_sign
                            } else {
                                last_char_char_sign
                            };

                            ss.push(char_tools::copy_sign(char_sign_to_use, cs));
                        }
                    }
                }
            } else {
                // We do not have an occurrence... just insert the subsection found as is (next iteration will start behind it)
                ss.push_str(&found_in_text);
            }
        }
    }
    ss
}

fn conditional_replace_but_keep_signs_no_func(str: &str, find: &str, sub: &str) -> String {
    conditional_replace_but_keep_signs(str, find, sub, |_, _, _| true)
}

// This validator will only replace findings, if they are a complete word, and not just part of a word.
fn validator_finding_is_complete_word(original: &str, finding: &str, index: usize) -> bool {
    // Quick-accept: Original-string length matches finding-string length
    if original.len() == finding.len() {
        return true;
    }

    // Assign surrounding chars, if possible
    let last_char_breaks_word = if index > 0 {
        !char_tools::is_letter(original.chars().nth(index - 1).unwrap())
    } else {
        true // Default is true, because this value stays in case there is no last/next char.
    };

    let next_char_breaks_word = if (index + finding.len()) < original.len() {
        !char_tools::is_letter(original.chars().nth(index + finding.len()).unwrap())
    } else {
        true // In this case, "no character" would imply the word to be broken.
    };

    // If both the last and the next character are word-breaking, replace.
    if last_char_breaks_word && next_char_breaks_word {
        true
    } else {
        false
    }
}

/// Will make a boring string look sooper dooper kawaii and cute :3
fn make_uwu(boring_string: &str) -> String {
    let mut uwu_string = String::from(boring_string);
    // Easy ones first
    // none, lol

    // Slightly more complex... Multichar replacements, but we have to keep capitalization...
    uwu_string = conditional_replace_but_keep_signs_no_func(uwu_string.as_str(), "th", "tw");
    uwu_string = conditional_replace_but_keep_signs_no_func(uwu_string.as_str(), "ove", "uv");
    uwu_string = conditional_replace_but_keep_signs_no_func(uwu_string.as_str(), "have", "haf");
    uwu_string = conditional_replace_but_keep_signs_no_func(uwu_string.as_str(), "tr", "tw");
    uwu_string = conditional_replace_but_keep_signs_no_func(uwu_string.as_str(), "up", "uwp");

    // Let's do some language adjustments
    uwu_string = conditional_replace_but_keep_signs(uwu_string.as_str(), "twank you", "you're twe best <3333 xoxo", validator_finding_is_complete_word);
    uwu_string = conditional_replace_but_keep_signs(uwu_string.as_str(), "good", "sooper dooper", validator_finding_is_complete_word);
    uwu_string = conditional_replace_but_keep_signs(uwu_string.as_str(), "suwper", "sooper dooper", validator_finding_is_complete_word);
    uwu_string = conditional_replace_but_keep_signs(uwu_string.as_str(), "well", "sooper dooper", validator_finding_is_complete_word);
    uwu_string = conditional_replace_but_keep_signs(uwu_string.as_str(), "emacs", "vim", validator_finding_is_complete_word);
    uwu_string = conditional_replace_but_keep_signs(uwu_string.as_str(), "twanks", "you're twe best :33 xoxo", validator_finding_is_complete_word);
    uwu_string = conditional_replace_but_keep_signs(uwu_string.as_str(), "hello", "hiiiiiii", validator_finding_is_complete_word);
    uwu_string = conditional_replace_but_keep_signs(uwu_string.as_str(), "dear", "hiiiiiii", validator_finding_is_complete_word);

    // Let's extend some phonetics
    uwu_string = conditional_replace_but_keep_signs_no_func(uwu_string.as_str(), "hi", "hiiiiiii");
    uwu_string = conditional_replace_but_keep_signs_no_func(uwu_string.as_str(), "ay", "aaay");
    uwu_string = conditional_replace_but_keep_signs_no_func(uwu_string.as_str(), "ey", "eeey");

    // Replace N with Ny, but only if succeeded by a vowel, and not (preceded by an o and succeeded by an "e{nonletter}"): "one" has such a niche pronunciation...
    uwu_string = conditional_replace_but_keep_signs(uwu_string.as_str(), "n", "ny", |original, finding, index| {
        // Don't replace, if we are on the last char
        if (index + finding.len()) == original.len() {
            return false;
        }

        let next_char = char_tools::make_lower(original.chars().nth(index + finding.len()).unwrap());
        let have_last_char = index > 0; // Do we even have a last char?
        let last_char = if have_last_char {
            char_tools::make_lower(original.chars().nth(index - 1).unwrap())
        } else {
            ' '
        };
        // Apply the complex "one\b"-rule:
        // (don't replace if 'n' is preceded by 'o' and succeeded by 'e', which is succeeded by a word break)
        {
            let mut next_next_char_is_not_letter = false;
            let mut next_char = ' ';

            // How much length is left including `nextChar`?
            let size_left = original.len() - (index + finding.len());

            // We have room to pick the nextNext char...
            if size_left > 1 {
                next_char = char_tools::make_lower(original.chars().nth(index + finding.len() + 1).unwrap());
                next_next_char_is_not_letter = !char_tools::is_letter(next_char);
            }

            let next_char_breaks_word = size_left == 1 || next_next_char_is_not_letter;

            // Don't replace if:
            // (lastChar == o) && (nextChar == e) && (nextNextCharBreaksWord)
            if have_last_char && last_char == 'o' && next_char == 'e' && next_char_breaks_word {
                return false;
            }
        }

        // Is this a vowel?
        if char_tools::is_vowel_case_insensitive(next_char) {
            return true;
        }

        false
    });

    // Replace R with W, but only if not succeeded by a non-vowel, and if it's not the first character of a word
    uwu_string = conditional_replace_but_keep_signs(uwu_string.as_str(), "r", "w", |original, finding, index| {
        // Don't replace, if we are on the last char
        if (index + finding.len()) == original.len() {
            return false;
        }

        // Don't replace if we're at index 0
        if index == 0 {
            return false;
        }
        let next_char = char_tools::make_lower(original.chars().nth(index + finding.len()).unwrap());
        let last_char = char_tools::make_lower(original.chars().nth(index - 1).unwrap());

        // Is this a non-vowel?
        if !char_tools::is_vowel_case_insensitive(next_char) {
            return false;
        }

        // Don't replace if the last char is not a letter
        if !char_tools::is_letter(last_char) {
            return false;
        }

        // Else, Replace!
        true
    });

    // Replace L with W, but only if not followed or preceded by another L, and if it's not the first character of a word
    uwu_string = conditional_replace_but_keep_signs(uwu_string.as_str(), "l", "w", |original, finding, index| {
        // Our segment has to be at least two characters long
        if original.len() < finding.len() + 2 {
            return false;
        }

        // Don't replace if we're at index 0
        if index == 0 {
            return false;
        }

        let last_char = char_tools::make_lower(original.chars().nth(index - 1).unwrap());
        let next_char = char_tools::make_lower(original.chars().nth(index + finding.len()).unwrap());

        // Don't replace if the last char is not a letter
        if !char_tools::is_letter(last_char) {
            return false;
        }

        last_char != 'l' && next_char != 'l'
    });

    // Replace LL with WW, but only if followed by a vowel
    uwu_string = conditional_replace_but_keep_signs(uwu_string.as_str(), "ll", "ww", |original, finding, index| {
        // Don't replace, if we are on the last char
        if (index + finding.len()) == original.len() {
            return false;
        }

        let next_char = char_tools::make_lower(original.chars().nth(index + finding.len()).unwrap());

        char_tools::is_vowel_case_insensitive(next_char)
    });

    // Replace ER with A, but only if it's the last two letters of a word
    uwu_string = conditional_replace_but_keep_signs(uwu_string.as_str(), "er", "a", |original, finding, index| {
        // Replace if we're at the end of this line/segment
        if (index + finding.len()) == original.len() {
            return true;
        }

        // Fetch the next char
        let next_char = char_tools::make_lower(original.chars().nth(index + finding.len()).unwrap());

        // Replace if the next char is not a letter
        !char_tools::is_letter(next_char)
    });

    // Replace random punctuation with uwwwwu cute symbols
    // About evewy fifteenth symbol
    let mut ss: String = String::new();
    let rng = &mut rand::thread_rng();
    let rand_int: i32 = rng.gen_range(0, 15);

    for (_, c) in uwu_string.chars().enumerate() {
        if c == '.' && rand_int == 0 {
            ss.push_str(" <3333 ^.^ ");
        } else if c == '!' && rand_int == 0 {
            ss.push_str("!! Thadws impowtant! <3 ");
        } else if c == ',' && rand_int == 0 {
            ss.push_str(" <3 aaaaaand ");
        } else if c == '?' && rand_int == 0 {
            ss.push_str("?? now tell me! >:( ");
        } else {
            ss.push(c);
        }
    }

    uwu_string = ss;

    // Also replace some ascii-"emojis'
    uwu_string = string_tools::replace_str_with_str(uwu_string.as_str(), ":)", "UwU :D");
    uwu_string = string_tools::replace_str_with_str(uwu_string.as_str(), ":D", ":3");
    uwu_string = string_tools::replace_str_with_str(uwu_string.as_str(), ":-)", "UwwwU :3");
    uwu_string = string_tools::replace_str_with_str(uwu_string.as_str(), "^^", "^.^ UwU");

    // Some language replacement should happen after these more complex rules
    uwu_string = conditional_replace_but_keep_signs_no_func(uwu_string.as_str(), "c++", "c++ (rust is hella cutewr btw ^^)");

    uwu_string
}

fn main() {
    // We have arguments. Uwwuifie these instead
    if env::args().len() > 1 {
        let mut uwu_string = String::new();
        for arg in env::args().skip(1) {
            uwu_string.push_str(&arg);
            uwu_string.push(' ');
        }

        println!("{}", make_uwu(uwu_string.as_str()));
    } else {
        // No arguments. Read from stdin
        loop {
            let mut uwu_string = String::new();
            io::stdin().read_line(&mut uwu_string).unwrap();

            if uwu_string.len() == 0 {
                break;
            }

            println!("{}", make_uwu(uwu_string.as_str()));
        }
    }
}
