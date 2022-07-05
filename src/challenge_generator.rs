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
    let file = File::open(dictionary).unwrap();
    let reader = BufReader::new(file);

    for (_, line) in reader.lines().enumerate() {
        // println!("Line: {:?}", line.as_ref());
        match line.as_ref() {
            Err(err) => {
                // println!("Error: {:?} reading line {:?}", err, line.as_ref());
                continue;
            },
            _ => {}
        }
        if line.as_ref().unwrap().contains("adj.") {
            words_list.adjs.push(line.unwrap().split(" ").collect::<Vec<&str>>()[0].to_string());
        } else if line.as_ref().unwrap().contains("adv.") {
            words_list.advs.push(line.unwrap().split(" ").collect::<Vec<&str>>()[0].to_string());
        } else if line.as_ref().unwrap().contains("det.") {
            words_list.dets.push(line.unwrap().split(" ").collect::<Vec<&str>>()[0].to_string());
        } else if line.as_ref().unwrap().contains("prep.") {
            words_list.preps.push(line.unwrap().split(" ").collect::<Vec<&str>>()[0].to_string());
        } else if line.as_ref().unwrap().contains("pron.") {
            words_list.prons.push(line.unwrap().split(" ").collect::<Vec<&str>>()[0].to_string());
        } else if line.as_ref().unwrap().contains("subst.") {
            words_list.substs.push(line.unwrap().split(" ").collect::<Vec<&str>>()[0].to_string());
        } else if line.as_ref().unwrap().contains("verbe.") {
            words_list.verbes.push(line.unwrap().split(" ").collect::<Vec<&str>>()[0].to_string());
        } else {
            // println!("Unknown line: {:?}", line.as_ref());
            continue;
        }
    }
    return words_list;
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
