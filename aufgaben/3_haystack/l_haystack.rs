fn main() {
    let haystack = "Es ist kurz vor 3 und ich will nach Hause!".to_string();
    let needle = 's';
    let count = count(&haystack, needle);
    println!("Der Charakter '{}' ist {}. mal in dem Satz:'{}'", needle, count, haystack);
}


fn count(haystack: &String, needle: char) -> u64 {
    let mut count = 0;
    for c in haystack.chars() {
        if c == needle {
            count += 1;
        }
    }

    count
}