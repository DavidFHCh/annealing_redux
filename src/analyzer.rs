extern crate annealing_redux;
extern crate config;

use config::{Config, File, FileFormat, Value};
use annealing_redux as ar;
use ar::db::make_cities;
use ar::solution::{DistMatrix, Solution};
use std::rc::Rc;

fn main() {
    let dists = Rc::new(make_cities().unwrap());
    let mut c = Config::new();
    c.merge(File::new("Analyze", FileFormat::Toml).required(true))
        .expect("NO CONFIGURATION FILE 'Analyze.toml'");
    let solutions: Vec<Value> = c.get_array("solutions")
        .expect("No array of solutions (solutions) declared in Analyze.toml");

    for solution in solutions {
        let city_ids = to_u16_vec(solution.into_array().unwrap());
        analyze(city_ids, dists.clone());
    }
}

fn to_u16_vec(values: Vec<Value>) -> Vec<u16> {
    let mut v = Vec::with_capacity(values.len());
    for vs in values.clone() {
        v.push(vs.into_int().unwrap() as u16);
    }
    v
}

fn analyze(city_ids: Vec<u16>, dists: DistMatrix) {
    let sol = Solution::new(city_ids, dists);
    println!("{}", sol);
}
