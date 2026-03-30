# brcurl - curl that acts like a browser

agents can easily run curls but many times the result isn't useful/interpretable for humans

with brcurl you see what the user actually sees when they are in a browser which is executing the js/wasm

its also useful when using coding agents for wasm or web dev which prefer CLI tools so that the agent understands what is on the site 

to see the result when a user has had the output open for 60 seconds use: `brcurl -t 60 https://github.com/RohanAdwankar`

to save a screenshot in addition to the default stdout use: `brcurl -o https://github.com/RohanAdwankar`

to see what it does check out the examples of text output in [bcurl](./tests/bcurl/003_https___github_com_RohanAdwankar.txt), screenshots in [bcurl-o](./tests/bcurl/003_https___github_com_RohanAdwankar.png), and the baseline curls in [curl](./tests/curl/003_https___github_com_RohanAdwankar.txt)
