#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some((name, price)) = s.products.iter().find(|(n, _)| *n == ele) {
            self.items.push((name.clone(), *price));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, p)| *p).collect();

        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut discounted: Vec<f32> = prices.clone();
        let mut i = 0;

        while i + 2 < prices.len() {
            let group = &prices[i..i + 3];
            let min_price = group.iter().cloned().fold(f32::INFINITY, f32::min);

            let group_sum: f32 = group.iter().sum();

            for j in 0..3 {
                let idx = i + j;
                let proportion = prices[idx] / group_sum;
                discounted[idx] = (prices[idx] - proportion * min_price * 1.0).round_to_cents();
            }
            i += 3;
        }

        discounted = discounted.into_iter().map(|x| x.round_to_cents()).collect();

        self.receipt = discounted.clone();
        self.receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());

        self.receipt.clone()
    }
}

trait RoundTwo {
    fn round_to_cents(self) -> f32;
}

impl RoundTwo for f32 {
    fn round_to_cents(self) -> f32 {
        (self * 100.0).round() / 100.0
    }
}
