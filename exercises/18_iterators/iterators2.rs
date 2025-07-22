// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    let mut res = match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string(),
    };
    let c = chars.map(|x| x).collect();
    res.push_str(&c);
    res
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // ???
    words
        .iter()
        .map(|x| {
            let mut res = String::new();
            res.push_str(&x[0..1].to_uppercase());
            res.push_str(&x[1..]);
            res
        })
        .collect::<Vec<String>>()
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    // ???
    words
        .iter()
        .map(|x| {
            let mut res = String::new();
            res.push_str(&x[0..1].to_uppercase());
            res.push_str(&x[1..]);
            res
        })
        .collect()
}

fn main() {
    // You can optionally experiment here.
    let words = vec!["hello", " ", "world"];
    println!("{:?}", capitalize_words_string(&words));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
