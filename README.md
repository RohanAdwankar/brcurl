# brcurl - curl that acts like a browser

agents can easily run curls but many times the result isn't useful/interpretable for humans

with brcurl you see what the user actually sees when they are in a browser which is executing the js/wasm

to see the result when a user has had the output open for 60 seconds use: `brcurl -t 60 https://github.com/RohanAdwankar`

to save a screenshot in addition to the default stdout use: `brcurl -o https://github.com/RohanAdwankar`

to print the fully rendered html after js executes use: `brcurl -v https://github.com/RohanAdwankar`

to see what it does check out the examples of text output in [bcurl](./tests/bcurl/003_https___github_com_RohanAdwankar.txt), screenshots in [bcurl-o](./tests/bcurl-o/003_https___github_com_RohanAdwankar.png), rendered html in [bcurl-v](./tests/bcurl-v/003_https___github_com_RohanAdwankar.html), and the baseline curls in [curl](./tests/curl/003_https___github_com_RohanAdwankar.html)

as you can see (for example from opening the youtube curl in a browser vs opening bcurl -v in the browser), the html bcurl generates has the neccesary information about the layout and content for the agent to iterate on the frontend while the curl site is blank 
