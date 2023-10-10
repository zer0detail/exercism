use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut res: HashSet<&str> = HashSet::new();
    let word_lower = word.to_lowercase();
    let mut word_lower_sorted = word_lower
        .chars()
        .collect::<Vec<char>>();
    word_lower_sorted.sort_unstable();
    let word_lower_sorted: String = word_lower_sorted
        .into_iter()
        .collect();
    for w in possible_anagrams.iter() {
        let orig = w.to_owned();
        let w_lower = w.to_lowercase();
        let mut w_lower_sorted = w_lower 
            .chars()
            .collect::<Vec<char>>();
        w_lower_sorted.sort_unstable();
        let w_lower_sorted: String = w_lower_sorted
            .into_iter()
            .collect();
        println!("{} - {}", word_lower_sorted, w_lower_sorted);
        if *w_lower != word_lower && w_lower_sorted == word_lower_sorted{

            res.insert(orig);
        }
    }
    res
}
