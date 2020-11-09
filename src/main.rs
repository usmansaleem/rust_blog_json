use anyhow::Result;
use blog_json::{Blog, BlogCategory};

fn main() -> Result<()> {
    println!("Reading Blog data from data.json");

    let mut blog = Blog::read_from_file("data.json")?;

    println!("Length of blog items: {}", blog.blog_entries.len());
    println!("Next Blog Entry ID: {}", blog.next_id);

    if let Some(entry) = blog.find_by_url("wrapper_over_runas_utility") {
        println!("{:#?}", entry)
    } else {
        println!("Unexpected behavior - wrapper_over_runas_utility not found");
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

    if let Some(entry) = blog.find_by_url("new_test") {
        println!("{:#?}", entry)
    } else {
        println!("Unexpected behavior - new_test not found");
    }

    // delete blog entry 2
    println!("Deleting Blog Entry 2");
    blog.delete_blog_entry(2);
    println!("Length of blog items: {}", blog.blog_entries.len());
    println!("Next Blog Entry ID: {}", blog.next_id);

    match blog.find_by_url("wrapper_over_runas_utility") {
        Some(entry) => println!("Unexpected - should have been deleted. {:#?}", entry),
        _ => println!("Blog Entry Not Found"),
    }

    Ok(())
}
