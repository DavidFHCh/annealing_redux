extern crate rand;

use self::rand::{Rng};
use std;
use std::fmt;

#[derive(Clone)]
pub struct Solution<'a> {
    pub path: Vec<u16>,
    pub cost: f64,
    pub avg_d: f64,
    pub max_d: f64,
    pub dists: &'a [[f64; 278]; 278],
    pub seed: u32
}

impl<'a> Solution<'a> {

    pub fn new(path: Vec<u16>, dist: &[[f64; 278]; 278], seed: u32) -> Solution {
        let mut s = Solution{
            path: path,
            cost: 0.0,
            avg_d: 0.0,
            max_d: 0.0,
            dists: dist,
            seed: seed
        };

        let (max, avg) = s.max_dist();
        s.max_d = max;
        s.avg_d = avg;
        s.cost = s.cost();
        s
    }

    pub fn neighbor<T: Rng>(&self, rng: &mut T) -> Solution<'a> {
        let (path_new, i, j) = Solution::swap(self.path.clone(), rng);
        let new_c = self.new_cost(i,j);

        Solution {
            path: path_new,
            cost: new_c,
            avg_d: self.avg_d,
            max_d: self.max_d,
            dists: self.dists,
            seed: self.seed
        }
    }

    pub fn feasible(&self) -> bool {
        let mut f = true;
        for i in 0..self.path.len() - 1 {
            let d = self.dists[self.path[i] as usize][self.path[i+1] as usize];
            if d < 0.0{
                f = false;
                break;
            }
        }
        f
    }

    fn cost(&self) -> f64 {
        let mut sum = 0.0;
        let n = self.path.len() - 1;
        for i in 0..n {
            sum += self.distance_between(self.path[i], self.path[i+1]);
        }
        sum/(self.avg_d * (n as f64))
    }

    fn swap<T: Rng>(mut path: Vec<u16>, rng: &mut T) -> (Vec<u16>, usize, usize) {
        let mut i: usize = 0;
        let mut j: usize = 0;
        while i == j {
            i = rng.gen_range(0, path.len());
            j = rng.gen_range(0, path.len());
        }
        let tmp = path[i];
        path[i] = path[j];
        path[j] = tmp;
        (path, i, j)
    }

    fn new_cost(&self, i: usize, j: usize) -> f64 {
        let n = self.path.len() - 1;
        let path = &self.path;
        let mut new_c = self.cost * self.avg_d * (n as f64);

        if ((i as i32) - (j as i32)).abs() == 1 {
            let ii = std::cmp::min(i,j);
            let jj = std::cmp::max(i,j);
            if ii != 0 {
                new_c -= self.distance_between(path[ii-1], path[ii]);
                new_c += self.distance_between(path[ii-1], path[jj]);
            }
            if jj != n {
                new_c -= self.distance_between(path[jj], path[jj+1]);
                new_c += self.distance_between(path[ii], path[jj+1]);
            }
        } else {
            if i != 0 {
                new_c -= self.distance_between(path[i-1], path[i]);
                new_c += self.distance_between(path[i-1], path[j]);
            }
            if i != n {
                new_c -= self.distance_between(path[i], path[i+1]);
                new_c += self.distance_between(path[j], path[i+1]);
            }
            if j != 0 {
                new_c -= self.distance_between(path[j-1], path[j]);
                new_c += self.distance_between(path[j-1], path[i]);

            }
            if j != n {
                new_c -= self.distance_between(path[j], path[j+1]);
                new_c += self.distance_between(path[i], path[j+1]);
            }
        }
        
        new_c/(self.avg_d * (n as f64))
    }


    fn max_dist(&self) -> (f64, f64) {
        let mut max = 0.0;
        let mut avg = 0.0;
        let mut count = 0;

        for i in 0..self.path.len() {
            for j in (i+1)..self.path.len() {
                let d = self.dists[self.path[i] as usize][self.path[j] as usize];
                if d > 0.0 {
                    max = if d > max { d } else { max };
                    avg += d;
                    count +=1;
                }
            }
        }
        (max, avg/(count as f64))
    }
    
    fn distance_between(&self, id1: u16, id2:u16)  -> f64 {
        let disc_dist = 3.5 * self.max_d;
        let dist = self.dists[id1 as usize][id2 as usize];
        if (1.0 + dist).abs() < 0.0001 { disc_dist } else { dist }
    }
    

}

impl<'a> fmt::Display for Solution<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}\n cost:{} \tfeasible: {} \tseed: {}", self.path, self.cost, self.feasible(), self.seed)
    }
}
