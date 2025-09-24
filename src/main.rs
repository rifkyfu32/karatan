fn main() {
    println!("Hello, world lorem!");
}
#[test]
fn test_hello() {
    println!("Hello, world test!");
}
#[test]
fn test_variable() {
    let name = "Lorem Ipsum Dolor";
    println!("Hello {}", name);
}
#[test]
fn test_mutable() {
    let mut name = "Lorem";
    println!("Hello {}", name);

    name = "Dolor";
    println!("Hello {}", name);
}
#[test]
fn test_static_typing() {
    let name = "Lorem";
    println!("Hello {}", name);
    // name = 10;
    println!("Hello {}", name);
}
#[test]
fn test_shadowing() {
    let name = "Lorem";
    println!("Hello {}", name);
    let name = 10;
    println!("Hello {}", name);
}
#[test]
fn test_explicit() {
    let age: u8 = 20;
    println!("Age: {}", age);
}
#[test]
fn test_number() {
    let a: u8 = 10;
    println!("a: {}", a);
    let b: f32 = 10.5;
    println!("b: {}", b);
}
#[test]
fn test_number_conversion() {
    let a: u32 = 4000000000;
    println!("a: {}", a);
    let b: i16 = a as i16;
    println!("b: {}", b);
    let c: u32 = a as u32;
    println!("c: {}", c);
}
#[test]
fn test_numeric_operator() {
    let a = 5;
    let b = 20;
    let c = a * b;
    println!("a: {}", a);
    println!("c: {}", c);
}
#[test]
fn test_augmented_assigment() {
    let mut a = 10;
    println!("a: {}", a);
    a += 10;
    println!("a: {}", a);
    a *= 10;
    println!("a: {}", a);
}
#[test]
fn test_boolean() {
    let a = true;
    let b: bool = false;
    println!("{} {}", a, b);
}
#[test]
fn test_comparison() {
    let result: bool = 10 > 20;
    println!("result: {}", result);
}
#[test]
fn test_boolean_operator() {
    let presensi = 75;
    let nilai_akhir = 80;
    let lulus = presensi >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;
    let lulus_final = lulus && lulus_nilai_akhir;
    println!("Hasil: {}", lulus_final);
}
#[test]
fn test_char_type() {
    let char1: char = 'a';
    let char2: char = 'b';
    println!("{} {}", char1, char2);
}
#[test]
fn test_tuple() {
    let data: (i32, f64, bool) = (10, 3.14, true);
    println!("{:?}", data);
    let a = data.0;
    let b = data.1;
    let c = data.2;
    println!("{} {} {}", a, b, c);
    let (d, e, f) = data;
    println!("{} {} {}", d, e, f);
}
#[test]
fn test_tuple_mutable() {
    let mut data: (i32, f64, bool) = (10, 3.14, true);
    println!("{:?}", data);

    let (d, e, f) = data;
    println!("{} {} {}", d, e, f);

    data.0 = 20;
    data.1 = 2.5;
    data.2 = false;
    println!("{:?}", data);
}
fn unit() {
    println!("Hello unit");
}
#[test]
fn test_unit() {
    let result = unit();
    println!("{:?}", result);
    let test = ();
    println!("{:?}", test);
}
#[test]
fn test_array() {
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", array);
    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);
}
#[test]
fn test_array_mutable() {
    let mut array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", array);
    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);
    array[0] = 10;
    array[2] = 9;
    println!("{:?}", array);
    let length = array.len();
    println!("length: {}", length);
}
#[test]
fn test_two_dimension_array() {
    let matrix = [[1, 2], [3, 4]];
    println!("{:?}", matrix);
    println!("{}", matrix[0][0]);
    println!("{}", matrix[0][1]);
    println!("{}", matrix[1][0]);
    println!("{}", matrix[1][1]);
}
const MAXIMUM: u8 = 20;
#[test]
fn test_constant() {
    const MINIMUM: u8 = 1;
    println!("{} {}", MINIMUM, MAXIMUM)
}
#[test]
fn test_variable_scope() {
    let lorem = 1;
    {
        //inner scope
        println!("inner lorem: {}", lorem);
        let ipsum = 2;
        println!("inner ipsum: {}", ipsum);
    }
    //cannot access
    //println!("inner ipsum: {}", ipsum);
}
fn function_a() {
    let a = 10;
    let b = String::from("lorem");
    println!("{} {}", a, b);
}
fn function_b() {
    let a = 10;
    let b = String::from("ipsun");
    println!("{} {}", a, b);
}
#[test]
fn test_stack_heap() {
    function_a();
    function_b();
}
#[test]
fn test_str() {
    let name: &str = "  Lorem ipsum ";
    let trim = name.trim();
    println!("name: {}", name);
    println!("trim: {}", trim);
}
#[test]
fn test_string() {
    let mut name: String = String::from("Lorem ipsum");
    name.push_str(" dolor");
    println!("name: {}", name);
    let amet = name.replace("ipsum", "amet");
    println!("amet: {}", amet);
    println!("name: {}", name);
}
#[test]
fn test_ownership_rules() {
    // a tidak bisa diakses disini, belum dideklarasikan
    let a = 10; // a bisa diakses mulai dari sini
    {
        // b tidak bisa diakses disini, belum dideklarasikan
        let b = 20; // b bisa diakses mulai dari sini
        println!("{}", b);
    } // scope b selesai, b dihapus, b tidak bisa diakses lagi
    println!("{}", a);
} // scope a selesai, a dihapus, a tidak bisa diakses lagi
#[test]
fn test_data_copy() {
    let a = 10;
    let b = a;
    println!("{} {}", a, b);
    /* data a dicopy ke variable b karena
    bersifat fix size, primitif disimpan di stack */
}
#[test]
fn test_ownership_movement() {
    /*
    karena String disimpan di Heap,
    hanya reference yang ada di Stack,
    yang terjadi bukan copy tetapi transfer ownership
     */
    let name1: String = String::from("lorem");
    //ownership name1 dipindahkan ke name2
    let name2 = name1;
    // setelah transfer ownership, name1 tidak bisa diakses disini
    //println!("{}" , name1);
    println!("{}", name2);
}
#[test]
fn test_clone() {
    /*
    data di Stack akan dicopy,
    sedangkan data di Heap akan dipindahkan ownershipnya
    clone adalah membuat data tiruan yang sama dari data aslinya
    semua tipe data yang disimpan di Heap memiliki method clone()
    */
    let name1 = String::from("lorem");
    let name2 = name1.clone();
    //karena clone, name1 tetap bisa diakses
    println!("name1: {}", name1);
    println!("name2: {}", name2);
}
#[test]
fn test_if_expression() {
    let value = 9;
    if value >= 8 {
        println!("Good");
    } else if value >= 6 {
        println!("Not Good");
    } else {
        println!("Bad");
    }
}

