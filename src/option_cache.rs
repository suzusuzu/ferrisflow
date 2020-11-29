use std::collections::HashMap;

pub type FlowDatas = HashMap<u16, Vec<u8>>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct OptionCacheKey {
    exporter_ip: String,
}

impl OptionCacheKey {
    pub fn new(exporter_ip: String) -> OptionCacheKey {
        OptionCacheKey {
            exporter_ip: exporter_ip,
        }
    }
}

#[derive(Debug)]
pub struct OptionCache {
    map: HashMap<OptionCacheKey, FlowDatas>,
}

impl OptionCache {
    pub fn new() -> OptionCache {
        OptionCache {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: OptionCacheKey, v: FlowDatas) -> Option<FlowDatas> {
        self.map.insert(k, v)
    }

    pub fn get(&self, k: &OptionCacheKey) -> Option<&FlowDatas> {
        self.map.get(k)
    }

    pub fn contains_key(&self, k: &OptionCacheKey) -> bool {
        self.map.contains_key(k)
    }
}
