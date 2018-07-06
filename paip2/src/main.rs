extern crate rand;

use rand::Rng;

// problem 1: append moves the second to the end of the first
// problem 2: append modifies first

fn sentence() -> Vec<String> {
    let np = noun_phrase();
    let vp = verb_phrase();
    let mut rt = vec![];
    rt.extend(np.iter().cloned());
    rt.extend(vp.iter().cloned());
    rt
}

fn noun_phrase() -> Vec<String> {
    let  art = article();
    let  np = noun();
    let mut rt = vec![];
    rt.extend(art.iter().cloned());
    rt.extend(np.iter().cloned());
    rt
}

fn verb_phrase() -> Vec<String> {
    let  vb = verb();
    let  np = noun_phrase();
    let mut rt = vec![];
    rt.extend(vb.iter().cloned());
    rt.extend(np.iter().cloned());
    rt
}

fn article() -> Vec<String> {
    one_of(vec![String::from("the"),
                String::from("a")])
}

fn noun() -> Vec<String> {
    one_of(vec![String::from("man"),
                String::from("ball"),
                String::from("woman"),
                String::from("table")])
}

fn verb() -> Vec<String> {
    one_of(vec![String::from("hit"),
                String::from("took"),
                String::from("saw"),
                String::from("liked")])
}

fn one_of(v : Vec<String>) -> Vec<String> {
    let val = random_elt(v);
    vec![val]
}

fn random_elt(choices : Vec<String>) -> String {
    let val = rand::thread_rng().choose(&choices).clone();
    match val {
        Some(v) => v.clone(),
        None => String::from("")
    }
}

fn main() {
    let s = sentence();
    println!("{:?}", s);
}
