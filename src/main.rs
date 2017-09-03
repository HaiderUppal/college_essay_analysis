use std::fs::{read_dir, File, DirEntry};
use std::path::Path;
use std::io::Read;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Essay {
    university: String,
    prompt:     String,
    data:       String
}

impl Essay {
    pub fn from_file(_file: DirEntry) -> Essay {
        let _filename = _file.file_name().into_string().unwrap();
        let filename = _filename.split(" ").nth(1).unwrap();
        let mut split_data = filename.split("-");
        let mut uni = split_data.nth(0).unwrap().to_string();
        if uni == "common" {
            uni.push_str(" ");
            uni.push_str(split_data.nth(0).unwrap());
        }
        let prompt_vec: Vec<String> = split_data.map(|x| x.to_string()).collect();
        let _prompt = prompt_vec.join(" ");
        let prompt = _prompt.chars().take(_prompt.len() - 4).collect::<String>();

        let mut file = File::open(_file.path()).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data);

        let actual: Vec<String> = data.lines().map(String::from).take_while(|x| !x.starts_with("Enlist the expert help")).collect();
        
        Essay { university: uni,
                prompt: prompt,
                data: actual.join("\n") }
    }
}

fn word_analysis(essays: &Vec<Essay>, banned: &Vec<String>) -> Vec<(String, u32)> {
    let mut map: HashMap<String, u32> = HashMap::new();

    fn add_words(essay: Essay, map: &mut HashMap<String, u32>, banned: &Vec<String>) {
        let puncts: Vec<&str> = vec!["!", ";", ":", ".", ",", "\"", "'"];
        for _word in essay.data.clone().split(" ").map(String::from) {
            let mut word = _word.trim().to_string();
            word = word.replace("\n", "");
            for punct in &puncts {
                if word.ends_with(punct) {
                    word = word.chars().take(word.len() - 1).collect::<String>();
                }
                if word.starts_with(punct) {
                    word = word.chars().skip(1).collect::<String>();
                }
            }
            if !banned.contains(&word) {
                *map.entry(word.clone().to_lowercase()).or_insert(0) += 1;
            }
        }
    }

    for essay in essays {
        add_words(essay.clone(), &mut map, banned);
    }

    let mut vec: Vec<(String, u32)> = map.into_iter().collect();

    fn s_cmp(a: (String, u32), b: (String, u32)) -> Ordering {
        let (_aa, _ab) = a;
        let (_ba, _bb) = b;
        _ab.cmp(&_bb)
    }

    vec.sort_by( |a, b| { 
        let &(ref _aa, ref _ab) = a;
        let &(ref _ba, ref _bb) = b;
        _ab.cmp(_bb)
    } );
    vec.reverse();
    vec
}


fn main() {
    let essays = read_dir("essays/").unwrap();
    let mut v_essays: Vec<Essay> = Vec::new();
    for dir in essays {
        v_essays.push(Essay::from_file(dir.ok().unwrap()));
    }
    let banned = vec!["the", "a", "it", "or", "to", "i", "and", "of", "in"];
    let banned = banned.iter().map(|x| x.to_string()).collect();
    let v_essays = v_essays; // set to immut
    let analysis = word_analysis(&v_essays, &banned);
    println!("Essays Analyzed [{}]", v_essays.len());
    println!("Top 5 words:");
    for (ind, tup) in analysis.iter().enumerate() {
        let &(ref word, ref occ) = tup; 
        println!("[{}] [{}]: {} time{}!", ind, word, occ, if occ > &1 { "s" } else { "" } );
    }

}
