// 泛化和接口
use log::info;
pub fn max<T>(arr: Option<&[T]>) -> Option<T>
where
    T: PartialOrd + Copy,
{
    if arr.is_none() {
        return None;
    }
    let array = arr.unwrap_or(&[]);
    if array.is_empty() {
        return None;
    }
    let mut max_index = 0;
    for i in 1..array.len() {
        if array[i] > array[max_index] {
            max_index = i;
        }
    }
    Some(array[max_index])
}

pub fn exec_test_option_t() {
    let array = [1.00, 3.69, 2.25, 5.010, 5.000];
    let result = max(Some(&array));
    let result_value;
    match result {
        Some(max_value) => {
            result_value = max_value;
            info!("The maximum value is: {}", result_value)
        }
        None => info!("The array is empty or None."),
    }

    // Case with None
    let none_array: Option<&[i32]> = None;
    let result = max(none_array);
    match result {
        Some(max_value) => info!("The maximum value is: {}", max_value),
        None => info!("The array is empty or None."),
    }

    // Case with empty array
    let empty_array: &[i32] = &[];
    let result = max(Some(empty_array));
    match result {
        Some(max_value) => info!("The maximum value is: {}", max_value),
        None => info!("The array is empty or None."),
    }
}

// 接口或者说是特性
pub trait Descriptive {
    fn describe(&self) -> String;
}

pub trait Value<T> {
    fn get_age(&self) -> T;
}
#[derive(Debug)]
pub struct Person {
    name: String,
    age: u8,
}

impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}

impl Value<u8> for Person {
    fn get_age(&self) -> u8 {
        return self.age;
    }
}

impl Person {
    pub fn create_impl(name: String, age: u8) -> impl Descriptive + Value<u8> {
        Person { name, age }
    }
    pub fn create(name: String, age: u8) -> Person {
      Person { name, age }
  }
}

// pub fn person() -> impl Descriptive {
//    Person {
//        name: String::from("Cali"),
//        age: 24
//    }
// }

// pub fn exec_interface() {
//    let value: Person = Person::create(String::from("tine"), 8);
//    info!("value.get_age() = {}", value.get_age());
//    info!("value.describe() = {}", value.describe());
// }
