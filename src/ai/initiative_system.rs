use crate::{Attributes, Initiative, MyTurn, Position, RunState};
use specs::prelude::*;

pub struct InitiativeSystem {}

impl<'a> System<'a> for InitiativeSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'a, Initiative>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, MyTurn>,
        Entities<'a>,
        WriteExpect<'a, rltk::RandomNumberGenerator>,
        ReadStorage<'a, Attributes>,
        WriteExpect<'a, RunState>,
        ReadExpect<'a, Entity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut initiatives,
            positions,
            mut turns,
            entities,
            mut rng,
            attributes,
            mut runstate,
            player,
        ) = data;

        if *runstate != RunState::Ticking {
            return;
        }

        turns.clear();

        for (entity, initiative, _pos) in (&entities, &mut initiatives, &positions).join() {
            initiative.current -= 1;
            if initiative.current < 1 {
                turns
                    .insert(entity, MyTurn {})
                    .expect("Unable to insert turn");

                initiative.current = 6 + rng.roll_dice(1, 6);

                if let Some(attr) = attributes.get(entity) {
                    initiative.current -= attr.quickness.bonus;
                }

                // TODO: More initiative granting boosts/penalties will go here later

                if entity == *player {
                    *runstate = RunState::AwaitingInput;
                }
            }
        }
    }
}
