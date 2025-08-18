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
            .map(|price| {
                let adjusted = price * adjustment_factor;
                (adjusted * 100.0).round() / 100.0
            })
            .collect();

        let sum_adjusted: f32 = adjusted_prices.iter().sum();
        let discrepancy = total_after_discount - sum_adjusted;
        if discrepancy.abs() > 0.001 {
            if let Some(max_idx) = adjusted_prices
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(idx, _)| idx)
            {
                adjusted_prices[max_idx] += discrepancy;
                adjusted_prices[max_idx] = (adjusted_prices[max_idx] * 100.0).round() / 100.0;
            }
        }

        adjusted_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        self.receipt = adjusted_prices.clone();
        adjusted_prices
    }
}