#[test]
fn test_if_expression_let_statement() {
    let value = 9;
    let result: &str;
    if value >= 8 {
        result = "Good";
    } else if value >= 6 {
        result = "Not Good";
    } else {
        result = "Bad";
    }
    println!("{}", result);
    let hasil: &str = if value >= 8 {
        "Good"
    } else if value >= 6 {
        "Not Good"
    } else {
        "Bad"
    };
    println!("{}", hasil);
}
#[test]
fn test_loop_expression() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter >= 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }
        println!("{}", counter);
    }
}
#[test]
fn test_loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter >= 10 {
            break counter * 2;
        }
    };
    println!("{}", result);
}
#[test]
fn test_loop_label() {
    let mut urutan = 1;
    'Outer: loop {
        let mut pengali = 1;
        if urutan > 10 {
            break 'Outer;
        }
        loop {
            if pengali > 10 {
                break;
            }
            println!("{} x {} = {}", urutan, pengali, urutan * pengali);
            pengali += 1;
        }
        urutan += 1;
    }
}
#[test]
fn test_while_loop() {
    let mut counter = 1;
    while counter < 10 {
        if counter % 2 == 0 {
            println!("{}", counter);
        }
        counter += 1;
    }
}
#[test]
fn test_array_iteration() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut i = 0;
    while i < array.len() {
        println!("{}", array[i]);
        i += 1;
    }
    for item in array {
        println!("item {}", item);
    }
}
#[test]
fn test_range() {
    let array = ["A", "B", "C", "D", "E"];
    let range = 0..5;
    println!("Range start: {}", range.start);
    println!("Range end: {}", range.end);
    for index in range {
        println!("Item array {}", array[index])
    }
    let range_inclusive = 0..=4;
    for i in range_inclusive {
        println!("Item for dengan range inklusif {}", array[i]);
    }
}
fn say_hello(name: &str) {
    println!("Hello, {}!", name);
}
#[test]
fn test_say_hello() {
    say_hello("Lorem Ipsum");
}
fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbye {} {}", first_name, last_name);
}
#[test]
fn test_say_goodbye() {
    say_goodbye("Lorem", "Ipsum");
    say_goodbye("John", "Doe");
    say_goodbye("Dolor", "Amet");
}
fn factorial_loop(n: i64) -> i64 {
    if n < 1 {
        return 0;
    }
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}
#[test]
fn test_factorial_loop() {
    let result1 = factorial_loop(5);
    println!("factorial 5: {}", result1);
    let result2 = factorial_loop(10);
    println!("factorial 10: {}", result2);
    let result3 = factorial_loop(20);
    println!("factorial 20: {}", result3);
    println!("factorial: {}", factorial_loop(-5));
}
/*
Recursion: metode pemecahan masalah yang solusinya bergantung
pada solusi dari masalah yang lebih kecil yang merupakan bagian
dari masalah tersebut.
RUST memperbolehkan sebuah fungsi untuk memanggil dirinya sendiri
Recursive function: fungsi yang memanggil fungsi itu sendiri
 */
