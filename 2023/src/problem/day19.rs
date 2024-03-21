use std::time::{Duration, Instant};

use ahash::{HashMap, HashMapExt};


use schema::Solver;


#[derive(Debug, PartialEq, Eq)]
enum Compare {
    GT,
    LT,
    None
}

fn parse(input: &str) -> AsmIns<'_> {

    let (rs, ps) = input.split_once("\n\n").unwrap();

    let mut rules = HashMap::with_capacity(rs.len());
    let mut parts = Vec::with_capacity(ps.len());

    for r in rs.lines() {

        let mut ruleset = Ruleset::default();

        let (n, rule_s) = r.split_once('{').unwrap();

        let mut rts = Vec::new();
        for rt in rule_s.trim_end_matches('}').split(',') {
            let Some((ops, res)) = rt.split_once(':') else {
                rts.push(Rule::Default(rt));
                continue;
            };

            let pt: Vec<_> = ops.split_inclusive(|c| c == '<' || c == '>' ).collect();
            let (p, op) = pt[0].split_at(1);
            let val = pt[1];

            let out = match res.len() {
                1 if res == "A" || res == "R" => Outcome::from(*res.as_bytes().first().unwrap()),
                2 | 3 => Outcome::from(res),
                _ => panic!("Invalid Outcome"),
            };

            let mut top = Ops::new();

            top.p = p.as_bytes()[0];
            top.op = match op.as_bytes()[0] {
                b'>' => Compare::GT,
                b'<' => Compare::LT,
                _ => unreachable!()
            };
            top.value = val.parse::<u32>().expect("Invalid value");

            top.init_path(out);

            rts.push(Rule::Rule(top));

        }
        ruleset.0.extend(rts);

        rules.insert(n, ruleset);

    }

    for p in ps.lines() {
        let inner = p.trim_matches(|c| c == '{' || c == '}').split(',');

        let mut num = Part::default();
        for t in inner {
            let (n, v) = t.split_once('=').unwrap();

            match n {
                "x" => num.x = v.parse::<u32>().unwrap(),
                "m" => num.m = v.parse::<u32>().unwrap(),
                "a" => num.a = v.parse::<u32>().unwrap(),
                "s" => num.s = v.parse::<u32>().unwrap(),
                _   => unreachable!()
            }
        }

        parts.push(num);

    }

    AsmIns { rules, parts }

}

#[derive(Debug, Default, PartialEq, Eq)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32
}

impl Part {
    fn get(&self, p: u8) -> u32 {
        match p {
            b'x' => self.x,
            b'm' => self.m,
            b'a' => self.a,
            b's' => self.s,
            _ => unreachable!()
        }
    }

    fn sum(&self) -> u64 {
        self.x as u64 + self.m as u64 + self.a as u64 + self.s as u64
    }
}

#[derive(Debug)]
struct Ruleset<'a> (Vec<Rule<'a>>);

impl Default for Ruleset<'_> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

#[derive(Debug, Default)]
struct AsmIns<'a> {
    rules: HashMap<&'a str, Ruleset<'a>>,
    parts: Vec<Part>
}

#[derive(Debug, Default)]
enum Rule<'a> {
    Rule(Ops<'a>),
    Default(&'a str),
    #[default]
    None
}

#[derive(Debug)]
struct Ops<'a> {
    p: u8,
    op: Compare,
    value: u32,
    dpath: Box<Outcome<'a>>
}


impl<'a> Ops<'a> {
    fn init_path(&mut self, path: Outcome<'a>) {
        self.dpath = Box::new(path);
    }

    fn new() -> Self {
        Self { p: 0, op: Compare::None, value: 0, dpath: Box::new(Outcome::None) }
    }

    fn idx(&self) -> usize {
        match self.p {
            b'x' => 0,
            b'm' => 1,
            b'a' => 2,
            b's' => 3,
            _ => unreachable!()
        }
    }

}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Outcome<'a> {
    Reject = b'R',
    Accept = b'A',
    R(&'a str),
    None,
}

