# Rust Threed

Dokumentasi ini disusun dari penjelasan pada `src/main.rs`, terutama di module `tests`, dengan format yang lebih rapi dan berurutan.

## Tujuan Project

Project ini berisi contoh dasar:
- Parallel programming
- Concurrency
- Threading di Rust (`std::thread`)
- Komunikasi antar thread dengan channel (`std::sync::mpsc`)

## Ringkasan Konsep Dasar

### 1. Pengenalan Parallel Programming
Saat ini kita hidup di era multicore, dimana jarang sekali kita menggunakan prosesor yang single core.  
Semakin canggih perangkat keras, maka software pun akan mengikuti, 
dimana sekarang kita bisa dengan mudah membuat proses parallel di aplikasi.
Parallel programming sederhananya adalah memecahkan suatu masalah dengan cara membaginya 
menjadi yang lebih kecil, dan dijalankan secara bersamaan pada waktu yang bersamaan pula

**Kesimpulan:**
Parallel programming adalah memecah pekerjaan besar menjadi bagian kecil, lalu dijalankan bersamaan.

### 2. Contoh Parallel Programming

- Menjalankan beberapa aplikasi sekaligus di sistem operasi kita (office, editor, browser, dan lain-lain)
- Beberapa koki menyiapkan makanan di restoran, dimana tiap koki membuat makanan masing-masing
- Antrian di Bank, dimana tiap teller melayani nasabah nya masing-masing

**conoth:** 
- Menjalankan banyak aplikasi sekaligus
- Beberapa koki memasak di waktu yang sama
- Banyak teller melayani nasabah masing-masing

### 3. Process vs Thread

- Process adalah sebuah eksekusi program <=> Thread adalah segmen dari process
- Process mengkonsumsi memory besar <=> Thread menggunakan memory kecil
- Process saling terisolasi dengan process lain <=> Thread bisa saling berhubungan jika dalam process yang sama
- Process lama untuk dijalankan dan dihentikan <=> Thread cepat untuk dijalankan dan dihentikan

**Kesimpulan:**
- Process: eksekusi program, memori lebih besar, terisolasi
- Thread: segmen dari process, lebih ringan, bisa berbagi konteks dalam process yang sama

### 4. Parallel vs Concurrency

Berbeda dengan paralel (menjalankan beberapa pekerjaan secara bersamaan),
concurrency adalah menjalankan beberapa pekerjaan secara bergantian.
Dalam parallel kita biasanya membutuhkan banyak Thread, sedangkan dalam concurrency,
kita hanya membutuhkan sedikit Thread.

**Kesimpulan:**
- Parallel: beberapa pekerjaan berjalan benar-benar bersamaan
- Concurrency: beberapa pekerjaan dijalankan bergantian secara efektif

### 5. Contoh Concurrency

Saat kita makan di cafe, kita bisa makan, lalu ngobrol, lalu minum, makan lagi, ngobrol lagi, minum lagi, dan seterusnya. 
Tetapi kita tidak bisa pada saat yang bersamaan minum, makan dan ngobrol, hanya bisa melakukan satu hal pada satu waktu, 
namun bisa berganti kapanpun kita mau.

**Kesimpulan:**
Analogi di cafe: makan, ngobrol, minum secara bergantian, bukan semua sekaligus.

### 6. CPU-Bound

Banyak algoritma dibuat yang hanya membutuhkan CPU untuk menjalankannya. Algoritma jenis ini biasanya sangat tergantung dengan kecepatan CPU.
Contoh yang paling populer adalah Machine Learning, oleh karena itu sekarang banyak sekali teknologi Machine Learning yang banyak menggunakan
GPU karena memiliki core yang lebih banyak dibanding CPU biasanya. 
Jenis algoritma seperti ini tidak ada benefitnya menggunakan Concurrency Programming, namun bisa dibantu dengan implementasi Parallel Programming.

**Kesimpulan:**
CPU-bound: bottleneck di CPU, sangat tergantung kecepatan CPU, cocok dibantu parallelism.

### 7. I/O-Bound

I/O-bound adalah kebalikan dari sebelumnya, dimana biasanya algoritma atau aplikasinya sangat tergantung dengan kecepatan input output devices 
yang digunakan. 
Contohnya aplikasi seperti membaca data dari file, database, dan lain-lain.
Kebanyakan saat ini, biasanya kita akan membuat aplikasi jenis seperti ini.
Aplikasi jenis I/O-bound, walaupun bisa terbantu dengan implementasi Parallel Programming, tapi benefitnya akan lebih baik jika menggunakan Concurrency Programming.
Bayangkan kita membaca data dari database, dan Thread harus menunggu 1 detik untuk mendapat balasan dari database, padahal waktu 1 detik itu jika menggunakan Concurrency
Programming, bisa digunakan untuk melakukan hal lain lagi.

