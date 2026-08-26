#!/bin/sh

while IFS=': ' read -r name url; do
    git submodule add "$url" "$name"
done <<'EOF'
atelier: git@github.com:atelierbram/base16-atelier-schemes.git
atlas: git@github.com:ajlende/base16-atlas-scheme.git
black-metal: git@github.com:metalelf0/base16-black-metal-scheme.git
brogrammer: git@github.com:piggyslasher/base16-brogrammer-scheme.git
brushtrees: git@github.com:WhiteAbeLincoln/base16-brushtrees-scheme.git
circus: git@github.com:stepchowfun/base16-circus-scheme.git
classic: git@github.com:detly/base16-classic-scheme.git
codeschool: git@github.com:blockloop/base16-codeschool-scheme.git
cupertino: git@github.com:Defman21/base16-cupertino.git
default: git@github.com:chriskempson/base16-default-schemes.git
dracula: git@github.com:dracula/base16-dracula-scheme.git
fruit-soda: git@github.com:jozip/base16-fruit-soda-scheme.git
github: git@github.com:Defman21/base16-github-scheme.git
gruvbox: git@github.com:dawikur/base16-gruvbox-scheme.git
heetch: git@github.com:tealeg/base16-heetch-scheme.git
ia: git@github.com:aramisgithub/base16-ia-scheme.git
icy: git@github.com:icyphox/base16-icy-scheme.git
materia: git@github.com:Defman21/base16-materia.git
materialtheme: git@github.com:ntpeters/base16-materialtheme-scheme.git
material-vivid: git@github.com:joshyrobot/base16-material-vivid-scheme.git
mellow: git@github.com:gidsi/base16-mellow-scheme.git
mexico-light: git@github.com:drzel/base16-mexico-light-scheme.git
nord: git@github.com:8-uh/base16-nord-scheme.git
one-light: git@github.com:purpleKarrot/base16-one-light-scheme.git
onedark: git@github.com:tilal6991/base16-onedark-scheme.git
outrun: git@github.com:hugodelahousse/base16-outrun-schemes.git
papercolor: git@github.com:jonleopard/base16-papercolor-scheme.git
porple: git@github.com:AuditeMarlow/base16-porple-scheme.git
purpledream: git@github.com:archmalet/base16-purpledream-scheme.git
rebecca: git@github.com:vic/base16-rebecca.git
snazzy: git@github.com:h404bi/base16-snazzy-scheme.git
solarflare: git@github.com:mnussbaum/base16-solarflare-scheme.git
solarized: git@github.com:aramisgithub/base16-solarized-scheme.git
summerfruit: git@github.com:cscorley/base16-summerfruit-scheme.git
tomorrow: git@github.com:chriskempson/base16-tomorrow-scheme.git
tokyonight: git@github.com:viniciusmuller/base16-tokyonight-scheme.git
twilight: git@github.com:hartbit/base16-twilight-scheme.git
unikitty: git@github.com:joshwlewis/base16-unikitty.git
woodland: git@github.com:jcornwall/base16-woodland-scheme.git
zenburn: git@github.com:elnawe/base16-zenburn-scheme.git
xcode-dusk: git@github.com:gonsie/base16-xcode-dusk-scheme.git
EOF
