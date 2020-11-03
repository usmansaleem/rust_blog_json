use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Blog {
    blog_entries: Vec<BlogEntry>,
    next_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BlogEntry {
    id: u32,
    url_friendly_id: String,
    title: String,
    description: String,
    body: String,
    blog_section: String,
    created_on: String,
    modified_on: String,
}

fn main() -> Result<()> {
    println!("Reading Blog data from data.json");

    let blog = read_blog_items_from_file("data1.json")?;
    let blog_str = serde_json::to_string_pretty(&blog).unwrap();
    println!("{}", blog_str);

    println!("Length of blog items: {}", blog.blog_entries.len());
    println!("Next Blog Entry ID: {}", blog.next_id);

    //create new blog entry and attempt to insert it into Blog

    Ok(())
}

fn read_blog_items_from_file<P: AsRef<Path>>(path: P) -> Result<Blog> {
    // open file in read-only mode
    let file = File::open(path.as_ref())
        .with_context(|| format!("Unable to read file {}", path.as_ref().display()))?;
    let reader = BufReader::new(file);

    // read json from file as vector
    let blog: Blog = serde_json::from_reader(reader)?;

    Ok(blog)
}