**Kesimpulan:**
I/O-bound: bottleneck di I/O (file, database, network), sering lebih terbantu concurrency karena bisa melakukan hal lain saat menunggu.

### 8. Thread di Rust

Saat kita menjalankan aplikasi, aplikasi akan dijalankan dalam process, process akan diatur oleh sistem operasi.
Dalam process, kita bisa membuat thread untuk menjalankan kode secara parallel dan asynchronous.
Di Rust, kita bisa menggunakan module `std::thread` untuk membuat thread.

**Referensi:**
- https://doc.rust-lang.org/std/thread/

## Peta Section di Module `tests`

| Section | Fungsi Test | Fokus |
|---|---|---|
| Membuat Thread | `test_create_threed` | Membuat thread dengan `thread::spawn` |
| Join Thread | `test_threed_join` | Menunggu thread selesai dan ambil nilai return |
| Keutamaan Thread | `test_sequential`, `test_parallel` | Perbandingan sequential vs parallel |
| Closure | `test_closure`, `test_closure_as_fn_thread` | Closure dan ownership saat dipakai bersama thread |
| Kenapa Error | (penjelasan sebelum `test_closure_move`) | Error `E0373` dan alasan lifecycle |
| Closure `move` | `test_closure_move` | Memindahkan ownership ke closure |
| Current Thread | `test_current_thread` | Ambil info thread aktif |
| Thread Factory | `test_thread_factory` | Konfigurasi thread via `thread::Builder` |
| Thread Communication | (pengantar channel) | Konsep komunikasi antar thread |
| Channel | `test_chanel` | Kirim/terima 1 data via channel |
| Mengirim Banyak Data | `test_send_may_data_to_chanel` | Multi-message dalam channel |
| Channel Lifecycle | `test_chanel_livecycle` | Perilaku sender/receiver saat lifecycle berakhir |

## Penjelasan Detail per Section Test

### Membuat Thread (`test_create_threed`)
- Untuk membuat thread baru yang berjalan secara parallel dan asynchronous,
  kita bisa menggunakan `std::thread::spawn(closure)`.
- Pada contoh ini, main test diberi jeda (`sleep`) agar output dari thread sempat terlihat.

```rust
#[test]
fn test_create_threed() {
    thread::spawn(|| {
        for i in 1..=5 {
            println!("counter : {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });
    println!("Application start successfully");
    thread::sleep(Duration::from_secs(7));
}
```

### Join Thread (`test_threed_join`)
- Saat menjalankan thread dengan `spawn`, Rust mengembalikan `JoinHandle<T>`.
- `JoinHandle` dapat digunakan untuk melakukan join melalui method `join()`.
- `join()` akan mengembalikan `Result<T>`, sesuai return value dari thread-nya.

```rust
#[test]
fn test_threed_join() {
    let join_handle: JoinHandle<i32> = thread::spawn(|| {
        let mut counter = 0;
        for i in 1..=5 {
            println!("counter : {}", i);
            thread::sleep(Duration::from_secs(1));
            counter += 1;
        }
        counter
    });

    let result = join_handle.join();
    match result {
        Ok(counter) => println!("Thread finished with counter value: {}", counter),
        Err(e) => println!("Thread panicked: {:?}", e),
    }
}
```

### Keutamaan Menggunakan Thread (`test_sequential`, `test_parallel`)
- Jika dua kalkulasi berat dijalankan tanpa thread, eksekusi menjadi synchronous dan sequential.
- Misalnya tiap kalkulasi butuh 5 detik, totalnya bisa menjadi 10 detik.
- Jika dijalankan dengan thread, kalkulasi berjalan asynchronous dan parallel,
  sehingga total waktu bisa mendekati 5 detik.

```rust
fn calculate() -> i32 {
    let mut counter = 0;
    for i in 1..=5 {
        println!("counter : {}", i);
        thread::sleep(Duration::from_secs(1));
        counter += 1;
    }
    counter
}

#[test]
fn test_sequential() {
    let result1: i32 = calculate();
    let result2: i32 = calculate();

    println!("toal counter 1 : {}", result1);
    println!("toal counter 2 : {}", result2);
    println!("process finish");
}

#[test]
fn test_parallel() {
    let handle1: JoinHandle<i32> = thread::spawn(|| calculate());
    let handle2: JoinHandle<i32> = thread::spawn(|| calculate());

    let result1 = handle1.join();
    let result2 = handle2.join();

    match result1 {
        Ok(counter) => println!("Thread 1 finished with counter value: {}", counter),
        Err(e) => println!("Thread 1 panicked: {:?}", e),
    }

    match result2 {
        Ok(counter) => println!("Thread 2 finished with counter value: {}", counter),
        Err(e) => println!("Thread 2 panicked: {:?}", e),
    }

    println!("process finish");
}
```

