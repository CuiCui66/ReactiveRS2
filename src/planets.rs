#![feature(box_syntax, box_patterns)]
#![feature(plugin)]
#![plugin(promacros)]

#[macro_use] extern crate ReactiveRS2;
extern crate rand;
extern crate time;

use ReactiveRS2::process::*;
use ReactiveRS2::signal::*;
use ReactiveRS2::engine::*;
use ReactiveRS2::node::ChoiceData;
use ReactiveRS2::node::ChoiceData::*;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use time::SteadyTime;

#[derive(Clone)]
struct Planet {
    mass: f32,
    pos: [f32; 3],
    speed: [f32; 3]
}

impl Planet {
    fn new<R: Rng>(rng: &mut R) -> Planet {
        Planet {
            mass: 1.0,
            pos: random_pos(rng),
            speed: random_speed(rng),
        }
    }
}

const GRAVITATION_CONST: f32 = 6.67;
const DT: f32 = 0.1;

type EnvSignal = SignalRuntimeRef<MCSignalValue<Planet, Vec<Planet>>>;


fn random_speed<R: Rng>(rng: &mut R) -> [f32; 3] {
    let between = Range::new(-50., 50.);
    [between.ind_sample(rng), between.ind_sample(rng), between.ind_sample(rng)]
}

fn random_pos<R: Rng>(rng: &mut R) -> [f32; 3] {
    let between = Range::new(-100., 100.);
    [between.ind_sample(rng), between.ind_sample(rng), between.ind_sample(rng)]
}

fn distance2(pos1: &[f32; 3], pos2: &[f32; 3]) -> f32 {
    (pos1[0]-pos2[0])*(pos1[0]-pos2[0])
    +(pos1[1]-pos2[1])*(pos1[1]-pos2[1])
    +(pos1[2]-pos2[2])*(pos1[2]-pos2[2])
}

fn distance(pos1: &[f32; 3], pos2: &[f32; 3]) -> f32 {
    distance2(pos1,pos2).sqrt()
}


fn force(planet1: &Planet, planet2: &Planet) -> [f32; 3] {
    let d2 = distance2(&planet1.pos, &planet2.pos);
    let d = distance(&planet1.pos, &planet2.pos);
    if d == 0. {
        [0.,0.,0.]
    } else {
        let f12 = GRAVITATION_CONST * planet1.mass * planet2.mass / d2;
        [
            f12 * (planet1.pos[0] - planet2.pos[0]) / d,
            f12 * (planet1.pos[1] - planet2.pos[1]) / d,
            f12 * (planet1.pos[2] - planet2.pos[2]) / d,
        ]
    }
}

fn next_pos(planet: &mut Planet, planets: &Vec<Planet>) {
    let mut current_force = [0.,0.,0.];
    for other_planet in planets {
        let planet_force = force(planet, other_planet);
        current_force[0] += planet_force[0];
        current_force[1] += planet_force[1];
        current_force[2] += planet_force[2];
    }

    planet.speed[0] += current_force[0] * DT;
    planet.speed[1] += current_force[1] * DT;
    planet.speed[2] += current_force[2] * DT;

    planet.pos[0] += planet.speed[0] * DT;
    planet.pos[1] += planet.speed[1] * DT;
    planet.pos[2] += planet.speed[2] * DT;
}


fn main() {
    let env = EnvSignal::new_mc(vec![], box |emit_value: Planet, current_value: &mut Vec<Planet>| {
        current_value.push(emit_value);
    });
    let mut rng = rand::thread_rng();

    let mut sun = Planet {
        pos: [0.,0.,0.],
        speed: [0.,0.,0.],
        mass: 30000.,
    };

    let mut sun_process = pro!(
        loop {
            emit_vs(env.clone(), sun);
            pause();
            value(True::<(),()>(()))
        }
    );

    let mut planet_processes = vec![];

    for i in 0..1000 {
        let mut planet = Planet::new(&mut rng);
        let planet_process = pro!(
        value(planet);
        loop {
            |planet: Planet| -> (Planet, Planet) {
                (planet.clone(), planet)
            };
            emit_s_in(env.clone());
            await_s_in(env.clone());
            |(planets, mut current_planet): (Vec<Planet>, Planet)| -> ChoiceData<Planet,()> {
                next_pos(&mut current_planet, &planets);
                True(current_planet)
            }
        });

        planet_processes.push(planet_process);
    }

    let planets_process = pro!(big_join(planet_processes));
    let mut runtime = rt!(value(((),())); (planets_process || sun_process); |_| {});

    let n = 1000;
    let start = SteadyTime::now();
    runtime.instantn(n);
    let frequency = (SteadyTime::now() - start).num_nanoseconds().unwrap() as f32 / 1_000_000_000.;
    println!("{} iterations of planets were executed  in {} second.", n, frequency);
}
