use std::convert::TryFrom;

use postflop_solver::*;

mod exporter;
use exporter::Exporter;


fn main() {
    let mut game = generate_game();

    // solve the game
    game.allocate_memory(false);
    let max_num_iterations = 1000;
    let target_exploitability = game.tree_config().starting_pot as f32 * 0.005; // 0.5% of the pot

    game.play(1);
    game.lock_current_strategy(&
    [0.000,0.000,0.000,60.000,60.000,0.000,0.000,0.700,0.700,0.700,0.000,0.000,60.000,60.000,0.000,0.700,0.000,0.700,0.700,0.000,60.000,60.000,0.700,0.700,0.000,0.700,1.000,1.000,0.000,0.700,0.700,0.700,0.700,0.000,0.000,0.000,1.000,1.000,1.000,60.000,60.000,0.000,0.000,0.700,0.700,0.700,0.000,0.000,1.000,1.000,1.000,60.000,60.000,0.000,0.700,0.000,0.700,0.700,0.000,1.000,1.000,1.000,60.000,60.000,0.700,0.700,0.000,0.700,1.000,1.000,1.000,1.000,1.000,0.000,0.700,0.700,0.700,0.700,0.000,0.000,0.000,1.000,1.000,60.000,60.000,0.000,0.000,0.700,0.700,0.700,0.000,0.000,1.000,1.000,0.000,60.000,60.000,0.000,0.700,0.000,0.700,0.700,0.000,1.000,1.000,0.000,60.000,60.000,0.700,0.700,0.000,0.700,1.000,1.000,0.000,1.000,1.000,0.000,0.700,0.700,0.700,0.700,0.000,0.000,0.000,0.750,0.000,60.000,60.000,0.000,0.000,0.000,0.000,0.700,0.700,0.700,0.000,0.000,0.750,0.000,0.000,60.000,60.000,0.000,0.000,0.000,0.700,0.000,0.700,0.700,0.000,0.750,0.000,0.000,60.000,60.000,0.000,0.000,0.000,0.700,0.700,0.000,0.700,0.750,0.000,0.750,1.000,1.000,0.000,0.000,0.000,0.700,0.700,0.700,0.700,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,60.000,60.000,60.000,0.000,0.000,0.000,0.000,0.700,0.700,0.700,0.000,0.000,0.000,0.000,0.000,0.000,60.000,60.000,60.000,0.000,0.000,0.000,0.700,0.000,0.700,0.700,0.000,0.000,0.000,0.000,0.000,0.000,60.000,60.000,60.000,0.000,0.000,0.000,0.700,0.700,0.000,0.700,0.000,0.000,0.000,0.000,0.000,0.750,1.000,1.000,0.000,0.000,0.000,0.700,0.700,0.700,0.700,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,1.000,1.000,0.963,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,1.000,0.000,1.000,0.933,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,1.000,1.000,0.000,0.901,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,1.000,1.000,1.000,0.996,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,1.000,1.000,0.976,30.000,30.000,1.000,1.000,0.300,0.300,0.300,1.000,0.975,30.000,30.000,1.000,0.300,1.000,0.300,0.300,1.000,30.000,30.000,0.300,0.300,1.000,0.300,0.000,0.000,1.000,0.300,0.300,0.300,0.300,1.000,1.000,0.998,0.000,0.000,0.000,30.000,30.000,1.000,1.000,0.300,0.300,0.300,1.000,1.000,0.000,0.000,0.000,30.000,30.000,1.000,0.300,1.000,0.300,0.300,1.000,0.000,0.000,0.000,30.000,30.000,0.300,0.300,1.000,0.300,0.000,0.000,0.000,0.000,0.000,1.000,0.300,0.300,0.300,0.300,1.000,1.000,0.990,0.000,0.000,30.000,30.000,1.000,1.000,0.300,0.300,0.300,1.000,0.947,0.000,0.000,1.000,30.000,30.000,1.000,0.300,1.000,0.300,0.300,0.979,0.000,0.000,1.000,30.000,30.000,0.300,0.300,1.000,0.300,0.000,0.000,1.000,0.000,0.000,1.000,0.300,0.300,0.300,0.300,1.000,1.000,1.000,0.250,1.000,30.000,30.000,1.000,0.996,0.999,1.000,0.300,0.300,0.300,1.000,1.000,0.250,1.000,1.000,30.000,30.000,0.993,1.000,0.986,0.300,1.000,0.300,0.300,1.000,0.250,1.000,1.000,30.000,30.000,1.000,1.000,1.000,0.300,0.300,1.000,0.300,0.250,1.000,0.250,0.000,0.000,1.000,1.000,1.000,0.300,0.300,0.300,0.300,1.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,30.000,30.000,30.000,1.000,1.000,1.000,1.000,0.300,0.300,0.300,1.000,1.000,1.000,1.000,1.000,1.000,30.000,30.000,30.000,1.000,1.000,1.000,0.300,1.000,0.300,0.300,0.995,1.000,1.000,1.000,1.000,1.000,30.000,30.000,30.000,1.000,1.000,1.000,0.300,0.300,1.000,0.300,1.000,1.000,1.000,1.000,1.000,0.250,0.000,0.000,1.000,1.000,1.000,0.300,0.300,0.300,0.300,0.800,0.800,1.000,1.000,1.000,1.000,1.000,1.000,1.000,0.800,0.800,0.800,1.000,1.000,1.000,1.000,0.800,1.000,1.000,1.000,1.000,0.800,1.000,1.000,1.000,0.800,0.800,0.800,1.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,0.800,1.000,1.000,1.000,0.800,0.800,0.800,1.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,0.883,0.878,0.897,1.000,1.000,1.000,0.893,0.731,0.000,0.000,0.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,0.894,1.000,0.894,0.950,1.000,1.000,1.000,0.882,0.000,0.773,0.000,0.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,0.903,0.919,1.000,0.950,1.000,1.000,1.000,0.845,0.000,0.000,0.842,0.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,0.889,0.930,0.915,0.967,1.000,1.000,1.000,0.809,0.000,0.000,0.000,0.000,1.000,1.000,1.000,1.000,1.000,1.000,1.000,0.800,0.800,1.000,0.957,0.935,0.869,1.000,0.955,1.000,0.996,1.000,1.000,1.000,1.000,0.800,0.800,0.800,1.000,1.000,1.000,1.000,0.800,0.925,0.954,1.000,0.870,0.946,0.952,1.000,0.984,1.000,1.000,1.000,1.000,0.800,0.800,0.800,1.000,1.000,0.996,1.000,0.918,0.965,0.919,0.902,0.958,0.980,1.000,0.996,1.000,1.000,1.000,1.000,0.800,0.800,0.800,1.000,1.000,1.000,1.000,1.000,0.895,0.904,0.878,0.625,0.922,0.955,0.824,1.000,0.170,0.155,1.000,1.000,1.000,1.000,0.897,1.000,0.941,0.909,0.936,0.437,0.992,0.859,0.231,1.000,0.228,1.000,1.000,1.000,1.000,0.886,0.904,1.000,0.901,0.963,0.989,0.276,0.868,0.434,0.443,0.438,1.000,1.000,1.000,1.000,0.852,0.872,0.891,0.962,0.906,0.931,0.931,0.860,0.324,0.332,0.332,1.000,1.000,1.000,1.000,1.000,0.978,0.999,0.905,0.003,0.000,0.000,1.000,1.000,1.000,0.942,1.000,1.000,0.975,0.000,0.189,0.000,1.000,1.000,1.000,0.982,1.000,0.743,0.992,0.000,0.000,0.000,1.000,1.000,1.000,0.999,1.000,1.000,0.995,0.000,0.000,0.000,1.000,1.000,1.000,0.032,0.000,0.000,0.000,0.112,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.024,10.000,10.000,0.000,0.000,0.000,0.000,0.000,0.000,0.025,10.000,10.000,0.000,0.000,0.000,0.000,0.000,0.000,10.000,10.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.002,0.000,0.000,0.000,10.000,10.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,10.000,10.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,10.000,10.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.010,0.000,0.000,10.000,10.000,0.000,0.000,0.000,0.000,0.000,0.000,0.053,0.000,0.000,0.000,10.000,10.000,0.000,0.000,0.000,0.000,0.000,0.021,0.000,0.000,0.000,10.000,10.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,10.000,10.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,10.000,10.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,10.000,10.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,10.000,10.000,10.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,10.000,10.000,10.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.005,0.000,0.000,0.000,0.000,0.000,10.000,10.000,10.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.200,0.200,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.200,0.200,0.200,0.000,0.000,0.000,0.000,0.200,0.000,0.000,0.000,0.000,0.200,0.000,0.000,0.000,0.200,0.200,0.200,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.200,0.000,0.000,0.000,0.200,0.200,0.200,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.117,0.122,0.103,0.000,0.000,0.000,0.107,0.269,0.000,0.000,0.037,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.106,0.000,0.106,0.050,0.000,0.000,0.000,0.118,0.000,0.227,0.000,0.067,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.097,0.081,0.000,0.050,0.000,0.000,0.000,0.155,0.000,0.000,0.158,0.099,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.111,0.070,0.085,0.033,0.000,0.000,0.000,0.191,0.000,0.000,0.000,0.004,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.200,0.200,0.000,0.043,0.065,0.131,0.000,0.045,0.000,0.004,0.000,0.000,0.000,0.000,0.200,0.200,0.200,0.000,0.000,0.000,0.000,0.200,0.075,0.046,0.000,0.130,0.054,0.048,0.000,0.016,0.000,0.000,0.000,0.000,0.200,0.200,0.200,0.000,0.000,0.004,0.000,0.082,0.035,0.081,0.098,0.042,0.020,0.000,0.004,0.000,0.000,0.000,0.000,0.200,0.200,0.200,0.000,0.000,0.000,0.000,0.000,0.105,0.096,0.122,0.375,0.078,0.045,0.176,0.000,0.830,0.845,0.000,0.000,0.000,0.000,0.103,0.000,0.059,0.091,0.064,0.563,0.008,0.141,0.769,0.000,0.772,0.000,0.000,0.000,0.000,0.114,0.096,0.000,0.099,0.037,0.011,0.724,0.132,0.566,0.557,0.562,0.000,0.000,0.000,0.000,0.148,0.128,0.109,0.038,0.094,0.069,0.069,0.140,0.675,0.668,0.664,0.000,0.000,0.000,0.000,0.000,0.022,0.001,0.095,0.997,1.000,1.000,0.000,0.000,0.000,0.058,0.000,0.000,0.025,1.000,0.811,1.000,0.000,0.000,0.000,0.018,0.000,0.257,0.008,1.000,1.000,1.000,0.000,0.000,0.000,0.001,0.000,0.000,0.005,1.000,1.000,1.000,0.000,0.000,0.000,0.968,1.000,1.000,1.000,0.888,1.000,1.000,1.000,1.000,1.000,1.000,1.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.004,0.001,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.007,0.000,0.014,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.001,0.000,0.005,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000]
        );
    game.back_to_root();

    solve(&mut game, max_num_iterations, target_exploitability, true);

    let mut exporter = Exporter::new(
        &mut game,
        vec!["8d", "7h", "2s", "Ad", "Tc", "6d"],
    ).unwrap();

    exporter.export_init();
    exporter.export(0);
}