### Closure (`test_closure`, `test_closure_as_fn_thread`)
- Saat menjalankan thread, parameter pada `spawn()` biasanya ditulis dalam bentuk closure.
- Closure boleh menggunakan variabel dari luar scope.
- Namun, jika closure dikirim ke function lain seperti `spawn()`, ownership variabel
    yang dipakai closure harus aman untuk lifecycle thread.

```rust
#[test]
fn test_closure() {
    let name: String = String::from("Kim");
    let closure = || {
        thread::sleep(Duration::from_secs(1));
        println!("Hello, {}", name);
    };

    closure();
}

#[test]
fn test_closure_as_fn_thread() {
    let name: String = "Abdillah".to_string();
    let closure = || {
        thread::sleep(Duration::from_secs(2));
        println!("Hello, {}", name);
    };

    // Kode berikut akan error E0373 jika diaktifkan:
    // let handle = thread::spawn(closure);
    // handle.join().unwrap();

    closure();
}
```

### Kenapa Error `E0373`
- Rust akan menolak pola tertentu dengan error `E0373`.
- Penyebab utamanya: variabel yang dipakai closure bisa saja lifecycle-nya lebih pendek
    dari thread yang berjalan.
- Ini mencegah kasus dangling pointer (thread mengakses data yang sudah hilang dari memori).
- Solusinya: jangan gunakan data luar yang lifecycle-nya tidak aman, atau pindahkan ownership
    ke closure dengan keyword `move`.

```rust
// Contoh pemicu E0373 (jika closure menangkap referensi non-'static):
// let name = String::from("Kim");
// let closure = || println!("{}", name);
// thread::spawn(closure); // berpotensi ditolak karena masalah lifetime
```

Referensi:
- https://doc.rust-lang.org/error_codes/E0373.html

### Closure `move` (`test_closure_move`)
- Dengan `move`, ownership variabel dipindahkan ke closure.
- Cara ini aman saat closure dijalankan dalam thread.
- Konsekuensinya, variabel yang sudah dipindah tidak bisa dipakai lagi di main thread.

```rust
#[test]
fn test_closure_move() {
    let name: String = "Kim".to_string();
    let closure = move || {
        thread::sleep(Duration::from_secs(2));
        println!("Hello, {}", name);
    };

    let handle: JoinHandle<()> = thread::spawn(closure);
    handle.join().unwrap();

    // println!("{}", name); // error: ownership sudah dipindahkan
    println!("main thread finished");
}
```

### Current Thread (`test_current_thread`)
- Semua program Rust berjalan di thread, termasuk saat tidak membuat thread manual.
- Unit test Rust juga berjalan dalam thread.
- Untuk mendapatkan thread yang sedang aktif, gunakan `thread::current()`.
- Informasi thread bisa berupa nama thread (jika tersedia) atau ID thread.

```rust
fn calculate_current_thread() -> i32 {
    let mut counter: i32 = 0;
    let current_thread = thread::current();

    for i in 1..=5 {
        match current_thread.name() {
            Some(name) => println!("{} counter : {}", name, i),
            None => println!("{:?} : counter : {}", current_thread.id(), i),
        }
        thread::sleep(Duration::from_secs(2));
        counter += 1;
    }

    counter
}

#[test]
fn test_current_thread() {
    let counter = calculate_current_thread();
    println!("Final counter value: {}", counter);
}
```

Referensi:
- https://doc.rust-lang.org/std/thread/struct.Thread.html

### Thread Factory (`test_thread_factory`)
- Saat membuat thread dengan `thread::spawn()`, kita memakai thread factory default dari Rust.
- Jika butuh konfigurasi khusus, kita bisa membuat thread factory manual dengan `thread::Builder`.
- Contoh konfigurasi: nama thread dan ukuran stack.

```rust
#[test]
fn test_thread_factory() {
    let thread_factory = thread::Builder::new()
        .name("MyThread".to_string())
        .stack_size(4 * 1024 * 1024);

    let handle: JoinHandle<i32> = thread_factory
        .spawn(|| {
            let mut counter: i32 = 0;
            for i in 1..=5 {
                thread::sleep(Duration::from_secs(2));
                println!("counter : {}", i);
                counter += 1;
            }
            counter
        })
        .expect("Failed to create new Thread");

    let result = handle.join().unwrap();
    println!("total counter : {}", result);
}
```

Referensi:
- https://doc.rust-lang.org/std/thread/struct.Builder.html

