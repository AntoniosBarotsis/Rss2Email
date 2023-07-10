//! [`EmailProvider`] implementation using [`Resend`](https://resend.com/).

use resend_rs::{mail::Mail, resend_client::ResendClient};

use crate::info;

use super::{email_provider::EmailProvider, error::EmailError, EnvLoader};

#[derive(Default, Debug)]
pub struct Resend {
  api_key: Option<String>,
}

impl Resend {
  pub(crate) fn new(env_loader: &EnvLoader) -> Self {
    Self {
      api_key: env_loader.api_key.clone(),
    }
  }
}

impl EmailProvider for Resend {
  fn send_email(&self, address: &str, contents: &str) -> Result<(), EmailError> {
    let api_key = self
      .api_key
      .as_ref()
      .ok_or_else(|| EmailError::Config("Cannot use Resend without API_KEY".to_owned()))
      .cloned()?;

    // dbg!(contents);
    // let contents = r#"<h1>rss2email - 2023-07-10</h1><h2>bytebytego</h2><ul><li><a href=\"https://www.youtube.com/watch?v=uu32ggf-dwg\">demystifying the unusual evolution of the netflix api architecture</a></li></ul><h2>fireship</h2><ul><li><a href=\"https://www.youtube.com/watch?v=p6yw0bx5dbw\">chatgpt just leveled up big time...</a></li><li><a href=\"https://www.youtube.com/watch?v=r-gsgh2rxjs\">htmx in 100 seconds</a></li><li><a href=\"https://www.youtube.com/watch?v=3as5x05xiyg\">the sad truth about twitter's rate limit</a></li></ul><h2>rust</h2><ul><li><a href=\"https://www.youtube.com/watch?v=1saeu32agya\">rust zurisee live stream 2023-07-03</a></li></ul><h2>no boilerplate</h2><ul><li><a href=\"https://www.youtube.com/watch?v=dbsaqsikqxk\">hack your brain with obsidian.md</a></li></ul><h2>dreams of code</h2><ul><li><a href=\"https://www.youtube.com/watch?v=_qkgo8bphwc\">these are the weirdest http status codes</a></li></ul><h2>rust blog</h2><ul><li><a href=\"https://blog.rust-lang.org/2023/07/05/regex-1.9.html\">announcing regex 1.9</a></li></ul><h2>awesome rust weekly</h2><ul><li><a href=\"https://rust.libhunt.com/newsletter/367\">💻 issue 367 - we've discussed the name squatting situation in our team meetings over the past weeks and concluded that it might be time for a crates.io policy update</a></li></ul><h2>this week in rust</h2><ul><li><a href=\"https://this-week-in-rust.org/blog/2023/07/05/this-week-in-rust-502/\">this week in rust 502</a></li></ul><h2>andrew gallant's blog on andrew gallant's blog</h2><ul><li><a href=\"https://blog.burntsushi.net/regex-internals/\">regex engine internals as a library</a></li></ul><h2>hacker noon - rust</h2><ul><li><a href=\"https://hackernoon.com/7-9-2023-noonification?source=rss\">the noonification: why we rewrote ockam in rust (7/9/2023)</a></li></ul><h2>cliffle</h2><ul><li><a href=\"http://cliffle.com/blog/async-decl-coords/\">getting file/line in await traces</a></li><li><a href=\"http://cliffle.com/blog/composed-concurrency-in-drivers/\">composing concurrency in drivers</a></li></ul><h2>console - interesting developer tools</h2><ul><li><a href=\"https://console.dev/tools/axiom\">axiom</a></li><li><a href=\"https://console.dev/tools/redpanda\">redpanda</a></li><li><a href=\"https://console.dev/tools/zed\">zed</a></li></ul><h2>console - developer tool beta releases</h2><ul><li><a href=\"https://github.com/kelindar/column\">column</a></li></ul><h2>console - devtools company profiles</h2><ul><li><a href=\"https://console.dev/profiles/speakeasy\">speakeasy</a></li></ul><h2>github changelog</h2><ul><li><a href=\"https://github.blog/changelog/2023-07-06-anonymous-users-have-access-to-new-code-view-and-navigation\">anonymous users have access to new code view and navigation</a></li><li><a href=\"https://github.blog/changelog/2023-07-05-workato-is-now-a-github-secret-scanning-partner\">workato is now a github secret scanning partner</a></li><li><a href=\"https://github.blog/changelog/2023-07-05-new-and-updated-iso-and-csa-star-certifications-are-now-available\">new and updated iso and csa star certifications are now available</a></li><li><a href=\"https://github.blog/changelog/2023-07-05-dependabot-alerts-can-be-enabled-at-the-repository-organization-and-enterprise-levels-in-ghes-3-9\">dependabot alerts can be enabled at the repository, organization, and enterprise levels in ghes 3.9</a></li><li><a href=\"https://github.blog/changelog/2023-07-05-code-scanning-with-codeql-supports-swift-5-8\">code scanning with codeql supports swift 5.8</a></li><li><a href=\"https://github.blog/changelog/2023-07-03-new-rate-limit-is-coming-for-the-audit-log-api-endpoints\">new rate limit is coming for the audit log api endpoints</a></li></ul><h2>discord blog</h2><ul><li><a href=\"https://discord.com/blog/celebrating-disability-pride-month-with-two-inclusive-communities\">celebrating disability pride month with two inclusive communities</a></li><li><a href=\"https://discord.com/blog/meme-up-some-fun-with-remix\">meme up some fun with remix</a></li></ul><h2>blog - vlad mihalcea</h2><ul><li><a href=\"https://vladmihalcea.com/high-performance-java-persistence-newsletter-issue-52/\">high-performance java persistence newsletter, issue 52</a></li></ul><h2>fasterthanli.me</h2><ul><li><a href=\"https://fasterthanli.me/articles/cracking-electron-apps-open\">cracking electron apps open</a></li></ul><h2>hello world</h2><ul><li><a href=\"https://qristin.wordpress.com/2023/07/03/akson-er-dodt-hva-na/\">akson er dodt - hva na?</a></li></ul>"#;
    let mail = Mail::new("rss2email@resend.dev", address, "rss2email", contents);
    let client = ResendClient::new(api_key);

    match client.send(mail) {
      Ok(()) => {
        info!("Email request sent");
        Ok(())
      }
      Err(e) => Err(EmailError::from(e)),
    }
  }
}
