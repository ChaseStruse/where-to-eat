use clap::Parser;
use rand::Rng;
/// Tool that takes a list of restaurants and picks a random one 
#[derive(Parser)]
struct Cli {
    /// Path to restuarant text file.
    path: std::path::PathBuf,
    /// Category
    category: String,
}

fn get_random_restaurant(matching_restaurants: Vec<&str>, category: String) -> String {
    let mut rng = rand::thread_rng();
    let rand_num: usize = rng.gen_range(0..matching_restaurants.len());
    matching_restaurants[rand_num].trim_end_matches(&category).to_string()
}

fn main() {
    let args = Cli::parse();

    let result = std::fs::read_to_string(&args.path);

    let restaurants = match result {
        Ok(restaurants) => { restaurants },
        Err(error) => { panic!("Error parsing file {}", error)},
    };

    let mut matching_restaurants: Vec<&str> = Vec::new();
    
    for restaurant in restaurants.lines() {
        if restaurant.contains(&args.category) {
            matching_restaurants.push(restaurant);
        }
    }

    if matching_restaurants.len() != 0 {
        println!("{}", get_random_restaurant(matching_restaurants, args.category)); 
    }
    else {
        println!("no matches found");
    }
}