fn factorial_recursive(n: u64) -> u64 {
    if n == 1 {
        return 1;
    }
    n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_recursive() {
    println!("{}", factorial_recursive(10));
}
fn print_text(value: String, counter: u32) {
    if counter < 1 {
        return;
    } else {
        println!("{} ke {}", value, counter);
    }
    print_text(value, counter - 1);
}
#[test]
fn test_print_text() {
    print_text("Lorem ipsum".to_string(), 5);
}
fn print_number(number: i32) {
    println!("function print number: {}", number);
}
fn hi(name: String) {
    println!("function hi, {}", name);
}
#[test]
fn test_function_ownership() {
    let number = 10;
    print_number(number); // owner number tetap karena disimpan di Stack
    println!("number: {}", number);
    let name = "Lorem".to_string();
    hi(name); // owner variable name menjadi milik fungsi hi
    // println!("name: {}", name);
}
fn full_name(first_name: String, last_name: String) -> String {
    format!("{} {}", first_name, last_name)
}
#[test]
fn test_return_value_ownership() {
    let first_name = "Lorem".to_string();
    let last_name = "Ipsum".to_string();
    /*
    ownership first & last berpindah ke fungsi full name
    kemudian berpindah lagi ke name yang memanggil fungsi full name
     */
    let name = full_name(first_name, last_name);
    println!("{}", name);
    /*
    error karena sudah berpindah ownership
    println!("{}", first_name);
    println!("{}", last_name);
    */
}

fn full_name_ownership(first_name: String, last_name: String) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);
    (full_name, first_name, last_name)
}
#[test]
fn test_mengembalikan_ownership() {
    let first_name = "Lorem".to_string();
    let last_name = "Ipsum".to_string();
    let (full_name, first_name, last_name) = full_name_ownership(first_name, last_name);
    println!("{}", first_name);
    println!("{}", last_name);
    println!("{}", full_name);
}
fn full_name_reference(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}
#[test]
fn test_full_name_reference() {
    let first_name = "Lorem".to_string();
    let last_name = "Ipsum".to_string();
    let full_name = full_name_reference(&first_name, &last_name);
    println!("{}", first_name);
    println!("{}", last_name);
    println!("{}", full_name);
}
fn change_value(value: &mut String) {
    value.push_str(" Dolor");
}
#[test]
fn test_borrowing() {
    let mut name = "Lorem".to_string();
    change_value(&mut name);
    println!("{}", name);
}
#[test]
fn test_slice_reference() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice1 = &array[..];
    println!("{:?}", slice1);
    let mut slice2 = &array[..=5];
    println!("{:?}", slice2);
    slice2 = &array[..];
    println!("{:?}", slice2);
    let slice3 = &array[5..];
    println!("{:?}", slice3);
}
#[test]
fn test_string_slice() {
    let name = "Lorem Ipsum dolor";
    let first_name: &str = &name[..=4];
    println!("first name: {}", first_name);
    let last_name = &name[12..];
    println!("last name: {}", last_name);
}
struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8,
}
#[test]
fn test_struct_person() {
    let person = Person {
        first_name: "Lorem".to_string(),
        middle_name: "Ipsum".to_string(),
        last_name: "Dolor".to_string(),
        age: 24,
    };
    println!("fist name: {}", person.first_name);
    println!("middle name: {}", person.middle_name);
    println!("last name: {}", person.last_name);
    println!("age: {}", person.age);
}
fn print_person(person: &Person) {
    println!("first name: {}", person.first_name);
    println!("middle name: {}", person.middle_name);
    println!("last name: {}", person.last_name);
    println!("age: {}", person.age);
}
#[test]
fn test_print_person() {
    let person = Person {
        first_name: "John".to_string(),
        middle_name: "Doe".to_string(),
        last_name: "Ipsum".to_string(),
        age: 27,
    };
    print_person(&person);
}
#[test]
fn test_init_shorthand() {
    let first_name = "Lorem".to_string();
    let last_name = "Dolor".to_string();
    let person = Person {
        first_name,
        middle_name: "Ipsum".to_string(),
        last_name,
        age: 20,
    };
    print_person(&person);
    let mut person2 = Person {
        first_name: person.first_name.clone(),
        middle_name: person.middle_name.clone(),
        last_name: person.last_name.clone(),
        ..person
    };
    person2.first_name = "Change".to_string();
    print_person(&person2);
    print_person(&person);
}
struct GeoPoint(f64, f64);
#[test]
fn test_geopoint() {
    let geo_point = GeoPoint(-6.43234323, 104.34320002);
    println!("latitute: {}", geo_point.0);
    println!("longitute: {}", geo_point.1);
}
#[derive(Debug)]
struct Nothing;
#[test]
fn test_struct_nothing() {
    let nothing1 = Nothing;
    let nothing2 = Nothing {};
    println!("{:?}", nothing1);
    println!("{:?}", nothing2);
}
impl Person {
    fn say_hello(&self, name: &str) {
        println!("Hello, {} my name is {}", name, self.first_name);
    }
}
#[test]
fn test_method_person() {
    let person = Person {
        first_name: "John".to_string(),
        middle_name: "Doe".to_string(),
        last_name: "Lorem".to_string(),
        age: 32,
    };
    person.say_hello("Ipsum");
}
impl GeoPoint {
    fn new(lat: f64, lon: f64) -> GeoPoint {
        GeoPoint(lat, lon)
    }
}
#[test]
fn test_associated_function() {
    let geo_point = GeoPoint::new(-6.234122, 103.23404);
    println!("Latitude: {}", geo_point.0);
    println!("Longitude: {}", geo_point.1);
}
enum Level {
    Regular,
    Premium,
    Platinum,
}
#[test]
fn test_enum_level() {
    let _platinum = Level::Platinum;
    let _regular = Level::Regular;
    let _premium = Level::Premium;
}
enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String, String),
}
impl Payment {
    fn pay(&self, amount: u32) {
        match self {
            Payment::BankTransfer(bank, number) => println!(
                "Paying with bank transfer {} {} amount {}",
                bank, number, amount
            ),
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} amount {}", number, amount)
            }
            Payment::EWallet(wallet, number) => println!(
                "Paying with E-wallet {} {} amount {}",
                wallet, number, amount
            ),
        }
    }
}
#[test]
fn test_enum_payment() {
    let bank_transfer = Payment::BankTransfer("BCA".to_string(), "2139102390291".to_string());
    bank_transfer.pay(1000);
    let ewallet = Payment::EWallet("Gopay".to_string(), "085326967372".to_string());
    ewallet.pay(5000);
    let credit_card = Payment::CreditCard("234234234234234".to_string());
    credit_card.pay(90000);
}
#[test]
fn test_pattern_matching_enum() {
    let level = Level::Platinum;
    match level {
        Level::Platinum => println!("Level platinum"),
        Level::Premium => println!("Level premiun"),
        Level::Regular => println!("Level regular"),
    }
}
#[test]
fn test_pattern_matching_value() {
    let name = "Fuad";
    match name {
        "Fuad" | "Rifky" => println!("Hello bos!"),
        "Ipsum" => println!("Name is Ipsum"),
        "Dolor" => println!("Name is Dolor"),
        _other => println!("Name is Lorem"),
    }
}
#[test]
fn test_range_pattern() {
    let value = 1;
    match value {
        75..=100 => println!("Great"),
        60..=74 => println!("Good"),
        40..=59 => println!("Bad"),
        0..=39 => println!("Very Bad"),
        _other => println!("Value {} undefined", value),
    }
}
#[test]
fn test_struct_pattern() {
    let geo_point = GeoPoint::new(0.0, 0.0);
    match geo_point {
        GeoPoint(0.0, 0.0) => println!("Undefined value"),
        GeoPoint(lat, 0.0) => println!("Latitude: {}", lat),
        GeoPoint(0.0, lon) => println!("Longitude: {}", lon),
        GeoPoint(lat, lon) => println!("Latitude: {}, Longitude: {}", lat, lon),
    }
    let person = Person {
        first_name: "Lorem".to_string(),
        middle_name: "Ipsum".to_string(),
        last_name: "Dolor".to_string(),
        age: 38,
    };
    match person {
        Person {
            first_name,
            last_name,
            ..
        } => println!("Hello {} {}", first_name, last_name),
    }
}
#[test]
fn test_match_expression() {
    let value = 8;
    let result = match value {
        0 => "Januari",
        1 => "Febuari",
        2 => "Maret",
        3 => "April",
        4 => "Mei",
        5 => "Juni",
        6 => "Juli",
        7 => "Agustus",
        8 => "September",
        _ => "Invalid",
    };
    println!("Bulan: {}", result);
}
type Age = u8;
type IdentityNumber = String;
struct Customer {
    id: IdentityNumber,
    age: Age,
    name: String,
}
#[test]
fn test_alias() {
    let customer = Customer {
        id: "3324123123".to_string(),
        age: 24,
        name: "Lorem Ipsum".to_string(),
    };
    println!("id: {}", customer.id);
    println!("age: {}", customer.age);
    println!("name: {}", customer.name);
}
mod model;
#[test]
fn test_modul() {
    let user = model::User {
        id: "43123123".to_string(),
        name: "Lorem Ipsum".to_string(),
        age: 24,
    };
    println!("id: {}", user.id);
    println!("name: {}", user.name);
    println!("age: {}", user.age);
    user.say_hello("Dolor");
}

