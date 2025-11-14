use std::string;

pub struct Article {
    pub name: String,
    pub price: u64,
}

pub struct Purchase {
    id: u32,
    pub article: Article,
    pub quantity: u64,
}

pub struct Cart {
    pub purchases: Vec<Purchase>,
}

impl Purchase {
    pub fn buy_total_price(&self) -> u64 {
        self.quantity * self.article.price
    }
    pub fn increase_quantity(&mut self) {
        self.quantity += 1;
    }
    pub fn decrease_quantity(&mut self) {
        self.quantity -= 1;
    }

}

impl Cart {
    pub fn new() -> Self {
        Cart {
            purchases: Vec::new(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.purchases.is_empty()
    }
    pub fn cart_total_price(&self) -> u64 {
        self.purchases.iter().map(|buy| buy.buy_total_price()).sum()
    }
    pub fn add(&mut self, buy: Purchase) {
        self.purchases.push(buy);
    }
    pub fn increase_buy_quantity(&mut self, id:u32) {
        let found = self.purchases.iter().position(|p| p.id == id);
        self.purchases[found.unwrap()].increase_quantity();
    }
    pub fn decrease_buy_quantity(&mut self, id:u32) {
        let found = self.purchases.iter().position(|p| p.id == id);
        self.purchases[found.unwrap()].decrease_quantity();
    }
    pub fn remove_buy(&mut self, id:u32) {
        let found = self.purchases.iter().position(|p| p.id == id);
        self.purchases.remove(found.unwrap());
    }
    pub fn total_number_articles(&mut self) -> u64 {
        let total_articles = self.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panier_vide_works() {
        let panier = Cart::new();
        assert!(panier.purchases.is_empty());
        assert_eq!(panier.purchases.len(), 0);
    }

    #[test]
    fn panier_avec_articles() {
        let mut cart = Cart::new();
        cart.purchases.push(Purchase {
            id: 1,
            article : Article{
                name : "tank".parse().unwrap(),
                price : 16000000
            },
            quantity: 2,
        });
        assert_eq!(cart.purchases.len(), 1);
    }
}
