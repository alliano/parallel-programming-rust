/* Pengenalan palalel programming
 *
 * Saat ini kita hidup di era multicore, dimana jarang sekali kita menggunakan prosesor yang
 * single core Semakin canggih perangkat keras, maka software pun akan mengikuti,
 * dimana sekarang kita bisa dengan mudah membuat proses parallel di aplikasi.
 * Parallel programming sederhananya adalah memecahkan suatu masalah dengan cara membaginya
 * menjadi yang lebih kecil, dan dijalankan secara bersamaan pada waktu yang bersamaan pula
 */

/*
 * Contoh Parallel Programming
 *
 * Menjalankan beberapa aplikasi sekaligus di sistem operasi kita (office, editor, browser, dan lain-lain)
 * Beberapa koki menyiapkan makanan di restoran, dimana tiap koki membuat makanan masing-masing
 * Antrian di Bank, dimana tiap teller melayani nasabah nya masing-masing
 */

/*
 * Process Vs Thread
 *
 * - Process adalah sebuah eksekusi program <=> Thread adalah segmen dari process
 * - Process mengkonsumsi memory besar <=> Thread menggunakan memory kecil
 * - Process saling terisolasi dengan process lain <=> Thread bisa saling berhubungan jika dalam process yang sama
 * - Process lama untuk dijalankan dihentikan <=> Thread cepat untuk dijalankan dan dihentikan>
 */

/*
 * Paraller Vs Concurrency
 *
 * Berbeda dengan paralel (menjalankan beberapa pekerjaan secara bersamaan),
 * concurrency adalah menjalankan beberapa pekerjaan secara bergantian
 * Dalam parallel kita biasanya membutuhkan banyak Thread, sedangkan dalam concurrency,
 * kita hanya membutuhkan sedikit Thread
 */

/*
 * Contoh Concurrency
 *
 * Saat kita makan di cafe, kita bisa makan, lalu ngobrol, lalu minum, makan lagi, ngobrol lagi, minum lagi, dan seterusnya.
 * Tetapi kita tidak bisa pada saat yang bersamaan minum, makan dan ngobrol, hanya bisa melakukan satu hal pada satu waktu,
 * namun bisa berganti kapanpun kita mau.
 */

/*
 * CPU Bound
 * Banyak algoritma dibuat yang hanya membutuhkan CPU untuk menjalankannya. Algoritma jenis ini biasanya sangat tergantung dengan kecepatan CPU.
 * Contoh yang paling populer adalah Machine Learning, oleh karena itu sekarang banyak sekali teknologi Machine Learning yang banyak menggunakan
 * GPU karena memiliki core yang lebih banyak dibanding CPU biasanya.
 * Jenis algoritma seperti ini tidak ada benefitnya menggunakan Concurrency Programming, namun bisa dibantu dengan implementasi Parallel Programming.
 */

/*
 * I/O Bound
 *
 * I/O-bound adalah kebalikan dari sebelumnya, dimana biasanya algoritma atau aplikasinya sangat tergantung dengan kecepatan input output devices
 * yang digunakan.
 * Contohnya aplikasi seperti membaca data dari file, database, dan lain-lain.
 * Kebanyakan saat ini, biasanya kita akan membuat aplikasi jenis seperti ini.
 * Aplikasi jenis I/O-bound, walaupun bisa terbantu dengan implementasi Parallel Programming, tapi benefitnya akan lebih baik jika menggunakan Concurrency Programming.
 * Bayangkan kita membaca data dari database, dan Thread harus menunggu 1 detik untuk mendapat balasan dari database, padahal waktu 1 detik itu jika menggunakan Concurrency
 * Programming, bisa digunakan untuk melakukan hal lain lagi
 */

