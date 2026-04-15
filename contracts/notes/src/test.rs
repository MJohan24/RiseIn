#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

#[test]
fn test_lost_and_found() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LostAndFoundContract);
    let client = LostAndFoundContractClient::new(&env, &contract_id);

    // Lapor barang hilang
    let nama_barang = String::from_str(&env, "Kunci Motor");
    let lokasi = String::from_str(&env, "Parkiran Gedung B");
    let hadiah = 50000;
    
    let res = client.lapor_barang_hilang(&nama_barang, &lokasi, &hadiah);
    assert_eq!(res, String::from_str(&env, "Laporan barang hilang berhasil ditambahkan"));

    // Cek daftar barang
    let daftar_barang = client.get_semua_barang();
    assert_eq!(daftar_barang.len(), 1);
    
    let barang = daftar_barang.get(0).unwrap();
    assert_eq!(barang.nama_barang, nama_barang);
    assert_eq!(barang.lokasi_terakhir, lokasi);
    assert_eq!(barang.hadiah_imbalan, hadiah);
    assert_eq!(barang.status, StatusBarang::Hilang);

    // Tandai barang ditemukan
    let res_ditemukan = client.tandai_ditemukan(&barang.id);
    assert_eq!(res_ditemukan, String::from_str(&env, "Barang berhasil ditandai sebagai ditemukan"));

    // Cek kembali status barang
    let daftar_barang_update = client.get_semua_barang();
    let barang_update = daftar_barang_update.get(0).unwrap();
    assert_eq!(barang_update.status, StatusBarang::Ditemukan);
}
