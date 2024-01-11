mod address_query;
mod asset_query;
mod case_query;
mod network_query;
mod reporter_query;

pub(super) fn replacer<V: ToString>(v: &V) -> String {
    v.to_string()
        .replace("\"", "")
        .replace("_", "")
        .to_lowercase()
}
