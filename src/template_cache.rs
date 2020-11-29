use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TemplateCacheKey {
    exporter_ip: String,
    source_id: u32,
    template_id: u16,
    version: u16,
}

impl TemplateCacheKey {
    pub fn new(
        exporter_ip: String,
        source_id: u32,
        template_id: u16,
        version: u16,
    ) -> TemplateCacheKey {
        TemplateCacheKey {
            exporter_ip: exporter_ip,
            source_id,
            template_id,
            version,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Field {
    pub type_: u16,
    pub length: u16,
}

impl Field {
    pub fn new(type_: u16, length: u16) -> Field {
        Field { type_, length }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TemplateCacheValue {
    pub fields: Vec<Field>,
    pub scope_fields: Vec<Field>,
    pub is_option: bool,
}

impl TemplateCacheValue {
    pub fn new(
        fields: Vec<Field>,
        scope_fields: Vec<Field>,
        is_option: bool,
    ) -> TemplateCacheValue {
        TemplateCacheValue {
            fields: fields,
            scope_fields: scope_fields,
            is_option: is_option,
        }
    }
}

#[derive(Debug)]
pub struct TemplateCache {
    map: HashMap<TemplateCacheKey, TemplateCacheValue>,
}

impl TemplateCache {
    pub fn new() -> TemplateCache {
        TemplateCache {
            map: HashMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        k: TemplateCacheKey,
        v: TemplateCacheValue,
    ) -> Option<TemplateCacheValue> {
        self.map.insert(k, v)
    }

    pub fn get(&self, k: &TemplateCacheKey) -> Option<&TemplateCacheValue> {
        self.map.get(k)
    }

    pub fn contains_key(&self, k: &TemplateCacheKey) -> bool {
        self.map.contains_key(k)
    }
}
