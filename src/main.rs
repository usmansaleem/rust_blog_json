use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Blog {
    blog_entries: Vec<BlogEntry>,
    next_id: u32,
}

impl Blog {
    fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Blog> {
        // open file in read-only mode
        let file = File::open(path.as_ref())
            .with_context(|| format!("Unable to read file {}", path.as_ref().display()))?;
        let reader = BufReader::new(file);

        // read json from file as vector
        let blog: Blog = serde_json::from_reader(reader)?;

        Ok(blog)
    }

    fn new_blog_entry(
        &mut self,
        url_friendly_id: String,
        title: String,
        description: String,
        body: String,
        categories: Vec<BlogCategory>,
    ) {
        let nid = self.next_id;
        self.next_id = self.next_id + 1;
        let create_date = Utc::now().format("%Y-%m-%d").to_string();
        let modify_date = create_date.clone();
        let blog_entry = BlogEntry {
            id: nid,
            url_friendly_id,
            title,
            description,
            body,
            blog_section: "Main".to_string(),
            created_on: create_date,
            modified_on: modify_date,
            categories,
        };

        // insert into vec (ownership transferred)
        self.blog_entries.insert(0, blog_entry);
    }

    fn delete_blog_entry(&mut self, id: u32) {
        self.blog_entries.retain(|entry| entry.id != id);
    }

    fn find_by_url(&self, url_friendly_id: &String) -> Option<&BlogEntry> {
        self.blog_entries
            .iter()
            .find(|entry| entry.url_friendly_id == *url_friendly_id)
    }
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
    categories: Vec<BlogCategory>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BlogCategory {
    name: String,
}

impl BlogCategory {
    fn new(name: String) -> BlogCategory {
        BlogCategory { name }
    }
}

fn main() -> Result<()> {
    println!("Reading Blog data from data.json");

    let mut blog = Blog::read_from_file("data.json")?;

    println!("Length of blog items: {}", blog.blog_entries.len());
    println!("Next Blog Entry ID: {}", blog.next_id);

    match blog.find_by_url(&"wrapper_over_runas_utility".to_string()) {
        Some(entry) => println!("{:#?}", entry),
        _ => println!("Blog Entry Not Found"),
    }

    // insert a new blog entry in blog
    println!("Inserting new Blog Entry");
    blog.new_blog_entry(
        "new_test".to_string(),
        "Rust Test".to_string(),
        "Testing Blog in Rust".to_string(),
        "Testing Blog in Rust".to_string(),
        vec![BlogCategory::new("Rust".to_string())],
    );

    println!("Length of blog items: {}", blog.blog_entries.len());
    println!("Next Blog Entry ID: {}", blog.next_id);

    match blog.find_by_url(&"new_test".to_string()) {
        Some(entry) => println!("{:#?}", entry),
        _ => println!("Blog Entry Not Found"),
    }

    // delete blog entry 2
    println!("Deleting Blog Entry 2");
    blog.delete_blog_entry(2);
    println!("Length of blog items: {}", blog.blog_entries.len());
    println!("Next Blog Entry ID: {}", blog.next_id);

    match blog.find_by_url(&"wrapper_over_runas_utility".to_string()) {
        Some(entry) => println!("{:#?}", entry),
        _ => println!("Blog Entry Not Found"),
    }

    Ok(())
}
