use crate::commands::command_prelude::*;
use chrono::{Duration, Utc};
use regex::Regex;
use white_rabbit::DateResult;

lazy_static::lazy_static! {
    static ref TIME_REGEX: Regex = Regex::new("(\\d+)([dhms])").expect("Failed to compile time regex");
}

#[command]
#[description = "Set a reminder"]
#[usage = "<%d%h%m%s> <message>"]
#[example = "1h15m Get your hot pocket"]
#[min_args(2)]
pub fn remind(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let time = {
        let first = args.single::<String>()?;
        let error = || {
            msg.channel_id.say(
                &ctx,
                "Incorrect date format, make sure it's in the following format: `0d0h0m0s`",
            )?;

            Err(CommandError("Incorrect reminder timestamp".to_string()))
        };

        let mut time = Duration::seconds(0);
        let (mut days, mut hours, mut minutes, mut seconds) = (false, false, false, false);
        for segment in (*TIME_REGEX).find_iter(&first) {
            let mut segment = segment.as_str().chars().collect::<Vec<_>>();

            match segment[segment.len() - 1] {
                'd' => {
                    if !days {
                        days = true;

                        segment.pop();
                        let mut num_days = if let Ok(int) =
                            segment.into_iter().collect::<String>().parse::<i64>()
                        {
                            int
                        } else {
                            (error)()?
                        };

                        if num_days > 7 {
                            num_days = 7;
                        }

                        time = time + Duration::days(num_days);
                    }
                }
                'h' => {
                    if !hours {
                        hours = true;

                        segment.pop();
                        let mut num_hours = if let Ok(int) =
                            segment.into_iter().collect::<String>().parse::<i64>()
                        {
                            int
                        } else {
                            (error)()?
                        };

                        if num_hours > 24 * 7 {
                            num_hours = 24 * 7;
                        }

                        time = time + Duration::hours(num_hours);
                    }
                }
                'm' => {
                    if !minutes {
                        minutes = true;

                        segment.pop();
                        let mut num_minutes = if let Ok(int) =
                            segment.into_iter().collect::<String>().parse::<i64>()
                        {
                            int
                        } else {
                            (error)()?
                        };

                        if num_minutes > 60 * 24 * 2 {
                            num_minutes = 60 * 24 * 2;
                        }

                        time = time + Duration::minutes(num_minutes);
                    }
                }
                's' => {
                    if !seconds {
                        seconds = true;

                        segment.pop();
                        let mut num_seconds = if let Ok(int) =
                            segment.into_iter().collect::<String>().parse::<i64>()
                        {
                            int
                        } else {
                            (error)()?
                        };

                        if num_seconds > 60 * 60 {
                            num_seconds = 60 * 60;
                        }

                        time = time + Duration::seconds(num_seconds);
                    }
                }
                _ => {
                    (error)()?;
                }
            }
        }

        time
    };
    if time == Duration::seconds(0) {
        msg.channel_id
            .say(&ctx, "You must specify a valid time for a reminder")?;
        return Err(CommandError("No reminder duration".to_string()));
    }

    let args = args.rest().to_string();

    let confirmation = MessageBuilder::new()
        .push(format!(
            "I'll remind you in {}: ",
            vec![
                if time.num_days() > 0 {
                    format!("{} days", time.num_days())
                } else {
                    String::new()
                },
                if time.num_hours() > 0 {
                    format!("{} hours", time.num_hours())
                } else {
                    String::new()
                },
                if time.num_minutes() > 0 {
                    format!("{} minutes", time.num_minutes())
                } else {
                    String::new()
                },
                if time.num_seconds() > 0 {
                    format!("{} seconds", time.num_seconds())
                } else {
                    String::new()
                },
            ]
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(", ")
        ))
        .push_mono_line_safe(&args)
        .build();

    msg.channel_id.say(&ctx, confirmation)?;

    let scheduler = {
        let mut ctx = ctx.data.write();
        match ctx.get_mut::<crate::data::SchedulerContainer>() {
            Some(scheduler) => Ok(scheduler.clone()),
            None => Err(CommandError("Failed to get scheduler".to_string())),
        }?
    };

    let http = ctx.http.clone();
    let msg = msg.clone();

    let mut attempts = 0;
    let name = if let Some(nick) = msg.author_nick(&ctx) {
        nick
    } else {
        msg.author.name.clone()
    };
    let color = {
        let default = 0x5125B5.into();

        if let Some(member) = msg.member(&ctx) {
            if let Some(color) = member.colour(&ctx) {
                color
            } else {
                default
            }
        } else {
            default
        }
    };

    msg.react(&ctx, "✅")?;

    let mut scheduler = scheduler.write();
    scheduler.add_task_duration(time, move |_| {
        if let Err(err) = msg.channel_id.send_message(&http, |m| {
            m.content(MessageBuilder::new().mention(&msg.author).build())
                .embed(|e| {
                    e.author(|a| {
                        a.icon_url(utils::avatar_url(&msg.author)).name(
                            MessageBuilder::new()
                                .push("Reminder for ")
                                .push_safe(&name)
                                .build(),
                        );

                        if let Ok(url) = utils::message_link(&msg) {
                            a.url(url);
                        }

                        a
                    })
                    .description(&args)
                    .color(color);

                    e
                });

            m
        }) {
            error!("Error sending message: {:?}.", err);

            if attempts > 5 {
                DateResult::Done
            } else {
                attempts += 1;
                DateResult::Repeat(Utc::now() + Duration::milliseconds(5000))
            }
        } else {
            DateResult::Done
        }
    });

    Ok(())
}
