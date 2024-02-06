open feeds.txt | str trim | str replace -ar "#.*" "" | lines -s | str join ","
