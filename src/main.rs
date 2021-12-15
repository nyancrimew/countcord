use std::{env, thread::sleep, time::Duration};

use discord_rpc_client::Client;
use chrono::{DateTime, TimeZone, Utc};

const ONE_MINUTE: i64 = 60;
const ONE_HOUR: i64 = 60 * ONE_MINUTE;
const ONE_DAY: i64 = 24 * ONE_HOUR;
const ONE_YEAR: i64 = 365 * ONE_DAY;

fn help(cmd: String) {
    println!("usage: {} countdown_to [event] [event_reach_text]
    - countdown_to: unix timestamp to count down to
    - event: name of the event you're counting down to
    - event_reach_text: text to display once event time is reached", cmd);
    std::process::exit(1);
}

fn main() {
    // TODO: i should probably just use an arg parsing library lol
    let args: Vec<String> = env::args().collect();

    let countdown_to: DateTime<Utc>;
    let event: String;
    let event_reach_text: String;

    if args.len() < 2 || args.len() > 4 {
        help(args[0].clone());
        return;
    } else {
        let arg = args[1].parse::<i64>();
        if arg.is_err() {
            eprintln!("error: expected unix timestamp");
            help(args[0].clone());
        }
        countdown_to = Utc.timestamp(arg.unwrap(), 0);

        if args.len() >= 3 {
            event = args[2].clone();
        } else {
            event = "".to_string();
        }
        if args.len() == 4 {
            event_reach_text = args[3].clone();
        } else {
            event_reach_text = "".to_string();
        }
    }

    let mut client = Client::new(920431105012809789);
    client.start();

    loop {
        let duration = countdown_to.signed_duration_since(Utc::now()).num_seconds();
        let state: String;
        if duration <= 0 {
            if event_reach_text.to_owned() != "" {
                state = event_reach_text.to_owned();
            } else {
                state = "It's happening!!".to_string();
            }
        } else {
            state = to_duration_string(duration);
        }
        let result = client.set_activity(|act| {
            if event.to_owned() != "" {
                return act
                    .assets(|assets| assets.large_image("clock-icon"))
                    .details(event.to_owned())
                    .state(state);
            }
            return act
                .assets(|assets| assets.large_image("clock-icon"))
                .state(state);
        });
        if result.is_err() {
            println!("couldn't set discord activity")
        }

        // discord appears to update statuses every two seconds
        sleep(Duration::from_secs(2));
    }
}

// based on https://gist.github.com/elliotchance/a23259e24c2f1cb85add61c4ae3e912f
fn to_duration_string(duration: i64) -> String {
    // TODO: move out of here
    let units = vec![
        (ONE_YEAR, "year"),
        (ONE_DAY, "day"),
        (ONE_HOUR, "hour"),
        (ONE_MINUTE, "min"),
        (1, "sec"),
    ];

    let mut separate: Vec<String> = Vec::new();
    let mut condensed: Vec<String> = Vec::new();

    let mut seconds = duration;
    for (period, word) in units {
        if seconds >= period {
            let n = seconds / period;
            if period == ONE_YEAR || period == ONE_DAY || duration < ONE_DAY {
                separate.push(pluralize(n, word));
            } else {
                condensed.push(format!("{:02}", n));
            }
            seconds -= n * period;
        }
    }

    if condensed.is_empty() {
        let out = separate.join(", ");
        let parts = out.rsplit_once(" , ");
        if parts.is_some() {
            let parts = parts.unwrap();
            return format!("{} and {} left", parts.0, parts.1);
        }
        return format!("{} left", out);
    }
    let condensed = condensed.join(":");
    if !separate.is_empty() {
        return format!("{} and {} left", separate.join(", "), condensed);
    }
    return format!("{} left", condensed);
}

fn pluralize(n: i64, word: &str) -> String {
    if n == 1 {
        return format!("{} {}", n, word);
    }
    return format!("{} {}s", n, word);
}
