use std::string;

#[derive(Clone)]
pub struct Article {
    pub name: String,
    pub price: u64,
}

#[derive(Clone)]
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
        let mut total_articles = 0;
        for p in self.purchases.iter() {
            total_articles += p.quantity;
        }
        return total_articles
    }

    pub fn total_number_purchases(&self) -> u64 {
        let mut total_purchases = 0;
        for c in self.purchases.iter() {
            total_purchases += 1;
        }
        return total_purchases
    }

    pub fn empty_cart(&mut self) {
        self.purchases = Vec::new();
    }

    pub fn is_cart_empty(&self) -> bool {
        self.is_empty()
    }

    pub fn list_of_purchases_as_list(&self) -> Vec<Purchase> {
        let mut purchases_list = Vec::new();
        for p in self.purchases.iter() {
            purchases_list.push(p.clone());
        }
        return purchases_list
    }
    pub fn list_of_purchases_to_json(&self) -> String {
        let mut items = Vec::new();

        for purchase in &self.purchases {
            let item = format!(
                r#"{{"id":{},"article":{{"name":"{}","price":{}}},"quantity":{}}}"#,
                purchase.id,
                purchase.article.name,
                purchase.article.price,
                purchase.quantity
            );
            items.push(item);
        }

        format!(r#"{{"purchases":[{}]}}"#, items.join(","))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== Tests pour Purchase =====

    #[test]
    fn buy_total_price_works() {
        let purchase = Purchase {
            id: 1,
            article: Article {
                name: String::from("Book"),
                price: 20,
            },
            quantity: 3,
        };
        assert_eq!(purchase.buy_total_price(), 60);
    }

    #[test]
    fn increase_quantity_works() {
        let mut purchase = Purchase {
            id: 1,
            article: Article {
                name: String::from("Coffee"),
                price: 5,
            },
            quantity: 1,
        };
        purchase.increase_quantity();
        assert_eq!(purchase.quantity, 2);
    }

    #[test]
    fn decrease_quantity_works() {
        let mut purchase = Purchase {
            id: 1,
            article: Article {
                name: String::from("Juice"),
                price: 4,
            },
            quantity: 5,
        };
        purchase.decrease_quantity();
        assert_eq!(purchase.quantity, 4);
    }

    // ===== Tests pour Cart =====

    #[test]
    fn cart_new_works() {
        let cart = Cart::new();
        assert!(cart.purchases.is_empty());
    }

    #[test]
    fn is_empty_works() {
        let cart = Cart::new();
        assert!(cart.is_empty());
    }

    #[test]
    fn cart_total_price_works() {
        let mut cart = Cart::new();
        cart.add(Purchase {
            id: 1,
            article: Article {
                name: String::from("Mouse"),
                price: 25,
            },
            quantity: 2,
        });
        cart.add(Purchase {
            id: 2,
            article: Article {
                name: String::from("Keyboard"),
                price: 75,
            },
            quantity: 1,
        });
        assert_eq!(cart.cart_total_price(), 125); // (25*2) + (75*1)
    }

    #[test]
    fn add_works() {
        let mut cart = Cart::new();
        cart.add(Purchase {
            id: 1,
            article: Article {
                name: String::from("Tablet"),
                price: 500,
            },
            quantity: 1,
        });
        assert_eq!(cart.purchases.len(), 1);
    }

    #[test]
    fn increase_buy_quantity_works() {
        let mut cart = Cart::new();
        cart.add(Purchase {
            id: 1,
            article: Article {
                name: String::from("Notebook"),
                price: 5,
            },
            quantity: 3,
        });
        cart.increase_buy_quantity(1);
        assert_eq!(cart.purchases[0].quantity, 4);
    }

    #[test]
    fn decrease_buy_quantity_works() {
        let mut cart = Cart::new();
        cart.add(Purchase {
            id: 1,
            article: Article {
                name: String::from("Eraser"),
                price: 1,
            },
            quantity: 10,
        });
        cart.decrease_buy_quantity(1);
        assert_eq!(cart.purchases[0].quantity, 9);
    }

    #[test]
    fn remove_buy_works() {
        let mut cart = Cart::new();
        cart.add(Purchase {
            id: 1,
            article: Article {
                name: String::from("Cable"),
                price: 10,
            },
            quantity: 2,
        });
        cart.add(Purchase {
            id: 2,
            article: Article {
                name: String::from("Adapter"),
                price: 15,
            },
            quantity: 1,
        });
        cart.remove_buy(1);
        assert_eq!(cart.purchases.len(), 1);
        assert_eq!(cart.purchases[0].id, 2);
    }

    #[test]
    fn total_number_articles_works() {
        let mut cart = Cart::new();
        cart.add(Purchase {
            id: 1,
            article: Article {
                name: String::from("Socks"),
                price: 5,
            },
            quantity: 4,
        });
        cart.add(Purchase {
            id: 2,
            article: Article {
                name: String::from("Shoes"),
                price: 50,
            },
            quantity: 2,
        });
        assert_eq!(cart.total_number_articles(), 6);
    }

    #[test]
    fn total_number_purchases_works() {
        let mut cart = Cart::new();
        cart.add(Purchase {
            id: 1,
            article: Article {
                name: String::from("Hat"),
                price: 15,
            },
            quantity: 5,
        });
        cart.add(Purchase {
            id: 2,
            article: Article {
                name: String::from("Scarf"),
                price: 20,
            },
            quantity: 1,
        });
        assert_eq!(cart.total_number_purchases(), 2);
    }

    #[test]
    fn empty_cart_works() {
        let mut cart = Cart::new();
        cart.add(Purchase {
            id: 1,
            article: Article {
                name: String::from("Item"),
                price: 10,
            },
            quantity: 2,
        });
        cart.empty_cart();
        assert!(cart.purchases.is_empty());
    }

    #[test]
    fn is_cart_empty_works() {
        let cart = Cart::new();
        assert!(cart.is_cart_empty());
    }

    #[test]
    fn list_of_purchases_as_list_works() {
        let mut cart = Cart::new();
        cart.add(Purchase {
            id: 1,
            article: Article {
                name: String::from("Apple"),
                price: 2,
            },
            quantity: 5,
        });
        let list = cart.list_of_purchases_as_list();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, 1);
    }

    #[test]
    fn list_of_purchases_to_json_works() {
        let mut cart = Cart::new();
        cart.add(Purchase {
            id: 1,
            article: Article {
                name: String::from("Laptop"),
                price: 1000,
            },
            quantity: 1,
        });
        let json = cart.list_of_purchases_to_json();
        assert_eq!(
            json,
            r#"{"purchases":[{"id":1,"article":{"name":"Laptop","price":1000},"quantity":1}]}"#
        );
    }

    #[test]
    fn list_of_purchases_to_json_empty_cart() {
        let cart = Cart::new();
        let json = cart.list_of_purchases_to_json();
        assert_eq!(json, r#"{"purchases":[]}"#);
    }
}

