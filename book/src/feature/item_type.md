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
But ItemType in Generic is not supported yet.(like `SyncReturn<Id>`)