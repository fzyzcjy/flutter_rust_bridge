#[derive(Debug, Clone)]
pub struct MyTreeNode {
    pub value_i32: i32,
    pub value_vec_u8: Vec<u8>,
    pub value_boolean: bool,
    pub children: Vec<MyTreeNode>,
}

pub fn handle_complex_struct(s: MyTreeNode) -> MyTreeNode {
    // info!("handle_complex_struct({:?})", &s);
    let s_cloned = s.clone();
    s
}

#[derive(Debug, Clone, Copy)]
pub enum Weekdays {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, Clone)]
pub struct MyNestedStruct {
    pub tree_node: MyTreeNode,
    pub weekday: Weekdays,
}

pub fn handle_nested_struct(s: MyNestedStruct) -> MyNestedStruct {
    println!("handle_nested_struct({s:?})");
    let s_cloned = s.clone();
    s
}
