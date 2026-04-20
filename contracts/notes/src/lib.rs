#![no_std]
    // =========================
    // GET ALL ITEMS
    // =========================
    pub fn get_items(env: Env) -> Vec<InventoryItem> {
        env.storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // =========================
    // ADD ITEM
    // =========================
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

    // =========================
    // DELETE ITEM
    // =========================
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

    // =========================
    // UPDATE QUANTITY
    // =========================
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

mod test;