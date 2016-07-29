extern crate glob;
extern crate regex;
extern crate time;

use glob::Pattern;
use regex::Regex;
use std::boxed::Box;
use time::PreciseTime;

fn main() {
    let runner = Runner {
        regex: Box::new(Regex::new(".*.weedmaps.com").unwrap()),
        glob: Box::new(Pattern::new("*.weedmaps.com").unwrap())
    };
    bench_regex(&runner);
    bench_glob(&runner);
}

fn bench_regex(runner: &Runner) {
    println!("1,000,000 regex matches and mismatches");
    bench(&runner, |ref runner| {
        runner.regex.is_match("www.weedmaps.com");
        runner.regex.is_match("www.growone.com");
    });
}

fn bench_glob(runner: &Runner) {
    println!("1,000,000 glob matches and mismatches");
    bench(&runner, |ref runner| {
        runner.glob.matches("www.weedmaps.com");
        runner.glob.matches("www.growone.com");
    });
}

fn bench<F>(runner: &Runner, op: F) where F: Fn(&Runner) {
    let mut iter = 0..1000000;
    let start = PreciseTime::now();
    while let Some(_) = iter.next() {
        op(&runner);
    };
    let duration = start.to(PreciseTime::now());
    println!("took {:?} s total", duration.num_milliseconds() as f64 / 1000.0);
    println!("aver {:?} s/iter", duration.num_milliseconds() as f64 / 1000000.0);
}

pub struct Runner {
    regex: Box<Regex>,
    glob: Box<Pattern>
}

#[cfg(test)]
mod tests {
    use super::*;
    use glob::Pattern;
    use regex::Regex;

    fn build_weedmaps_runner() -> Runner {
        Runner {
            regex: Box::new(Regex::new(".*.weedmaps.com").unwrap()),
            glob: Box::new(Pattern::new("*.weedmaps.com").unwrap())
        }
    }

    #[test]
    fn regex_works() {
        let runner = build_weedmaps_runner();
        assert!(runner.regex.is_match("www.weedmaps.com"));
        assert!(!runner.regex.is_match("www.growone.com"));
    }

    #[test]
    fn glob_works() {
        let runner = build_weedmaps_runner();
        assert!(runner.glob.matches("www.weedmaps.com"));
        assert!(!runner.glob.matches("www.growone.com"));
    }
}
