# Converts the `feeds.txt` contents into a corretly formatted string that
# can be used as an environment variable.

((Get-Content "./feeds.txt") -replace "#.*$", "" | Where-Object {$_.trim() -ne "" }) -join ";"