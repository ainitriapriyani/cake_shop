# 🍰 Decentralized Cake Shop Management on Stellar

## 📌 Description

This project is a decentralized Cake Shop Management system built using Soroban smart contracts on the Stellar testnet.
It enables users to manage cake inventory such as adding, viewing, updating, and deleting cake data directly on-chain, ensuring transparency and decentralization.

---

## 🧩 Features

* Create new cake (name, description, price, stock)
* Retrieve all cakes
* Update cake price and stock
* Delete cake by ID

---

## 🛠 Tech Stack

* Rust
* Soroban Smart Contracts
* Stellar Testnet

---

## 📂 Project Structure

```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs        # CakeShopContract logic (CRUD)
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

---

## 🚀 How to Run

### 1. Build Contract

```bash
cd contracts/hello_world
stellar contract build
```

---

### 2. Deploy Contract

```bash
stellar contract deploy --wasm /app/target/wasm32v1-none/release/hello_world.wasm --source alice --network testnet
```

👉 Save the Contract ID after deployment.

---

## ⚙️ Contract Functions

### ➕ Create Cake

```bash
stellar contract invoke --id <CONTRACT_ID> --source alice --network testnet -- create_cake --name chocolate --description sweet --price 20000 --stock 10
```

---

### 🔍 Get Cakes

```bash
stellar contract invoke --id <CONTRACT_ID> --source alice --network testnet -- get_cakes
```

---

### ✏️ Update Cake

```bash
stellar contract invoke --id <CONTRACT_ID> --source alice --network testnet -- update_cake --id <CAKE_ID> --new_price 25000 --new_stock 15
```

---

### ❌ Delete Cake

```bash
stellar contract invoke --id <CONTRACT_ID> --source alice --network testnet -- delete_cake --id <CAKE_ID>
```

---

## 🔑 Contract ID

```
CAQQVJ5ETSO63UGA5WPM3TBBWTUK6GUCG4Y6YD4AVBR3TRABVPXLDTM2
```
<img width="957" height="426" alt="image" src="https://github.com/user-attachments/assets/133420a0-17f9-47d9-b1b7-a92a61a2ebc2" />


---

## 📝 Notes

* This contract uses Soroban storage to store cake data on-chain.
* Each cake has a unique ID generated using pseudo-random function.
* Data is stored as a vector of Cake structs.

---

## 🎯 Conclusion

This project demonstrates how a simple CRUD-based inventory system can be implemented using Soroban smart contracts on the Stellar blockchain.
It provides a real-world example of decentralized data management for a cake shop.
