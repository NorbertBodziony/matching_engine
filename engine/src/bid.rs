use priority_queue::PriorityQueue;

use crate::order::Order;
#[derive(Debug, Clone)]
pub struct BidSide {
    pub orders: PriorityQueue<Order, u128>,
    pub counter: u128,
}

impl BidSide {
    pub fn new() -> Self {
        return BidSide {
            orders: PriorityQueue::new(),
            counter: 0,
        };
    }
    pub fn add_order(&mut self, amount: u128, price: u128, filled: u128) {
        self.orders
            .push(Order::new(amount, price, filled, self.counter), price);
        self.counter += 1;
    }
    pub fn market_take(&mut self, mut amount: u128) {
        let mut should_pop: bool = false;
        loop {
            should_pop = false;
            match self.orders.peek_mut() {
                Some((order, _)) => {
                    if order.amount > amount + order.filled {
                        // println!("filled {}", order.filled);
                        order.set_filled(order.filled + amount);
                        // println!("fill {}", amount);

                        break;
                    } else if order.amount < amount + order.filled {
                        should_pop = true;
                        // println!("{}", amount);
                        // println!("{}", order.amount);
                        // println!("{}", order.filled);
                        amount = amount + order.filled - order.amount;
                    // println!("pop reduce {}", amount);
                    } else {
                        self.orders.pop();
                        // println!("pop");
                        break;
                    }
                }
                None => {
                    break;
                }
            }
            if should_pop == true {
                self.orders.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_offer_side() {
        let offer_side = BidSide::new();
        assert_eq!(offer_side.orders.is_empty(), true);
        assert_eq!(offer_side.counter, 0);
    }
    #[test]
    fn add_order() {
        let mut offer_side = BidSide::new();
        offer_side.add_order(12, 13, 0);
        offer_side.add_order(12, 12, 0);
        match offer_side.orders.peek() {
            Some(val) => {
                assert_eq!(val.1, &13);
            }
            None => {}
        }

        assert_eq!(offer_side.orders.is_empty(), false);
        assert_eq!(offer_side.orders.len(), 2);
        assert_eq!(offer_side.counter, 2);
    }
    #[test]
    fn market_take() {
        let mut offer_side = BidSide::new();
        offer_side.add_order(12, 13, 0);
        offer_side.add_order(12, 12, 0);
        offer_side.market_take(18);
        match offer_side.orders.peek() {
            Some((order, piority)) => {
                assert_eq!(order.filled, 6);
                assert_eq!(piority, &12);
            }
            None => {}
        }
        assert_eq!(offer_side.orders.is_empty(), false);
        assert_eq!(offer_side.orders.len(), 1);

        offer_side.market_take(6);
        assert_eq!(offer_side.orders.is_empty(), true);
    }
}
