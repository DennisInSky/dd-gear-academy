use gstd::{
    cmp::{max, min},
    exec,
    prelude::*,
    ActorId,
};

const FED_LEVEL_MIN: u64 = 1;
const FED_LEVEL_MAX: u64 = 10000;
const ENTERTAINED_LEVEL_MIN: u64 = 1;
const ENTERTAINED_LEVEL_MAX: u64 = 10000;
const RESTED_LEVEL_MIN: u64 = 1;
const RESTED_LEVEL_MAX: u64 = 10000;
const HUNGER_PER_BLOCK: u64 = 1;
const BOREDOM_PER_BLOCK: u64 = 2;
const ENERGY_PER_BLOCK: u64 = 2;
const FILL_PER_FEED: u64 = 1000;
const FILL_PER_ENTERTAINMENT: u64 = 1000;
const FILL_PER_SLEEP: u64 = 1000;

pub(crate) struct Tamagotchi {
    owner_id: ActorId,
    transferor_id: Option<ActorId>,
    name: String,
    date_of_birth: u64,
    fed_level: u64,
    fed_block: u32,
    entertained_level: u64,
    entertained_block: u32,
    rested_level: u64,
    rested_block: u32,
}

impl Tamagotchi {
    pub fn new(owner_id: ActorId, name: String) -> Self {
        let current_block = exec::block_height();
        Tamagotchi {
            owner_id,
            transferor_id: None,
            name,
            date_of_birth: exec::block_timestamp(),
            fed_level: FED_LEVEL_MAX,
            fed_block: current_block,
            entertained_level: ENTERTAINED_LEVEL_MAX,
            entertained_block: current_block,
            rested_level: ENTERTAINED_LEVEL_MAX,
            rested_block: current_block,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn age(&self) -> u64 {
        exec::block_timestamp() - self.date_of_birth
    }

    pub fn feed(&mut self) {
        let current_block = exec::block_height();
        self.fed_level = Tamagotchi::increased_level(
            self.fed_level_at(current_block),
            FILL_PER_FEED,
            FED_LEVEL_MAX,
        );
        self.fed_block = current_block
    }

    pub fn play(&mut self) {
        let current_block = exec::block_height();
        self.entertained_level = Tamagotchi::increased_level(
            self.entertained_level_at(current_block),
            FILL_PER_ENTERTAINMENT,
            ENTERTAINED_LEVEL_MAX,
        );
        self.entertained_block = exec::block_height()
    }

    pub fn sleep(&mut self) {
        let current_block = exec::block_height();
        self.rested_level = Tamagotchi::increased_level(
            self.rested_level_at(current_block),
            FILL_PER_SLEEP,
            RESTED_LEVEL_MAX,
        );
        self.rested_block = exec::block_height()
    }

    pub fn transfer(&mut self, actor_id: ActorId, new_owner_id: ActorId) {
        assert_eq!(actor_id, self.owner_id);
        self.owner_id = new_owner_id;
    }

    pub fn grant_transfer_permission(&mut self, actor_id: ActorId, transferor_id: ActorId) {
        assert_eq!(actor_id, self.owner_id);
        self.transferor_id = Some(transferor_id);
    }

    pub fn revoke_transfer_permission(&mut self, actor_id: ActorId) {
        assert_eq!(actor_id, self.owner_id);
        self.transferor_id = None;
    }

    fn fed_level_at(&self, block: u32) -> u64 {
        Tamagotchi::dropped_level_at(
            self.fed_level,
            self.fed_block,
            FED_LEVEL_MIN,
            HUNGER_PER_BLOCK,
            block,
        )
    }

    fn entertained_level_at(&self, block: u32) -> u64 {
        Tamagotchi::dropped_level_at(
            self.entertained_level,
            self.entertained_block,
            ENTERTAINED_LEVEL_MIN,
            BOREDOM_PER_BLOCK,
            block,
        )
    }

    fn rested_level_at(&self, block: u32) -> u64 {
        Tamagotchi::dropped_level_at(
            self.rested_level,
            self.rested_block,
            RESTED_LEVEL_MIN,
            ENERGY_PER_BLOCK,
            block,
        )
    }

    fn dropped_level_at(
        level_set_to: u64,
        level_set_at_block: u32,
        level_min: u64,
        level_drop_per_block: u64,
        block: u32,
    ) -> u64 {
        max(
            level_set_to
                .saturating_sub(((block - level_set_at_block) as u64) * level_drop_per_block),
            level_min,
        )
    }

    fn increased_level(level: u64, level_increase: u64, level_max: u64) -> u64 {
        min(level.saturating_add(level_increase), level_max)
    }
}

impl From<&Tamagotchi> for tamagotchi_io::Tamagotchi {
    fn from(entity: &Tamagotchi) -> Self {
        let current_block = exec::block_height();
        tamagotchi_io::Tamagotchi {
            name: entity.name.clone(),
            date_of_birth: entity.date_of_birth,
            owner: entity.owner_id,
            fed: entity.fed_level_at(current_block),
            fed_block: entity.fed_block as u64,
            entertained: entity.entertained_level_at(current_block),
            entertained_block: entity.entertained_block as u64,
            rested: entity.rested_level_at(current_block),
            rested_block: entity.rested_block as u64,
            allowed_account: entity.transferor_id,
        }
    }
}
