#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data inventory mahasiswa
#[contracttype]
#[derive(Clone, Debug)]
pub struct InventoryItem {
    id: u64,
    nama_barang: String,
    jumlah: u32,
    pemilik: String,
}

// Storage key
const INVENTORY_DATA: Symbol = symbol_short!("INV_DATA");

#[contract]
pub struct InventoryContract;

#[contractimpl]
impl InventoryContract {

    // Ambil semua data inventory
    pub fn get_items(env: Env) -> Vec<InventoryItem> {
        env.storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Tambah item baru
    pub fn add_item(env: Env, nama_barang: String, jumlah: u32, pemilik: String) -> String {
        let mut items: Vec<InventoryItem> = env.storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env));

        let item = InventoryItem {
            id: env.prng().gen::<u64>(),
            nama_barang,
            jumlah,
            pemilik,
        };

        items.push_back(item);

        env.storage().instance().set(&INVENTORY_DATA, &items);

        String::from_str(&env, "Item berhasil ditambahkan")
    }

    // Hapus item berdasarkan id
    pub fn delete_item(env: Env, id: u64) -> String {
        let mut items: Vec<InventoryItem> = env.storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            if items.get(i).unwrap().id == id {
                items.remove(i);
                env.storage().instance().set(&INVENTORY_DATA, &items);
                return String::from_str(&env, "Item berhasil dihapus");
            }
        }

        String::from_str(&env, "Item tidak ditemukan")
    }

    // Update jumlah barang
    pub fn update_jumlah(env: Env, id: u64, jumlah_baru: u32) -> String {
        let mut items: Vec<InventoryItem> = env.storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            let mut item = items.get(i).unwrap();

            if item.id == id {
                item.jumlah = jumlah_baru;
                items.set(i, item);

                env.storage().instance().set(&INVENTORY_DATA, &items);
                return String::from_str(&env, "Jumlah berhasil diupdate");
            }
        }

        String::from_str(&env, "Item tidak ditemukan")
    }
}

mod test;