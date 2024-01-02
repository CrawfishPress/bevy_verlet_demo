### Bevy Queries and smart-pointers

Note: the Bevy Query, returns a *smart-pointer* - figuring out how
to unwrap it, was tricky. (The Rust forums were very helpful...)

    Mut<'_, bevy::prelude::Transform>

    pub fn solve_for_verlet(mut balls_qry: Query<(Entity, &mut VerletData, &mut Transform)>,)
    {
        for (entity_id, mut verlet_data, mut entity_pos) in balls_qry.iter_mut() {
            apply_gravity(&mut verlet_data, &mut *entity_pos);
        }
    }
    
    pub fn apply_gravity(mut verlet_data: Mut<'_, VerletData>, 
                         mut entity_pos: Mut<'_, bevy::prelude::Transform>) {}

And *then*, I needed to unwrap it, so I could take a reference...
 - https://bevy-cheatbook.github.io/pitfalls/split-borrows.html

### Bevy Queries and multiple-borrows

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

### And another multi-borrow (well, split-borrow)
I had a Resource with two integer values on it, and tried to split them up into
two different Egui sliders - had to jump through some hoops to do it.

https://bevy-cheatbook.github.io/pitfalls/split-borrows.html
https://doc.rust-lang.org/nomicon/borrow-splitting.html

    let mut tmp_radius = random_data.radius_slider_value;
    let mut tmp_ball = random_data.ball_slider_value;

    let radius_slider = egui::Slider::new(&mut tmp_radius, 10..=50);
    let ball_slider = egui::Slider::new(&mut tmp_ball, 0..=500);

And it still didn't work... According to the docs, it is possible to split
integers in a struct, but for some reason, they didn't. Possibly because they were
being passed into two separate objects (Sliders), and at that point it was
impossible to track them?

Turns out I was referencing each variable separately, instead of the struct as a
whole. When I did that, it actually worked!