fn generate_game() -> PostFlopGame {
    // ranges of OOP and IP in string format
    let oop_range = "22+,A2+,K2s+,K5o+,Q5s+,Q8o+,J5s+,J9o+,T7s+,T9o,96s+,98o,86s+,75s+,65s,54s";
    let ip_range = "99-88:0.5,77-22,ATs-A9s:0.5,A8s-A6s,A5s-A3s:0.5,A2s,AJo-ATo:0.5,A9o-A4o,A3o-A2o:0.5,K9s+:0.5,K8s-K2s,KJo+:0.5,KTo-K7o,K6o-K5o:0.5,Q9s+:0.5,Q8s-Q2s,Q8o+,J7s+:0.5,J6s-J4s,J3s-J2s:0.5,J8o+,T6s+:0.5,T8o+,96s+:0.5,98o,86s+:0.5,85s,87o,75s+:0.5,74s,76o,64s+:0.5,63s,54s:0.5,53s,43s";

    let card_config = CardConfig {
        range: [oop_range.parse().unwrap(), ip_range.parse().unwrap()],
        flop: flop_from_str("Kh9d7c").unwrap(),
        // turn: card_from_str("7c").unwrap(),
        turn: NOT_DEALT,
        river: NOT_DEALT,
    };

    // bet sizes -> 60% of the pot, geometric size, and all-in
    // raise sizes -> 2.5x of the previous bet
    // see the documentation of `BetSizeCandidates` for more details
    let bet_sizes = BetSizeCandidates::try_from(("30%, 100%, a", "60%, a")).unwrap();

    let expected_state = match (card_config.turn != NOT_DEALT, card_config.river != NOT_DEALT) {
        (false, _) => BoardState::Flop,
        (true, false) => BoardState::Turn,
        (true, true) => BoardState::River,
    };

    let tree_config = TreeConfig {
        initial_state: expected_state,
        starting_pot: 600,
        effective_stack: 2000,
        rake_rate: 0.0,
        rake_cap: 0.0,
        flop_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()], // [OOP, IP]
        turn_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
        river_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
        turn_donk_sizes: None, // use default bet sizes
        river_donk_sizes: None,
        add_allin_threshold: 1.5, // add all-in if (maximum bet size) <= 1.5x pot
        force_allin_threshold: 0.15, // force all-in if (SPR after the opponent's call) <= 0.15
        merging_threshold: 0.1,
    };

    // build the game tree
    let action_tree = ActionTree::new(tree_config).unwrap();
    PostFlopGame::with_config(card_config, action_tree).unwrap()
}