mod first;
mod second;
mod third;
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque},
    fmt::Debug,
    i32,
    ops::{Add, Deref},
    rc::Rc,
};

use first::say_hello as say_hello_first;
use first::second::third::say_hello as say_hello_first_chain;
use second::say_hello as say_hello_second;
#[test]
fn test_use_modul() {
    say_hello_first();
    say_hello_second();
    say_hello_first_chain();
}
trait CanSayHello {
    fn hello(&self) -> String {
        "Hello".to_string()
    }
    fn say_hello_trait(&self) -> String;
    fn say_hello_to(&self, name: String) -> String;
}
impl CanSayHello for Person {
    fn say_hello_trait(&self) -> String {
        format!("Hello, my name is {} from cansayhello", self.first_name)
    }

    fn say_hello_to(&self, name: String) -> String {
        format!(
            "Hello {}, my name is {} from cansayhello",
            name, self.first_name
        )
    }
}
#[test]
fn test_can_say_hello() {
    let person = Person {
        first_name: "Lorem".to_string(),
        middle_name: "Ipsum".to_string(),
        last_name: "Dolor".to_string(),
        age: 24,
    };
    println!("{}", person.hello());
    println!("{}", person.say_hello_trait());
    println!("{}", person.say_hello_to("John".to_string()));
}
fn trait_param(person: &impl CanSayHello) {
    println!("{}", person.say_hello_trait());
}
#[test]
fn test_trait_param() {
    let person = Person {
        first_name: "Lorem".to_string(),
        middle_name: "Ipsum".to_string(),
        last_name: "Dolor".to_string(),
        age: 24,
    };
    trait_param(&person);
}
trait CanSayGoodbye {
    fn say_goodbye(&self) -> String;
    fn say_goodbye_to(&self, name: &str) -> String;
}
impl CanSayGoodbye for Person {
    fn say_goodbye(&self) -> String {
        format!("Goodbye {}", self.first_name)
    }

