// random stat generators
use rand::thread_rng;
use rand::Rng;
use rand::seq::SliceRandom;
use std::f64::consts::E;

use rand::prelude::*;
use rand::distributions::WeightedIndex;

pub fn name(gender: &str) -> String {
    let first_name = if gender == "male" { male_name() } else { female_name() };
    let last_name = last_name();
    vec![first_name, last_name].join(" ")
}

fn male_name() -> &'static str {
    let m_list = vec!["Rupert", "Hugo", "Ignatius","Alfred", "Warrington", "Rufus", "Wilbur", "Barnaby", "Roland", "Eugene", "Montgomery", "Archibald", "Ned", "Spatz", "Gil", "Wallace", "Abraham", "Harold", "Claudius", "Arthur", "Calvin", "Arsenio", "August", "Coleman", "Crispus", "Lemuel", "Moses", "Winston", "Robert"];
    m_list.choose(&mut rand::thread_rng()).unwrap()
}

fn female_name() -> &'static str {
    let f_list = vec!["Ingrit", "Agatha", "Betheny", "Helen", "Mary", "Brenda", "Winifred", "Elvira", "Dolores", "Sidney", "Jane", "Bernadette", "Lois", "Eileen", "Patricia", "Janet", "Wanda", "Margaret", "Bessie", "Maggie", "Octavia"];
    f_list.choose(&mut rand::thread_rng()).unwrap()
}

fn last_name() -> &'static str {
    let last_names = vec!["Baskins", "Heathrow", "Williams", "Wainwright", "Hollandaise", "Walker", "Poitier", "Robinson", "Fuller", "Poor", "Marshall", "Haynes", "Hammon", "Bluford", "Young", "Cromwell", "Butler", "Moore", "Cooke", "Smith", "Jones", "Cobb", "King", "Brown", "Mason", "Franklin", "Peters", "Dempsey", "Dalton", "Sutcliffe", "Coffey", "Bartley", "Burris", "Agnello", "Burns", "Webster", "Adair", "Dalloway", "Elrod", "Martel", "Cohen", "Proudhon", "Washington", "Jefferson", "Lenoir", "Bell", "Nutter", "Alston", "Huxley", "Ives", "Anderton", "Russ", "Dankworth", "Garfield", "Hastings", "Bread", "Cotton", "Farley", "Barton", "Crawford", "Jackson", "Davis", "Lewis", "Hall", "Jenkins", "Cooper", "Dixon", "Barnes", "Ford", "Holmes", "Marshall", "Hughes", "Reid", "Frazier", "Greene", "Glover", "Henry", "Stanton", "West", "Strauss", "Trumbull", "Harlan", "Ruckstell", "Keesling", "Cameron", "Filmer", "Welch", "Shannon", "Newhouse", "McLaughlin", "Hadeler", "Lutgens", "Doyle", "Westbrook", "Pomeroy", "Roebling"];
    last_names.choose(&mut rand::thread_rng()).unwrap()
}

// rolls age, weighted toward 65
pub fn age() -> usize {
    let range: Vec<usize> = (65..121).collect();
    let weights = decay_curve(56_usize, -0.2_f64, 5000_f64);
    return pick_weighted(&range, &weights)
}

// determines number of ailments with decay_curve(), chooses randomly
pub fn ailments(age: usize) -> Vec<String> {

    let list = vec![
        "Arthritis".to_string(), 
        "Asthma".to_string(), 
        "Diabetus".to_string(), 
        "Mobile-impaired".to_string(), 
        "Obese".to_string(), 
        "High Blood Pressure".to_string(), 
        "Parkinson's".to_string(),
        "Alzheimer's".to_string(),
        "Cataracts".to_string(),
        "Hard-of-hearing".to_string(), 
        "Osteoporosis".to_string(),
        "Frail".to_string(),
    ];

    let range = vec![0, 1, 2, 3, 4, 5, 6];
    let amp: f64 = to_f64(age).unwrap() / -14_f64 + 13_f64;
    let weights = decay_curve(7_usize, -0.7_f64, amp);
    let num = pick_weighted(&range, &weights);

    let sample: Vec<String> = 
        list.choose_multiple(&mut rand::thread_rng(), num)
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>();

    return sample
}

// higher age -> more powerful, but more ailments
pub fn pick_weighted(range: &Vec<usize>, weights: &Vec<usize>) -> usize {
    let mut rng = thread_rng();
    let dist = WeightedIndex::new(weights).unwrap();
    return range[dist.sample(&mut rng)]
}

// rolls stat by age and a stat-specific modifier due to ailments
pub fn stat(age: usize, modifier: f64) -> usize {
    let amp: f64 = to_f64(age).unwrap() / 75_f64;
    let weights = growth_curve(100_usize, amp);
    let range = (0..100).collect();
    let num = pick_weighted(&range, &weights);
    let modded = to_f64(num).unwrap() * modifier;

    return to_usize(modded)
}

// used to roll age and number of ailments; 
pub fn decay_curve(length: usize, k: f64, amp: f64) -> Vec<usize> {
    println!("decay: {} {} {}", length, k, amp);
    let mut vec = Vec::new();
    for num in 0..length {
        let x: f64 = to_f64(num).unwrap();
        let exp: f64 = k*x;
        let val = f64::powf(E, exp)*amp;
        let int = val as usize;
        if int > 0 { vec.push(int) } else { vec.push(1) };
    }
    return vec
}

// used to roll stats, which increase with age
pub fn growth_curve(length: usize, amp: f64) -> Vec<usize> {
    println!("growth: {} {}", length, amp);
    let mut vec = Vec::new();
    for num in 0..length {
        let x: f64 = to_f64(num).unwrap();
        let val = x.ln()*amp;
        let int = val as usize;
        if int > 0 { vec.push(int) } else { vec.push(1) };
    }
    return vec
}

//
// UNUSED ROLLING ALGORITHMS
// 

#[allow(dead_code)]
pub fn normal() -> usize {
    thread_rng().gen_range(0..100)
}

#[allow(dead_code)]
pub fn exp_curve(length: usize, soft:f64) -> Vec<usize> {
    let mut vec = Vec::new();
    for num in 0..length {
        let x: f64 = to_f64(num).unwrap();
        let exp = x/soft;
        let val = f64::powf(E,  exp);
        let int = val as usize;
        vec.push(int);
    }
    return vec
}

#[allow(dead_code)]
pub fn straight_line(length:usize) -> Vec<usize> {
    let mut vec = Vec::new();
    for num in 1..length+1 {
        vec.push(num);
    }
    return vec
}

//
// CONVERSIONS
// 

fn to_f64(x: usize) -> Result<f64, &'static str> {
    let result = x as f64;
    if result as usize != x {
        return Err("conversion error (to_f64)")
    }
    Ok(result)
}

fn to_usize(x: f64) -> usize {
    x.round() as usize
}


