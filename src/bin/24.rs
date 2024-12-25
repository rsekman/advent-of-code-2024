use std::cell::RefCell;
use std::error::Error;
use std::io::prelude::*;
use std::rc::Rc;

use std::collections::{BTreeMap, BTreeSet};

use itertools::iproduct;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, newline, space1},
    combinator::{map, opt, value},
    multi::separated_list0,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

// DATA TYPES

#[derive(Copy, Clone)]
enum Op {
    And,
    Or,
    Xor,
}

// Enum for the Components of a circuit, i.e. Gates and incoming Wires
// The Rc<RefCell<_>> pattern allows mutable access to the Components.
type ComponentRef = Rc<RefCell<Component>>;
enum Component {
    Wire {
        value: bool,
    },
    Gate {
        left: ComponentRef,
        right: ComponentRef,
        op: Op,
    },
}
// A backtrace from a single Component can be easily built by tree traversal, but if we want the
// full DAG we'll need to put the Components into something that can be indexed
type Netlist<'a> = BTreeMap<&'a str, ComponentRef>;

impl Component {
    // Set the value of a Wire. Does nothing if the Component is a Gate.
    fn set(&mut self, new: bool) {
        match self {
            Component::Wire { value } => *value = new,
            _ => {}
        };
    }

    fn eval(&self) -> bool {
        use Component::*;
        match self {
            Wire { value } => *value,
            Gate { left, right, op } => {
                // If the upstream values could not change, it would make sense to cache the result
                // of eval() in the Gate itself. Because upstream values *are* mutable, we
                // cannot do this and must re-evaluate the whole chain anew on each call.
                // Of course, cache invalidation could be implemented.
                let (lv, rv) = (left.borrow().eval(), right.borrow().eval());
                match op {
                    Op::Or => lv || rv,
                    Op::And => lv && rv,
                    Op::Xor => lv ^ rv,
                }
            }
        }
    }
}

// A RawComponent is a Component referring to its inputs by their names
enum RawComponent<'a> {
    Wire(&'a str, bool),
    Gate(&'a str, &'a str, Op, &'a str),
}
impl<'a> RawComponent<'a> {
    fn name(&self) -> &'a str {
        match self {
            RawComponent::Wire(n, _) => n,
            RawComponent::Gate(n, _, _, _) => n,
        }
    }
}
type RawNetlist<'a> = BTreeMap<&'a str, RawComponent<'a>>;

// PARSERS

fn parse_wire(input: &str) -> IResult<&str, RawComponent> {
    map(
        separated_pair(
            alphanumeric1,
            tag(": "),
            alt((value(false, char('0')), value(true, char('1')))),
        ),
        |(n, v)| RawComponent::Wire(n, v),
    )(input)
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    alt((
        value(Op::And, tag("AND")),
        value(Op::Or, tag("OR")),
        value(Op::Xor, tag("XOR")),
    ))(input)
}

fn parse_gate(input: &str) -> IResult<&str, RawComponent> {
    map(
        separated_pair(
            tuple((alphanumeric1, space1, parse_op, space1, alphanumeric1)),
            delimited(space1, tag("->"), space1),
            alphanumeric1,
        ),
        |((l, _, op, _, r), out)| RawComponent::Gate(out, l, op, r),
    )(&input)
}

fn parse_component(input: &str) -> IResult<&str, RawComponent> {
    alt((parse_wire, parse_gate))(input)
}

fn parse_rawnetlist(input: &str) -> IResult<&str, RawNetlist> {
    separated_list0(newline, opt(parse_component))(&input).map(|(s, v)| {
        (
            s,
            v.into_iter()
                .filter_map(|x| x)
                .map(|c| (c.name(), c))
                .collect(),
        )
    })
}

// LIBRARY

// Topologically sort a RawNetlist, i.e. a graph of gates and wires identified by strings
fn sort_topologically<'a>(netlist: &'a RawNetlist) -> Vec<&'a RawComponent<'a>> {
    use RawComponent::*;
    let mut vs = BTreeSet::new(); // Visited components

    // Post-order depth first traveral gives a valid topological order
    let mut stack: Vec<&'a str> = netlist.iter().map(|(&k, _)| k).collect();
    let mut out = Vec::new();
    while stack.len() > 0 {
        let &v = stack.last().unwrap();
        if vs.contains(&v) {
            // This component has already been visited
            stack.pop();
            continue;
        }
        let c = netlist.get(v).unwrap();
        match c {
            // Wires are leaves
            Wire(_, _) => {
                out.push(c);
                vs.insert(v);
                stack.pop();
            }
            Gate(_, left, _, right) => {
                // *Post-order* traversal; see also solution to day 05
                let push_child = |&n| {
                    if !vs.contains(n) && netlist.contains_key(n) {
                        stack.push(n);
                        true
                    } else {
                        false
                    }
                };
                if vec![left, right].into_iter().any(push_child) {
                    continue;
                }
                // Both children are marked visited, now we can visit this node
                out.push(c);
                vs.insert(v);
                stack.pop();
            }
        }
    }
    out
}