    fn say_goodbye_to(&self, name: &str) -> String {
        format!("Goodbye, {} my name is {}", name, self.first_name)
    }
}
fn hello_goodbye(person: &(impl CanSayHello + CanSayGoodbye)) {
    println!("{}", person.say_hello_trait());
    println!("{}", person.say_hello_to("John".to_string()));
    println!("{}", person.say_goodbye());
    println!("{}", person.say_goodbye_to("Doe"));
}
#[test]
fn test_hello_goodbye() {
    let person = Person {
        first_name: "Psikjih".to_string(),
        middle_name: "Arutala".to_string(),
        last_name: "Young".to_string(),
        age: 25,
    };
    hello_goodbye(&person);
}
struct SimplePerson {
    name: String,
}
impl CanSayGoodbye for SimplePerson {
    fn say_goodbye(&self) -> String {
        format!("Goodbye {}", self.name)
    }

    fn say_goodbye_to(&self, name: &str) -> String {
        format!("Goodbye {}, my name is {}", name, self.name)
    }
}
fn create_simple(name: String) -> impl CanSayGoodbye {
    SimplePerson { name: name }
}
#[test]
fn test_trait_return() {
    let person = create_simple("Lorem".to_string());
    println!("{}", person.say_goodbye());
    println!("{}", person.say_goodbye_to("Doe"));
}
trait CanSay: CanSayHello + CanSayGoodbye {
    fn say(&self) {
        println!("{}", self.say_goodbye());
        println!("{}", self.say_hello_trait());
    }
}
impl CanSay for Person {
    fn say(&self) {
        std::println!("{}", self.say_goodbye());
        std::println!("{}", self.say_hello_trait());
    }
}
#[test]
fn test_can_say() {
    let person = Person {
        first_name: "John".to_string(),
        middle_name: "Doe".to_string(),
        last_name: "Lorem".to_string(),
        age: 20,
    };
    person.say();
}
struct Point<T = i32> {
    x: T,
    y: T,
}
#[test]
fn test_generic_point() {
    let float = Point { x: 3.14, y: 4.5 };
    println!("x: {} y: {}", float.x, float.y);
    let integer = Point { x: 4, y: 10 };
    println!("x: {} y: {}", integer.x, integer.y);
}
enum Value<T> {
    Value(T),
}
#[test]
fn test_enum_generic() {
    let value = Value::Value("pointer str".to_string());
    match value {
        Value::Value(val) => println!("{}", val),
    }
}
struct Hi<T: CanSayGoodbye> {
    value: T,
}
#[test]
fn test_generic_with_trait() {
    let person = Hi {
        value: SimplePerson {
            name: "John Doe".to_string(),
        },
    };
    println!("{}", person.value.say_goodbye());
}
fn min<T: PartialOrd>(value1: T, value2: T) -> T {
    if value1 < value2 { value1 } else { value2 }
}
#[test]
fn test_function_generic() {
    let value1 = 3.14;
    let value2 = 1.5;
    let result = min(value1, value2);
    println!("{}", result);
}
impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
}
#[test]
fn test_method_generic() {
    let point = Point { x: 3.24, y: 12.3 };
    println!("x: {} y: {}", point.get_x(), point.get_y());
}
trait GetValue<T>
where
    T: PartialOrd,
{
    fn get_value(&self) -> &T;
}
impl<T> GetValue<T> for Point<T>
where
    T: PartialOrd,
{
    fn get_value(&self) -> &T {
        self.get_x()
    }
}
#[test]
fn test_trait_generic() {
    let point = Point { x: 2, y: 3 };
    println!("{}", point.get_value())
}
#[test]
fn test_generic_default_value() {
    let point = Point { x: 2, y: 3 };
    println!("point: x{} y{}", point.x, point.y);
}
struct Apple {
    quantity: i32,
}
impl Add for Apple {
    type Output = Apple;

