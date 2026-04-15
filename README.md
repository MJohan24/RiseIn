<img width="1919" height="912" alt="Screenshot 2026-04-15 122337" src="https://github.com/user-attachments/assets/ecdeff51-225a-45f3-9872-cce3c53f9da4" />
# 🔍 Lost & Found DApp (Stellar / Soroban)

**Lost & Found DApp** - Decentralized application (DApp) berbasis blockchain Stellar untuk melaporkan dan melacak barang hilang maupun ditemukan.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Network](https://img.shields.io/badge/network-Stellar_Testnet-green.svg)
![Smart_Contract](https://img.shields.io/badge/smart_contract-Soroban-orange.svg)

---

## 📖 Deskripsi Proyek

**Lost & Found DApp** adalah solusi *smart contract* yang dibangun di atas jaringan blockchain Stellar menggunakan SDK Soroban. Aplikasi ini menyediakan platform yang aman, transparan, dan tidak dapat diubah (immutable) untuk mencatat laporan barang yang hilang di ruang publik (seperti kampus, mall, atau acara besar) beserta janji imbalan (reward) bagi penemunya.

Berbeda dengan sistem terpusat, data yang disimpan di dalam *smart contract* ini bersifat transparan, dapat diakses oleh siapa saja, dan permanen.

## ✨ Fitur Utama

1. **Lapor Barang Hilang (`lapor_barang_hilang`)**
   Memungkinkan pengguna untuk mencatat barang yang hilang ke dalam blockchain. Data yang disimpan meliputi:
   - Nama Barang.
   - Lokasi Terakhir Dilihat.
   - Hadiah / Imbalan (Reward dalam bentuk token/uang).
   - Status otomatis diubah menjadi `Hilang`.

2. **Daftar Pencarian Terbuka (`get_semua_barang`)**
   Menampilkan seluruh katalog barang yang sedang dicari beserta hadiah yang dijanjikan. Data ini bisa ditarik secara real-time dan diintegrasikan dengan mudah ke Frontend (Web/Mobile).

3. **Konfirmasi Barang Ditemukan (`tandai_ditemukan`)**
   Mengubah status barang. Jika ada pengguna yang menemukan barang tersebut, status barang dapat diperbarui menjadi `Ditemukan` menggunakan ID barang, yang menandakan bahwa pencarian telah selesai.

---

## 🚀 Panduan Penggunaan (Quick Start)

### 1. Persiapan Lingkungan (Prerequisites)
Pastikan Anda telah menginstal **Stellar CLI** dan **Rust** terbaru di sistem Anda.
```bash
rustup update
cargo install --locked stellar-cli --features opt
```

### 2. Membuat Buat Akun Wallet Testnet
Buat akun baru di jaringan Stellar Testnet untuk uji coba.
```bash
stellar keys generate mywallet --network testnet --fund
```

### 3. Build Smart Contract
Masuk ke direktori contract dan lakukan *build* kode Rust menjadi file WebAssembly (`.wasm`).
```bash
cd contracts/notes
stellar contract build
```

### 4. Deploy ke Stellar Testnet
Deploy file `.wasm` hasil build ke jaringan Testnet menggunakan wallet yang telah dibuat.
```bash
stellar contract deploy \
  --wasm ../../target/wasm32v1-none/release/notes.wasm \
  --source-account mywallet \
  --network testnet
```
*(Catat **Contract ID** yang dihasilkan dari perintah ini, misalnya: `CABGE4IXVW...`)*

### 5. Berinteraksi (Invoke) dengan Contract
Ganti `[CONTRACT_ID]` dengan ID yang Anda dapatkan setelah proses deploy.

**📍 Melaporkan Barang Hilang:**
```bash
stellar contract invoke \
  --id [CONTRACT_ID] \
  --source-account mywallet \
  --network testnet \
  -- lapor_barang_hilang \
  --nama_barang "Dompet" \
  --lokasi_terakhir "Kantin" \
  --hadiah_imbalan 50000
```

**📍 Melihat Semua Data Barang:**
```bash
stellar contract invoke \
  --id [CONTRACT_ID] \
  --source-account mywallet \
  --network testnet \
  -- get_semua_barang
```

---

## ⛓️ Contract Detail (Testnet)
- **Contract ID:** `CABGE4IXVW4OIZ34NVKBBQTXKU55NT7OV3UC2E3W6MBZHWXPGTKRKPU6`
- **Network:** Stellar Testnet
- **Stellar Expert Explorer:** [Lihat Histori Transaksi di sini](https://stellar.expert/explorer/testnet/contract/CABGE4IXVW4OIZ34NVKBBQTXKU55NT7OV3UC2E3W6MBZHWXPGTKRKPU6)
- **Live Monitoring ⬇️⬇️⬇️**
<img width="1919" height="912" alt="Screenshot 2026-04-15 122337" src="https://github.com/user-attachments/assets/b50f98b2-46a2-41c3-94e1-6fc8aa686e50" />

---

## 🔮 Rencana Pengembangan ke Depan (Roadmap)

- [ ] **Sistem "Escrow" Vault:** Menahan token imbalan (XLM/USDC) secara otomatis di *Contract Balance* saat melapor, dan otomatis mentransfer token ke dompet sang penemu ketika barang disetujui.
- [ ] **Bukti Temuan (Proof of Discovery):** Sistem bagi penemu untuk mengunggah bukti/hash gambar bahwa ia benar-benar menemukan barang.
- [ ] **User Verification:** Hanya pemilik barang asli yang dapat menandai bahwa barang tersebut telah "Ditemukan" dan mencairkan *reward*.

---
*Dibuat dengan ❤️ menggunakan [Soroban SDK](https://soroban.stellar.org/)*