/*
 * Thread
 *
 * Saat kita menjalankan aplikasi, aplikasi akan dijalankan dalam process, process akan diatur oleh sistem operasi
 * Dalam process, kita bisa membuat thread untuk menjalankan kode secara parallel dan asynchronous
 * Di Rust, kita bisa menggunakan module std::thread untuk membuat thread
 * https://doc.rust-lang.org/std/thread/
 */

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use std::sync::atomic::{AtomicI32, Ordering};
    use std::thread::{self, JoinHandle};
    use std::time::Duration;

    /*
     * Membuat Thread
     * Untuk membuat thread baru yang berjalan secara parallel dan async, kita bisa menggunakan std::thread::spawn(closure)
     */
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

    /*
     * Join Thread
     *
     * Saat kita menjalankan thread menggunakan spawn function, maka dia akan mengembalikan data JoinHandle<T>
     * JoinHandler bisa digunakan untuk melakukan join thread dengan memanggil method join()
     * Method join() akan mengembalikan Result<T>, sesuai dengan return dari thread nya
     */
    #[test]
    fn test_threed_join() {
        let join_handle: JoinHandle<i32> = thread::spawn(|| {
            let mut counter = 0;
            for i in 1..=5 {
                println!("counter : {}", i);
                thread::sleep(Duration::from_secs(1));
                counter += 1;
            }

            return counter;
        });
        let result = join_handle.join();
        match result {
            Ok(counter) => println!("Thread finished with counter value: {}", counter),
            Err(e) => println!("Thread panicked: {:?}", e),
        }
    }

    /*
     * Keuatamaan Menggunakan Thread
     *
     * Misal kita butuh melakukan dua kalkulasi berat, jika kita lakukan tanpa menggunakan thread, artinya kode akan dieksekusi secara
     * synchronous dan sequential
     * Jika tiap kalkulasi membutuhkan waktu misal 5 detik, maka kita butuh 10 detik untuk menyelesaikan tiap kalkulasi
     * Namun jika kita jalankan menggunakan thread, artinya kalkulasi akan dijalankan secara asynchronous dan parallel,
     * sehingga bisa jadi total waktu untuk menyelesaikan seluruh kalkulasi, hanya butuh waktu 5 detik
     */

    fn calculate() -> i32 {
        let mut counter = 0;
        for i in 1..=5 {
            println!("counter : {}", i);
            thread::sleep(Duration::from_secs(1));
            counter += 1;
        }
        return counter;
    }

    // contoh sequential, dimana tiap kalkulasi dijalankan secara synchronous dan sequential
    #[test]
    fn test_sequential() {
        let result1: i32 = calculate();
        let result2: i32 = calculate();

        println!("toal counter 1 : {}", result1);
        println!("toal counter 2 : {}", result2);

        println!("process finish");
    }

    // contoh parallel, dimana tiap kalkulasi dijalankan secara asynchronous dan parallel
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

    /*
     * Closure
     *
     * Saat kita menjalankan thread, kita menggunakan spawn() function, dimana parameternya adalah function
     * Biasanya, kita akan menggunakan function dalam bentuk closure
     * Saat kita menggunakan variable dari luar closure, hal ini diperbolehkan, seperti yang sudah kita bahas di materi Rust Dasar
     * Namun, jika closure tersebut dikirim sebagai parameter di function lain, contoh di spawn(), maka Rust melarang itu, karena variable yang digunakan oleh closure tersebut harus dipindahkan ownership nya ke closure
     */

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

        //   Kode ini akan error karena variable name yang digunakan di closure, harus dipindahkan ownership nya ke closure, sehingga tidak bisa digunakan lagi di luar closure, termasuk di main thread

        //   let handle = thread::spawn(closure);
        //   handle.join().unwrap();

        closure();
    }

    /*
     * Kenapa Error ?
     *
     * Rust akan menjadikan hal ini error, yaitu error : https://doc.rust-lang.org/error_codes/E0373.html
     * Sederhananya adalah, jika kita melakukan hal itu, ditakutkan variable yang digunakan dalam Closure alur hidupnya tidak sepanjang thread yang berjalan
     * Karena bisa jadi variable nya sudah dihapus dari memory, sedangkan thread nya masih berjalan, dengan demikian secara otomatis thread akan menggunakan variable yang datanya sudah tidak ada di memory alias Dangling Pointer
     * Solusinya adalah, kita tidak boleh menggunakan variable diluar scope closure, atau memaksa variable tersebut berpindah scope ke closure, menggunakan kata kunci move
     */

    #[test]
    fn test_closure_move() {
        let name: String = "Kim".to_string();
        let closure = move || {
            thread::sleep(Duration::from_secs(2));
            println!("Hello, {}", name);
        };

        let handle: JoinHandle<()> = thread::spawn(closure);
        handle.join().unwrap();

        // ini akna error karena variable name sudah dipindahkan ownership nya ke closure, sehingga tidak bisa digunakan lagi di main thread
        // println!("{}", name);
        println!("main thread finished");
    }

    /*
     * Current Thread
     * Semua program di Rust akan berjalan di thread, termasuk walaupun kita tidak menggunakan thread sama sekali, secara default akan berjalan di main thread
     * Termasuk saat menjalankan unit test, Rust juga akan membuat thread secara otomatis
     * Untuk mendapatkan thread saat ini yang sedang digunakan untuk menjalankan kode program, kita bisa menggunakan thread::current()
     * Object thread direpresentasikan dalam struct Thread
     * https://doc.rust-lang.org/std/thread/struct.Thread.html
     */

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
        return counter;
    }

    #[test]
    fn test_current_thread() {
        let counter = calculate_current_thread();
        println!("Final counter value: {}", counter);
    }

    /*
     * Thread Factory
     *
     * Saat kita membuat thread, sebenarnya kita menggunakan Thread Factory, yaitu object untuk membuat thread
     * Secara default, Rust sudah membuatkan default Thread Factory, dan ketika menggunakan thread::spawn(), maka kita akan menggunakan default Thread Factory yang disediakan oleh Rust
     * Namun, kita juga bisa membuat Thread Factory secara manual
     * Hal ini mungkin dibutuhkan ketika kita ingin mengatur konfigurasi Thread Factory, atau nanti menggunakan library/framework yang membutuhkan Thread Factory
     * https://doc.rust-lang.org/std/thread/struct.Builder.html
     */

    #[test]
    fn test_thread_factory() {
        let thread_factory = thread::Builder::new()
            .name("MyThread".to_string())
            .stack_size(4 * 1024 * 1024); // 4 MB

        let handle: JoinHandle<i32> = thread_factory
            .spawn(|| {
                let mut counter: i32 = 0;
                for i in 1..=5 {
                    thread::sleep(Duration::from_secs(2));
                    println!("counter : {}", i);
                    counter += 1;
                }
                return counter;
            })
            .expect("Failed to create new Thread");

        let result = handle.join().unwrap();
        println!("total counter : {}", result);
    }

    /*
     * Thread comunication
     *
     * Saat kita membuat beberapa thread, mungkin yang menjadi pertanyaan adalah, bagaimana cara berkomunikasi antar thread? Misal kita mau
     * mengirim data dari satu thread ke thread lain?
     * Rust mirip seperti bahasa pemrograman Golang, yaitu menggunakan konsep Channel untuk berkomunikasi antar thread
     * https://doc.rust-lang.org/std/sync/mpsc/index.html
     */

    /*
     * Chanel
     *
     * Channel merupakan struktur data mirip seperti antrian (Queue), dimana thread bisa mengirim data ke channel dan bisa menerima data dari channel
     * Jadi antar thread tidak ada komunikasi secara langsung, melainkan melalui channel
     * Di dalam Channel, terdapat dua pihak, Sender (pengirim data) dan Receiver (penerima data)
     * Thread dalam satu waktu bisa saja berperan sebagai Sender sekaligus Receiver
     * Channel di Rust direpresentasikan dalam module mpsc (Multi Producer, Single Consumer)
     * https://doc.rust-lang.org/std/sync/mpsc/index.html
     */

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

    /*
     * Mengirim Banyak Data
     *
     * Karena channel itu bentuknya seperti struktur data queue, artinya kita bisa memasukkan banyak data kedalam channel
     * Saat sender mengirim data ke channel, dia akan langsung sukses walaupun data tersebut tidak ada yang mengambil
     * Sedangkan receiver ketika mengambil data dari channel, ketika tidak ada nya, maka receiver akan menunggu sampai datanya ada
     */

    #[test]
    fn test_send_may_data_to_chanel() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle_sender: JoinHandle<()> = thread::spawn(move || {
            for i in 1..=5 {
                thread::sleep(Duration::from_secs(1));
                let message = format!("Hello from thread 1, message {}", i);
                sender.send(message);
            }
            sender.send("Exit".to_string());
        });

        let handle_receiver: JoinHandle<()> = thread::spawn(move || {
            loop {
                let message = receiver.recv().unwrap();

                if message == "Exit" {
                    break;
                }

                println!("Received message : {}", message);
            }
        });

        let _ = handle_sender.join();
        let _ = handle_receiver.join();
    }

    /*
     * Chanel Livecycle
     *
     * Saat membuat channel, secara otomatis akan dibuatkan Sender dan Receiver
     * Saat life cycle Sender berakhir, dan Sender dihapus dari memori, secara otomatis kita tidak akan bisa menerima data apapun dari Receiver
     * Oleh karena itu, sebenarnya kita tidak perlu membuat kode break menggunakan perulangan seperti pada kode sebelumya
     * Receiver merupakan implementasi dari Iterator, sehingga kita bisa lakukan iterasi menggunakan for loop
     * Begitu juga sebaliknya, ketika life cycle Receiver sudah berakhir, saat kita mengirim ke Sender, maka akan terjadi error
     */

    #[test]
    fn test_chanel_livecycle() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();
        let handle_sender: JoinHandle<()> = thread::spawn(move || {
            for i in 1..=5 {
                thread::sleep(Duration::from_secs(1));
                let message = format!("Hello from thread 1, message {}", i);
                sender.send(message);
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

    /*
     * Multi Sender
     *
     * Di awal kita tahu bahwa module untuk channel bernama Multi Producer Single Consumer, artinya kita tahu bahwa harusnya kita bisa membuat Multi Producer / Sender
     * Namun bagaimana caranya? Karena ownership dari Sender kita pindahkan ke Closure di Thread nya?
     * Caranya adalah kita bisa melakukan Clone data Sender, secara otomatis Sender hasil Clone akan mengirim ke Receiver yang sama
     */
    #[test]
    fn test_multi_sender() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();
        let sender_clone = sender.clone();

        let result_join_handle1 =
            thread::Builder::new()
                .name("thread kim".to_string())
                .spawn(move || {
                    for i in 1..=5 {
                        thread::sleep(Duration::from_secs(2));
                        sender_clone.send("send from sender clone".to_string());
                    }
                });

        let result_join_handle2 = thread::Builder::new()
            .name("thread abdillah".to_string())
            .spawn(move || {
                for i in 1..=5 {
                    thread::sleep(Duration::from_secs(2));
                    sender.send("send from main sender".to_string());
                }
            });

        let result_receiver_join_hendle = thread::Builder::new()
            .name("receiver thread".to_string())
            .spawn(move || {
                for message in receiver.iter() {
                    println!("{}", message);
                }
            });

        match result_join_handle1 {
            Ok(handel) => {
                let _ = handel.join();
            }
            Err(_) => {}
        }
        match result_join_handle2 {
            Ok(handle) => {
                let _ = handle.join();
            }
            Err(_) => {}
        }
        match result_receiver_join_hendle {
            Ok(hendle) => {
                let _ = hendle.join();
            }
            Err(_) => {}
        }
    }

    /*
     * Race Condition
     *
     * Salah satu masalah ketika kita membuat aplikasi berbasis multi thread adalah, masalah Race Condition
     * Race Condition adalah kejadian dimana dua atau lebih thread mengubah ke mutable data yang sama
     * Ketika cara mengubahnya salah, maka bisa terjadi yang masalah Race Condition, sehingga hasil data tidak sesuai dengan yang kita inginkan
     * Misal, kita akan membuat beberapa thread untuk mengubah data counter
     */

    static mut COUNTER: i32 = 0;

    #[test]
    fn test_thread_race_condition() {
        let mut handles = vec![];
        for _ in 0..=10 {
            let handle = thread::spawn(|| unsafe {
                for j in 0..=100000 {
                    COUNTER += 1;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("counter : {}", unsafe { COUNTER }); // hasilnya tidak akan pernah sama karena terjadi race condition
    }

    /*
     * Bagaimana Caranya Mengatasi Race Condition
     *
     * Ada beberapa cara untuk mengatasi Race Condition
     * Menggunakan Atomic
     * Atau menggunakan Lock
     * Kita akan bahas di materi-materi sendiri
     */

    /*
     * Atomic
     *
     * Atomic merupakan tipe data yang digunakan untuk sharing untuk beberapa thread
     * Atomic sendiri merupakan tipe data yang membungkus tipe data aslinya
     * Kita bisa pilih jenis tipe data Atomic, sesuai dengan tipe data aslinya yang akan kita gunakan
     * Tipe data Atomic digaransi aman terhadap Race Condition
     * https://doc.rust-lang.org/std/sync/atomic/index.html
     */

    #[test]
    fn test_atomic() {
        use std::sync::atomic::{AtomicI32, Ordering};

        static counter: AtomicI32 = AtomicI32::new(0);
        let mut handles = vec![];
        for _ in 1..=10 {
            let hendle = thread::spawn(move || {
                for _ in 1..=100000 {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            });
            handles.push(hendle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("counter : {}", counter.load(Ordering::Relaxed));
    }

    /*
     * Atomic Reference
     *
     * Salah satu problem ketika sharing data menggunakan multi thread di Rust adalah, ownership dari data harus dipindah ke thread, sedangkan dalam satu waktu, hanya boleh satu thread yang own data tersebut
     * Oleh karena itu pada kode Atomic sebelumnya, kita gunakan static agar scope nya global, namun kadang tidak semua kasus kita bisa menggunakan static
     * Rust menyediakan Arc (Atomic Reference Counted), yaitu tipe data yang bisa digunakan untuk membuat reference ke data lain, tipe ini mirip seperti tipe Rc, namun karena semua operasi Arc itu atomic, oleh karena itu operasinya lebih mahal tapi keuntungannya adalah thread safe
     * https://doc.rust-lang.org/std/sync/struct.Arc.html
     */

    use std::sync::Arc;
    #[test]
    fn test_atomic_reference() {
        let counter = Arc::new(AtomicI32::new(0));
        let mut handles = vec![];
        for _ in 0..=10 {
            let counter_clone = Arc::clone(&counter);
            let hendle = thread::spawn(move || {
                for j in 0..=100000 {
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                }
            });
            handles.push(hendle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("counter : {}", counter.load(Ordering::Relaxed));
    }

    /*
     * Mutex
     *
     * Mutex adalah Mutual Exclusion, yaitu tipe data yang digunakan untuk melindungi data yang di-sharing ke lebih dari satu thread
     * Mutex akan memblok thread dan menunggu sampai lock (kunci) tersedia
     * Kita bisa menggunakan method lock() pada Mutex untuk menunggu sampai mendapatkan data, dan setelah data keluar dari scope, maka lock (kunci) akan dikembalikan ke Mutex sehingga thread lain bisa mengambil lock (kunci) nya
     * https://doc.rust-lang.org/std/sync/struct.Mutex.html
     */

    #[test]
    fn test_mutex() {
        let counter: Arc<std::sync::Mutex<i32>> = Arc::new(std::sync::Mutex::new(0));
        let mut handles = vec![];
        for _ in 0..=10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for j in 0..100000 {
                    let mut num = counter_clone.lock().unwrap();
                    *num += 1;
                }
                // data akan di lock secara otomatis ketika keluar dari scope, sehingga thread lain bisa mengambil lock (kunci) nya
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap()
        }

        println!("counter : {}", *counter.lock().unwrap());
    }

    /*
     * Thread Local
     *
     * Rust memiliki fitur untuk menyimpan data di Thread bernama Thread Local
     * Konsep Thread Local di Rust mirip seperti di Java, dimana alur hidup data akan mengikuti Thread, jika Thread selesai, maka data di Thread Local akan di drop
     * Hal ini cocok ketika kita ingin membuat data yang memang ingin digunakan dalam scope thread selama thread tersebut aktif, dan tidak bertukar dengan thread lain
     */

    /*
     * Membuat Data di Thread Local
     *
     * Untuk membuat data di Thread Local, kita harus buat menggunakan macro thread_local!
     * Kita bisa tentukan menggunakan Cell atau RefCell, tergantung apakah tipe datanya mutable atau tidak
     */

    thread_local! {
       pub static NAME: std::cell::RefCell<String> = std::cell::RefCell::<String>::new("Default".to_string());
    }

    #[test]
    fn test_thread_local() {
        let handle = thread::spawn(|| {
            NAME.with_borrow_mut(|name| {
                *name = "Kim".to_string();
            });

            NAME.with_borrow(|name| {
                println!("name : {}", name);
            })
        });
        let _ = handle.join();
        NAME.with_borrow(|name| {
            println!("name : {}", name);
        })
    }

    /*
     * Thread Panic
     *
     * Apa akibatnya ketika terjadi panic di dalam thread?
     * Maka thread tersebut akan berhenti, tapi tidak akan menghentikan thread lainnya
     * Jadi tidak perlu khawatir ketika menjalankan thread baru, dan terjadi panic pada thread tersebut, maka thread utama (main) tidak akan berhenti, karena berbeda thread
     * Kecuali jika terjadi panic di thread utama (main), otomatis thread utama akan berhenti
     */

    #[test]
    fn test_thread_panic() {
        let handle: JoinHandle<_> = thread::spawn(|| {
            for i in 1..=5 {
                println!("processing : {}", i);
                thread::sleep(Duration::from_secs(1));
                if i == 3 {
                    panic!("Thread panicked at processing {}", i);
                }
            }
        });
        match handle.join() {
            Ok(_) => println!("Thread completed successfully"),
            Err(e) => println!("Thread panicked: {:?}", e),
        }
    }

    /*
     * Barier
     *
     * Barier merupakan tipe data yang bisa digunakan agar beberapa thread menunggu sebelum melakukan pekerjaannya secara bersamaan
     * Contoh misal, kita akan membuat kode program yang menunggu jika 10 thread sudah ada, baru semuanya boleh berjalan, jika belum 10 thread, maka program tidak boleh berjalan terlebih dahulu
     * https://doc.rust-lang.org/std/sync/struct.Barrier.html
     */

    #[test]
    fn test_barier() {
        let barier = std::sync::Arc::new(std::sync::Barrier::new(10));
        let mut handles = vec![];
        for i in 0..=10 {
            let barier_clone = std::sync::Arc::clone(&barier);
            let handle = thread::spawn(move || {
                println!("Thread {} is waiting at the barrier", i);
                barier_clone.wait();
                println!("Thread {} passed the barrier", i);
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }

    /*
     * Once
     *
     * Kadang ada kasus kita membuat variable yang perlu diinisialisasi datanya diawal cukup sekali saja
     * Namun ingin memastikan bahwa hanya ada satu thread yang bisa memanggil proses inisialisasi datanya
     * Kita bisa menggunakan Once untuk membantu hal ini
     * Once bisa menjaga bahwa hanya ada satu thread saja yang bisa memanggil proses inisialisasi, dan hanya sekali saja dipanggil
     * https://doc.rust-lang.org/std/sync/struct.Once.html
     */

    static mut TOTAL_COUNTER: i32 = 0;
    static TOTAL_INIT: std::sync::Once = std::sync::Once::new();

    fn get_total_counter() -> i32 {
        unsafe {
            TOTAL_INIT.call_once(|| {
                TOTAL_COUNTER += 1;
            });
            return TOTAL_COUNTER;
        }
    }

    #[test]
    fn test_once() {
        let mut handles = vec![];
        for _ in 0..10 {
            let handle = thread::spawn(|| {
                let total = get_total_counter();
                println!("total : {}", total);
            });

            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }

    /*
     * Future
     *
     * Future adalah representasi dari komputasi asynchronous
     * Future merupakan value yang memungkinkan komputasinya belum selesai. Dengan menggunakan Future, memungkinkan thread untuk melanjutkan pekerjaan lainnya, selama menunggu nilainya ada pada Future
     * Future mirip dengan Promise di JavaScript, atau mirip dengan Future di Java
     * https://doc.rust-lang.org/std/future/trait.Future.html
     */

    /*
     * Pool
     *
     * Future memiliki satu method bernama poll(), yang digunakan untuk mengambil data di Future
     * Hasil dari poll() method adalah data enum Poll
     * Pada enum Poll, terdapat dua opsi, Ready jika data sudah ada, Pending jika data belum tersedia
     * https://doc.rust-lang.org/std/task/enum.Poll.html
     */

    /*
     * Membuat Future
     *
     * Future merupakan Trait, untuk membuat Future, kita perlu menggunakan method dengan kata kunci async
     * Method dengan kata kunci async, secara otomatis datanya akan mengembalikan tipe data Future
     * Kata kunci async akan kita bahas dimateri selanjutnya
     */

    /*
     * Async
     *
     * Seperti yang dijelaskan di awal, untuk membuat Future, kita tidak buat secara manual, kita akan menggunakan kata kunci async
     * Function yang menggunakan kata kunci async, maka return value nya adalah Future
     */

    async fn get_async_data() -> String {
        thread::sleep(Duration::from_secs(3));
        return "Hello from async".to_string();
    }

    /*
     * Memanggil Kode Async
     *
     * Kode async tidak bisa dipanggil pada kode non async, oleh karena itu untuk memanggil kode async, kita harus menggunakan kode async
     * Sayangnya, secara default Rust hanya menyediakan kontrak untuk membuat kode async, ketika ingin menjalankan kode async, kita perlu menggunakan Runtime / Executor, dan secara default Rust tidak menyediakan
     * Oleh karena itu, kita perlu menggunakan library Runtime / Executor untuk menjalankan kode async
     */

    /*
     * Liberary Runtime unutk Async
     *
     * Ada banyak library Runtime untuk Async, seperti :
     * Tokio : https://docs.rs/tokio/latest/tokio/
     * Async Std : https://docs.rs/async-std/latest/async_std/
     * Smol : https://docs.rs/smol/latest/smol/
     * kita akan menggunakan Tokio, salah satu library Runtime untuk Async yang lumayan populer di Rust
     */

    /*
     * Menginstall Tokio
     *
     * Silahkan tambahkan Library Tokio menggunakan perintah :
     * cargo add tokio --features full
     */

    /*
     * Async test
     * 
     * Untuk melakukan pengetesan kode Async, kita bisa menggunakan Tokio
     * Hal ini karena secara default Rust tidak mendukung unit test kode async
     * Kita bisa menggunakan attribute tokio::test
     */


    #[tokio::test]
    async fn test_future() {
        let result = get_async_data();

        /*
         * Await
         * 
         * Secara default, Future merupakan tipe data Lazy, artinya tidak akan dieksekusi jika tidak dijalankan
         * Agar Future dieksekusi, kita bisa menggunakan await
         * Await hanya bisa digunakan dalam kode async, karena yang dilakukan await sebenarnya adalah melakukan poll() terhadap Future, berbeda dengan join() pada Thread
         */

        let result_data = result.await;
        println!("result : {}", result_data);
    }

    /*
     * Masalah Dengan Thread
     * 
     * Salah satu permasalahan dengan Thread adalah, Thread masih dianggap mahal jika kita menggunakan terlalu banyak
     * Thread akan dijalankan dalam OS (Operating System) Thread, yang artinya ukuran per Thread bisa mencapai 2-4MB
     * Dengan begitu, akan sangat terbatas dengan jumlah memory yang kita gunakan
     * Di bahasa pemrograman seperti Golang atau Kotlin, terdapat fitur Lightweight Thread, seperti Goroutines atau Coroutines
     * Di Rust, fitur ini juga tersedia, dan bernama Task
     * https://doc.rust-lang.org/std/task/index.html 
     */

    /*
     * Tokio task
     * 
     * Rust menyediakan kontrak untuk Task, namun implementasinya tetap kita perlu menggunakan Runtime Async yang kita gunakan
     * Kita bisa menggunakan Tokio Task untuk membuat Task, dan cara penggunaannya mirip seperti Thread
     * https://docs.rs/tokio/latest/tokio/task/index.html 
     * Yang perlu diperhatikan adalah, saat menggunakan Task, jangan menggunakan fitur Thread seperti Sleep, karena itu bisa menghentikan Thread yang digunakan oleh Task
     */

    /*
     * Concurrent
     * 
     * Task adalah implementasi dari Concurrent, dimana jika kita menggunakan Thread, Thread tidak bisa berpindah-pindah pekerjaan, harus menyelesaikan pekerjaan sampai selesai
     * Sedangkan Task, sebenarnya secara internal, Task tetap akan dijalankan dalam Thread, namun Thread yang menjalankan Task, bisa berpindah-pindah Task sesuai kebutuhan, misal ketika kita menghentikan Task dengan sleep(), Thread akan menjalankan Task yang lainnya
     */

    async fn load_data_from_database(wait: u64) -> String {
        println!("Start loading data from database...");
        tokio::time::sleep(Duration::from_secs(wait)).await;
        println!("Finished loading data from database");
        return "Data from database".to_string();
    }

    #[tokio::test]
    async fn test_concurrent() {
        let mut handles = vec![];
        for i in 0..=5 {
            let handle = tokio::spawn(load_data_from_database(i));
            handles.push(handle);
        }
        for handle in handles {
            let result = handle.await.unwrap();
            println!("result : {}", result);
        }
    }

    /*
     * Task Runtime
     * 
     * Secara default Tokio sudah menyediakan Runtime yang bisa langsung kita gunakan
     * Namun, pada keadaan tertentu, mungkin kita ingin membuat Runtime sendiri agar lebih flexible, seperti mengatur jumlah thread nya misalnya
     * Hal ini bisa kita lakukan, hanya saja, Tokio Runtime secara default ketika di-drop (keluar dari scope), dia tidak boleh di-drop di kode async
     * Oleh karena itu, kita harus buat Tokio Runtime di scope kode non async
     */

    async fn run_concurrent_process(runtime: std::sync::Arc::<tokio::runtime::Runtime>){
        let mut handles = vec![];
        for i in 0..=5{
            let handle = runtime.spawn(load_data_from_database(i));
            handles.push(handle);
        }

        for handle in handles {
            let result = handle.await.unwrap();
            println!("result : {}", result);
        }
    }

    #[test]
    fn test_runtime() {
        let runtime = std::sync::Arc::new(tokio::runtime::Builder::new_multi_thread()
            .worker_threads(10)
            .enable_time()
            .build()
            .unwrap());
        
        // menjalankan async function didalam runtime
        runtime.block_on(run_concurrent_process(std::sync::Arc::clone(&runtime)));

    }
}
