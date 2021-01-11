use crate::hand::HandRestrictions;
use crate::types::{DragonColor, NumberedSuit, Tile, WindDirection, WWYD};
use std::collections::{HashMap, HashSet};

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
        let expected_capacity = TilePool::compute_total_tiles_available_with(restrictions);
        let mut tile_pool: HashMap<Tile, u8> = HashMap::with_capacity(expected_capacity as usize);

        Self::fill_pool_with_suit_tiles(
            &mut tile_pool,
            &restrictions.suit_variants_allowed(),
            &restrictions.suit_numbers_allowed(),
        );

        Self::fill_pool_with_honor_tiles(
            &mut tile_pool,
            &restrictions.wind_directions_allowed(),
            &restrictions.dragon_colors_allowed(),
        );

        TilePool { pool: tile_pool }
    }

    fn compute_total_tiles_available_with(restrictions: &HandRestrictions) -> u8 {
        let honor_tiles_available_count: u8 = restrictions.usable_honor_variants_count() * 4;
        let suit_tiles_available_count: u8 = restrictions.usable_suit_variants_count()
            * restrictions.suit_numbers_allowed().len() as u8
            * 4;

        honor_tiles_available_count + suit_tiles_available_count
    }

    fn fill_pool_with_suit_tiles(
        pool: &mut HashMap<Tile, u8>,
        suits_allowed: &HashSet<NumberedSuit>,
        numbers_allowed: &HashSet<u8>,
    ) {
        for suit in suits_allowed {
            for number in numbers_allowed {
                let tile = Tile::new_numbered(*suit, *number, false);
                pool.entry(tile).or_insert(4);
            }
        }
    }

    fn fill_pool_with_honor_tiles(
        pool: &mut HashMap<Tile, u8>,
        wind_directions_allowed: &HashSet<WindDirection>,
        dragon_colors_allowed: &HashSet<DragonColor>,
    ) {
        for direction in wind_directions_allowed {
            let tile = Tile::new_wind(*direction);
            pool.entry(tile).or_insert(4);
        }

        for color in dragon_colors_allowed {
            let tile = Tile::new_dragon(*color);
            pool.entry(tile).or_insert(4);
        }
    }
}
