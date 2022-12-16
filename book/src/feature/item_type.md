# Item Type

There is an example supported:
```
pub type Id = u64;

pub fn handle_type_alias_id(input: Id) -> Id {
    input
}
pub struct TestModel {
    pub id: Id,
    pub name: String,
}

pub fn handle_type_alias_model(input: Id) -> TestModel {
    TestModel {
        id: input,
        name: "TestModel".to_owned(),
    }
}
```

You can use the Id, or you can use the Id in the Model.

What’s more, you can also use an 'ItemType' like follow
`type Sth = MyFancyStruct` or `type Sth = MyFancyEnum`

But, `ItemType` in Generic is not supported yet.(like `SyncReturn<Id>`)
The Nest ItemType may also be not supported