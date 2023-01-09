use hyper::body::HttpBody;
use hyper::{Body, Response};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::error::Error;
use tokio::io::{stdout, AsyncWriteExt};

/// Deserializes the body stream of the response to the given type.
pub async fn deserialize<T>(resp: &mut Response<Body>) -> Result<T, Box<dyn Error>>
where
    for<'de> T: Deserialize<'de>,
{
    let mut data = vec![];
    while let Some(chunk) = resp.body_mut().data().await {
        data.extend(&chunk?);
    }
    let result: T = serde_json::from_slice(&data).expect("Unable to deserialize response");
    Ok(result)
}

/// Reads the body stream of the response and prints this to the console.
pub async fn print_response_body(resp: &mut Response<Body>) -> Result<(), Box<dyn Error>> {
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }
    Ok(())
}

pub trait PushUnique {
    /// Checks if the given string slice if not already present in the given vector and pushes it
    /// if it doesn't exist yet.
    fn push_unique(&mut self, val: &str) -> Self;
}

impl PushUnique for Vec<String> {
    /// Checks if the given string slice if not already present in the given vector and pushes it
    /// if it doesn't exist yet.
    fn push_unique(&mut self, val: &str) -> Self {
        // Check if we do not have this scope defined yet.
        if !self.iter().any(|perm| perm.eq(&val)) {
            self.push(val.to_string());
        }

        self.to_owned()
    }
}

/// Generic group by trait.
pub trait GroupBy<T> {
    /// Groups a generic BinaryTreeMap
    /// Determines which items can be grouped based on the given 'compare_fn'
    /// Assignes the grouped items to a key determined by the given 'key_selector'
    fn ordered_group_by<F, K, KT>(&self, key_selector: K, compare_fn: F) -> BTreeMap<&KT, Vec<&T>>
    where
        K: Fn(&T) -> &KT,
        F: Fn(&T, &T) -> bool,
        KT: Ord;
}

impl<T> GroupBy<T> for Vec<T> {
    /// Groups a generic BinaryTreeMap
    /// Determines which items can be grouped based on the given 'compare_fn'
    /// Assignes the grouped items to a key determined by the given 'key_selector'
    fn ordered_group_by<F, K, KT>(&self, key_selector: K, compare_fn: F) -> BTreeMap<&KT, Vec<&T>>
    where
        K: Fn(&T) -> &KT,
        F: Fn(&T, &T) -> bool,
        KT: Ord,
    {
        let mut result: BTreeMap<&KT, Vec<&T>> = BTreeMap::new();
        for row in self {
            let entry = result.entry(key_selector(row)).or_insert(vec![&row]);
            if !entry.into_iter().any(|items| compare_fn(items, row)) {
                entry.push(row);
            }
        }
        result
    }
}
