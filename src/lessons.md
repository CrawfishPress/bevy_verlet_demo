## Bevy Queries and smart-pointers

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

# Bevy Queries and multiple-borrows

A first-pass at porting this from the video (or from my Python-version), would have the Collisions
handling, run a loop within a loop, through the objects:

    for (entity_id, mut verlet_data, mut entity_pos) in balls_qry.iter_mut() {
        for (entity_id2, mut verlet_data2, mut entity_pos2) in balls_qry.iter_mut() {
            println!("E1 = {:?}, E2 = {:?}", entity_id, entity_id2)
        }
    }

If you know anything about Rust, you know it doesn't like mutable-borrowing more than once,
so this is right out. (I *did* expect it before doing it, just wanted to put some code in the
collision-handler, and see the error-message).

The solution (well, there may be others), is to "flatten" the query into a Vector of
mutable objects, then loop through that.

    let mut balls: Vec<(Entity, Mut<'_, VerletData>, Mut<'_, Transform>)> = balls_qry.iter_mut().collect();
    for (cur_index, (cur_entity, some_verlet, some_transform)) in balls.iter_mut().enumerate() {
        println!("entity[{}]: {:?} = {:?}", cur_index, cur_entity, some_verlet);
    }
