use std::io::Write;
use std::process::ExitCode;
use std::fs::File;
use std::path::Path;
use rand::Rng;
// A simulation of twitch gambling
// Some initial coniditions and goals are set as global constants:

fn main() -> Result<(), ExitCode> {
    // Output filepath
    let output_filepath = Path::new("/home/arcwand/projects/twitch-gamble/R/runs.csv");

    // This sets the number of simulations to run. More is more accurate
    const REPS: i32 = 1000000;
    // This sets the maximum number of turns that the simulation will run for
    const TURNS: i32 = 200;

    // The chance of winning any gamble
    const WIN_CHANCE: f64 = 0.6;

    // The principle is the initial amount of points that one starts with
    const PRINCIPLE: i64 = 2600000;
    // The percentage one bets in each gamble
    const BET: f64 = 0.15;

    // The end balance is the target number of points to reach by the end
    const TARGET_BALANCE: i64 = 21000000;
    // The number of points that, below which, a run will be considered dead
    const DEAD_THRESHOLD: i64 = 1000;


    // Check for plausibility
    if ((1 as f64 + BET).powi(TURNS) * PRINCIPLE as f64) < (TARGET_BALANCE as f64) {
        println!("Warning: The principle is too low to reach the target balance in {} turns.", TURNS);
        return Err(ExitCode::from(1));
    }

    let mut outfile = match File::create(&output_filepath) {
        Ok(file) => file,
        Err(_) => return Err(ExitCode::from(2)),
    };


    // Stats that are recorded for each run:
    // - Died after x turns (will remain -1 for runs which don't die)
    // - Number of turns to reach the target balance. Remains -1 for dead runs.
    //     - This will be set even if the run reaches the target but dies later.
    // - Ending balance. How many points remain after the max number of turns.

    let mut stats: Vec<[i64; 3]> = Vec::new();

    for _ in 0..REPS {
        stats.push(run_sim(TURNS, WIN_CHANCE, PRINCIPLE, BET, TARGET_BALANCE, DEAD_THRESHOLD));
    }

    // Stats that we are interested in knowing:
    // What percentage of runs die?
    // What is the average profit?
    // What percentage of runs reach the target?

    println!("Attempting to write");
    outfile.write_all( "Dead After,Turns to Target,End Balance\n".as_bytes())
        .expect("Error writing header");
    for tup in stats {
        outfile.write_all(
            format!("{},{},{}\n", tup[0], tup[1], tup[2]).as_bytes()
            ).expect("Error writing to output file.");
    }

    println!("Ran {} simulations successfully. Stored in {:?}", REPS, output_filepath);

    Ok(())
}

// Runs the simulation once. Returns 3 stats:
// - Died after x turns (will remain -1 for runs which don't die)
// - Number of turns to reach the target balance. Remains -1 for dead runs.
//     - This will be set even if the run reaches the target but dies later.
// - Ending balance. How many points remain after the max number of turns.
fn run_sim(turns: i32, win_chance: f64, principle: i64, bet: f64, target_balance: i64, dead_threshold: i64)
    -> [i64; 3] {
        let mut rng = rand::thread_rng();

        // Initialize the return variables
        let mut died_after: i32 = -1;
        let mut turns_to_target: i32 = -1;
        let mut balance: i64 = principle;

        for i in 1..=turns {
            // Determine if you win
            let win: bool = rng.gen_bool(win_chance);

            // Update stats
            if win { balance += ((balance as f64) * bet) as i64; }
            else { balance -= ((balance as f64) * bet) as i64; }

            // Determine if you have died
            if balance < dead_threshold {
                died_after = i;
                break;
            }

            // Determine if you have reached the target
            if balance >= target_balance && turns_to_target == -1 {
                turns_to_target = i;
            }
        }

        // Return tuple for stats
        [died_after as i64, turns_to_target as i64, balance]
}

