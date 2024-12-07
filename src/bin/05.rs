use std::error::Error;
use std::io::prelude::*;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

type TopologicalOrder = BTreeMap<u32, BTreeSet<u32>>;

type Update = Vec<u32>;

fn restrict_topological_order(vs: &BTreeSet<u32>, full: &TopologicalOrder) -> TopologicalOrder {
    let get_or_empty = |v| full.get(v).cloned().unwrap_or_else(BTreeSet::new);
    vs.iter()
        .map(|v| (*v, vs.intersection(&get_or_empty(v)).copied().collect()))
        .collect()
}

fn is_topologically_sorted(upd: &Update, order: &TopologicalOrder) -> bool {
    let mut dag = restrict_topological_order(&BTreeSet::from_iter(upd.iter().copied()), order);
    for v in upd {
        if dag.get(&v).map(BTreeSet::len).unwrap_or(0) > 0 {
            return false;
        }
        for (_, deps) in dag.iter_mut() {
            deps.remove(&v);
        }
    }
    true
}

fn sort_topologically(upd: &Update, order: &TopologicalOrder) -> Update {
    let mut vs = BTreeSet::from_iter(upd.iter().copied());
    let mut dag = restrict_topological_order(&vs, order);

    let mut stack = upd.clone();
    let mut out = VecDeque::new();
    while stack.len() > 0 {
        let &v = stack.last().unwrap();
        if !vs.contains(&v) {
            stack.pop();
            continue;
        }
        //println!("Considering {}", v);
        if let Some(d) = dag.get_mut(&v).and_then(|x| x.pop_first()) {
            stack.push(d);
        } else {
            out.push_front(v);
            stack.pop();
            vs.remove(&v);
        }
    }
    out.into_iter().rev().collect()
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
            order.insert(after, BTreeSet::from([before]));
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
