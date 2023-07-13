#[macro_export]
macro_rules! render_and_index_scraped_data {
    ($type: ty, $source: expr, $target: literal, $additional: expr, $index: ident) => {
        match render_scraped::<$type, _, _>(&$source, concat!("output/", $target), $additional) {
            Ok(rendered) => {
                if let Some(index) = &$index {
                    if let Err(e) = index
                        .add_or_replace(&rendered.iter().cloned().map(|(_, page)| page).collect_vec(), Some("id"))
                        .await
                    {
                        eprintln!("Could not update meilisearch index: {:?}", e);
                    }
                }
                println!(concat!("Successfully rendered ", $target, " folder"));
                rendered
            }
            Err(e) => {
                eprintln!(concat!("Error while rendering ", $target, " folder : {}"), e);
                FAILED_COMPENDIA.fetch_add(1, Ordering::SeqCst);
                vec![]
            }
        }
    };
}

#[macro_export]
macro_rules! render_and_index {
    ($type: ty, $source: expr, $target: literal, $additional: expr, $index: ident) => {
        match render::<$type, _, _>(&$source, concat!("output/", $target), $additional) {
            Ok(rendered) => {
                if let Some(index) = &$index {
                    if let Err(e) = index
                        .add_or_replace(&rendered.iter().cloned().map(|(_, page)| page).collect_vec(), Some("id"))
                        .await
                    {
                        eprintln!("Could not update meilisearch index: {:?}", e);
                    }
                }
                println!(concat!("Successfully rendered ", $target, " folder"));
                rendered
            }
            Err(e) => {
                eprintln!(concat!("Error while rendering ", $target, " folder : {}"), e);
                FAILED_COMPENDIA.fetch_add(1, Ordering::SeqCst);
                vec![]
            }
        }
    };
}
