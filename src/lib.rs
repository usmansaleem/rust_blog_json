use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Blog {
    pub blog_entries: Vec<BlogEntry>,
    pub next_id: u32,
}

impl Blog {
    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Blog> {
        // open file in read-only mode
        let file = File::open(path.as_ref())
            .with_context(|| format!("Unable to read file {}", path.as_ref().display()))?;
        let reader = BufReader::new(file);

        // read json from file as vector
        let blog: Blog = serde_json::from_reader(reader)
            .with_context(|| format!("Error parsing as JSON {}", path.as_ref().display()))?;

        Ok(blog)
    }

    pub fn new_blog_entry(
        &mut self,
        url_friendly_id: String,
        title: String,
        description: String,
        body: String,
        categories: Vec<BlogCategory>,
    ) {
        let blog_entry = BlogEntry::new(
            self.next_id,
            url_friendly_id,
            title,
            description,
            body,
            categories,
        );
        self.next_id += 1;

        // insert into vec (ownership transferred)
        self.blog_entries.insert(0, blog_entry);
    }

    pub fn delete_blog_entry(&mut self, id: u32) {
        self.blog_entries.retain(|entry| entry.id != id);
    }

    pub fn find_by_url(&self, url_friendly_id: &str) -> Option<&BlogEntry> {
        self.blog_entries
            .iter()
            .find(|entry| entry.url_friendly_id == *url_friendly_id)
    }

    // TODO: Save blog back to file
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlogEntry {
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

impl BlogEntry {
    fn new(
        id: u32,
        url_friendly_id: String,
        title: String,
        description: String,
        body: String,
        categories: Vec<BlogCategory>,
    ) -> BlogEntry {
        BlogEntry {
            id,
            url_friendly_id,
            title,
            description,
            body,
            blog_section: "Main".to_string(),
            created_on: Utc::now().format("%Y-%m-%d").to_string(),
            modified_on: Utc::now().format("%Y-%m-%d").to_string(),
            categories,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlogCategory {
    name: String,
}

impl BlogCategory {
    pub fn new(name: String) -> BlogCategory {
        BlogCategory { name }
    }
}
