// use std::time::Instant;

use engine::state::State;

fn main() {
    let mut engine_state = State::new();
    // let now = Instant::now();
    let items = 10;
    for n in 0..items {
        // println!("{}", n)
        engine_state.offer_side.add_order(10 - n, 100 - n, 0);
    }
    for n in 0..items {
        // println!("{}", n)
        engine_state.bid_side.add_order(10 - n, 80 + n, 0);
    }
    // let ts1 = now.elapsed().as_micros();
    // println!("{}", ts1);
    // println!("Time per add_order = {}", ts1 / items);
    // println!("{}", engine_state.offer_side.orders.len());
    for _ in 0..items {
        // println!("{}", n)
        engine_state.offer_side.market_take(2);
    }
    for _ in 0..items {
        // println!("{}", n)
        engine_state.bid_side.market_take(3);
    }
    for n in 0..2 {
      // println!("{}", n)
      engine_state.bid_side.add_order(10 - n, 90 + n, 0);
  }
    // let ts2 = now.elapsed().as_micros() - ts1;
    // println!("{}", ts2);
    // println!("Time per market_take = {}", ts2 / items);
    // println!("{}", engine_state.offer_side.orders.len());
    // println!("{}", engine_state.offer_side.orders.len());
    let mut stack = Vec::new();
    println!("Top 5 Offers");
    for _ in 0..5 {
        match engine_state.offer_side.orders.pop() {
            Some((order, _)) => stack.push(order),
            None => {}
        }
    }

    while let Some(top) = stack.pop() {
        // Prints 3, 2, 1
        println!("{}", top);
    }
    println!("-------------------------------");
    for _ in 0..5 {
        match engine_state.bid_side.orders.pop() {
            Some((order, _)) => println!("{}", order),
            None => {}
        }
    }
}
