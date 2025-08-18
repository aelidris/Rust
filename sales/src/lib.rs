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
        if let Some(product) = s.products.iter().find(|(name, _)| *name == ele) {
            self.items.push((product.0.clone(), product.1));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let total_original: f32 = prices.iter().sum();
        let mut total_discount = 0.0;

        let groups = prices.len() / 3;
        for i in 0..groups {
            total_discount += prices[i * 3];
        }

        let total_after_discount = total_original - total_discount;

        if total_original == 0.0 {
            self.receipt = vec![];
            return self.receipt.clone();
        }

        let adjustment_factor = total_after_discount / total_original;
        let mut adjusted_prices: Vec<f32> = prices
            .iter()
            .map(|price| (price * adjustment_factor * 100.0).round() / 100.0)
            .collect();

        adjusted_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        self.receipt = adjusted_prices.clone();
        adjusted_prices
    }
}