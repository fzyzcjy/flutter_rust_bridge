#![allow(unused_variables)]

use anyhow::{anyhow, Result};

pub fn simple_adder(a: i32, b: i32) -> Result<i32> {
    Ok(a + b)
}

pub fn primitive_types(my_i32: i32, my_i64: i64, my_f64: f64, my_bool: bool) -> Result<i32> {
    println!(
        "primitive_types({}, {}, {}, {})",
        my_i32, my_i64, my_f64, my_bool
    );
    Ok(42)
}

pub fn handle_string(s: String) -> Result<String> {
    println!("handle_string({})", &s);
    let s2 = s.clone();
    Ok(s + &s2)
}

pub fn handle_vec_u8(v: Vec<u8>) -> Result<Vec<u8>> {
    println!("handle_vec_u8(first few elements: {:?})", &v[..5]);
    Ok(v.repeat(2))
}

#[derive(Debug, Clone)]
pub struct MySize {
    pub width: i32,
    pub height: i32,
}

pub fn handle_struct(arg: MySize, boxed: Box<MySize>) -> Result<MySize> {
    println!("handle_struct({:?}, {:?})", &arg, &boxed);
    Ok(MySize {
        width: arg.width + boxed.width,
        height: arg.height + boxed.height,
    })
}

#[derive(Debug)]
pub struct NewTypeInt(pub i64);

pub fn handle_newtype(arg: NewTypeInt) -> Result<NewTypeInt> {
    println!("handle_newtype({:?})", &arg);
    Ok(NewTypeInt(arg.0 * 2))
}

pub fn handle_list_of_struct(mut l: Vec<MySize>) -> Result<Vec<MySize>> {
    println!("handle_list_of_struct({:?})", &l);
    let mut ans = l.clone();
    ans.append(&mut l);
    Ok(ans)
}

#[derive(Debug, Clone)]
pub struct MyTreeNode {
    pub value_i32: i32,
    pub value_vec_u8: Vec<u8>,
    pub children: Vec<MyTreeNode>,
}

pub fn handle_complex_struct(s: MyTreeNode) -> Result<MyTreeNode> {
    println!("handle_complex_struct({:?})", &s);
    let s_cloned = s.clone();
    Ok(s)
}

pub fn return_err() -> Result<i32> {
    Err(anyhow!(
        "return_err() is called, thus deliberately return Err"
    ))
}

pub fn return_panic() -> Result<i32> {
    panic!("return_panic() is called, thus deliberately panic")
}
