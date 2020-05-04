#![allow(non_upper_case_globals,
    non_camel_case_types,
    non_snake_case)]
    
#![repr(C)]

use std::mem;
use std::arch::x86_64::*;
use std::f64::consts::PI;
    
struct body{
    position: [f64; 3],
    velocity: [f64; 3],
    mass: f64,
}    

const SOLAR_MASS: f64 = 4. * PI * PI;
const DAYS_PER_YEAR: f64 = 365.24;
const BODIES_COUNT: usize = 5;

static mut solar_Bodies: [body; BODIES_COUNT] = [
    body { // Sun
        mass: SOLAR_MASS,
        position: [0.; 3],
        velocity: [0.; 3],
    },
    body {    // Jupiter
        position: [ 
             4.84143144246472090e+00,
            -1.16032004402742839e+00,
            -1.03622044471123109e-01
        ],
        velocity: [
             1.66007664274403694e-03 * DAYS_PER_YEAR,
             7.69901118419740425e-03 * DAYS_PER_YEAR,
            -6.90460016972063023e-05 * DAYS_PER_YEAR
        ],
        mass: 9.54791938424326609e-04 * SOLAR_MASS
    },
    body {    // Saturn
        position: [
             8.34336671824457987e+00,
             4.12479856412430479e+00,
            -4.03523417114321381e-01
        ],
        velocity: [
            -2.76742510726862411e-03 * DAYS_PER_YEAR,
             4.99852801234917238e-03 * DAYS_PER_YEAR,
             2.30417297573763929e-05 * DAYS_PER_YEAR
        ],
        mass: 2.85885980666130812e-04 * SOLAR_MASS
    },
    body {    // Uranus
        position: [
             1.28943695621391310e+01,
            -1.51111514016986312e+01,
            -2.23307578892655734e-01
        ],
        velocity: [
             2.96460137564761618e-03 * DAYS_PER_YEAR,
             2.37847173959480950e-03 * DAYS_PER_YEAR,
            -2.96589568540237556e-05 * DAYS_PER_YEAR
        ],
        mass: 4.36624404335156298e-05 * SOLAR_MASS
    },
    body {    // Neptune
        position: [
             1.53796971148509165e+01,
            -2.59193146099879641e+01,
             1.79258772950371181e-01
        ],
        velocity: [
             2.68067772490389322e-03 * DAYS_PER_YEAR,
             1.62824170038242295e-03 * DAYS_PER_YEAR,
            -9.51592254519715870e-05 * DAYS_PER_YEAR
        ],
        mass: 5.15138902046611451e-05 * SOLAR_MASS
    }
];

unsafe fn offset_Momentum(bodies: *mut body){
    for i in 0..BODIES_COUNT {
        for m in 0..3 {
            (*bodies.add(i)).velocity[m] -=
                (*bodies.add(i)).velocity[m] 
                * (*bodies.add(i)).mass / SOLAR_MASS; 
        }
    }
}

unsafe fn output_Energy(bodies: *mut body){
    let mut energy = 0.;
    for i in 0..BODIES_COUNT {
        // add kinectic energy to each body
        energy += 0.5 * (*bodies.add(i)).mass
            * (
                (*bodies.add(i)).velocity[0] * (*bodies.add(i)).velocity[0] 
                + (*bodies.add(i)).velocity[1] * (*bodies.add(i)).velocity[1] 
                + (*bodies.add(i)).velocity[2] * (*bodies.add(i)).velocity[2] 

            );
        
        // Add the potential energy between this body and every other body
        for j in i+1..BODIES_COUNT {
            let mut position_Delta = [mem::MaybeUninit::<f64>::uninit(); 3];
            for m in 0..3 {
                position_Delta[m].as_mut_ptr().write(
                    (*bodies.add(i)).position[m] - (*bodies.add(j)).position[m]
                ); 
            }
            let position_Delta: [f64; 3] = mem::transmute(position_Delta);

            energy -= (*bodies.add(i)).mass * (*bodies.add(j)).mass
                / f64::sqrt(
                    position_Delta[0] * position_Delta[0]
                    + position_Delta[1] * position_Delta[1]
                    + position_Delta[2] * position_Delta[2]
                );
        } 

    }
    println!("{:.9}", energy);
}

unsafe fn advance (bodies: *mut body){
   const INTERACTIONS_COUNT: usize = BODIES_COUNT * (BODIES_COUNT - 1) / 2;
   const ROUNDED_INTERACTIONS_COUNT: usize = INTERACTIONS_COUNT + INTERACTIONS_COUNT % 2;
   
   #[repr(align(16))]
   #[derive(Copy, Clone)]
   struct Align16([f64; ROUNDED_INTERACTIONS_COUNT]);

    static mut position_Deltas: [Align16; 3] = [Align16([0.; ROUNDED_INTERACTIONS_COUNT]); 3];
    static mut magnitudes: Align16 = Align16([0.; ROUNDED_INTERACTIONS_COUNT]);

    let mut k = 0;
    for i in 0..BODIES_COUNT-1 {
        for j in i+1..BODIES_COUNT-1 {
            for m in 0..3 {
                position_Deltas[m].0[k] = (*bodies.add(i)).position[m] - (*bodies.add(j)).position[m];
            }
            k += 1;
        }
    }

    for i in 0..ROUNDED_INTERACTIONS_COUNT / 2 {
        let mut position_Delta = [mem::MaybeUninit::<__m128d>::uninit(); 3];
        for m in 0..3 {
            position_Delta[m].as_mut_ptr().write(
                *(&position_Deltas[m].0 as *const f64 as *const __m128d).add(i)
            );
        }
        let position_Delta: [__m128d; 3] = mem::transmute(position_Delta);
    }

}

fn main() {
    println!("Hello, world!");
}
