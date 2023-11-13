use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;


mod concurrency{
    use std::{thread, time::Duration, sync::mpsc};


   pub fn run_concurrency(){

     let (tx, rx) = mpsc::channel::<String>();
     thread::spawn(move || {
       let val = String::from("Hello");
       tx.send(val).unwrap();
     });

    let received = rx.try_recv().unwrap();
    println!("msg from tx: {}", received);

     let t1 = thread::spawn(||{
        for i in 1..10{
            println!("Child A thread, {}", i);
            thread::sleep(Duration::from_millis(1));
        }
     });

     let t2 = thread::spawn(||{
        for i in 1..100{
            println!("Child B thread, {}", i);
            thread::sleep(Duration::from_millis(1));
        }
     });


     for i in 1..5{
        println!("Main thread, {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    t1.join().unwrap();
    t2.join().unwrap();
   }

}

mod refcell{
    use std::cell::RefCell;
    pub fn refcell_runner(){
      let a = 5;
      let aa = RefCell::new(a);
      let mut b = aa.borrow_mut();
      *b = 3;


      println!("b is {}", b);
    }
}



mod smart_pointers {
    use std::ops::Deref;
    use std::rc::Rc;

    pub fn run_smart_pointers() {
        let x = 25;
        let y = MyBox::new(x);
        // println!("X = {}", x);
        // println!("Y = {}", *y);
        // println!("Sum of X and Y it = {}", x + *y);

        let st = A { x: 2 };
        let a = Rc::new(st);
        println!("The count to this reference is {}", Rc::strong_count(&a));

        {
            let b = Rc::clone(&a);
            let b = Rc::clone(&a);
            println!("The count to this reference is {}", Rc::strong_count(&a));
        }
        println!("The count to this reference is {}", Rc::strong_count(&a));
    }

    struct A {
        x: i32,
    }

    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            println!("Deref Called");
            &self.0
        }
    }

    impl<T> Drop for MyBox<T> {
        fn drop(&mut self) {
            println!("Dropped");
        }
    }
}

mod closures {
    pub fn run_closures() {
        let is_odd = |x: i32| {
            return x % 2 == 1;
        };

        let num = 10;
        println!("is 10 odd? = {}", is_odd(num));
        let add_num = |n: i32| num + n;
        println!("Adder {}", add_num(6));
    }
}

mod iterators {
    pub fn run_iterators() {
        let mut nums = vec![1, 2, 3, 4, 5];
        //   let it = nums.iter();
        //  let it = nums.into_iter();
        //  for val in it {
        //     println!("Val {}", val)
        //  }
        //  println!("{:?}", nums)// not allowed
        //   for val in it{
        //     println!("testing iters, {}", val);
        //   }

        let it_mut = nums.iter_mut();
        for val in it_mut {
            *val = *val * 2;
        }
        println!("{:?}", nums); // reuse -- borrowing
    }
}

mod file {
    use std::{
        fs::OpenOptions,
        io::{Read, Write},
    };

    pub fn run_filer() {
        let mut f = std::fs::File::create("hello.tcxt").expect("Unable to create file");
        f.write_all("Jus wrtting the first line".as_bytes())
            .expect("Could not write to file");
        f.write_all("Writting to Second Line".as_bytes())
            .expect("Write failed");

        let mut fr = std::fs::File::open("hello.tcxt").unwrap();
        let mut str = String::new();
        fr.read_to_string(&mut str).unwrap();

        println!("Read the following from file , {}\n", str);
        let mut fa = OpenOptions::new()
            .append(true)
            .open("hello.tcxt")
            .expect("cannot open file");

        fa.write_all("\nbuffer sample2\n".as_bytes())
            .expect("Error Writing");
        println!("Appended data to the file succesfully");
    }
}

mod io {
    pub fn io_runner() {
        let reader = std::io::stdin();
        let mut line = String::new();
        let b = reader.read_line(&mut line).unwrap();
        println!("Line {}", line);
        println!("Num of bytes {}", b);
        use std::io::Write;
        let mut writer = std::io::stdout();
        let b1 = writer.write("abcd\n xy".as_bytes()).unwrap();
        println!("Num of bytes written {}", b1);
    }
}

mod traiter {
    pub fn run_traiter() {
        let d = Dog {
            color: "Brown".to_string(),
            age: 3,
        };

        let c = Cat {
            color: "Blue".to_string(),
            age: 3,
            breed: "kings".to_string(),
        };

        d.speak();
        c.speak();
        d.voice_level();
        c.voice_level();
    }

    struct Dog {
        color: String,
        age: i32,
    }

    struct Cat {
        color: String,
        age: i32,
        breed: String,
    }

    trait voice {
        fn speak(&self);
        fn voice_level(&self) {
            println!("Voice level is Default");
        }
    }

    impl voice for Dog {
        fn speak(&self) {
            println!("A dog says Bhow Bhow. We call it Gbo Gbo in Yoruba :)")
        }
    }

    impl voice for Cat {
        fn speak(&self) {
            println!("A dog says Bhow Meew. We call it Meawwww in Yoruba :)")
        }
    }
}

mod generics {

