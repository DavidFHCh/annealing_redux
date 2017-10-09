extern crate rand;

use self::rand::{Rng, SeedableRng, XorShiftRng};
use solution::{DistMatrix, Solution};

pub struct Annealer {
    batch_size: u32,
    rng: XorShiftRng,
    accepted_percent: f64,
    s_init: Solution,
    init_temp: f64,
    min_temp: f64,
    e_p: f64,
    phi: f64,
}

impl Annealer {
    pub fn new(
        mut city_ids: Vec<u16>,
        bs: u32,
        seed: [u32; 4],
        ap: f64,
        it: f64,
        mt: f64,
        ep: f64,
        phi: f64,
        dists: DistMatrix,
    ) -> Annealer {
        let mut rng: XorShiftRng = SeedableRng::from_seed(seed);
        rng.shuffle(&mut city_ids);
        let s = Solution::new(city_ids, dists);
        let mut anneal = Annealer {
            batch_size: bs,
            rng: rng,
            accepted_percent: ap,
            s_init: s,
            init_temp: it,
            min_temp: mt,
            e_p: ep,
            phi: phi,
        };
        anneal.init_temp = anneal.initial_temperature(it);
        anneal
    }

    pub fn threshold_accepting(&mut self) -> Vec<Solution> {
        use std::f64::INFINITY;
        let mut p = INFINITY;
        let mut t = self.init_temp;
        let mut s = self.s_init.clone();
        let mut solutions: Vec<Solution> = Vec::new();
        let mut s_best = s.clone();
        let mut p_new;


        while t > self.min_temp {
            p_new = 0.0;
            while (p - p_new).abs() > self.e_p {
                p_new = p;
                let (pp, ss, sb) = self.make_batch(s, t);
                p = pp;
                s = ss;
                if sb.cost < s_best.cost {
                    s_best = sb;
                }
                //println!(
                //    "cost: {:.8?} \ttemp:{:.8?} \tcost_diff: {:.8?}",
                //    s.cost,
                //    t,
                //    (p - p_new).abs()
                //);
                solutions.push(s.clone());
            }
            t *= self.phi;
        }

        solutions.push(s_best.clone());
        solutions
    }

    fn make_batch(&mut self, mut s: Solution, t: f64) -> (f64, Solution, Solution) {
        let mut c = 0;
        let mut r = 0.0;
        let mut tries = 0;
        let mut s_new: Solution;
        let max_tries = self.batch_size * self.batch_size;
        let mut s_best = s.clone();

        while c < self.batch_size {
            if tries >= max_tries {
                break;
            }
            tries += 1;
            s_new = s.neighbor(&mut self.rng);
            if s_new.cost < s.cost + t {
                s = s_new;
                c += 1;
                r += s.cost
            }
            if s.cost <= s_best.cost {
                s_best = s.clone();
            }
        }

        (r / (self.batch_size as f64), s, s_best)
    }

    fn initial_temperature(&mut self, mut t: f64) -> f64 {
        let s = self.s_init.clone();
        let mut p_n = self.accepted_percentage(s.clone(), t);
        let p = self.accepted_percent;
        let e_p: f64 = 0.01;
        let t1: f64;
        let t2: f64;
        if (p - p_n).abs() < e_p {
            return t;
        }
        if p_n < p {
            while p_n < p {
                t = 2.0 * t;
                p_n = self.accepted_percentage(s.clone(), t);
            }
            t1 = t / 2.0;
            t2 = t;
        } else {
            while p_n > p {
                t = t / 2.0;
                p_n = self.accepted_percentage(s.clone(), t);
            }
            t1 = t;
            t2 = 2.0 * t;
        }
        self.binary_search(s.clone(), t1, t2)
    }

    fn binary_search(&mut self, s: Solution, t1: f64, t2: f64) -> f64 {
        let e_p = 0.01;
        let e_t = 0.01;
        let tm = (t1 + t2) / 2.0;
        let p = self.accepted_percent;

        if (t2 - t1) < e_t {
            return tm;
        }

        let p_n = self.accepted_percentage(s.clone(), tm);

        if (p - p_n).abs() < e_p {
            return tm;
        }

        if p_n > p {
            return self.binary_search(s, t1, tm);
        } else {
            return self.binary_search(s, tm, t2);
        }
    }

    fn accepted_percentage(&mut self, mut s: Solution, t: f64) -> f64 {
        let mut c = 0;
        let mut s_new: Solution;
        let n = 500;

        for _ in 0..n {
            s_new = s.neighbor(&mut self.rng);
            if s_new.cost <= s.cost + t {
                c += 1;
            }
            s = s_new;
        }
        (c as f64) / (n as f64)
    }
}
