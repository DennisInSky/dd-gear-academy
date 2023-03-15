use gstd::{exec, prelude::*};

pub(crate) struct Tamagotchi {
    name: String,
    date_of_birth: u64,
}

impl Tamagotchi {
    pub fn new(name: String) -> Self {
        Tamagotchi {
            name,
            date_of_birth: exec::block_timestamp(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn age(&self) -> u64 {
        exec::block_timestamp() - self.date_of_birth
    }
}

impl From<&Tamagotchi> for tamagotchi_io::Tamagotchi {
    fn from(entity: &Tamagotchi) -> Self {
        tamagotchi_io::Tamagotchi {
            name: entity.name.clone(),
            date_of_birth: entity.date_of_birth,
        }
    }
}
