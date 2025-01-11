Berikut adalah penjelasan logika program yang telah saya buat. Program ini adalah sistem pelacak progres berbasis terminal dengan fitur edit dan penyimpanan.

---

### **1. Struktur Program**

Program dibagi menjadi beberapa bagian utama:

- **Inisialisasi Terminal**: Masuk ke mode terminal alternatif dan raw mode.
- **Pengolahan Input**: Menangkap input keyboard untuk navigasi, edit, dan penyimpanan.
- **Render Grid**: Menampilkan grid (kotak-kotak kuartal).
- **Penyimpanan dan Pemulihan Data**: Menyimpan grid ke file dan memuat grid dari file saat program dimulai.

---

### **2. Alur Kerja Program**

1. **Inisialisasi Terminal**
    - Terminal diatur ke mode alternatif menggunakan `termion::screen::AlternateScreen`.
    - Raw mode diaktifkan dengan `stdout.into_raw_mode()`.
    - Mode ini memungkinkan program membaca input keyboard tanpa perlu menekan Enter.
2. **Inisialisasi Grid**
    - Grid adalah array karakter yang merepresentasikan kotak-kotak kuartal tahun 2025.
    - Setiap kuartal memiliki jumlah kotak berbeda:
        - Kuartal 1: 90 kotak
        - Kuartal 2: 91 kotak
        - Kuartal 3: 92 kotak
        - Kuartal 4: 92 kotak
    - Jika file `progress.txt` ada, data dari file ini digunakan untuk memuat grid.
3. **Pengolahan Input**
    - Program membaca input keyboard menggunakan `stdin.keys()`.
    - Input yang didukung:
        - **Navigasi**:
            - Panah atas, bawah, kiri, dan kanan untuk berpindah antar-kotak.
            - Pindah hanya ke kotak yang valid (tidak keluar grid).
        - **Edit Mode**:
            - Tekan `E` untuk masuk ke mode edit.
            - Saat mode edit aktif, kotak yang dihover berubah warna.
            - Gunakan `W`, `L`, atau `D` untuk menandai kotak.
        - **Simpan atau Keluar**:
            - Tekan `S` untuk menyimpan.
            - Setelah menekan `S`, konfirmasi dengan:
                - `Y`: Menyimpan perubahan ke file.
                - `T`: Kembali ke mode edit tanpa menyimpan.
            - Tekan `Q` untuk keluar dari program.
4. **Render Grid**
    - Fungsi `render_grid` menampilkan grid ke terminal.
    - Kotak yang sedang dihover diberi warna hijau jika mode edit aktif.
    - Setiap baris memiliki 7 kotak untuk tampilan yang teratur.
5. **Simpan dan Muat Data**
    - **Simpan**: Fungsi `save_progress` menyimpan grid ke file `progress.txt`.
    - **Muat**: Fungsi `load_progress` membaca file `progress.txt` dan mengisi grid.

---

### **3. Fungsi Utama**

Berikut adalah fungsi utama dan logikanya:

### **a. `render_grid`**

- **Tujuan**: Menampilkan grid di terminal.
- **Logika**:
    - Hapus layar (`termion::clear::All`).
    - Iterasi setiap kotak di grid.
    - Jika kotak sedang dihover, tambahkan warna hijau.
    - Tampilkan baris baru setelah setiap 7 kotak.

### **b. `save_progress`**

- **Tujuan**: Menyimpan grid ke file `progress.txt`.
- **Logika**:
    - Buka file dengan opsi `write`, `create`, dan `truncate`.
    - Tulis setiap elemen grid ke file.

### **c. `load_progress`**

- **Tujuan**: Memuat grid dari file `progress.txt` (jika ada).
- **Logika**:
    - Buka file `progress.txt`.
    - Baca isinya ke dalam string.
    - Konversi string menjadi array karakter.

---

### **4. Skenario Penggunaan**

1. **Masuk ke Mode Edit**:
    - Tekan `E`. Kotak yang dihover berubah warna.
2. **Navigasi Kotak**:
    - Gunakan panah untuk memilih kotak.
    - Hanya kotak valid yang bisa dipilih.
3. **Edit Kotak**:
    - Tekan `W`, `L`, atau `D` untuk menandai kotak.
4. **Simpan Progres**:
    - Tekan `S`, lalu:
        - `Y` untuk menyimpan.
        - `T` untuk kembali ke mode edit.
5. **Keluar**:
    - Tekan `Q` untuk keluar dari program.

---

### **5. Troubleshooting**

Jika terjadi error:

1. **File Tidak Ditemukan**:
    - Pastikan file `progress.txt` ada atau biarkan program membuat file baru saat menyimpan.
2. **Terminal Tidak Mendukung**:
    - Pastikan menggunakan terminal yang mendukung raw mode, seperti terminal default Ubuntu (Gnome Terminal).
3. **Behavior yang Tidak Sesuai**:
    - Gunakan `println!` untuk debug posisi atau nilai variabel.

---

### **6. Cara Mengembangkan Lebih Lanjut**

1. **Tambahkan Warna Lain**:
    - Gunakan warna berbeda untuk setiap status kotak (`W`, `L`, `D`).
2. **Tambahkan Pilihan Kuartal**:
    - Buat mode navigasi untuk memilih kuartal tertentu.
3. **Optimasi Grid**:
    - Gunakan representasi grid 2D (array dua dimensi) untuk mempermudah navigasi.