impl Outcome<'_> {
    fn as_str(&self) -> &'_ str {
        match self {
            Self::Accept => "A",
            Self::Reject => "R",
            Self::R(a)   => *a,
            _ => unreachable!()
        }
    }
}

impl From<u8> for Outcome<'_> {
    fn from(value: u8) -> Self {
        match value {
            b'A' => Self::Accept,
            b'R' => Self::Reject,
            _    => unreachable!()
        }
    }
}

impl<'a> From<&'a str> for Outcome<'a> {
    fn from(value: &'a str) -> Self {
        Self::R(value)
    }
}


fn size(ranges: [(u32, u32); 4]) -> u64 {
    ranges.iter().map(|&(f, s)| s as u64 - f as u64 + 1).fold(1, |acc, i| acc * i)
}

fn dyn_part_b(rules: &HashMap<&str, Ruleset<'_>>, mut range: [(u32, u32); 4], inp: &str) -> u64 {
    let mut ans = 0;

    let mut solve = |range: [(u32, u32); 4], dest: &str| {
        if dest == "A" {
            ans += size(range);
        }
        else if dest != "R" {
            ans += dyn_part_b(rules, range, dest);
        }
    };

    for rule in &rules.get(inp).unwrap().0 {
        match rule {
            Rule::Rule(a) => {
                                let mut n_r = range;
                                let v = &mut n_r[a.idx()];
                                let r = &mut range[a.idx()];
                                match a.op {
                                    Compare::GT if v.1 > a.value => {
                                        v.0 = v.0.max(a.value + 1);
                                        r.1 = r.1.min(a.value);
                                    },
                                    Compare::LT if v.0 < a.value => {
                                        v.1 = v.1.min(a.value - 1);
                                        r.0 = r.0.max(a.value);
                                    },
                                    _ => continue
                                }

                                solve(n_r, a.dpath.as_str());

                            },
            Rule::Default(a) => {
                                solve(range, a);
                            },
            Rule::None => panic!(),
        }

    }
    

    ans
}

pub struct Day19;

impl Solver for Day19 {
    fn title(&self) -> &'static str {
        r"Aplenty"
    }

    fn part_a(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {

        let mut ans = 0;
        let ts = Instant::now();

        let part_list = parse(input);

        for part in part_list.parts {
            let mut pt: String = "in".to_string();

            loop {
                let work = part_list.rules.get(pt.as_str()).expect("BAD PARSE");

                for rule in &work.0 {
                    match rule {
                        Rule::Rule(a) => {
                                            if match a.op {
                                                Compare::GT => {part.get(a.p) > a.value},
                                                Compare::LT => {part.get(a.p) < a.value},
                                                _ => panic!("BAD OP")
                                            } {
                                                pt = a.dpath.as_ref().as_str().to_string();
                                                break;
                                            }

                                        },
                        Rule::Default(a) => {
                                            pt = a.to_owned().to_string();
                                            break;
                                        },
                        Rule::None => panic!(),
                    }

                }

                if pt == "A" {
                    ans += part.sum();
                    break;
                }
                else if pt == "R" {
                    break;
                }

            }
        }

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = ans.into();

        Ok(d)
    }

    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {

        let range = [(1, 4000); 4];

        let ts = Instant::now();

        let part_list = parse(input);

        let ans = dyn_part_b(&part_list.rules, range, "in");

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = ans.into();

        Ok(d)
    }

    fn is_empty(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod test {
    use std::{sync::Arc, sync::Mutex};

    use schema::*;

    use indoc::indoc;

    use super::Day19;

    const CASE: &str = indoc! {"
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    "};

    #[test]
    fn part_a() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day19.part_a(CASE, out.clone());
        assert_eq!(*out.lock().unwrap(), Answer::Number(19114));
    }

    #[test]
    fn part_b() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day19.part_b(CASE, out.clone());
        assert_eq!(*out.lock().unwrap(), Answer::Number(167409079868000_u64));
    }
}
