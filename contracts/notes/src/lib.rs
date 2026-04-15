#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Enum untuk status barang
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum StatusBarang {
    Hilang,
    Ditemukan,
}

// Struktur data yang akan menyimpan informasi barang
#[contracttype]
#[derive(Clone, Debug)]
pub struct Barang {
    pub id: u64,
    pub nama_barang: String,
    pub lokasi_terakhir: String,
    pub hadiah_imbalan: u32,
    pub status: StatusBarang,
}

// Storage key untuk data barang
const BARANG_DATA: Symbol = symbol_short!("BRG_DATA");

#[contract]
pub struct LostAndFoundContract;

#[contractimpl]
impl LostAndFoundContract {
    // Fungsi untuk mendapatkan semua data barang yang dilaporkan
    pub fn get_semua_barang(env: Env) -> Vec<Barang> {
        env.storage().instance().get(&BARANG_DATA).unwrap_or(Vec::new(&env))
    }

    // Fungsi untuk melaporkan barang hilang, sekarang bisa menyertakan nominal imbalan
    pub fn lapor_barang_hilang(env: Env, nama_barang: String, lokasi_terakhir: String, hadiah_imbalan: u32) -> String {
        let mut daftar_barang: Vec<Barang> = env.storage().instance().get(&BARANG_DATA).unwrap_or(Vec::new(&env));
        
        let barang_baru = Barang {
            id: env.prng().gen::<u64>(),
            nama_barang: nama_barang,
            lokasi_terakhir: lokasi_terakhir,
            hadiah_imbalan: hadiah_imbalan,
            status: StatusBarang::Hilang,
        };
        
        daftar_barang.push_back(barang_baru);
        env.storage().instance().set(&BARANG_DATA, &daftar_barang);

        String::from_str(&env, "Laporan barang hilang berhasil ditambahkan")
    }

    // Fungsi untuk mengubah status barang menjadi sudah ditemukan berdasarkan ID
    pub fn tandai_ditemukan(env: Env, id: u64) -> String {
        let mut daftar_barang: Vec<Barang> = env.storage().instance().get(&BARANG_DATA).unwrap_or(Vec::new(&env));
        let mut diubah = false;

        for i in 0..daftar_barang.len() {
            let mut barang = daftar_barang.get(i).unwrap();
            if barang.id == id {
                barang.status = StatusBarang::Ditemukan;
                daftar_barang.set(i, barang);
                diubah = true;
                break;
            }
        }

        if diubah {
            env.storage().instance().set(&BARANG_DATA, &daftar_barang);
            String::from_str(&env, "Barang berhasil ditandai sebagai ditemukan")
        } else {
            String::from_str(&env, "Data barang dengan ID tersebut tidak ditemukan")
        }
    }
}

mod test;