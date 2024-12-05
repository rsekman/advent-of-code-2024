use std::error::Error;
use std::io::prelude::*;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type TopologicalOrder = HashMap<u32, HashSet<u32>>;

type Update = Vec<u32>;

fn restrict_topological_order(vs: HashSet<u32>, full: &TopologicalOrder) -> TopologicalOrder {
    let get_or_empty = |v| full.get(v).cloned().unwrap_or(HashSet::new());
    vs.iter()
        .map(|v| {
            (
                *v,
                HashSet::intersection(&vs, &get_or_empty(v))
                    .copied()
                    .collect(),
            )
        })
        .collect()
}

fn is_topologically_sorted(upd: &Update, order: &TopologicalOrder) -> bool {
    let mut dag = restrict_topological_order(HashSet::from_iter(upd.iter().cloned()), order);
    for v in upd {
        if dag.get(&v).map(HashSet::len).unwrap_or(0) > 0 {
            return false;
        }
        for (_, deps) in dag.iter_mut() {
            deps.remove(&v);
        }
    }
    true
}

fn visit(v: u32, vs: &mut HashSet<u32>, dag: &TopologicalOrder, out: &mut VecDeque<u32>) {
    if !vs.contains(&v) {
        return;
    }
    for u in dag.get(&v).cloned().unwrap_or(HashSet::new()) {
        visit(u, vs, dag, out)
    }
    vs.remove(&v);
    out.push_front(v);
}

fn sort_topologically(upd: &Update, order: &TopologicalOrder) -> Update {
    let mut vs = HashSet::from_iter(upd.iter().cloned());
    let dag = restrict_topological_order(vs.clone(), order);

    let mut out = VecDeque::new();
    for v in vs.clone() {
        visit(v, &mut vs, &dag, &mut out);
    }
    out.into_iter().collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();

    let order_lock = stdin.lock();
    let mut order = TopologicalOrder::new();
    for line in order_lock.lines() {
        let line = line?;
        if line == "" {
            break;
        }
        let mut s = line.split("|");
        let before = s
            .next()
            .ok_or(format!("Failed to parse line: {}", line))?
            .parse()?;
        let after = s
            .next()
            .ok_or(format!("Failed to parse line: {}", line))?
            .parse()?;

        if let Some(s) = order.get_mut(&after) {
            s.insert(before);
        } else {
            order.insert(after, HashSet::from([before]));
        };
    }

    let mut page_sum_sorted = 0;
    let mut page_sum_unsorted = 0;
    let pages_lock = stdin.lock();
    for line in pages_lock.lines() {
        let line = line?;
        let u: Update = line.split(",").filter_map(|s| s.parse().ok()).collect();
        if is_topologically_sorted(&u, &order) {
            page_sum_sorted += u[u.len() / 2]
        } else {
            let w = sort_topologically(&u, &order);
            page_sum_unsorted += w[w.len() / 2]
        }
    }
    println!(
        "Sum of middle pages of correctly sorted updates: {}",
        page_sum_sorted
    );
    println!(
        "Sum of middle pages of incorrectly sorted updates: {}",
        page_sum_unsorted
    );

    return Ok(());
}
