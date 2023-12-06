use std::{collections::HashMap, u32::MAX, vec};

use itertools::Itertools;
const TEST_INPUT: &str = include_str!("../data/day05/test.txt");
const REAL_INPUT: &str = include_str!("../data/day05/input.txt");

pub fn solution() -> (usize, usize) {
    let (seeds, rest) = REAL_INPUT.split_once("\n\n").unwrap();

    let (_, seeds) = seeds.split(": ").collect_tuple().unwrap();
    let seeds = seeds
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_vec();

    let layers = rest
        .split("\n\n")
        .map(|r| {
            r.lines()
                .skip(1)
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_vec()
        })
        .collect_vec();

    let part1 = layers.iter().fold(seeds.clone(), |seeds, m| {
        seeds
            .iter()
            .map(|seed| {
                m.iter()
                    .find(|&&(_, src, range)| (src..src + range).contains(seed))
                    .map(|(dst, src, _)| dst + seed - src)
                    .unwrap_or(*seed)
            })
            .collect_vec()
    });

    let seeds = seeds.into_iter().tuples().map(|(s, len)| (s, s + len)).collect_vec();
    let locations = layers.iter().fold(seeds, |seeds, mappings|
    seeds.iter().flat_map(|&(start, end): &(usize, usize)| {
      let mut mapped = Vec::new();
      let mut unmapped = vec![(start, end)];
      for &(dst, src, len) in mappings {
        let mut m = Vec::new();
        for (start, end) in unmapped {
          let a = (start, end.min(src));
          let b = (start.max(src), (src+len).min(end));
          let c = ((src+len).max(start), end);
          if a.0 < a.1 { m.push(a); }
          if b.0 < b.1 { mapped.push((b.0-src+dst, b.1-src+dst)); }
          if c.0 < c.1 { m.push(c); }
        }
        unmapped = m;
      }
      mapped.extend(unmapped);
      mapped
    }).collect()
  );
    let part2 = locations.into_iter().map(|r| r.0).min().unwrap();
    (*part1.iter().min().unwrap(), part2)
}
