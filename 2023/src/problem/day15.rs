use std::time::{Duration, Instant};


use schema::Solver;

fn hash(input: &str) -> u32 {
    input.as_bytes().iter().fold(0, |a, &c| ((a + c as u32) * 17) % 256)
}

pub struct Day15;


impl Solver for Day15 {
    fn title(&self) -> &'static str {
        r"Lens Library"
    }

    fn part_a(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {

        let ts = Instant::now();

        let hashes = input.trim().split(',').collect::<Vec<_>>();

        let total: u32 = hashes.iter().map(|h| hash(h)).sum();
        
        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = total.into();

        Ok(d)
    }

    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {

        let ts = Instant::now();
        
        let lenses = input.trim().split(',').collect::<Vec<_>>();

        const INNER: Vec<(String, u8)> = vec![];
        let mut boxes = vec![INNER; 256];

        lenses.iter().for_each(|&lens| {
            let el = lens.find(|c| c == '=' || c == '-').unwrap();
            let (name, r) = lens.split_at(el);
            let (f, num) = r.split_at(1);

            let hash = hash(name);

            let lens_box = &mut boxes[hash as usize];
            if f == "=" {
                if let Some(lens_pos) = lens_box.iter().position(|l| l.0 == name) {
                    lens_box[lens_pos] = (name.to_string(), num.parse::<u8>().unwrap());
                } else {
                    lens_box.push((name.to_string(), num.parse::<u8>().unwrap()));
                }
            }
            else {
                lens_box.retain(|(ln, _)| ln != name);
            }

        });

        let total = boxes.iter().enumerate().fold(0, |a, (idx, l)| 
            l.iter().enumerate().map(|(b_idx, (_, l))| (b_idx + 1) * *l as usize * (idx + 1)).sum::<usize>() + a
        );

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = total.into();

        Ok(d)
    }

    fn is_empty(&self) -> bool {
        false
    }
}
