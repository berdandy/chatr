#!/bin/sh

echo Generating offline caches from latest api...

gw2search -jp > src/professions.json
gw2search -jl > src/legends.json
gw2search -jS > src/specializations.json
gw2search -jP > src/pets.json