    fn add(self, rhs: Self) -> Self::Output {
        Apple {
            quantity: self.quantity + rhs.quantity,
        }
    }
}
#[test]
fn test_overloadable_operator() {
    let apple1: Apple = Apple { quantity: 5 };
    let apple2 = Apple { quantity: 10 };
    let apple3 = apple1 + apple2;
    println!("apple 1: {}", apple3.quantity);
}
fn double(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i * 2),
    }
}
#[test]
fn test_optional_value() {
    let result = double(Some(15));
    println!("{:?}", result);
    let none = double(None);
    println!("{:?}", none)
}
impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }

    fn lt(&self, other: &Self) -> bool {
        self.partial_cmp(other)
            .is_some_and(std::cmp::Ordering::is_lt)
    }

    fn le(&self, other: &Self) -> bool {
        self.partial_cmp(other)
            .is_some_and(std::cmp::Ordering::is_le)
    }

    fn gt(&self, other: &Self) -> bool {
        self.partial_cmp(other)
            .is_some_and(std::cmp::Ordering::is_gt)
    }

    fn ge(&self, other: &Self) -> bool {
        self.partial_cmp(other)
            .is_some_and(std::cmp::Ordering::is_ge)
    }
}
#[test]
fn test_compare() {
    let apple1 = Apple { quantity: 30 };
    let apple2 = Apple { quantity: 20 };
    println!("apple1 > apple2 {}", apple1 > apple2);
    println!("apple1 == apple2 {}", apple1 == apple2);
    println!("apple1 < apple2 {}", apple1 < apple2);
}
#[test]
fn test_string_manipulation() {
    let name = "Lorem Ipsum Dolor".to_string();
    println!("{}", name.to_lowercase());
    println!("{}", name.to_uppercase());
    println!("{}", name.len());
    println!("{}", name.replace("Lorem", "John"));
    println!("{}", name.contains("Dolor"));
    println!("{}", name.starts_with("Lorem"));
    println!("{}", name.ends_with("Dolor"));
    println!("{}", name.trim());
    println!("{:?}", name.get(0..5));
}
struct Category {
    id: String,
    name: String,
}
impl Debug for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}
#[test]
fn test_debug_category() {
    let category = Category {
        id: "123".to_string(),
        name: "Mens".to_string(),
    };
    println!("category: {:?}", category);
}
#[test]
fn test_closure() {
    let sum = |value1: &i32, value2: &i32| -> i32 { value1 + value2 };
    let a = 32;
    let b = 54;
    let result = sum(&a, &b);
    println!("{}", result);
    println!("{} + {} = {}", a, b, result);
}
fn print_with_filter(value: &str, filter: fn(&str) -> String) {
    let result = filter(value);
    println!("{}", result);
}
#[test]
fn test_closure_as_params() {
    let name = "Lorem ipsum dolor".to_string();
    print_with_filter(&name, |value: &str| -> String { value.to_uppercase() });
    println!("{}", name);
}
fn to_uppercase_custom(value: &str) -> String {
    value.to_uppercase()
}
#[test]
fn test_closure_param_function() {
    let name = "john doe";
    print_with_filter(name, to_uppercase_custom);
}
#[test]
fn test_closure_scope() {
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        println!("Increment!");
    };
    increment();
    increment();
    increment();
    println!("counter: {}", counter);
}
struct Counter {
    counter: i32,
}
impl Counter {
    fn increment(&mut self) {
        self.counter += 1;
        println!("Increment");
    }
}
#[test]
fn test_closure_scope_with_struct() {
    let mut counter = Counter { counter: 0 };
    counter.increment();
    counter.increment();
    counter.increment();
    counter.increment();
    println!("{}", counter.counter);
}
#[test]
fn test_vector() {
    let mut names = Vec::<String>::new();
    names.push("Lorem".to_string());
    names.push("Ipsum".to_string());
    names.push("Dolor".to_string());
    for name in &names {
        println!("{}", name)
    }
    names.remove(names.len() - 1);
    println!("{:?}", names);
}
#[test]
fn test_vec_deque() {
    let mut names = VecDeque::new();
    names.push_back("Lorem".to_string());
    names.push_front("Ipsum".to_string());
    names.push_back("Dolor".to_string());
    for name in &names {
        println!("{}", name);
    }
    names.swap_remove_back(0);
    println!("{:?}", names);

    names.remove(names.len() - 1);
    println!("{:?}", names);
    println!("{}", names[0]);
}
#[test]
fn test_linked_list() {
    let mut names = LinkedList::new();
    names.push_back("Satu");
    names.push_back("Dua");
    names.push_back("Tiga");
    names.push_front("empat");
    for name in &names {
        println!("{}", name)
    }
    names.pop_back();
    println!("{:?}", names);
}
#[test]
fn test_hash_map() {
    let mut map = HashMap::new();
    map.insert("name".to_string(), "Lorem".to_string());
    map.insert("age".to_string(), 21.to_string());
    let name = map.get("name");
    let age = map.get("age");
    let none = map.get("none");
    println!("name: {}", name.unwrap());
    println!("age: {}", age.unwrap());
    println!("none: {:?}", none);
    println!("{:?}", map);
}
#[test]
fn test_btree_map() {
    let mut btree = BTreeMap::new();
    btree.insert("name".to_string(), "Dolor".to_string());
    btree.insert("age".to_string(), 37.to_string());
    btree.insert("country".to_string(), "Indonesi".to_string());
    for tree in &btree {
        println!("{} : {}", tree.0, tree.1)
    }
    btree.remove_entry("age");
    for entry in btree {
        println!("{} : {}", entry.0, entry.1)
    }
}
#[test]
fn test_hash_set() {
    let mut set = HashSet::new();
    set.insert("Lorem".to_string());
    set.insert("Lorem".to_string());
    set.insert("Ipsum".to_string());
    set.insert("Ipsum".to_string());
    set.insert("Dolor".to_string());
    set.insert("dolor".to_string());
    for value in &set {
        println!("{}", value);
    }
    set.remove("dolor");
    println!("{:?}", set);
}
#[test]
fn test_btree_set() {
    let mut btree = BTreeSet::new();
    btree.insert("Lorem".to_string());
    btree.insert("Lorem".to_string());
    btree.insert("Ipsum".to_string());
    btree.insert("Ipsum".to_string());
    btree.insert("Dolor".to_string());
    btree.insert("Dolor".to_string());
    for tree in &btree {
        println!("{}", tree)
    }
    btree.remove("Dolor");
    println!("{:?}", btree);
    btree.clear();
    println!("{:?}", btree);
}
#[test]
fn test_iterator() {
    let array = [1, 2, 3, 4, 5, 6];
    let mut iterator = array.iter();

    while let Some(value) = iterator.next() {
        println!("while: {}", value);
    }
    println!("{:?}", iterator);
    iterator = array.iter();
    println!("{:?}", iterator);
    for value in iterator {
        println!("for: {}", value);
    }
    let double: Vec<i32> = array.iter().map(|x| x * 2).collect();
    println!("{:?}", double);
}
#[test]
fn test_iterator_method() {
    let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", vector);
    let sum: i32 = vector.iter().sum();
    println!("{}", sum);
    let count = vector.iter().count();
    println!("{}", count);
    let double: Vec<i32> = vector.iter().map(|x| x * 2).collect();
    println!("{:?}", double);
    let ood: Vec<&i32> = vector.iter().filter(|x| *x % 2 != 0).collect();
    println!("{:?}", ood);
}
fn connect_database(host: Option<String>) {
    match host {
        Some(value) => println!("Connect to database: {}", value),
        None => panic!("No database provided"),
    }
}
#[test]
fn test_connect_database() {
    connect_database(Some("localhost".to_string()));
}
fn connect_cache(host: Option<String>) -> Result<String, &'static str> {
    match host {
        Some(value) => Ok(value),
        None => Err("Cannot connect to cache"),
    }
}
#[test]
fn test_connect_cache() {
    let result = connect_cache(None);
    match result {
        Ok(value) => println!("Successfully connect to cache {}", value),
        Err(err) => println!("{}", err),
    }
}
fn connect_email(host: Option<String>) -> Result<String, &'static str> {
    match host {
        Some(value) => Ok(value),
        None => Err("Cannot connect to email service"),
    }
}
fn connect_application(host: Option<String>) -> Result<String, &'static str> {
    let cache_result = connect_cache(host.clone());
    match cache_result {
        Ok(_) => {}
        Err(err) => return Err(err),
    }
    let email_result = connect_email(host.clone());
    match email_result {
        Ok(_) => {}
        Err(err) => return Err(err),
    }
    Ok("Connected application".to_string())
}
#[test]
fn test_connect_application() {
    let result = connect_application(None);
    match result {
        Ok(value) => println!("{}", value),
        Err(err) => println!("Error connect application: {}", err),
    }
}
fn connect_application_operator(host: Option<String>) -> Result<&'static str, &'static str> {
    connect_cache(host.clone())?;
    connect_email(None)?;
    Ok("Connected to application")
}
#[test]
fn test_connect_application_operator() {
    let result = connect_application_operator(Some("localhost".to_string()));
    match result {
        Ok(value) => println!("{}", value),
        Err(err) => println!("Error with message: {}", err),
    }
}
#[test]
fn test_dangling_reference() {
    let _r: &i32;
    {
        let _q = 5;
        /*
        error karena _q sudah dihapus ketika keluar dari scope
        _r=&_q;
        */
    }
    //println!("{}", _r);
}
fn longest<'a>(value1: &'a str, value2: &'a str) -> &'a str {
    if value1.len() > value2.len() {
        return value1;
    } else {
        return value2;
    }
}
#[test]
fn test_lifetime_anotation() {
    let value1 = "lorem";
    let value2 = "ipsum";
    let result = longest(value1, value2);
    println!("{}", result);
}
#[test]
fn test_dangling_reference_lifetime_anotation() {
    let fist_name = "lorems".to_string();
    println!("{}", fist_name);
    //let result;
    {
        let last_name = "ipsum".to_string();
        println!("{}", last_name);
        /*
        `last_name` does not live long enough
        borrowed value does not live long enough
        result = longest(fist_name.as_str(), last_name.as_str());
         */
    }
    //println!("{}", result);
}
struct Student<'a> {
    name: &'a str,
}
fn longets_student_name<'a>(value1: &'a Student, value2: &'a Student) -> &'a Student<'a> {
    if value1.name.len() > value2.name.len() {
        value1
    } else {
        value2
    }
}
#[test]
fn test_longest_student() {
    let lorem = Student { name: "Lorem" };
    let ipsumm = Student { name: "Ipsumm" };
    let result = longets_student_name(&lorem, &ipsumm);
    println!("{}", result.name);
}
impl<'a> Student<'a> {
    fn longets_name(&self, student: &Student<'a>) -> &'a str {
        if self.name.len() > student.name.len() {
            self.name
        } else {
            student.name
        }
    }
}
#[test]
fn test_lifetime_anotation_method() {
    let lorem = Student { name: "Lorem" };
    let ipsumm = Student { name: "Ipsumm" };
    let result = lorem.longets_name(&ipsumm);
    println!("{}", result);
}
struct Teacher<'a, ID>
where
    ID: Ord,
{
    id: ID,
    name: &'a str,
}
#[test]
fn test_lifeteime_anotation_generic_struct() {
    let lorem = Teacher {
        id: 3,
        name: "Lorem",
    };
    let ipsum = Teacher {
        id: 1,
        name: "Ipsum",
    };
    println!("Lorem {} : {}", lorem.id, lorem.name);
    println!("ipsum {} : {}", ipsum.id, ipsum.name);
}
#[derive(Debug, PartialEq, PartialOrd)]
struct Company {
    name: String,
    location: String,
    website: String,
}
#[test]
fn test_derive_company() {
    let company = Company {
        name: "Lorem Ipsum".to_string(),
        location: "Indonesia".to_string(),
        website: "www.ikaen.go.id".to_string(),
    };
    println!("{:?}", company)
}
#[test]
fn test_box() {
    let lorem = Box::new(10);
    println!("{}", lorem);
}
enum Variant {
    Of(String, Box<Variant>),
    End,
}
impl Debug for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Of(arg0, arg1) => f.debug_tuple("Of").field(arg0).field(arg1).finish(),
            Self::End => write!(f, "End"),
        }
    }
}
#[test]
fn test_box_variant() {
    let variant = Variant::Of(
        "Apple".to_string(),
        Box::new(Variant::Of("Iphone".to_string(), Box::new(Variant::End))),
    );
    println!("{:?}", variant);
}
#[test]
fn test_dereference() {
    let value1 = Box::new(10);
    let value2 = Box::new(10);
    let result = *value1 * *value2;
    println!("{}", result);
}
#[derive(Debug)]
struct MyValue<T> {
    value: T,
}
impl<T> Deref for MyValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
#[test]
fn test_deref_struct() {
    let value = MyValue { value: 35 };
    let borrow = &value;
    println!("borrow value {:?}", borrow);
    let real_value = *value;
    println!("{:?}", value);
    println!("real value: {}", real_value);
}
#[test]
fn test_deref_coercion() {
    let value = MyValue {
        value: "Lorem Ipsum".to_string(),
    };
    say_hello(&value);
}
struct Book {
    title: String,
}
impl Drop for Book {
    fn drop(&mut self) {
        println!("drop: {:?}", self)
    }
}
impl Debug for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Book").field("title", &self.title).finish()
    }
}
#[test]
fn test_drop_trait_book() {
    let fiksi = Book {
        title: "Kitab suci adalah fiksi".to_string(),
    };
    println!("fiksi: {:?}", fiksi);
}
#[test]
fn test_multiple_ownership_box() {
    let apple = Variant::Of("Apple".to_string(), Box::new(Variant::End));
    println!("{:?}", apple);
    let laptop = Variant::Of("Laptop".to_string(), Box::new(apple));
    println!("{:?}", laptop);
    /*
    error use of moved value: `apple`
    let phone=Variant::Of("Handphone".to_string(), Box::new(apple));
    */
}
enum Brand {
    Of(String, Rc<Brand>),
    End,
}
impl Debug for Brand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Of(arg0, arg1) => f.debug_tuple("Of").field(arg0).field(arg1).finish(),
            Self::End => write!(f, "End"),
        }
    }
}
#[test]
fn test_multiple_ownership_rc() {
    let apple = Rc::new(Brand::Of("Apple".to_string(), Rc::new(Brand::End)));
    println!("Apple reference count: {}", Rc::strong_count(&apple));
    let phone = Brand::Of("Handphone".to_string(), Rc::clone(&apple));
    println!("Apple reference count: {}", Rc::strong_count(&apple));
    {
        let laptop = Brand::Of("Laptop".to_string(), Rc::clone(&apple));
        println!("Apple reference count: {}", Rc::strong_count(&apple));
        println!("{:?}", laptop);
    }
    println!("Apple reference count: {}", Rc::strong_count(&apple));
    println!("{:?}", phone);
}
struct Seller {
    name: RefCell<String>,
    active: RefCell<bool>,
}
impl Debug for Seller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Seller")
            .field("name", &self.name)
            .field("active", &self.active)
            .finish()
    }
}
#[test]
fn test_ref_cell() {
    let lorem = Seller {
        name: RefCell::new("Lorem Ipsum".to_string()),
        active: RefCell::new(true),
    };
    {
        let mut result = lorem.name.borrow_mut();
        *result = "John Doe".to_string().to_owned();
        println!("{}", result);
    }

    println!("{:?}", lorem);
}
static VERSION: &str = "1.0.0";
#[test]
fn test_static() {
    println!("version: {}", VERSION);
}
static mut COUNTER: i32 = 0;

#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn increment() {
    COUNTER += 1;
}
#[test]
#[allow(static_mut_refs)]
fn test_static_mut() {
    unsafe {
        println!("counter: {}", COUNTER);
        increment();
        println!("counter: {}", COUNTER);
    };
}
macro_rules! hi {
    () => {
        println!("Hi")
    };
    ($name: expr) => {
        println!("Hi, {}", $name)
    };
}
#[test]
fn test_macro() {
    hi!();
    hi!("lorem");
    hi! {
        "ipsum"
    }
}
macro_rules! iterate {
    ($array: expr) => {
        for i in $array{
            println!("{}", i);
        }
    };
    ($($item: expr),*) => {
        $(
            println!("{}",$item);
        )*
    };
}
#[test]
fn test_macro_iterate() {
    iterate!([2, 23, 43, 234, 2]);
    iterate!(1, 2, 3, 4, 5, 4);
}
