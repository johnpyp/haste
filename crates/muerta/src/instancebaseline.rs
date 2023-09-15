use crate::stringtables::StringTable;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // std
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
}

pub type Result<T> = std::result::Result<T, Error>;

pub const INSTANCE_BASELINE_TABLE_NAME: &str = "instancebaseline";

#[derive(Default)]
pub struct InstanceBaseline {
    strs: Vec<Option<Vec<u8>>>,
}

impl InstanceBaseline {
    pub fn update(&mut self, string_table: &StringTable, classes: usize) -> Result<()> {
        if self.strs.len() < classes {
            self.strs.resize(classes, None);
        }

        for (_entity_index, item) in string_table.items.iter() {
            let string = item
                .string
                .as_ref()
                .expect("instance baseline class id string");

            debug_assert!(
                string.len() <= 4,
                "unexpected len of instance baseline class id string: {}",
                string.len()
            );
            let string = unsafe { std::str::from_utf8_unchecked(string) };
            let class_id = string.parse::<i32>()?;
            self.strs[class_id as usize] = item.user_data.clone();
        }
        Ok(())
    }

    pub fn get_data(&self, class_id: i32) -> Option<&[u8]> {
        unsafe { self.strs.get_unchecked(class_id as usize) }.as_deref()
    }
}