use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;

fn main() -> Result<(), std::io::Error> {
    let mut line = String::new();

    loop {
        line.clear();
        io::stdin().read_line(&mut line)?;

        let v: Vec<u16> = line.trim_end()
                              .split(" ")
                              .map(|n: &str| match n.parse::<u16>() {
                                  Ok(num) => num,
                                  _ => 0
                              })
                              .collect();

        let hydra = Hydra {
            num_heads: v[0],
            num_tails: v[1]
        };

        if hydra.is_dead() {
            return Ok(())
        }

        println!("{}", fight_hydra(hydra));
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Hydra {
    num_heads: u16,
    num_tails: u16
}

impl Hydra {
    fn is_dead(&self) -> bool {
        self.num_heads == 0 && self.num_tails == 0
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Node {
    hydra: Hydra,
    depth: i32
}

// Fights the hydra, returns number of rounds before killed
// Return -1 if cannot be fought
fn fight_hydra(hydra: Hydra) -> i32 {

    if hydra.is_dead() {
        // No need to fight dead hydras
        return 0;
    }

    // Keep a list of seen hydras (or configurations of heads and tails)
    // so that we do not fight the same several times
    let mut seen_hydras: HashSet<Hydra> = HashSet::new();

    // We will do a breadth first search of the hydras to fight
    // the fight_order will be a queue of hydras
    let mut fight_order: VecDeque<Node> = VecDeque::new();

    seen_hydras.insert(hydra);
    fight_order.push_back(
        Node {
            hydra: hydra,
            depth: 0
        }
    ); // and pop_front

    loop {
        let curr_node = match fight_order.pop_front() {
            Some(n) => n,
            None => return -1
        };

        if let Some(child) = move_2(&curr_node) {
            if child.hydra.is_dead() {
                return child.depth
            }
            if !seen_hydras.contains(&child.hydra) {
                seen_hydras.insert(child.hydra);
                fight_order.push_back(child);
            }
        }

        if let Some(child) = move_3(&curr_node) {
            if child.hydra.is_dead() {
                return child.depth
            }
            if !seen_hydras.contains(&child.hydra) {
                seen_hydras.insert(child.hydra);
                fight_order.push_back(child);
            }
        }

        if let Some(child) = move_4(&curr_node) {
            if child.hydra.is_dead() {
                return child.depth
            }
            if !seen_hydras.contains(&child.hydra) {
                seen_hydras.insert(child.hydra);
                fight_order.push_back(child);
            }
        }
    }

}

// With the first move, Knight PyPy can cut off exactly one of Hydra’s heads.
// If PyPy cuts off exactly one head, a new head grows immediately.
// So why bother?

// With the second move, Knight PyPy can cut off exactly one of Hydra’s tails.
// If PyPy cuts off exactly one tail, two new tails grow immediately.
fn move_2(node: &Node) -> Option<Node> {
    if node.hydra.num_tails >= 1 {
        Some(
            Node {
                hydra: Hydra {
                    num_heads: node.hydra.num_heads,
                    num_tails: node.hydra.num_tails + 1
                },
                depth: node.depth + 1
            }
        )
    }
    else {
        None
    }
}

// With the third move, Knight PyPy can cut off exactly two of Hydra’s heads.
// If PyPy cuts off exactly two heads, nothing happens.
fn move_3(node: &Node) -> Option<Node> {
    if node.hydra.num_heads >= 2 {
        Some(
            Node {
                hydra: Hydra {
                    num_heads: node.hydra.num_heads - 2,
                    num_tails: node.hydra.num_tails
                },
                depth: node.depth + 1
            }
        )
    }
    else {
        None
    }
}

// With the fourth move, Knight PyPy can cut off exactly two of Hydra’s tails.
//If PyPy cuts off exactly two tails, a new head grows immediately.
fn move_4(node: &Node) -> Option<Node> {
    if node.hydra.num_tails >= 2 {
        Some(
            Node {
                hydra: Hydra {
                    num_heads: node.hydra.num_heads + 1,
                    num_tails: node.hydra.num_tails - 2
                },
                depth: node.depth + 1
            }
        )
    }
    else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fight_3_3() {
        assert_eq!(fight_hydra(Hydra {num_heads: 3, num_tails: 3}) , 9);
    }

    #[test]
    fn test_fight_1_1() {
        assert_eq!(fight_hydra(Hydra {num_heads: 1, num_tails: 1}) , 3);
    }

}