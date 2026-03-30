# brcurl - curl that acts like a browser

Agents can easily run curls but sometimes the result isn't useful/interpretable to agents/humans

with brcurl you see what the user actually sees when they are in a browser which is executing the js/wasm so that we see what actually loads

to see the result when a user has had the output open for 60 seconds use: `brcurl -t 60 https://example.com`

to save a screenshot in addition to the default stdout use: `brcurl -o page.png https://example.com`

to compare `curl` output against `brcurl` for every URL in `tests/url.txt`, run: `tests/compare.sh`
