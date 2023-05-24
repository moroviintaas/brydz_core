use std::ops::Index;
use log::debug;
use rand::distributions::Standard;
use rand::prelude::{Distribution};
use rand::Rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use smallvec::SmallVec;
use karty::cards::{Card, DECK_SIZE, STANDARD_DECK};
use karty::error::CardSetErrorGen;
use karty::figures::Figure;
use karty::hand::{CardSet, HandTrait};
use karty::suits::{Suit, SuitMap};
use karty::suits::Suit::Spades;
use karty::symbol::CardSymbol;
use crate::error::FuzzyCardSetErrorGen;
use crate::meta::HAND_SIZE;
use crate::player::side::{Side, SideMap, SIDES};
use crate::player::side::Side::{East, North, South, West};
use crate::sztorm::state::{FProbability, FuzzyCardSet};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct BiasedHandDistribution {
    side_probabilities: SideMap<FuzzyCardSet>
}

impl BiasedHandDistribution{

    pub fn card_probabilities(&self, card: &Card) -> SideMap<FProbability>{
        SideMap::new(
                self.side_probabilities[&North].card_probability(card),
                self.side_probabilities[&East].card_probability(card),
                self.side_probabilities[&South].card_probability(card),
                self.side_probabilities[&West].card_probability(card),
        )
    }

    fn allocate_card<R: Rng + ?Sized>(&self, card: &Card, map_of_closed: &SideMap<bool>, rng: &mut R) -> Result<Side, FuzzyCardSetErrorGen<Card>>{
        let card_probabilities = self.card_probabilities(card);
        let top_north = match map_of_closed[&North]{
            true => 0.0,
            false => f32::from(card_probabilities[&North])
        };
        let top_east = match map_of_closed[&East]{
            true => top_north,
            false => top_north + f32::from(card_probabilities[&East])
        };
        let top_south = match map_of_closed[&South]{
            true => top_east,
            false => top_east + f32::from(card_probabilities[&South])
        };
        let top_west = match map_of_closed[&West]{
            true => top_south,
            false => top_south + f32::from(card_probabilities[&West])
        };

        debug!("Sample top: {}", top_west);
        let sample = rng.gen_range(0f32..top_west);
        if sample < top_north{
            return Ok(North);
        } else if sample < top_east{
            return  Ok(East)
        } else if sample < top_south{
            return Ok(South);
        } else if sample < top_west{
            return Ok(West)
        }

        Err(FuzzyCardSetErrorGen::ImpossibleCardChoice)


    }

    pub fn sample_cards<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<SideMap<CardSet>, FuzzyCardSetErrorGen<Card>>{


        let mut card_sets = SideMap::new_symmetric(CardSet::empty());
        let mut cards_uncertain: SmallVec<[Card; 64]> = SmallVec::new();
        let mut cards_with_zero: SmallVec<[Card; 64]> = SmallVec::new();

        let mut closed_sides = SideMap::new_symmetric(false);

        let mut cards = STANDARD_DECK;
        cards.shuffle(rng);


        //phase 1, alloc certain One
        for c in cards{
            let card_probabilities = self.card_probabilities(&c);
            match choose_certain(&card_probabilities)?{
                None => {
                    match choose_certain_zero(&card_probabilities)?{
                        None => {
                            cards_uncertain.push(c);
                        }
                        Some(_s) => {
                            cards_with_zero.push(c);
                        }
                    }

                }
                Some(side) => {
                    card_sets[&side].insert_card(c)?;
                    if card_sets[&side].len() >= HAND_SIZE{
                        closed_sides[&side] = true;
                    }
                }
            }

        }

        // phase 2: alloc these with zero

        for c in cards_with_zero{
            let side = self.allocate_card(&c, &closed_sides, rng)?;
            card_sets[&side].insert_card(c)?;
            if card_sets[&side].len() >= HAND_SIZE{
                closed_sides[&side] = true;
            }

        }

        for c in cards_uncertain{
            let side = self.allocate_card(&c, &closed_sides, rng)?;
            card_sets[&side].insert_card(c)?;
            if card_sets[&side].len() >= HAND_SIZE{
                closed_sides[&side] = true;
            }

        }

        Ok(card_sets)

    }
}

impl Index<Side> for BiasedHandDistribution {
    type Output = FuzzyCardSet;

    fn index(&self, index: Side) -> &Self::Output {
        &self.side_probabilities[&index]
    }
}

impl Distribution<BiasedHandDistribution> for Standard{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BiasedHandDistribution {
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

        //let mut iteration = 0;
        let mut sides_shuffled = SIDES;
        loop{
            //iteration += 1;
            //debug!("Sampling try number {iteration:}");
            let mut probabilities = SideMap::new_symmetric(SuitMap::new_from_f(|_|[0.0f32; HAND_SIZE]));
            let mut sums_per_side = SideMap::new_symmetric(0.0f32);
            for i in 0..DECK_SIZE-1{
                let s = Suit::from_position(i/13).unwrap();
                let f = i%13;
                //let mut inner_iteration = 0;
                loop{
                    sides_shuffled.shuffle(rng);
                    //inner_iteration += 1;

                    let proba_1:f32 = rng.gen_range(0.0..=1.0);
                    let proba_2: f32 = rng.gen_range(0.0..=1.0);
                    let proba_3: f32 = rng.gen_range(0.0..=1.0);

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

            return BiasedHandDistribution { side_probabilities: SideMap::new(
                FuzzyCardSet::new_from_f32_derive_sum(probabilities[&North]).unwrap(),
                FuzzyCardSet::new_from_f32_derive_sum(probabilities[&East]).unwrap(),
                FuzzyCardSet::new_from_f32_derive_sum(probabilities[&South]).unwrap(),
                FuzzyCardSet::new_from_f32_derive_sum(probabilities[&West]).unwrap()) }

        }


    }
}


fn choose_certain(probabilities: &SideMap<FProbability>) -> Result<Option<Side>, FuzzyCardSetErrorGen<Card>>{
    for side in SIDES{
        if probabilities[&side] == FProbability::One{
            let probability_sum = probabilities
                .fold_on_ref(0.0f32, |acc, x|{
                    acc + f32::from(*x)
                });
            if probability_sum == 1.0{
                return Ok(Some(side))
            } else {
                return Err(FuzzyCardSetErrorGen::BadProbabilitiesSum{expected: 1.0, found: probability_sum})
            }
        }
    }
    Ok(None)
}

fn choose_certain_zero(probabilities: &SideMap<FProbability>) -> Result<Option<Side>, FuzzyCardSetErrorGen<Card>> {
    for side in SIDES{
        if probabilities[&side] == FProbability::Zero{
            return Ok(Some(side))
        }
    }
    Ok(None)
}



impl Distribution<SideMap<CardSet>> for BiasedHandDistribution{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SideMap<CardSet> {

        //let mut distribution_

        match self.sample_cards(rng){
            Ok(p) => p,
            Err(e) => {
                panic!("Error sampling cards from distribution {e:?}")
            }
        }


    }
}