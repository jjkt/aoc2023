// Part 1:
// ................
// Race:         1      2      3      4
// Time:        54     81     70     88
// Distance:   446   1292   1035   1007

// Part 2
// ................
// Race:         1
// Time:        54817088
// Distance:   446129210351007

// derive debug
#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn calculate_total_product(races: &Vec<Race>) {
    let mut total_product: u64 = 1;

    for race in races {
        println!("{:?}", race);

        let mut ways_to_beat = 0;
        for button_press_time_pekka in 1..race.time {
            let speed = button_press_time_pekka;
            let time_to_move = race.time - button_press_time_pekka;
            let distance = speed * time_to_move;

            if distance > race.distance {
                ways_to_beat += 1;
            }
        }
        total_product *= ways_to_beat;
    }
    println!("Total product: {}", total_product);
}
fn main() {
    let race1 = Race {
        time: 54,
        distance: 446,
    };

    // races 2..4:
    let race2 = Race {
        time: 81,
        distance: 1292,
    };

    let race3 = Race {
        time: 70,
        distance: 1035,
    };

    let race4 = Race {
        time: 88,
        distance: 1007,
    };

    let race5 = Race {
        time: 71530,
        distance: 940200,
    };

    let race6 = Race {
        time: 54817088,
        distance: 446129210351007,
    };

    calculate_total_product(&vec![race1, race2, race3, race4]);
    calculate_total_product(&vec![race5]);
    calculate_total_product(&vec![race6]);
}
