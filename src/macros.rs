#[macro_export]
macro_rules! save_all {
    ($data:ident) => {
        // Save the markov chain
        if let Some(markov) = $data.get::<$crate::inserts::MarkovKey>() {
            match markov.read() {
                Ok(markov) => {
                    if let Err(err) = markov.save() {
                        error!("Markov Save Error: {:?}", err);
                    }
                }
                Err(err) => error!("Markov Read Error: {:?}", err),
            }
        }
    };
}
