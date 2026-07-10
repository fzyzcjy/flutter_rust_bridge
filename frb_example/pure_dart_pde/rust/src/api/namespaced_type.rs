// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

pub mod first_store {
    pub struct ModelTwinNormal {
        pub value: i32,
    }
}

pub mod second_store {
    pub struct ModelTwinNormal {
        pub value: String,
    }
}

pub fn duplicate_named_models_twin_normal() -> Vec<second_store::ModelTwinNormal> {
    vec![second_store::ModelTwinNormal {
        value: "second".to_owned(),
    }]
}
