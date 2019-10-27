use log::error;
use serenity::{
    builder::CreateEmbed,
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::{channel::Message, guild::Member, user::User},
    utils::MessageBuilder,
};

group!({
    name: "general",
    options: {},
    commands: [playground, markov, user]
});

// TODO: Options
// TODO: Output vetting w/ gist
// TODO: Prettier output w/ error detection
// TODO: Add better user-side errors
// TODO: Add more languages to be evaluated using online evaluators
// Planned languages: Lua, Haskell, Brainfuck, Bash, Perl, C, C++, Ocaml,
// C#, ASM, Ruby, Go, Fortran, Pascal, Lisp, Java, Erlang, Swift, Objective-C, Scala,
// PHP, Node.js, Ada, M4, LLVM IR, Koltin, F#, Elixir,  and Nim
// Use https://tio.run / https://github.com/TryItOnline/tryitonline / https://github.com/TryItOnline/TioSetup / https://github.com/TryItOnline/tiodocker

#[command]
#[description("A Rust playground")]
#[usage("\\`\\`\\`<language>\n<code>\n\\`\\`\\`")]
#[aliases("eval")]
#[sub_commands(lang)]
#[bucket = "playground"]
fn playground(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    use super::playground::{self, EvalType};

    let (code, eval_type) = match args.rest().trim() {
        // Rust
        code if code.starts_with("```rust") && code.ends_with("```") => (
            code.trim_start_matches("```rust").trim_end_matches("```"),
            EvalType::Rust,
        ),
        code if code.starts_with("```rs") && code.ends_with("```") => (
            code.trim_start_matches("```rs").trim_end_matches("```"),
            EvalType::Rust,
        ),

        // Python
        code if code.starts_with("```python") && code.ends_with("```") => (
            code.trim_start_matches("```python").trim_end_matches("```"),
            EvalType::Python,
        ),
        code if code.starts_with("```py") && code.ends_with("```") => (
            code.trim_start_matches("```py").trim_end_matches("```"),
            EvalType::Python,
        ),

        // Invalid codeblock
        _ => {
            let _ = msg.channel_id.say(
                &ctx.http,
                "Invalid codeblock, try formatting your code as follows:\n\\`\\`\\`<language>\n<code>\n\\`\\`\\`",
            );
            return Ok(());
        }
    };

    let result = {
        // Start typing
        let _ = ctx.http.broadcast_typing(*msg.channel_id.as_u64());

        let data = ctx.data.read();

        match eval_type {
            EvalType::Rust => playground::eval_rust(&data, code),
            EvalType::Python => playground::eval_python(&data, code),
        }
    };

    match result {
        Ok(result) => {
            let _ = msg.channel_id.say(&ctx.http, &result);
        }
        Err(err) => {
            let _ = msg.channel_id.say(&ctx.http, &err);
        }
    }

    Ok(())
}

// TODO: This output kinda sucks, make it not. Probably should ask someone smart or something
#[command]
#[description("See the available languages for the playground")]
fn lang(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.say(
        &ctx.http,
        "\
         Languages:\n    \
         Rust: `rust`, `rs`\n    \
         Python: `python`, `py`\n    \
         ",
    );

    Ok(())
}

#[command]
#[usage("<string>")]
fn markov(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    use crate::inserts::MarkovKey;

    let data = ctx.data.read();

    if let Some(markov) = data.get::<MarkovKey>() {
        match markov.read() {
            Ok(markov) => {
                if args.is_empty() {
                    let chain = (*markov).generate_str();

                    let _ = msg.channel_id.say(&ctx.http, &chain);
                } else {
                    let chain = (*markov).generate_str_from_token(args.rest());

                    let _ = msg.channel_id.say(&ctx.http, &chain);
                }

                return Ok(());
            }
            Err(err) => error!("Markov Read Error: {:?}", err),
        }
    }

    let _ = msg.channel_id.say(
        &ctx.http,
        "I'm sorry, I had trouble with that, try again later",
    );

    Ok(())
}

#[command]
#[description("Get user information")]
#[usage("<user>")]
fn user(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let user = if args.is_empty() {
        &msg.author
    } else {
        &msg.mentions[0]
    };

    let member = if let Some(guild_id) = msg.guild_id {
        ctx.http.get_member(*guild_id.as_u64(), *user.id.as_u64())
    } else {
        Err(serenity::Error::Other("User not in guild"))
    };

    let embed = {
        let mut embed = CreateEmbed::default();
        if let Ok(member) = member {
            member_info(ctx, &mut embed, user, &member)
        } else {
            user_info(&mut embed, user)
        }
    };

    let _ = msg
        .channel_id
        .send_message(&ctx.http, move |message| message.embed(move |mut e| {
            e = embed;
            e
        }));

    Ok(())
}

fn user_info<'a>(embed: &'a mut CreateEmbed, user: &User) -> &'a mut CreateEmbed {
    embed
}

fn member_info<'a>(
    ctx: &mut Context,
    embed: &'a mut CreateEmbed,
    user: &User,
    member: &Member,
) -> &'a mut CreateEmbed {
    embed.author(|author| {
        author
            .name({
                if let Some(nickname) = &member.nick {
                    MessageBuilder::new()
                        .push_safe(&nickname)
                        .push(" (")
                        .push_safe(&user.name)
                        .push(')')
                        .build()
                } else {
                    user.name.clone()
                }
            })
            .icon_url({
                if let Some(profile_picture) = user.avatar_url() {
                    profile_picture
                } else {
                    user.default_avatar_url()
                }
            })
    });

    if let Ok(permissions) = member.permissions(&ctx.cache) {
        let permissions = format!("{:?}", permissions);

        for permission in permissions.split(' ').take(5) {}
    }

    embed
}
