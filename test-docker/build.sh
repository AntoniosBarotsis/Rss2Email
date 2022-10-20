# Small script for testing `mail` command version capability
# Include building of docker image & assembling a test

cd ..
cargo build --release

cp target/release/rss2email test-docker
cd test-docker

docker build --rm --tag debian-email:test .

docker run \
  -e "EMAIL"="MAIL_COMMAND" \
  -e "EMAIL_ADDRESS"="root@localhost" \
  -e "FEEDS"="https://blog.rust-lang.org/feed.xml;" \
  -e "DAYS"="50" \
  debian-email:test sh -c "rss2email && sleep 5 && cat /var/mail/mails"

rm rss2email
