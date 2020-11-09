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
    pub fn read_from_path<P: AsRef<Path>>(path: P) -> Result<Blog> {
        // open file in read-only mode
        let file = File::open(path.as_ref())
            .with_context(|| format!("Unable to read file {}", path.as_ref().display()))?;

        Blog::read_from_file(&file)
    }

    fn read_from_file(file: &File) -> Result<Blog> {
        let reader = BufReader::new(file);

        // read json from file as vector
        let blog: Blog = serde_json::from_reader(reader)
            .with_context(|| format!("Error parsing file as JSON"))?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::io::{Seek, SeekFrom, Write};
    use tempfile::tempfile;

    static TEST_DATA: &str = r#"{
            "blogEntries": [
                {
                    "id": 2,
                    "urlFriendlyId": "tcp_client_using_vertx_kotlin_gradle",
                    "title": "TCP Client using Vertx, Kotlin and Gradle build",
                    "description": "Step by Step guide to create simple TCP client using Vertx, Kotlin and Gradle",
                    "body": "<p>As part of my hobby project to control RaspberryPi using Google Home Mini and/or Alexa, I wanted to write a very simple TCP client that keeps a connection open to one of my custom written server in cloud (I will write another blog post to cover the server side on a later date). The requirement of the client is to send a shared secret upon connecting and then keep waiting for message from server. Vert.x, Kotlin and Gradle allow rapid development of such project. The generated jar can be executed on Raspberry Pi. These steps outline the project setup and related source code to showcase a Vert.x and Kotlin project with Gradle.</p>\n<h2>Project Directory Structure</h2>\n<p>From command line (or via Windows Explorer, whatever you prefer to use) create a directory for project,for instance <code>vertx-net-client</code>. Since we are using Kotlin, we will place all Kotlin files in <code>src/main/kotlin</code> folder. The <code>src/main/resources</code> folder will contain our logging configuration related files.</p>\n<pre class=\"editor-colors lang-\"><div class=\"line\"><span class=\"syntax--text syntax--plain syntax--null-grammar\"><span>cd&nbsp;vertx-net-client</span></span></div><div class=\"line\"><span class=\"syntax--text syntax--plain syntax--null-grammar\"><span>mkdir&nbsp;-p&nbsp;src/main/kotlin</span></span></div><div class=\"line\"><span class=\"syntax--text syntax--plain syntax--null-grammar\"><span>mkdir&nbsp;-p&nbsp;src/main/resources</span></span></div></pre><h3 id=\"project-files\">Project Files</h3>\n<p>We need to add following files in the project</p>\n<ul>\n<li><code>.gitignore</code>\nIf you want to check your project into git, you may consider adding following <code>.gitignore</code> file at root of your project\n</li>\n</ul>\n<script src=\"https://gist.github.com/usmansaleem/b5838484a20cb8b08f236f2265ad7a8e.js\"></script>\n\n<ul>\n<li><code>logback.xml</code>\nThis example is using slf4j and logback for logging. If you decide to use it in your project, you may also add following logback.xml file in <code>src/main/resources</code>. Modify it as per your requirements. This example will\nlog on console.\n</li>\n</ul>\n<script src=\"https://gist.github.com/usmansaleem/750c6d1cad0721b52be2ff00f758fb9f.js\"></script>\n\n<h2>Gradle Setup</h2>\n<p>We will use Gradle build system for this project. If you don’t already have Gradle available on your system, download and unzip gradle in a directory of your choice (<code>$GRADLE_HOME</code> is used here to represent this directory). This gradle distribution will be used as a starting point to create Gradle wrapper scripts for our project. These scripts will allow our project to download and use correct version of gradle distribution automatically without messing up system. Really useful when building your project on CI tool or on any other developer's machine.</p>\n<p>Run following command in project's directory</p>\n<pre class=\"editor-colors lang-\"><div class=\"line\"><span class=\"syntax--text syntax--plain syntax--null-grammar\"><span>$GRADLE_HOME/bin/gradle&nbsp;wrapper</span></span></div></pre><p>The above commands will generate following files and directories.</p>\n<pre class=\"editor-colors lang-\"><div class=\"line\"><span class=\"syntax--text syntax--plain syntax--null-grammar\"><span>gradle/&nbsp;&nbsp;gradlew&nbsp;&nbsp;gradlew.bat</span></span></div></pre><h3 id=\"gradle-build-file-build-gradle-\">Gradle build file <code>build.gradle</code></h3>\n<p>Create (and/or copy and modify) following <code>build.gradle</code> in your project's root directory. Our example gradle build file is using <a href=\"https://github.com/jponge/vertx-gradle-plugin/\">vertx-gradle-plugin</a>.\n</p>\n<script src=\"https://gist.github.com/usmansaleem/e723f25b827e0a925eaef2957a80132d.js\"></script>\n<p>In the project directory, run following command to download local gradle distribution:</p>\n<pre class=\"editor-colors lang-\"><div class=\"line\"><span class=\"syntax--text syntax--plain syntax--null-grammar\"><span>./gradlew</span></span></div></pre><p>(or <code>.\\gradlew.bat</code> if in Windows)</p>\n<p>At this stage we should have following file structure. This is also a good time to commit changes if you are working with git.</p>\n<ul>\n<li><code>.gitignore</code>                              </li>\n<li><code>build.gradle</code>                            </li>\n<li><code>gradle/wrapper/gradle-wrapper.jar</code>       </li>\n<li><code>gradle/wrapper/gradle-wrapper.properties</code></li>\n<li><code>gradlew</code>                                 </li>\n<li><code>gradlew.bat</code></li>\n<li><code>src/main/resources/logback.xml</code></li>\n</ul>\n<p>Now that our project structure is ready, time to add the meat of the project. You may use any IDE of your choice. My preference is IntelliJ IDEA.</p>\n<p>Create a new package under <code>src/main/kotlin</code>. The package name should be adapted from the following section of <code>build.gradle</code></p>\n<pre class=\"editor-colors lang-\"><div class=\"line\"><span class=\"syntax--text syntax--plain syntax--null-grammar\"><span>vertx&nbsp;{</span></span></div><div class=\"line\"><span class=\"syntax--text syntax--plain syntax--null-grammar\"><span>&nbsp;&nbsp;&nbsp;&nbsp;mainVerticle&nbsp;=&nbsp;\"info.usmans.blog.vertx.NetClientVerticle\"</span></span></div><div class=\"line\"><span class=\"syntax--text syntax--plain syntax--null-grammar\"><span>}</span></span></div></pre><p>From the above example, the package name is <code>info.usmans.blog.vertx</code></p>\n<p>Add a new Kotlin Class/file in <code>src/main/kotlin/info/usmans/blog/vertx</code> as <code>NetClientVerticle.kt</code></p>\n<p>The contents of this class is as follows</p>\n<script src=\"https://gist.github.com/usmansaleem/2a176a7b752fcb72f7f31964809696fe.js\"></script>\n\n<h2>Explaining the Code</h2>\n<p>The <code>fun main(args: Array&lt;String&gt;)</code> is not strictly required, it quickly allows running the Vert.x verticle from within IDE. You will also notice a small hack in the method for setting system property <code>vertx.disableDnsResolver</code> which is to avoid a Netty bug that I observed when running on Windows machine and remote server is down. Of course, since we are using vertx-gradle-plugin, we can also use <code>gradle vertxRun</code> to run our verticle. In this case the <code>main</code> method will not get called.</p>\n<p>The <code>override fun start()</code> method calls <code>fireReconnectTimer</code> which in turn calls <code>reconnect</code> method. <code>reconnect</code> method contains the connection logic to server as well as it calls <code>fireReconnectTimer</code> if it is unable to connect to server or disconnects from server. In <code>reconnect</code> method the <code>socket.handler</code> gets called when server send message to client.</p>\n<pre class=\"editor-colors lang-\"><div class=\"line\"><span class=\"syntax--text syntax--plain syntax--null-grammar\"><span>socket.handler({&nbsp;data&nbsp;-&gt;</span></span></div><div class=\"line\"><span class=\"syntax--text syntax--plain syntax--null-grammar\"><span>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;logger.info(\"Data&nbsp;received:&nbsp;${data}\")</span></span></div><div class=\"line\"><span class=\"syntax--text syntax--plain syntax--null-grammar\"><span>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;//</span><span class=\"syntax--storage syntax--type syntax--class syntax--todo\"><span>TODO</span></span><span>:&nbsp;Do&nbsp;the&nbsp;work&nbsp;here&nbsp;...</span></span></div><div class=\"line\"><span class=\"syntax--text syntax--plain syntax--null-grammar\"><span>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;})</span></span></div></pre>\n\n<h2 id=\"distributing-the-project\">Distributing the project</h2>\n<p>To create redistributable jar, use <code>./gradlew shadowJar</code> command. Or if using IntelliJ: from Gradle projects, Tasks, shadow, shadowJar (right click run). This command will generate <code>./build/libs/vertx-net-client-fat.jar</code>.</p>\n<h3 id=\"executing-the-client\">Executing the client</h3>\n<p>The client jar can be executed using following command:</p>\n<pre class=\"editor-colors lang-\"><div class=\"line\"><span class=\"syntax--text syntax--plain syntax--null-grammar\"><span>&nbsp;java&nbsp;-DserverHost=127.0.0.1&nbsp;-DserverPort=8888&nbsp;-DconnectMessage=\"hello\"&nbsp;-jar&nbsp;vertx-net-client-full.jar</span></span></div></pre><p>If you wish to use SLF4J for Vert.x internal logging, you need to pass system property <code>vertx.logger-delegate-factory-class-name</code> with value of <code>io.vertx.core.logging.SLF4JLogDelegateFactory</code>. The final command would look like:</p>\n<pre class=\"editor-colors lang-\"><div class=\"line\"><span class=\"syntax--text syntax--plain syntax--null-grammar\"><span>java&nbsp;-DserverHost=127.0.0.1&nbsp;-DserverPort=8888&nbsp;-DconnectMessage=\"hello\"&nbsp;-Dvertx.logger-delegate-factory-class-name=\"io.vertx.core.logging.SLF4JLogDelegateFactory\"&nbsp;-jar&nbsp;vertx-net-client-full.jar</span></span></div></pre><p>You can configure Vert.x logging levels in logback.xml file if required.</p>\n<h2 id=\"conclusion\">Conclusion</h2>\n<p>This post describes how easy it is to create a simple TCP client using Vert.x, Kotlin and Gradle build system. Hopefully the techniques shown here will serve as a starting point for your next DIY project.</p>",
                    "blogSection": "Main",
                    "createdOn": "2017-12-08",
                    "modifiedOn": "2017-12-14",
                    "categories": [
                      {
                        "name": "Vertx"
                      },
                      {
                        "name": "Kotlin"
                      },
                      {
                        "name": "Gradle"
                      }
                    ]
                },
                {
                    "id": 1,
                    "urlFriendlyId": "first_post_finally",
                    "title": "First Post, finally",
                    "description": "Read about how to First Post, finally",
                    "body": "<div style=\"clear:both;\"></div>This is the first post I am making on blogger....I hope this will grow as time passes...<div style=\"clear:both; padding-bottom:0.25em\"></div>",
                    "blogSection": "Main",
                    "createdOn": "2005-06-09",
                    "modifiedOn": "2005-06-09",
                    "categories": [
                      {
                        "name": "General"
                      }
                    ]
                 }
            ],
            "nextId": 3
        }"#;

    #[test]
    fn blog_is_loaded() -> Result<()> {
        // create temp file with raw json
        let mut file = tempfile()?;
        file.write_all(TEST_DATA.as_bytes())?;
        file.seek(SeekFrom::Start(0))?;

        let blog = Blog::read_from_file(&file)?;
        assert_eq!(blog.next_id, 3);
        assert_eq!(blog.blog_entries.len(), 2);

        Ok(())
    }

    #[test]
    fn can_find_by_friendly_url() -> Result<()> {
        // create temp file with raw json
        let mut file = tempfile()?;
        file.write_all(TEST_DATA.as_bytes())?;
        file.seek(SeekFrom::Start(0))?;

        let blog = Blog::read_from_file(&file)?;
        assert!(blog.find_by_url("first_post_finally").is_some());
        assert!(blog.find_by_url("non_existent_post").is_none());
        assert!(blog
            .find_by_url("tcp_client_using_vertx_kotlin_gradle")
            .is_some());

        Ok(())
    }

    #[test]
    fn can_delete_by_id() -> Result<()> {
        // create temp file with raw json
        let mut file = tempfile()?;
        file.write_all(TEST_DATA.as_bytes())?;
        file.seek(SeekFrom::Start(0))?;

        let mut blog = Blog::read_from_file(&file)?;
        blog.delete_blog_entry(1);

        assert_eq!(blog.blog_entries.len(), 1);
        assert!(blog.find_by_url("first_post_finally").is_none());

        Ok(())
    }

    #[test]
    fn can_add_new_blog_entry() -> Result<()> {
        // create temp file with raw json
        let mut file = tempfile()?;
        file.write_all(TEST_DATA.as_bytes())?;
        file.seek(SeekFrom::Start(0))?;

        let mut blog = Blog::read_from_file(&file)?;
        assert_eq!(2, blog.blog_entries.len(), "Unexpected Blog Entries length");
        assert_eq!(3, blog.next_id, "Unexpected Next Id");

        blog.new_blog_entry(
            "new_test".to_string(),
            "Rust Test".to_string(),
            "Testing Blog in Rust".to_string(),
            "Testing Blog in Rust".to_string(),
            vec![BlogCategory::new("Rust".to_string())],
        );

        assert_eq!(3, blog.blog_entries.len(), "Unexpected Blog Entries length");
        assert_eq!(4, blog.next_id, "Unexpected Next Id");

        match blog.find_by_url("new_test") {
            None => {
                assert!(false, "Unable to find new entry by URL");
            }
            Some(entry) => {
                assert_eq!(entry.id, 3);
                assert_eq!(entry.title, "Rust Test");
            }
        }

        Ok(())
    }
}
