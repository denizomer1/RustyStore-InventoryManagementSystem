```
rusty-store-inventory/
│
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── product.rs
│   │   ├── transaction.rs
│   │   └── user.rs
│   ├── controllers/
│   │   ├── mod.rs
│   │   ├── inventory_controller.rs
│   │   ├── sales_controller.rs
│   │   └── purchase_controller.rs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── authentication.rs
│   │   ├── inventory_service.rs
│   │   ├── sales_service.rs
│   │   └── purchase_service.rs
│   └── utils/
│       ├── mod.rs
│       ├── error.rs
│       └── report_generator.rs
│
└── tests/
    ├── inventory_tests.rs
    ├── sales_tests.rs
    └── authentication_tests.rs
```
