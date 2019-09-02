pub mod pig_latin {
    pub fn phrase(phrase: &mut str) -> String {
        let vowals = ['a', 'e', 'i', 'o', 'u'];
        let mut pig_phrase = String::new();

        for word in phrase.split_whitespace() {
            let mut wvec: Vec<char> = word.chars().collect();
            let fchar = wvec[0];
            wvec.push('-');
            if vowals.contains(&fchar) {
                wvec.push('h');
            } else {
                wvec.remove(0);
                wvec.push(fchar);
            }
            let rword: String = wvec.into_iter().collect();
            pig_phrase.push_str(&rword);
            pig_phrase.push_str("ay ");
        }
        pig_phrase
    }
}
