use std::fmt;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Order {
    pub amount: u128,
    pub price: u128,
    pub filled: u128,
    id: u128,
}

impl Order {
    pub fn new(amount: u128, price: u128, filled: u128, id: u128) -> Self {
        return Order {
            amount,
            price,
            filled,
            id,
        };
    }
    pub fn set_filled(&mut self, n: u128) {
        self.filled = n;
    }
}
impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "amount = {}, price = {} filled = {}",
            self.amount, self.price, self.filled
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_filled() {
        let mut test_order = Order::new(10, 10, 0, 0);
        test_order.set_filled(5);
        assert_eq!(test_order.filled, 5);
    }
    #[test]
    fn set_valid_id() {
        let test_order = Order::new(10, 10, 0, 1);
        let test_order2 = Order::new(10, 10, 0, 2);
        assert_eq!(test_order.id, 1);
        assert_eq!(test_order2.id, 2);
    }
}
