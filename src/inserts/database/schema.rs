table! {
    command_usage (cmd_name) {
        cmd_name -> Text,
        cmd_usages -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(command_usage,);
