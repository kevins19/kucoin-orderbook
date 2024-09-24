use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

/// Enum used to indicate the side of the market incremental
pub enum Direction {
    Buy,
    Sell,
}

/// Stores data related to each market incremental
pub struct Incremental {
    pub price: OrderedFloat<f64>,
    pub direction: Direction,
    pub quantity: i64,
}

/// Maintains the state of the orderbook
pub struct Orderbook {
    bids: BTreeMap<OrderedFloat<f64>, i64>,
    asks: BTreeMap<OrderedFloat<f64>, i64>,
}

impl Orderbook {
    /// Initializes an empty orderbook
    pub fn new() -> Orderbook {
        Orderbook {
            bids: BTreeMap::default(),
            asks: BTreeMap::default(),
        }
    }

    /// Processes a market incremental and uses it to update the state
    pub fn process(&mut self, inc: &Incremental) {
        match inc.direction {
            Direction::Buy => {
                if inc.quantity == 0 {
                    self.bids.remove(&inc.price);
                } else {
                    self.bids.insert(inc.price, inc.quantity);
                }
            }
            Direction::Sell => {
                if inc.quantity == 0 {
                    self.asks.remove(&inc.price);
                } else {
                    self.asks.insert(inc.price, inc.quantity);
                }
            }
        }
    }

    /// Display the orderbook in tabular form
    pub fn display(&mut self) {
        println!("-------------------------------------------------");
        println!(
            "{:<10} | {:<10} | {:<10} | {:<10}",
            "Bid Price", "Bid Qty", "Ask Price", "Ask Qty"
        );
        println!("{:-<10}-+-{:-<10}-+-{:-<10}-+-{:-<10}", "", "", "", "");
        let mut bid_iter = self.bids.iter().rev();
        let mut ask_iter = self.asks.iter();

        // print up to 10 lines for greater readability
        for _ in 0..10 {
            let bid = bid_iter.next();
            let ask = ask_iter.next();
            match (bid, ask) {
                (Some((bid_price, bid_qty)), Some((ask_price, ask_qty))) => {
                    println!(
                        "{:<10} | {:<10} | {:<10} | {:<10}",
                        bid_price, bid_qty, ask_price, ask_qty
                    );
                }
                (Some((bid_price, bid_qty)), None) => {
                    println!(
                        "{:<10} | {:<10} | {:<10} | {:<10}",
                        bid_price, bid_qty, "", ""
                    );
                }
                (None, Some((ask_price, ask_qty))) => {
                    println!(
                        "{:<10} | {:<10} | {:<10} | {:<10}",
                        "", "", ask_price, ask_qty
                    );
                }
                (None, None) => break,
            }
        }
        println!("-------------------------------------------------");
    }
}
