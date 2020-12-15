use crate::hand::{HandRestrictions, TileRequirement};
use crate::types::{Tile, WWYD};
use std::collections::HashMap;
use std::panic::resume_unwind;

pub struct HandGenerator {
    restrictions: HandRestrictions,
}

impl HandGenerator {
    pub fn new(restrictions: HandRestrictions) -> Self {
        // TODO: We should validate the restrictions
        Self { restrictions }
    }

    pub fn generate(shanten: u8) -> WWYD {
        /// Algorithm steps
        /// 1. Get usable suits
        /// 2. Get usable suit numbers
        /// 3. Get usable wind directions
        /// 4. Get usable dragon colors
        /// 5. Build tile pool with all usable tiles
        /// 6. Check if the hand shape is regular or not
        /// 6.A If shape is regular check which regular shapes are allowed
        /// 6.B If not regular check which are the hand's shapes
        /// 7. Start building the hand shape group by shape group taking tiles from the pool
        /// 8. Once the hand is complete add one random tile from any possible tile, taking
        /// into account the remaining tiles
        /// 9. For each shanten level selected switch one tile from the hand
        /// with one random available tile
        /// 10. After that compute in a separate method the optimal discards
        /// by giving each one a score.
        unimplemented!()
    }
}

struct TilePool {
    pool: HashMap<Tile, u8>,
}

impl TilePool {
    pub fn new(restrictions: &HandRestrictions) -> Self {
        let expected_capacity = compute_total_tiles_available_with(restrictions);
        let mut tile_pool: HashMap<Tile, u8> = HashMap::with_capacity(expected_capacity);

        TilePool { pool: tile_pool }
    }

    fn compute_total_tiles_available_with(restrictions: &HandRestrictions) -> u8 {
        let honor_tiles_available_count = restrictions
            .usable_honor_variants_count()
            .unwrap_or_default()
            * 4;
        let suit_tiles_available_count = restrictions.usable_suit_variants_count()
            * restrictions
                .suit_numbers_allowed()
                .unwrap_or(&vec![])
                .iter()
                .sum()
            * 4;

        honor_tiles_available_count + suit_tiles_available_count
    }
}
