use super::*;

pub fn validate_query_data(results: &Vec<Result<Vec<Log>, ProviderError>>) -> bool {
    for result in results.iter() {
        if let Err(err) = result {
            if err
                .to_string()
                .contains("query returned more than 10000 results")
            {
                println!(
                    "too many results. re-trying query with decreasing block range until success"
                );
                return false;
            } else {
                panic!("historical event query error: {}", err);
            }
        }
    }
    return true;
}

pub fn build_comptroller_filters(
    comptroller_events: &Vec<&str>,
    comptroller_address: Address,
    from_block: u64,
    to_block: u64,
) -> Vec<Filter> {
    comptroller_events
        .iter()
        .map(|event_signature| {
            Filter::new()
                .address(comptroller_address)
                .event(event_signature)
                .from_block(from_block)
                .to_block(to_block)
        })
        .collect()
}

// turns Vec<Result<Vec<Log>, Provider Error>> into Vec<Log>
pub fn flatten_into_logs(results: Vec<Result<Vec<Log>, ProviderError>>) -> Vec<Log> {
    results
        .into_iter()
        .filter_map(|result| result.ok())
        .flatten()
        .collect()
}

pub fn build_ctoken_filters(
    ctoken_events: &Vec<&str>,
    ctoken_addresses: Vec<&Address>,
    from_block: u64,
    to_block: u64,
) -> Vec<Filter> {
    ctoken_addresses
        .iter()
        .flat_map(|ctoken_address| {
            ctoken_events.iter().map(move |event_signature| {
                Filter::new()
                    .address(**ctoken_address)
                    .event(event_signature)
                    .from_block(from_block)
                    .to_block(to_block)
            })
        })
        .collect()
}
