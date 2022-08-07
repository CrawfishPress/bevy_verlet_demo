Note: the Bevy Query, returns a *smart-pointer* - figuring out how
to unwrap this:

    Mut<'_, bevy::prelude::Transform>

    pub fn solve_for_verlet(mut balls_qry: Query<(Entity, &mut VerletData, &mut Transform)>,)
    {
        for (entity_id, mut verlet_data, mut entity_pos) in balls_qry.iter_mut() {
            apply_gravity(&mut verlet_data, &mut *entity_pos);
        }
    }
    
    pub fn apply_gravity(mut verlet_data: Mut<'_, VerletData>, 
                         mut entity_pos: Mut<'_, bevy::prelude::Transform>) {}

was tricky.

And *then*, I needed to unwrap it, so I could take a reference...
 - https://bevy-cheatbook.github.io/pitfalls/split-borrows.html

