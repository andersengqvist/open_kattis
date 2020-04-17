use std::collections::HashMap;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut moves = String::new();
    io::stdin().read_line(&mut moves)?;

    // A state machine, just like in pragmatic programmer book
    // Should be easier to create a hashmap of hasmap though

    // State transitioning from position 1
    let mut state_1 = HashMap::new();
    state_1.insert('A', 2);
    state_1.insert('B', 1);
    state_1.insert('C', 3);

    let mut state_2 = HashMap::new();
    state_2.insert('A', 1);
    state_2.insert('B', 3);
    state_2.insert('C', 2);

    let mut state_3 = HashMap::new();
    state_3.insert('A', 3);
    state_3.insert('B', 2);
    state_3.insert('C', 1);

    let mut states = HashMap::new();
    states.insert(1, state_1);
    states.insert(2, state_2);
    states.insert(3, state_3);

    let mut curr_state = 1;
    for movee in moves.trim_end().chars() {
        // Getting stuff out of maps are hard
        curr_state = *states.get(&curr_state)
                            .and_then(|swap| swap.get(&movee))
                            .unwrap_or(&0);
    }

    println!("{}", curr_state);

    Ok(())
}