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

        let handle: JoinHandle<i32> = thread_factory.spawn(|| {
            let mut counter: i32 = 0;
            for i in 1..=5 {
                thread::sleep(Duration::from_secs(2));
                println!("counter : {}", i);
                counter += 1;
            }
            return counter;
        }).expect("Failed to create new Thread");

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
    
            let handle_receiver: JoinHandle<()> = thread::spawn(move || loop{
               let message = receiver.recv().unwrap();
               
               if message == "Exit" {
                break;
               }

               println!("Received message : {}", message);
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
} 