// Build a RawNetlist into a Netlist, i.e., a graph of gates and wires connected by Rc<RefCell<_>>:s
fn build_netlist<'a>(raw: &'a RawNetlist<'a>) -> Result<Netlist<'a>, String> {
    // The Netlist cannot be built with try_collect() because the dependencies of a node must exist
    // in the netlist before that node can be inserted, but it can be built with
    // try_fold()!
    let to_rcrc = |c| Rc::new(RefCell::new(c));
    let work = |mut netlist: Netlist<'a>, c: &RawComponent<'a>| {
        let get_or_err = |v, out| {
            netlist
                .get(v)
                .ok_or(format!(
                    "Invalid netlist: {out} needs input {v}, but {v} is not in the netlist."
                ))
                .map(Rc::clone)
        };
        match c {
            RawComponent::Wire(n, v) => netlist.insert(n, to_rcrc(Wire { value: *v })),
            RawComponent::Gate(out, l, op, r) => netlist.insert(
                out,
                to_rcrc(Gate {
                    left: get_or_err(l, out)?,
                    right: get_or_err(r, out)?,
                    op: *op,
                }),
            ),
        };
        Ok(netlist)
    };

    use Component::*;
    sort_topologically(&raw)
        .into_iter()
        .try_fold(Netlist::new(), work)
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input)?;

    let (_, raw_netlist) = parse_rawnetlist(&input).map_err(|e| format!("Invalid input: {e}"))?;
    let netlist = build_netlist(&raw_netlist)?;

    let mut out = 0;
    let mut n_bits_out = 0;
    for (n, c) in netlist.iter().rev() {
        if n.starts_with('z') {
            out <<= 1;
            let v = c.borrow().eval();
            out = out | (v as u64);
            n_bits_out += 1;
        }
    }
    println!("Output number: {out}");

    /* To solve part 2:
     * The code below tries adding (a << n) + (b << n) where a and b are 0 or 1.
     * This works because:
     *     - The k first bits of adding two n-bit numbers depend only on the first k bits of each
     *     number
     *     - The (n+1)th bit of (x + y), where x and y are n bits wide, depends on:
     *         - The n:th bit of (x_t + y_t) where x_t is x truncated to n-1 bits
     *         - The n:th bits of x and y
     *       To realize this, write x = x_n .. x_0, y = y_0 ... y 0
     *       Now x+y = x_n 000.. + y_n 000 + (x_{n-1} .. x_0 + y_{n-1} ... y_0);
     *       the two most significant bits are (x_n + y_n + (x+y)_{n})
     *     - Therefore, if the adder is correct when adding (n-1)-bit numbers AND it correctly adds
     *     (a << n) + (b << n) for all values of a and b, it is correct for n-bit numbers.
     *  When an incorrect output bit is detected the program exits.
     *  The general structure of the adder looks like this
     *  A      XOR B        -> z(N-1)
     *  A      AND B        -> C
     *  x(N-1) XOR  y(N-1)  -> D
     *  C OR D              -> E
     *  xN     XOR yN       -> F
     *  E XOR F             -> zN
     *  If the program finds that bit N is incorrect but bit N-1 was correct, start by identifying
     *  A and B, then follow the trace trying to find zN.
     *  Correct the wiring in the input file, then re-run the program.
     *  Repeat until the program exits without reporting incorrect bits.
     *  To put the answer into the format asked for, `diff` the edited file against the original
     *  input (or, log your edits (e.g, on paper) as you do them), then run your favourite sorting
     *  algorithm.
     */

    // First reset every input bit to 0
    for i in 0..n_bits_out {
        netlist
            .get(format!("x{i:02}").as_str())
            .map(|w| w.borrow_mut().set(false));
        netlist
            .get(format!("y{i:02}").as_str())
            .map(|w| w.borrow_mut().set(false));
    }

    'bitloop: for i in 1..n_bits_out {
        let set_wire = |n, v| netlist.get(n).map(|w| w.borrow_mut().set(v));
        let get = |n| netlist.get(n).map(|w| w.borrow().eval());

        let high = format!("z{i:02}");
        let low = format!("z{:02}", i - 1);
        let x = format!("x{:02}", i - 1);
        let y = format!("y{:02}", i - 1);

        for (xv, yv) in iproduct![vec![false, true], vec![false, true]] {
            set_wire(x.as_str(), xv);
            set_wire(y.as_str(), yv);

            let low_v = get(low.as_str()).ok_or(format!("Could not evaluate {low}"))? as u8;
            let high_v = get(high.as_str()).ok_or(format!("Could not evaluate {high}"))? as u8;
            let expected = (xv as u8) + (yv as u8);
            let out = (high_v << 1) + low_v;
            if out != expected {
                println!(
                    "Incorrect result of ({} << {shift}) + ({} << {shift})\nGot 0b{out:02b} << {shift}, expected 0b{expected:02b} << {shift}",
                    xv as u8,
                    yv as u8,
                    shift = i - 1,
                );

                break 'bitloop;
            }
        }

        // Reset for the next iteration
        set_wire(x.as_str(), false);
        set_wire(y.as_str(), false);
    }

    return Ok(());
}