    pub fn run_generics() {
        let mut vec_int: Vec<i32> = vec![5, 10];
        vec_int.push(20);
        //    vec_int.push("Sunkanmi");
        println!("The result of the vect, {:?}", vec_int);
        let t: Data<i32> = Data { val: 32 };
        let t2: Data<&str> = Data { val: "Sunkanmi" };
        let p: PointXY<i32> = PointXY { x: 10, y: 20 };
        let p2 = PointXY { x: 10.3, y: 20.4 };
        println!("compare {:?}", max_i32(3.4, 5.4));
        // println!("x = {}, y= {}", p2.x, p2.y);
        //  println!("Hello world {:?}", t);
        //  println!("Hello world {:?}", t2);

        fn max_i32<T: PartialOrd>(a: T, b: T) -> T {
            if b > a {
                return b;
            }
            return a;
        }
    }
    #[derive(Debug)]
    struct Data<T> {
        val: T,
    }

    struct PointXY<T> {
        x: T,
        y: T,
    }
}
mod option_enum {

    pub fn run_option_enum() {
        fn is_odd(num: i32) -> Option<bool> {
            if num % 2 == 0 {
                return None;
            }
            Some(true)
        }

        println!("Is odd 3 = {:?}", is_odd(3));
        fn print_tshirt_type(size: TShirt) {
            match size {
                TShirt::Small => println!("The shirt is Small"),
                TShirt::Large => println!("The shirt is Large"),
                TShirt::Medium => println!("The shirt is medium"),
                _ => {}
            }
        }
        #[derive(Debug)]
        enum TShirt {
            Small,
            Medium,
            Large,
        }

        println!("Size of shirt  {:?}", TShirt::Large);
    }
}

mod panicer {
    use core::panic;
    use std::fs::File;
    pub fn run_panic() {
        let f = File::open("dummy.txt");
        let f = match f {
            Ok(file) => file,
            Err(error) => {
                panic!("Problem opening file , {:?}", error)
            }
        };
        panic!("Just testing panic");
        let f = File::open("dummy.txt").expect("File not Found :(");
        // let f = File::open("dummy.txt").unwrap();
    }
}

mod enums {
    pub fn run_enums() {
        let mon = Day::Monday;
        let p1 = Person {
            name: String::from("Abc"),
            birthday: Day::Wednesday,
        };
        println!("{:?}", p1);
    }
    #[derive(Debug)]
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
    }
    #[derive(Debug)]
    struct Person {
        name: String,
        birthday: Day,
    }
}

mod struct_example {

    pub fn run_struct_examples() {
        struct Rectangle {
            length: u32,
            breadth: u32,
        }

        impl Rectangle {
            fn area(&self) -> u32 {
                self.breadth * self.length
            }
        }
        let r = Rectangle {
            length: 2,
            breadth: 3,
        };
        println!("Hello World, {}", r.area())
    }
}

mod structs {
    struct User {
        username: String,
        email: String,
        signin_count: u64,
        active: bool,
    }
    pub fn run_struct() {
        let user2 = build_user(
            String::from("Sunkanmi Adewumi"),
            String::from("user2@email.com"),
        );
        println!("name = {}", user2.username);
        println!("email = {}", user2.email);
    }

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            signin_count: 1,
            active: true,
        }
    }
}

mod tuple_structs {
    struct ColorRGB(i32, i32, i32);

    pub fn run_tuple_struct() {
        let red = ColorRGB(255, 0, 0);
        let blue = ColorRGB(0, 0, 255);
        let magenta = add_colors(red, blue);
        println!("magemta= [{}, {}, {}]", magenta.0, magenta.1, magenta.2);

        fn add_colors(col1: ColorRGB, col2: ColorRGB) -> ColorRGB {
            ColorRGB(col1.0 + col2.0, col1.1 + col2.1, col1.2 + col2.2)
        }
    }
}

mod slices {
    pub fn run_slices() {
        let s = String::from("Hello world");
        let s1 = &s[..5];
        let s2 = &s[6..];
        println!("s1 and s2 respectively, {}, {}", s1, s2);

        let s = String::from("Hello world");
        let hello = &s[0..5];
        let world = &s[6..11];

        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        println!("Slice[1] = {}", slice[1])
    }
}

use enums::run_enums;
use generics::run_generics;
use slices::run_slices;
use struct_example::run_struct_examples;
use structs::run_struct;
use tuple_structs::run_tuple_struct;
// use enums::run_enums;
use closures::run_closures;
use file::run_filer;
use io::io_runner;
use iterators::run_iterators;
use panicer::run_panic;
use smart_pointers::run_smart_pointers;
use traiter::run_traiter;
use refcell::refcell_runner;
use concurrency::run_concurrency;
// use option_enum::run_option_enum;
fn main() {
    // run_closures();
    run_concurrency(); // run_slices();
                          //    {
                          //     let mut s1 = String::from("Hello");
                          //     s1.push_str(", Word");

    //     let s2 = s1.clone();
    //     println!("S2 is = {}", s2);
    //     println!("S1 is = {}", s1);
    // }
    // let s = 1;
    // takes_ownership(s);
    // let s1 = gives_ownership();
    // println!("S1 is {}", s1);

    // let length = get_length(&s1);
    // println!("Length of String {}", length);

    // let mut s = String::from("Sunkanmi");
    // let m1 = &mut s;
    // let m2 = &mut s;

    // println!("We are on a steak, {}, {}", m1, m2);
    // add_hello(&mut s);
    // add_hello2(&mut s);
    // println!("New s Value = , {}", s);
}

// fn add_hello(s:&mut String){
//   s.push_str(", Hello");
// }

// fn add_hello2(s:&mut String){
//     s.push_str(", Hello");
//   }
// fn takes_ownership(s: i32){
//     println!("s = {}", s);
// }

// fn gives_ownership() -> String {
//     let s = String::from("Hello World");
//     s
// }

// fn get_length(s: &String) -> usize {
//     let l = s.len();
//     l
// }
