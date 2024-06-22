// NOTE: Multi-file is supported (e.g. put in submodules of crate::app, or configure frb input mod)

use flutter_rust_bridge::frb;

#[frb(ui_state)]
pub struct RustState {
    items: Vec<Item>,
    pub input_text: String,
    pub filter: Filter,
    next_id: i32,
}

#[derive(Clone)]
pub struct Item {
    pub id: i32,
    pub content: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

#[frb(ui_mutation)]
impl RustState {
    pub fn add(&mut self) {
        let id = self.next_id;
        self.next_id += 1;
        self.items.push(Item {
            id,
            content: self.input_text.clone(),
            completed: false,
        });
        self.input_text.clear();
    }

    pub fn remove(&mut self, id: i32) {
        self.items.retain(|x| x.id != id);
    }

    pub fn toggle(&mut self, id: i32) {
        let entry = self.items.iter_mut().find(|x| x.id == id).unwrap();
        entry.completed = !entry.completed;
    }
}

impl RustState {
    pub fn new() -> Self {
        Self {
            items: vec![],
            input_text: "".to_string(),
            filter: Filter::All,
            next_id: 0,
            base_state: Default::default(),
        }
    }

    pub fn filtered_items(&self) -> Vec<Item> {
        self.items
            .iter()
            .filter(|x| self.filter.check(x))
            .cloned()
            .collect()
    }
}

impl Filter {
    fn check(&self, item: &Item) -> bool {
        match self {
            Self::All => true,
            Self::Active => !item.completed,
            Self::Completed => item.completed,
        }
    }
}
