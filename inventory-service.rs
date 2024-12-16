use crate::models::Product;
use crate::utils::error::InventoryError;
use std::collections::HashMap;
use uuid::Uuid;

pub struct InventoryService {
    products: HashMap<Uuid, Product>,
}

impl InventoryService {
    pub fn new() -> Self {
        InventoryService {
            products: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) -> Result<Uuid, InventoryError> {
        if self.products.values().any(|p| p.name == product.name) {
            return Err(InventoryError::DuplicateProduct);
        }
        
        let product_id = product.id;
        self.products.insert(product_id, product);
        Ok(product_id)
    }

    pub fn update_product(&mut self, id: Uuid, 
                          name: Option<String>, 
                          description: Option<String>, 
                          price: Option<f64>, 
                          quantity: Option<u32>, 
                          cost_price: Option<f64>) -> Result<(), InventoryError> {
        match self.products.get_mut(&id) {
            Some(product) => {
                product.update(name, description, price, quantity, cost_price);
                Ok(())
            },
            None => Err(InventoryError::ProductNotFound),
        }
    }

    pub fn delete_product(&mut self, id: Uuid) -> Result<(), InventoryError> {
        match self.products.remove(&id) {
            Some(_) => Ok(()),
            None => Err(InventoryError::ProductNotFound),
        }
    }

    pub fn get_product(&self, id: Uuid) -> Option<&Product> {
        self.products.get(&id)
    }

    pub fn list_products(&self) -> Vec<&Product> {
        self.products.values().collect()
    }

    pub fn check_stock(&self, product_id: Uuid, quantity: u32) -> Result<(), InventoryError> {
        match self.products.get(&product_id) {
            Some(product) if product.quantity >= quantity => Ok(()),
            Some(_) => Err(InventoryError::InsufficientStock),
            None => Err(InventoryError::ProductNotFound),
        }
    }

    pub fn update_stock(&mut self, product_id: Uuid, quantity_change: i32) -> Result<(), InventoryError> {
        match self.products.get_mut(&product_id) {
            Some(product) => {
                let new_quantity = product.quantity as i32 + quantity_change;
                if new_quantity < 0 {
                    return Err(InventoryError::InsufficientStock);
                }
                product.quantity = new_quantity as u32;
                Ok(())
            },
            None => Err(InventoryError::ProductNotFound),
        }
    }
}
