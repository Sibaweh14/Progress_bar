use std::fs::{OpenOptions, File};
use std::io::{self, Write, Read, stdout, stdin};
use termion::event::Key;
use termion::input::{self, TermRead};
use termion::screen::IntoAlternateScreen;
use read_char;

#[derive(Clone, Copy, Debug)]
enum Status {
    Pagar, // Pagar biasa
    Win,   // Menang
    Lose,  // Kalah
    Draw,  // Seri
}

struct Grid {
    days: Vec<Status>, // Menyimpan status untuk setiap hari
}

impl Grid {
    fn new(size: usize) -> Self {
        Grid {
            days: vec![Status::Pagar; size],
        }
    }

    fn display(&self) {
        // Menampilkan status pagar dalam format 7 hari per baris
        for (i, status) in self.days.iter().enumerate() {
            let symbol = match status {
                Status::Pagar => "#",
                Status::Win => "W",
                Status::Lose => "L",
                Status::Draw => "D",
            };
            print!("{} ", symbol);

            // Pindah baris setelah 7 elemen
            if (i + 1) % 7 == 0 {
                println!();
            }
        }
    }

    fn update(&mut self, index: usize, new_status: Status) {
        self.days[index] = new_status;
    }
}

fn main() {
    let mut screen = stdout().into_alternate_screen().unwrap();



    // Tampilan awal
    writeln!(screen, "Welcome to Progress Tracker 2025! Press 'E' to enter edit mode.").expect("Failed to write to screen");
    writeln!(screen, "Use arrow keys to navigate, 'W', 'L', or 'D' to mark progress.").expect("Failed to write to screen");
    writeln!(screen, "Press 'S' to save progress, 'Q' to quit.").expect("Failed to write to screen");

    // Inisialisasi grid
      // Membuat grid untuk masing-masing kuartal
    let mut q1 = Grid::new(90);  // Kuartal 1
    let mut q2 = Grid::new(91);  // Kuartal 2
    let mut q3 = Grid::new(92);  // Kuartal 3
    let mut q4 = Grid::new(92);  // Kuartal 4

    let mut selected_index = 0;
    let mut in_edit_mode = false;
    let mut current_quarter = 1;

    loop {
        // Menampilkan grid sesuai dengan kuartal yang dipilih
        match current_quarter {
            1 => q1.display(),
            2 => q2.display(),
            3 => q3.display(),
            4 => q4.display(),
            _ => (),
        }

        // Menunggu input dari pengguna
        let mut input_char = [0];
        io::stdin().read(&mut input_char).unwrap();
        let input = input_char[0] as char;

        if in_edit_mode {
            match input {
                'w' => {
                    if selected_index > 0 {
                        selected_index -= 1; // Pindah ke atas
                    }
                }
                's' => {
                    if selected_index < 90 { // Pastikan tidak melampaui grid
                        selected_index += 1; // Pindah ke bawah
                    }
                }
                'a' => {
                    if selected_index % 7 != 0 {
                        selected_index -= 1; // Pindah ke kiri
                    }
                }
                'd' => {
                    if selected_index % 7 != 6 {
                        selected_index += 1; // Pindah ke kanan
                    }
                }
                'W' => {
                    match current_quarter {
                        1 => q1.update(selected_index, Status::Win),
                        2 => q2.update(selected_index, Status::Win),
                        3 => q3.update(selected_index, Status::Win),
                        4 => q4.update(selected_index, Status::Win),
                        _ => (),
                    }
                }
                'L' => {
                    match current_quarter {
                        1 => q1.update(selected_index, Status::Lose),
                        2 => q2.update(selected_index, Status::Lose),
                        3 => q3.update(selected_index, Status::Lose),
                        4 => q4.update(selected_index, Status::Lose),
                        _ => (),
                    }
                }
                'D' => {
                    match current_quarter {
                        1 => q1.update(selected_index, Status::Draw),
                        2 => q2.update(selected_index, Status::Draw),
                        3 => q3.update(selected_index, Status::Draw),
                        4 => q4.update(selected_index, Status::Draw),
                        _ => (),
                    }
                }
                'S' => {
                    // Simpan perubahan dan keluar dari mode edit
                    println!("Perubahan disimpan!");
                    in_edit_mode = false;
                }
                'T' => {
                    // Kembali ke mode edit
                    println!("Masih dalam mode edit");
                    in_edit_mode = true;
                }
                _ => {}
            }
        } else {
            match input {
                '1' => current_quarter = 1,
                '2' => current_quarter = 2,
                '3' => current_quarter = 3,
                '4' => current_quarter = 4,
                'E' => in_edit_mode = true, // Masuk ke mode edit
                _ => {}
            }
        }

        // Membersihkan layar untuk mencetak ulang
        write!(screen, "{}", termion::cursor::Goto(1, 1)).unwrap();
        screen.flush().unwrap();
    }
}