use regex::Regex;
use std::io::BufRead;

#[derive(Debug)]
pub struct Dimacs {
    pub n_vars: usize,
    pub clauses: Vec<Vec<i64>>,
}

pub fn parse_dimacs_from_buf_reader<F>(reader: &mut F) -> Dimacs
where
    F: std::io::BufRead,
{
    let mut n_clauses = 0usize;
    let mut n_vars = 0usize;
    let mut clauses = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('c') {
            continue;
        } else if line.starts_with('p') {
            let re_cnf = Regex::new(r"p\s+cnf\s+(\d+)\s+(\d+)").unwrap();
            if let Some(cap) = re_cnf.captures(&line) {
                n_vars = cap[1].parse().unwrap();
                n_clauses = cap[2].parse().unwrap();
            }
        } else {
            let re = Regex::new(r"(-?\d+)").unwrap();
            let mut cl = vec![];
            for (_, cap) in re.captures_iter(&line).enumerate() {
                let l = match cap[1].parse::<i64>().unwrap() {
                    0 => continue,
                    n => n,
                };

                cl.push(l);
            }
            clauses.push(cl);
            if clauses.len() == n_clauses {
                break;
            }
        }
    }

    Dimacs { n_vars, clauses }
}
