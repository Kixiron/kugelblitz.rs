
# Building

### Windows

I found that having `sqlite3.lib` in the root made compiling succeed, so that's what I did.  
Download Sqlite [here](https://sqlite.org/download.html), extract and run `link /lib /def:sqlite3.def /MACHINE:X64` with the Visual Studio build tools, then move `sqlite3.lib` into the project directory.  
If you have trouble installing `diesel_cli` ~~like I did~~, follow these [helpful instructions.](https://github.com/diesel-rs/diesel/issues/487)

# Configuration

Most configuration is done on startup, using variables set in a `.env` file next to the executable.  
Your file should follow this format

```env
# The bots token from https://discordapp.com/developers/
DISCORD_BOT_TOKEN=BOT_TOKEN

# A comma-delimited list of Owner Ids
DISCORD_BOT_OWNERS=1234567890123456,234567890123457,3456789012345

# A comma-delimited list of prefixes
DISCORD_BOT_PREFIXES=?,!

# The file to record bot logs in (Optional, but should be a .log file)
DISCORD_BOT_LOGGING_DESTINATION=bot_log.log

# Set the bots' logging level (Defaults to Info)
# Options (Increasing in Severity): Off, Trace, Debug, Info, Warn, Error (Note: Off will completely disable all output)
DISCORD_BOT_LOGGING_LEVEL=Info

# The database to connect to, should be a valid Sqlite3 database
DATABASE_URL=database.sqlite
```