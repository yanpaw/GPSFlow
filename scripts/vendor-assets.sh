#!/usr/bin/env bash
# ---------------------------------------------------------------------------
# Downloads MapLibre GL JS and Open Sans into dist/vendor so the packaged app runs
# without an internet connection.
#
# Run once, from the project root:
#     bash scripts/vendor-assets.sh
#
# The basemap is still fetched from CARTO at runtime and can't be bundled.
# ---------------------------------------------------------------------------
set -euo pipefail

VENDOR="$(cd "$(dirname "$0")/.." && pwd)/dist/vendor"
mkdir -p "$VENDOR"
cd "$VENDOR"

MAPLIBRE_VER="4.7.1"
BASE="https://unpkg.com/maplibre-gl@${MAPLIBRE_VER}/dist"

echo "==> MapLibre GL JS ${MAPLIBRE_VER}"
curl -fsSL "${BASE}/maplibre-gl.js"  -o maplibre-gl.js
curl -fsSL "${BASE}/maplibre-gl.css" -o maplibre-gl.css

# NOTE: the vector basemap itself (tiles, sprites, label fonts) is fetched
# from CARTO at runtime and cannot be bundled, so the basemap still needs a
# connection. Everything else - the app, the library, the fonts - works
# offline, and a session without tiles still shows the track, stats, runs
# and timeline on a blank background.

# ---------------------------------------------------------------------------
# Open Sans. Google serves different formats per user-agent, so we ask as a
# modern browser to get woff2, then rewrite the CSS to point at local copies.
# ---------------------------------------------------------------------------
echo "==> Open Sans"
UA="Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0 Safari/537.36"
CSS_URL="https://fonts.googleapis.com/css2?family=Open+Sans:wght@400;500;600;700&display=swap"

curl -fsSL -H "User-Agent: ${UA}" "$CSS_URL" -o fonts-remote.css

# Collect the woff2 URLs once, in a stable order, then download and rewrite
# using the same numbering. Pure shell - no Python needed, since Windows
# often has no working `python3` on PATH.
grep -o "https://fonts.gstatic.com[^)]*\.woff2" fonts-remote.css | sort -u > urls.txt

cp fonts-remote.css fonts.css
n=0
while read -r url; do
  n=$((n + 1))
  fname="opensans-${n}.woff2"
  curl -fsSL "$url" -o "$fname"
  # Rewrite through a temp file rather than sed -i. In-place editing is not
  # portable: GNU sed (Linux, Git Bash) wants `-i`, BSD sed (macOS) wants
  # `-i ''`, and neither form works on the other. Redirecting sidesteps the
  # whole argument. '|' is the delimiter so the slashes in the URL need no
  # escaping.
  sed "s|${url}|${fname}|g" fonts.css > fonts.css.tmp && mv fonts.css.tmp fonts.css
  echo "    ${fname}"
done < urls.txt

rm -f urls.txt fonts-remote.css

echo
echo "Done. Vendored into dist/vendor:"
ls -1 "$VENDOR"
