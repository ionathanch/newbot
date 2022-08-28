// SPDX-License-Identifier: GPL-3.0-or-later

#![warn(clippy::pedantic)]

use minreq::Error;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::env;

include!(concat!(env!("OUT_DIR"), "/emoji.rs"));

#[derive(Debug, Serialize)]
struct NewStatus {
    status: String,
    visibility: Visibility,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum Visibility {
    Direct,
    Unlisted,
}

#[derive(Debug, Deserialize)]
struct Emoji {
    shortcode: String,
}

fn draft_toot(emojos: &Vec<Emoji>, live: bool) -> Result<NewStatus, Error> {
    let n = thread_rng().gen_range(0..(EMOJI_SETS.len() + emojos.len()));
    let emoji = if n < EMOJI_SETS.len() {
        let set = EMOJI_SETS[n];
        Cow::from(set[thread_rng().gen_range(0..set.len())])
    } else {
        let n = n - EMOJI_SETS.len();
        Cow::from(format!(":{}:", emojos[n].shortcode))
    };

    Ok(NewStatus {
        status: format!(":newl:\u{200b}{}\u{200b}:newr:", emoji),
        visibility: if live {
            Visibility::Unlisted
        } else {
            Visibility::Direct
        },
    })
}

fn main() -> Result<(), Error> {
    let base = env::var("MASTO_BASE").unwrap();
    let token = env::var("MASTO_TOKEN").unwrap();
    let live = env::var_os("NEWBOT_LIVE_MODE").is_some();
    let emojos: Vec<Emoji> = minreq::get(&format!("{}/api/v1/custom_emojis", base))
        .with_header("authorization", format!("Bearer {}", &token))
        .send()?
        .json()?;

    minreq::post(&format!("{}/api/v1/statuses", &base))
        .with_header("authorization", format!("Bearer {}", &token))
        .with_json(&draft_toot(&emojos, live)?)?
        .send()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_draft_toot() {
        let status = super::draft_toot("https://types.pl", false).unwrap();
        assert!(status.status.len() > 18);
    }
}
