#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Cake {
    id: u64,
    name: String,
    price: u32,
    stock: u32,
}

const CAKE_DATA: Symbol = symbol_short!("CAKE_DATA");

#[contract]
pub struct CakeShopContract;

#[contractimpl]
impl CakeShopContract {

    // 🔍 READ → ambil semua cake
    pub fn get_cakes(env: Env) -> Vec<Cake> {
        env.storage().instance().get(&CAKE_DATA).unwrap_or(Vec::new(&env))
    }

    // ➕ CREATE → tambah cake baru
    pub fn create_cake(env: Env, name: String, price: u32, stock: u32) -> String {
        let mut cakes: Vec<Cake> = env.storage().instance().get(&CAKE_DATA).unwrap_or(Vec::new(&env));

        let cake = Cake {
            id: env.prng().gen::<u64>(),
            name,
            price,
            stock,
        };

        cakes.push_back(cake);

        env.storage().instance().set(&CAKE_DATA, &cakes);

        String::from_str(&env, "Cake berhasil ditambahkan")
    }

    // ✏️ UPDATE → update stock cake
    pub fn update_stock(env: Env, id: u64, new_stock: u32) -> String {
        let mut cakes: Vec<Cake> = env.storage().instance().get(&CAKE_DATA).unwrap_or(Vec::new(&env));

        for i in 0..cakes.len() {
            if cakes.get(i).unwrap().id == id {
                let mut cake = cakes.get(i).unwrap();
                cake.stock = new_stock;

                cakes.set(i, cake);
                env.storage().instance().set(&CAKE_DATA, &cakes);

                return String::from_str(&env, "Stock berhasil diupdate");
            }
        }

        String::from_str(&env, "Cake tidak ditemukan")
    }

    // ❌ DELETE → hapus cake
    pub fn delete_cake(env: Env, id: u64) -> String {
        let mut cakes: Vec<Cake> = env.storage().instance().get(&CAKE_DATA).unwrap_or(Vec::new(&env));

        for i in 0..cakes.len() {
            if cakes.get(i).unwrap().id == id {
                cakes.remove(i);

                env.storage().instance().set(&CAKE_DATA, &cakes);
                return String::from_str(&env, "Cake berhasil dihapus");
            }
        }

        String::from_str(&env, "Cake tidak ditemukan")
    }
}

mod test;