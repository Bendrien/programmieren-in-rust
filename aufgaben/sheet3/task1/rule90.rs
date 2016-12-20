//! Task 3.1: Rule 90

fn main() {
    // TODO: Task 1c)
    let mut state = read_input();
    let mut str = String::with_capacity(state.len());
    loop {
        for cell in state.iter() {
            match *cell {
                true => str.push('█'),
                false => str.push(' '),
            }
        }
        println!("{}", str);
        str.clear();
        std::thread::sleep(std::time::Duration::from_millis(500));
        state = next_step(&state);
    }
}

/// Reads a valid initial configuration for our automaton from the terminal.
fn read_input() -> Vec<bool> {
    // This tries to read a string from the terminal, checks whether it's
    // valid (only contains 1's and 0's). If the user fails to input a correct
    // string, this routine will ask again until the user finally manages to
    // give us a correct string.
    //
    // You don't need to understand this routine yet; that's why I've written
    // it already ;-)
    //
    // You only need to use the `input` variable (of type `String`). You can
    // also assume that it only contains '0' and '1' chars.
    let input = {
        let mut buffer = String::new();

        loop {
            println!("Please give me the initial configuration (a string of '0' and '1'!):");
            buffer.clear();

            // `read_line` returns an error if the input isn't valid UTF8 or if
            // a strange IO error occured. We just panic in that case...
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("something went seriously wrong :O");

            if buffer.trim().chars().all(|c| c == '1' || c == '0') {
                break;
            }
        }

        buffer.trim().to_string()
    };

    // TODO: Task 1a)
    let mut v = Vec::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '0' => v.push(false),
            '1' => v.push(true),
            _ => panic!("Wrong inputdata!"),
        }
    }
    v
}

// TODO: Task 1b)
fn next_step(state: &[bool]) -> Vec<bool> {
    let len = state.len();
    let mut v = Vec::with_capacity(len);
    let left_neighbor  = state.iter().cycle().skip(len-1).take(len);
    let right_neighbor = state.iter().cycle().skip(1).take(len);
    for (l, r) in left_neighbor.zip(right_neighbor) {
        v.push((!*l ^ !*r));
    }
    v
}

#[test]
fn rule90_rules() {
    assert_eq!(next_step(&[false, false, false]), vec![false, false, false]);
    assert_eq!(next_step(&[ true, false, false]), vec![false,  true,  true]);
    assert_eq!(next_step(&[ true,  true, false]), vec![ true,  true, false]);
    assert_eq!(next_step(&[ true,  true,  true]), vec![false, false, false]);
}
