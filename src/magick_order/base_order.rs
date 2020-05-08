use std::collections::HashMap;

const BASE_ORDER: &'static str = include_str!("base_order.txt");

pub struct BaseOrder {
    pub map: HashMap<&'static str, usize>,
    pub order: Vec<&'static str>,
}

pub fn base_order() -> BaseOrder {
    let mut map = HashMap::new();
    let mut order = Vec::new();
    for (i, line) in BASE_ORDER.lines().enumerate() {
        map.insert(line, i);
        order.push(line);
    }
    BaseOrder { map, order }
}