### Thread Communication
- Saat membuat beberapa thread, kita sering butuh mengirim data antar thread.
- Rust menggunakan konsep channel (mirip pendekatan di Golang) untuk komunikasi ini.
- Implementasinya ada di modul `std::sync::mpsc`.

```rust
// Pola dasar komunikasi:
// 1) Buat channel: let (sender, receiver) = mpsc::channel::<T>();
// 2) Sender mengirim data dari thread producer
// 3) Receiver menerima data di thread consumer
```

Referensi:
- https://doc.rust-lang.org/std/sync/mpsc/index.html

### Channel (`test_chanel`)
- Channel adalah struktur data mirip queue.
- Thread dapat mengirim data ke channel dan menerima data dari channel.
- Pihak di channel terdiri dari `Sender` (pengirim) dan `Receiver` (penerima).
- Thread tidak berkomunikasi langsung, tetapi melalui channel.
- Dalam satu waktu, thread bisa berperan sebagai sender sekaligus receiver.

```rust
#[test]
fn test_chanel() {
    let (sender, receiver) = std::sync::mpsc::channel::<String>();

    let handle_sender: JoinHandle<()> = thread::spawn(move || {
        thread::sleep(Duration::from_secs(3));
        let message = "Hello from thread 1".to_string();
        sender.send(message).unwrap();
    });

    let handle_receiver: JoinHandle<()> = thread::spawn(move || {
        let receive_message = receiver.recv().unwrap();
        println!("Received message : {}", receive_message);
    });

    let _ = handle_sender.join();
    let _ = handle_receiver.join();
}
```

### Mengirim Banyak Data (`test_send_may_data_to_chanel`)
- Karena channel berbentuk queue, kita bisa memasukkan banyak data ke dalam channel.
- Saat sender mengirim data, pengiriman bisa langsung sukses walau data belum diambil receiver.
- Saat receiver mengambil data dan belum ada isi channel, receiver akan menunggu (`blocking`).

```rust
#[test]
fn test_send_may_data_to_chanel() {
    let (sender, receiver) = std::sync::mpsc::channel::<String>();

    let handle_sender: JoinHandle<()> = thread::spawn(move || {
        for i in 1..=5 {
            thread::sleep(Duration::from_secs(1));
            let message = format!("Hello from thread 1, message {}", i);
            let _ = sender.send(message);
        }
        let _ = sender.send("Exit".to_string());
    });

    let handle_receiver: JoinHandle<()> = thread::spawn(move || loop {
        let message = receiver.recv().unwrap();

        if message == "Exit" {
            break;
        }

        println!("Received message : {}", message);
    });

    let _ = handle_sender.join();
    let _ = handle_receiver.join();
}
```

### Channel Lifecycle (`test_chanel_livecycle`)
- Saat channel dibuat, Rust otomatis membuat `Sender` dan `Receiver`.
- Jika lifecycle sender berakhir (sender di-drop), receiver tidak akan menerima data baru lagi.
- Karena receiver mengimplementasikan `Iterator`, data bisa diproses dengan `for` loop tanpa `break` manual.
- Sebaliknya, jika lifecycle receiver berakhir, pengiriman dari sender akan menghasilkan error.

```rust
#[test]
fn test_chanel_livecycle() {
    let (sender, receiver) = std::sync::mpsc::channel::<String>();

    let handle_sender: JoinHandle<()> = thread::spawn(move || {
        for i in 1..=5 {
            thread::sleep(Duration::from_secs(1));
            let message = format!("Hello from thread 1, message {}", i);
            let _ = sender.send(message);
        }
    });

    let handle_receiver: JoinHandle<()> = thread::spawn(move || {
        let message_iter = receiver.iter();
        for message in message_iter {
            println!("Received message : {}", message);
        }
    });

    let _ = handle_sender.join();
    let _ = handle_receiver.join();
}
```

## Sample Code Singkat

### Contoh `spawn` dan `join`

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| 42);
    let result = handle.join().unwrap();
    println!("result: {}", result);
}
```

### Contoh Channel Sender/Receiver

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel::<String>();

    let producer = thread::spawn(move || {
        sender.send("hello".to_string()).unwrap();
    });

    let consumer = thread::spawn(move || {
        let msg = receiver.recv().unwrap();
        println!("diterima: {}", msg);
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
```

## Cara Menjalankan

Jalankan semua test:

```bash
cargo test
```

Tampilkan output `println!` saat test:

```bash
cargo test -- --nocapture
```

Jalankan test tertentu:

```bash
cargo test test_chanel_livecycle -- --nocapture
```

## Catatan

- Penamaan fungsi/test mengikuti source asli (`threed`, `chanel`, `livecycle`)
- Fokus project ini adalah pembelajaran konsep dasar, bukan optimasi production-grade
