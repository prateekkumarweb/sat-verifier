use regex::Regex;
use std::env;
use std::ffi::CString;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::raw::{c_char, c_int};

mod dimacs;

extern "C" {
    fn drat_main(argc: c_int, argv: *const *const c_char) -> c_int;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 4 {
        panic!("c Usage: ./sat-verifier file.cnf file.out file.drat");
    }

    let cnf_file = File::open(&args[1]).unwrap();
    let out_file = File::open(&args[2]).unwrap();

    let mut cnf_reader = BufReader::new(cnf_file);
    let out_reader = BufReader::new(out_file);

    let dimacs = dimacs::parse_dimacs_from_buf_reader(&mut cnf_reader);

    let mut result = "".to_owned();
    let mut model = vec![];

    for line in out_reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('c') {
            continue;
        } else if line.starts_with('s') {
            let re_cnf = Regex::new(r"s\s+([A-Z]+)").unwrap();
            if let Some(cap) = re_cnf.captures(&line) {
                result = cap[1].parse().unwrap();
            }
        } else if line.starts_with('v') {
            let re = Regex::new(r"(-?\d+)").unwrap();
            for (_, cap) in re.captures_iter(&line).enumerate() {
                let v = match cap[1].parse::<i64>().unwrap() {
                    0 => break,
                    n => n,
                };
                model.push(v);
            }
        }
    }

    if result == "SATISFIABLE" {
        if model.len() != dimacs.n_vars {
            println!("c Incorrect number of vars");
            println!("s UNVERIFIED");
            return;
        }

        for i in 1..=dimacs.n_vars {
            if !model.contains(&(i as i64)) && !model.contains(&-(i as i64)) {
                println!("c var {} not found", i);
                println!("s UNVERIFIED");
                return;
            }
        }

        for cl in dimacs.clauses {
            let mut sat = false;
            for &l in cl.iter() {
                if model.contains(&l) {
                    sat = true;
                    break;
                }
            }
            if !sat {
                println!("c cl {:?} not satisfied", cl);
                println!("s UNVERIFIED");
                return;
            }
        }

        println!("s VERIFIED");
        return;
    } else if result == "UNSATISFIABLE" {
        unsafe {
            drat_main(
                3,
                [
                    CString::new("./drat-trim").unwrap().as_ptr(),
                    CString::new(args[1].as_bytes()).unwrap().as_ptr(),
                    CString::new(args[3].as_bytes()).unwrap().as_ptr(),
                ]
                .as_ptr(),
            );
        }
        return;
    }

    println!("c {}", result);
    println!("s UNVERIFIED");
}
