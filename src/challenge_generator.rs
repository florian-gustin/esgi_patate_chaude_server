use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub(crate) struct WordsList {
    adjs: Vec<String>,
    advs: Vec<String>,
    dets: Vec<String>,
    preps: Vec<String>,
    prons: Vec<String>,
    substs: Vec<String>,
    verbes: Vec<String>,
}

impl WordsList {
    pub(crate) fn new() -> WordsList {
        WordsList {
            adjs: Vec::<String>::new(),
            advs: Vec::<String>::new(),
            dets: Vec::<String>::new(),
            preps: Vec::<String>::new(),
            prons: Vec::<String>::new(),
            substs: Vec::<String>::new(),
            verbes: Vec::<String>::new(),
        }
    }
}

pub(crate) fn init_word_list(mut words_list: WordsList) -> WordsList {
    let dictionary = "res/dictionary.txt";
    let file = match File::open(dictionary) {
        Ok(file) => file,
        Err(e) => panic!("Couldn't open file: {}", e),
    };
    let reader = BufReader::new(file);

    for (_, line) in reader.lines().enumerate() {
        // println!("Line: {:?}", line_ref);
        let line_ref = match line.as_ref() {
            Ok(line_ref) => line_ref,
            Err(_) => continue,
        };

        if line_ref.contains("adj.") {
            words_list.adjs.push(extract_word_from_line(line_ref));
        } else if line_ref.contains("adv.") {
            words_list.advs.push(extract_word_from_line(line_ref));
        } else if line_ref.contains("det.") {
            words_list.dets.push(extract_word_from_line(line_ref));
        } else if line_ref.contains("prep.") {
            words_list.preps.push(extract_word_from_line(line_ref));
        } else if line_ref.contains("pron.") {
            words_list.prons.push(extract_word_from_line(line_ref));
        } else if line_ref.contains("subst.") {
            words_list.substs.push(extract_word_from_line(line_ref));
        } else if line_ref.contains("verbe.") {
            words_list.verbes.push(extract_word_from_line(line_ref));
        } else {
            // println!("Unknown line: {:?}", line_ref);
            continue;
        }
    }
    return words_list;
}

fn extract_word_from_line(line: &str) -> String {
    let line_string = line.split(" ").collect::<Vec<&str>>()[0].to_string();
    return line_string;
}

pub(crate) fn generate_sentence_from_words_list(words_list: &WordsList) -> String {
    let mut sentence = String::new();
    sentence.push_str(&words_list.prons[rand::random::<usize>() % words_list.prons.len()]);
    sentence.push_str(" ");
    sentence.push_str(&words_list.verbes[rand::random::<usize>() % words_list.verbes.len()]);
    sentence.push_str(" ");
    sentence.push_str(&words_list.preps[rand::random::<usize>() % words_list.preps.len()]);
    sentence.push_str(" ");
    sentence.push_str(&words_list.advs[rand::random::<usize>() % words_list.advs.len()]);
    sentence.push_str(" ");
    sentence.push_str(&words_list.dets[rand::random::<usize>() % words_list.dets.len()]);
    sentence.push_str(" ");
    sentence.push_str(&words_list.substs[rand::random::<usize>() % words_list.substs.len()]);
    sentence.push_str(" ");
    sentence.push_str(&words_list.adjs[rand::random::<usize>() % words_list.adjs.len()]);
    return sentence;
}
