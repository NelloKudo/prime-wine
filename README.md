<h1 align="center"> prime-wine </h1>
<p align="center"> Alternative approach to enable Prime Video HD playback on Linux, using Wine and Brave 📽️ </p>

<div align="center">

<img width="3824" height="1080" alt="merged_side_by_side_updated" src="https://github.com/user-attachments/assets/7c5bbf9b-fe41-4db5-b70f-2d1d027b8488" />

---

  &ensp;<a href="#installation-%EF%B8%8F"><kbd> <br> Installation <br> </kbd></a>&ensp;
  &ensp;<a href="#overview"><kbd> <br> Overview <br> </kbd></a>&ensp;

---

## Installation

You can set up Brave + Prime Video (with Wine) in two ways:

</div>

### Option 1: Lutris (Automated)
Use the provided Lutris script in this repo to install everything in one go:
- Install [Lutris](https://lutris.net/downloads/).
- Download the `.yml` script from [here](https://github.com/NelloKudo/prime-wine/blob/main/prime-wine-lutris.yaml).
- Import it into Lutris (`Lutris → "+" → Import Game → YAML`).
- Run the installer, it will handle Wine, dependencies and the whole setup for you.
- Open Prime, accept the Widevine DRM notification after you login and enjoy HD!

### Option 2: Manual
Assuming you have both `wine-staging` and `winetricks` installed and you're on a fairly recent Wine version:
- Create a prefix with all the needed components: `WINEPREFIX=~/prime winetricks -q dxvk vkd3d corefonts vcrun2022 win10`
- Download Brave's standalone Windows installer from their GitHub, here's [v1.82.161](https://github.com/brave/brave-browser/releases/tag/v1.82.161)
- Install it with Wine: `WINEPREFIX=~/prime wine ~/Downloads/BraveBrowserStandaloneSetup.exe`
- Run Prime Video from Brave as an app (important) with the following:
```
WINEPREFIX=~/prime wine ~/prime/drive_c/Program Files/BraveSoftware/Brave-Browser/Application/brave.exe --app=https://www.primevideo.com
```
- After logging in to Prime, accept the Widevine DRM notification you get in the website.

You can then just create a launch script for it or add it to Bottles or whatever launcher you prefer.

---

<div align="center">
  
## Overview

You're probably familiar with the following issue you get while using Prime Video on native Linux browsers:

<img width="671" height="222" alt="image" src="https://github.com/user-attachments/assets/e1e6fe09-31c1-43a7-9b37-3c20d5c29035" />

---

</div>

[Prime Video's help section](https://www.primevideo.com/help?nodeId=GUX9FYHU5D8LC9EJ) confirms Linux is indeed locked to **standard resolutions**, probably due to some check in the Widevine plugin
browsers install in order to watch content on streaming platforms (simply spoofing a Windows UserAgent doesn't indeed make it work).

While using a Windows VM works to watch HD content, it's still a very painful experience (and definitely not a sustainable one either): running a browser in Wine, on the other hand, surprisingly re-enables
HD playback and works really well.

Some more interesting points:

---

**Why did you create a whole repository just to install a browser in Wine?**
- Because installing one (and having it actually working) can be quite challenging. Not all browsers install fine, with Chrome and Opera dying as soon as the install is launched or Firefox's fullscreen feature not working.
- While Brave does install and work, touching some settings (even changing theme) manages to permanently break it on the next browser boot. Here's where the `--app` argument comes in clutch: with it, the browser effectively only acts as an Electron webapp, making breaking the browser almost impossible from my testing. xd

**Why not make a simple Electron wrapper for Prime Video and then run it in Wine?**
- My original project was indeed to make one, and I even made it "work" but Electron and Widevine, especially when it comes to Prime, can be quite tricky.
- For a working webapp, you need [a special Electron fork](https://github.com/castlabs/electron-releases) and to sign all of your executables using [EVS's VMP signing service](https://github.com/castlabs/electron-releases/wiki/EVS): while doing it is possible, one Electron update might be enough to break it, while browsers like Brave already do handle all this part themselves.

**Can this be done in services other than Prime Video?**
- Haven't personally tested here, but it probably can be done. After all Prime was, in my experience, the most painful one to get to work.
- You can switch to other websites by changing the arguments of the Brave executable to: `--app=https://www.yourwebsite.com`

---

That's pretty much it. Enjoy watching Prime Video at a decent quality, finally!
