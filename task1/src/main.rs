use rand::Rng;

const TIMESTAMPS_COUNT: usize = 50000;

const PROBABILITY_SCORE_CHANGED: f64 = 0.0001;

const PROBABILITY_HOME_SCORE: f64 = 0.45;

const OFFSET_MAX_STEP: i32 = 3;

const INITIAL_STAMP: Stamp = Stamp {
    offset: 0,
    score: Score { home: 0, away: 0 },
};

#[derive(Debug, Clone, Copy)]
struct Score {
    home: i32,
    away: i32,
}

#[derive(Debug, Clone, Copy)]
struct Stamp {
    offset: i32,
    score: Score,
}

fn generate_stamp(previous_value: Stamp) -> Stamp {
    let score_changed: bool = rand::thread_rng().gen_bool(PROBABILITY_SCORE_CHANGED);
    let home_score_change: bool = rand::thread_rng().gen_bool(PROBABILITY_HOME_SCORE);
    let offset_change: i32 = rand::thread_rng().gen_range(1..=OFFSET_MAX_STEP);

    Stamp {
        offset: previous_value.offset + offset_change,
        score: Score {
            home: previous_value.score.home
                + if score_changed && home_score_change {
                    1
                } else {
                    0
                },
            away: previous_value.score.away
                + if score_changed && !home_score_change {
                    1
                } else {
                    0
                },
        },
    }
}

fn generate_game() -> Vec<Stamp> {
    let mut stamps = vec![INITIAL_STAMP];
    let mut current_stamp = INITIAL_STAMP;

    for _ in 0..TIMESTAMPS_COUNT {
        current_stamp = generate_stamp(current_stamp);
        stamps.push(current_stamp);
    }

    stamps
}

fn get_score(game_stamps: &[Stamp], offset: i32) -> (i32, i32) {
    // Фиксации с таким счетом может не существовать, так как он может быть либо больше, чем
    // максимальный счет, меньше 0, либо попасть между двумя фиксациями.
    // В 1 случае, вернем последнюю фиксацию
    // В 2 случае, вернем первую фиксацию
    // В 3 случае, вернем фиксацию, меньшую по offset, самую близкую к введенному offset
    // game_stamps.len() == TIMESTAMPS_COUNT + 1;
    // При условии, что все offset во всех фиксациях, кроме первой, равны 3, то максимальный offset
    // может быть `(game_stamps.len() - 1) * 3`

    let len = game_stamps.len();
    // Проверка на значения за пределами массива
    if offset > (len as i32 - 1) * 3 {
        return (
            game_stamps[len - 1].score.home,
            game_stamps[len - 1].score.away,
        );
    } else if offset < 0 {
        return (game_stamps[0].score.home, game_stamps[0].score.away);
    }

    // Проверка на значения в пределах массива
    let res = game_stamps.binary_search_by_key(&offset, |s| s.offset);
    let res = match res {
        Ok(x) => x,
        Err(x) => x - 1, // Берем ту, что находится на предыдущей позиции
    };
    let stamp = game_stamps[res];
    return (stamp.score.home, stamp.score.away);
}

fn main() {
    let stamps = generate_game();
    let offset = rand::thread_rng().gen_range(0..=(TIMESTAMPS_COUNT as usize - 1) * 3) as i32;
    let (x, y) = get_score(&stamps, offset);
    println!("Score for offset {} is {} - {}", offset, x, y);
}
