use std::time::{SystemTime, UNIX_EPOCH};

// J2000.0 = January 1, 2000, 12:00:00 UTC. Declared as "start of Saturday (day 0)" for all planets.
const J2000_UNIX: f64 = 946728000.0;
const EARTH_DAY_SECS: f64 = 86400.0;
const TUESDAY: usize = 3;
const DAY_NAMES: [&str; 7] = [
    "Saturday",
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
];

struct Planet {
    name: &'static str,
    solar_day_hours: f64,
    use_sol: bool,
}

const PLANETS: [Planet; 9] = [
    Planet { name: "Mercury", solar_day_hours: 4222.6,   use_sol: false },
    Planet { name: "Venus",   solar_day_hours: 2802.0,   use_sol: false },
    Planet { name: "Earth",   solar_day_hours: 24.0,     use_sol: false },
    Planet { name: "Mars",    solar_day_hours: 24.6598,  use_sol: true  },
    Planet { name: "Jupiter", solar_day_hours: 9.9250,   use_sol: false },
    Planet { name: "Saturn",  solar_day_hours: 10.6562,  use_sol: false },
    Planet { name: "Uranus",  solar_day_hours: 17.2400,  use_sol: false },
    Planet { name: "Neptune", solar_day_hours: 16.1100,  use_sol: false },
    Planet { name: "Pluto",   solar_day_hours: 153.3,    use_sol: false },
];

struct TacoResult {
    planet_name: &'static str,
    current_day: usize,
    is_tuesday: bool,
    seconds_until: f64,
    use_sol: bool,
    solar_day_secs: f64,
}

fn get_now_unix() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before Unix epoch")
        .as_secs_f64()
}

fn compute_result(planet: &Planet, now_unix: f64) -> TacoResult {
    let solar_day_secs = planet.solar_day_hours * 3600.0;
    let elapsed = now_unix - J2000_UNIX;
    let total_days = elapsed / solar_day_secs;
    let week_pos = total_days.rem_euclid(7.0);
    let current_day = week_pos.floor() as usize;
    let frac = week_pos.fract();

    let (is_tuesday, seconds_until) = if current_day == TUESDAY {
        (true, (1.0 - frac) * solar_day_secs)
    } else {
        let days_until = (3.0 - week_pos).rem_euclid(7.0);
        (false, days_until * solar_day_secs)
    };

    TacoResult {
        planet_name: planet.name,
        current_day,
        is_tuesday,
        seconds_until,
        use_sol: planet.use_sol,
        solar_day_secs,
    }
}

fn format_planetary_duration(secs: f64, solar_day_secs: f64, use_sol: bool) -> String {
    let planet_days = (secs / solar_day_secs).floor() as u64;
    let remaining_secs = secs - planet_days as f64 * solar_day_secs;
    let hours = (remaining_secs / 3600.0).floor() as u64;
    let minutes = ((remaining_secs % 3600.0) / 60.0).floor() as u64;

    if planet_days == 0 {
        format!("{}h {}m", hours, minutes)
    } else if use_sol {
        let unit = if planet_days == 1 { "sol" } else { "sols" };
        format!("{} {} {}h {}m", planet_days, unit, hours, minutes)
    } else {
        format!("{}d {}h {}m", planet_days, hours, minutes)
    }
}

fn format_tuesday_remaining(secs: f64) -> String {
    let hours = (secs / 3600.0).floor() as u64;
    let minutes = ((secs % 3600.0) / 60.0).floor() as u64;
    format!("{}h {}m left", hours, minutes)
}

fn format_earth_equivalent(secs: f64) -> String {
    if secs < 3600.0 {
        let minutes = (secs / 60.0).round() as u64;
        format!("~{}m Earth", minutes)
    } else if secs < EARTH_DAY_SECS {
        let hours = (secs / 3600.0).round() as u64;
        format!("~{}h Earth", hours)
    } else {
        let days = secs / EARTH_DAY_SECS;
        if days < 2.0 {
            format!("~{:.1} Earth days", days)
        } else {
            format!("~{} Earth days", days.round() as u64)
        }
    }
}

/// Search forward in 1-hour steps (safe: Jupiter's Tuesday lasts ~9.9h).
/// Returns Unix timestamp of first moment all 9 planets are simultaneously in Tuesday.
fn find_next_alignment(now_unix: f64) -> Option<f64> {
    // Expected gap ≈ 7^8 random-hours ≈ ~2000 Earth years; search 20,000 to be safe.
    let max_secs = 20_000.0 * 365.25 * EARTH_DAY_SECS;
    let step = 3600.0;
    let mut t = now_unix;
    let end = now_unix + max_secs;

    while t <= end {
        let all_tuesday = PLANETS.iter().all(|p| {
            let day_secs = p.solar_day_hours * 3600.0;
            let week_pos = ((t - J2000_UNIX) / day_secs).rem_euclid(7.0);
            week_pos.floor() as usize == TUESDAY
        });
        if all_tuesday {
            return Some(t);
        }
        t += step;
    }
    None
}

fn format_earth_duration(secs: f64) -> String {
    let years = secs / (365.25 * EARTH_DAY_SECS);
    if years >= 1.0 {
        format!("{:.1} Earth years", years)
    } else {
        let days = (secs / EARTH_DAY_SECS).round() as u64;
        format!("{} Earth days", days)
    }
}

fn print_table(results: &[TacoResult]) {
    let planet_w = 9;
    let day_w = 11;

    println!(
        "{:<planet_w$} | {:<day_w$} | Until Taco Tuesday",
        "Planet", "Local Day",
        planet_w = planet_w,
        day_w = day_w,
    );
    println!("{}", "─".repeat(60));

    for r in results {
        let day_label = if r.is_tuesday {
            "Tuesday!".to_string()
        } else {
            DAY_NAMES[r.current_day].to_string()
        };

        let until_str = if r.is_tuesday {
            format!("🌮 Today! ({})", format_tuesday_remaining(r.seconds_until))
        } else {
            let planet_dur = format_planetary_duration(r.seconds_until, r.solar_day_secs, r.use_sol);
            let earth_equiv = format_earth_equivalent(r.seconds_until);
            format!("{}  ({})", planet_dur, earth_equiv)
        };

        println!(
            "{:<planet_w$} | {:<day_w$} | {}",
            r.planet_name,
            day_label,
            until_str,
            planet_w = planet_w,
            day_w = day_w,
        );
    }
}

fn main() {
    let now = get_now_unix();
    let results: Vec<TacoResult> = PLANETS.iter().map(|p| compute_result(p, now)).collect();
    print_table(&results);

    print!("\nSearching for next simultaneous Taco Tuesday (all 9 planets)... ");
    match find_next_alignment(now) {
        Some(t) => {
            let secs_from_now = t - now;
            println!("found!");
            println!("  In {} from now", format_earth_duration(secs_from_now));
        }
        None => println!("not within 20,000 Earth years."),
    }
}
