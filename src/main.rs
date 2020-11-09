use anyhow::Result;
use blog_json::Blog;

fn main() -> Result<()> {
    println!("Reading Blog data from data.json");

    let blog = Blog::read_from_path("data.json")?;

    println!("Length of blog items: {}", blog.blog_entries.len());
    println!("Next Blog Entry ID: {}", blog.next_id);

    Ok(())
}
