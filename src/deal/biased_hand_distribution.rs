use std::ops::Index;
use log::debug;
use rand::distributions::Standard;
use rand::prelude::{Distribution};
use rand::Rng;
use rand::seq::SliceRandom;
use karty::cards::{Card, DECK_SIZE};
use karty::figures::Figure;
use karty::hand::FuzzyCardSet;
use karty::suits::{Suit, SuitMap, SUITS};
use karty::suits::Suit::Spades;
use karty::symbol::CardSymbol;
use crate::meta::HAND_SIZE;
use crate::player::side::{Side, SideMap, SIDES};
use crate::player::side::Side::{East, North, South, West};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct BiasedDistribution{
    side_probabilities: SideMap<FuzzyCardSet>
}

impl Index<Side> for BiasedDistribution {
    type Output = FuzzyCardSet;

    fn index(&self, index: Side) -> &Self::Output {
        &self.side_probabilities[&index]
    }
}

impl Distribution<BiasedDistribution> for Standard{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BiasedDistribution {
        //start with north
        //we need 52 numbers to sum to 13
        //or we can have 52 areas in 0..13
        //so sample 51 numbers in 0.13 and sort them
        /*
        let mut intervals = [0; Card::SYMBOL_SPACE];
        for i in 0..intervals.len(){
            let p = 
        }*/

        /*let mut card_probabilities =  SideMap::new_symmetric(FuzzyCardSet::empty());
        for side in SIDES{
            &mut card_probabilities[&side].set_expected(13);
        }*/

        let mut iteration = 0;
        let mut sides_shuffled = SIDES;
        loop{
            iteration += 1;
            //debug!("Sampling try number {iteration:}");
            let mut probabilities = SideMap::new_symmetric(SuitMap::new_from_f(|_|[0.0f32; HAND_SIZE]));
            let mut sums_per_side = SideMap::new_symmetric(0.0f32);
            for i in 0..DECK_SIZE-1{
                let s = Suit::from_position(i/13).unwrap();
                let f = i%13;
                let mut inner_iteration = 0;
                loop{
                    sides_shuffled.shuffle(rng);
                    inner_iteration += 1;

                    let proba_1:f32 = (rng.gen_range(0.0..=1.0) as f32);
                    let proba_2: f32 = (rng.gen_range(0.0..=1.0) as f32);
                    let proba_3: f32 = (rng.gen_range(0.0..=1.0) as f32);

                    let proba_4: f32 = 1.0 - (proba_1 + proba_2 + proba_3);

                    if proba_4 >=0.0{
                        probabilities[&sides_shuffled[0]][s][f] = proba_1;
                        probabilities[&sides_shuffled[1]][s][f] = proba_2;
                        probabilities[&sides_shuffled[2]][s][f] = proba_3;
                        probabilities[&sides_shuffled[3]][s][f] = proba_4;

                        sums_per_side[&North] += probabilities[&North][s][f];
                        sums_per_side[&East] += probabilities[&East][s][f];
                        sums_per_side[&South] += probabilities[&South][s][f];
                        sums_per_side[&West] += probabilities[&West][s][f];

                        break;
                    }
                    //debug!("For card {i:} resampling")

                }




            }
            if sums_per_side[&North] > 13.0{
                debug!("North with probability_sum over 13: {}", sums_per_side[&North]);
                continue;
            }
            if sums_per_side[&East] > 13.0{
                debug!("East with probability_sum over 13: {}", sums_per_side[&East]);
                continue;
            }
            if sums_per_side[&South] > 13.0{
                debug!("South with probability_sum over 13: {}", sums_per_side[&South]);
                continue;
            }
            if sums_per_side[&West] > 13.0{
                debug!("West with probability_sum over 13: {}", sums_per_side[&West]);
                continue;
            }
            //debug!("Probabilities sum: {:?}", sums_per_side);
            for side in SIDES{
                probabilities[&side][Spades][Figure::SYMBOL_SPACE-1] = 13.0 - sums_per_side[&side];
            }

            return BiasedDistribution{ side_probabilities: SideMap::new(
                FuzzyCardSet::new_from_f32_derive_sum(probabilities[&North]).unwrap(),
                FuzzyCardSet::new_from_f32_derive_sum(probabilities[&East]).unwrap(),
                FuzzyCardSet::new_from_f32_derive_sum(probabilities[&South]).unwrap(),
                FuzzyCardSet::new_from_f32_derive_sum(probabilities[&West]).unwrap()) }

        }







        //for suit in SUITS{}
        /*
        for side in SIDES{
            for suit in SUITS{

            }
        }*/
    }
}