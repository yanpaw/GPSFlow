# GPSFlow

A viewer for windsurfing GPS sessions.

As a hobby-windsurfer, and I wanted a quicker way to look at my own tracks
after a session, so I built this. Sharing it in case it's useful to anyone
else.

  ![GPSFlow](https://github.com/user-attachments/assets/94b39fb4-a445-4f04-81f1-3d4a195bea72)
     

## What it reads

GPX files and Locosys `.sbp` files, which is what my two watches produce.

Other formats can be added. If you have a
watch that writes something else, open an issue with a sample file.

## What it shows

- Your track on a map, coloured by speed
- Distance, duration, top speed, and the number of planing runs
- Planing runs found automatically, ranked fastest first
- Best results for the usual categories: top speed, 2 seconds, 100 m, 250 m,
  500 m and nautical mile, ten of each
- A timeline of speed against time that you can step through with the arrow
  keys, with a marker following along the map
- Click a result to see just that stretch of track; ctrl-drag the timeline to
  measure any window you like

## Speed accuracy

I've checked the numbers against existing session software and they match.
However there is no build in Interpolation(yet) 

## Getting it

Installers are on the [Releases page](../../releases).

| Platform | File | Note |
|---|---|---|
| Windows | `.exe` or `.msi` | SmartScreen warns about an unknown publisher: More info → Run anyway |
| macOS | `.dmg` | Unsigned, so the first launch needs right-click → Open |
| Linux | `.AppImage` or `.deb` | `chmod +x` the AppImage first |

Once installed, `.gpx` and `.sbp` files open in it when double-clicked.

If you'd rather not install anything, `dist/index.html` is the whole app in
one file. Open it in a browser and everything works except the file
association.

## Keyboard

| Key | Does |
|---|---|
| `←` `→` | Step through the session one sample at a time |
| `Shift` + `←` `→` | Fifteen at a time |
| `+` `−` | Zoom the timeline |
| `Home` `End` | Jump to the start or end |
| `Ctrl` + drag | Select a time range |
| `Esc` | Clear the selection |

## Notes

- Everything happens on your machine. No account, no upload, no tracking.
- Map tiles come from [CARTO](https://carto.com/basemaps/) and need a
  connection the first time you look at an area. Offline, you still get the
  track, stats, runs and timeline on a blank background.
- A planing run means speed above 18 km/h held for at least 1.2 seconds.

## Building it yourself

See [BUILDING.md](BUILDING.md). Only needed if you want to compile it — the
Releases page has ready-made installers.

## Credits

Built by Florian Hallbauer.

The `.sbp` format was decoded with the help of
[Logiqx's GPS Wizard format notes](https://logiqx.github.io/gps-wizard/formats/sbp.html).
Maps use [MapLibre GL JS](https://maplibre.org/) with
[CARTO](https://carto.com/) basemaps and
[OpenStreetMap](https://www.openstreetmap.org/copyright) data.
