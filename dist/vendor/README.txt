This folder is populated by scripts/vendor-assets.sh.

Expected contents after running it:
  leaflet.js, leaflet.css, images/   -> map rendering
  fonts.css, opensans-*.woff2        -> Open Sans, self-hosted

Without these the app still runs (it falls back to a CDN for Leaflet and to
system fonts), but it will need an internet connection to start.
