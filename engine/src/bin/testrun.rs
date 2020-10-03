use engine::state::State;

fn main() {
    let mut engine_state = State::new();
    let items = 10;
    for n in 0..items {
        engine_state.offer_side.add_order(10 - n, 100 - n, 0);
    }
    for n in 0..items {
        engine_state.bid_side.add_order(10 - n, 80 + n, 0);
    }
    for n in 1..10 {

        match n {
            1 | 3 | 5 | 7 => {
                println!("Buing {} shares!", n);
                engine_state.offer_side.market_take(n);
            }

            _ => {
                println!("Selling {} shares!", n);
                engine_state.bid_side.market_take(n);
            }
        }
        // match engine_state.offer_side.orders.peek() {
        //     Some((order, _)) => println!("{}", order),
        //     None => println!("Empty"),
        // }
        // match engine_state.bid_side.orders.peek() {
        //     Some((order, _)) => println!("{}", order),
        //     None => println!("Empty"),
        // }
        let mut stack = Vec::new();
        println!("Order Book");
        println!("##################");
        let mut state_copy = engine_state.clone();
        for _ in 0..5 {
            match state_copy.offer_side.orders.pop() {
                Some((order, _)) => stack.push(order),
                None => println!("Empty"),
            }
        }

        while let Some(top) = stack.pop() {
            // Prints 3, 2, 1

            println!("{}", top);
        }
        println!("-------------------------------");
        for _ in 0..5 {
            match state_copy.bid_side.orders.pop() {
                Some((order, _)) => println!("{}", order),
                None => println!("Empty"),
            }
        }
    }
}
