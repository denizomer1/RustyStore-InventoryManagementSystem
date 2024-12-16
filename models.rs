use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

// Product Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: u32,
    pub cost_price: f64,
}

impl Product {
    pub fn new(name: String, description: String, price: f64, quantity: u32, cost_price: f64) -> Self {
        Product {
            id: Uuid::new_v4(),
            name,
            description,
            price,
            quantity,
            cost_price,
        }
    }

    pub fn update(&mut self, name: Option<String>, description: Option<String>, 
                  price: Option<f64>, quantity: Option<u32>, cost_price: Option<f64>) {
        if let Some(new_name) = name {
            self.name = new_name;
        }
        if let Some(new_description) = description {
            self.description = new_description;
        }
        if let Some(new_price) = price {
            self.price = new_price;
        }
        if let Some(new_quantity) = quantity {
            self.quantity = new_quantity;
        }
        if let Some(new_cost_price) = cost_price {
            self.cost_price = new_cost_price;
        }
    }
}

// Transaction Base Trait
pub trait Transaction {
    fn get_id(&self) -> Uuid;
    fn get_total_amount(&self) -> f64;
}

// Sales Transaction Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesTransaction {
    pub id: Uuid,
    pub product_id: Uuid,
    pub quantity: u32,
    pub sale_price: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Transaction for SalesTransaction {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_total_amount(&self) -> f64 {
        self.quantity as f64 * self.sale_price
    }
}

// Purchase Transaction Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseTransaction {
    pub id: Uuid,
    pub product_id: Uuid,
    pub quantity: u32,
    pub purchase_price: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Transaction for PurchaseTransaction {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_total_amount(&self) -> f64 {
        self.quantity as f64 * self.purchase_price
    }
}

// User Model with Role-Based Access Control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    Manager,
    Staff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub role: UserRole,
}

impl User {
    pub fn new(username: String, password: String, role: UserRole) -> Self {
        User {
            id: Uuid::new_v4(),
            username,
            password_hash: bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap(),
            role,
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password_hash).unwrap_or(false)
    }
}
