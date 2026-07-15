<h1 align="center"> prime-wine </h1>
<p align="center"> Alternative approach to enable Prime Video HD playback on Linux, using Wine and Brave 📽️ </p>

<div align="center">

<img width="3840" height="1080" alt="image" src="https://github.com/user-attachments/assets/e450a593-48cb-4a50-9087-376767a11c51" />

---

  &ensp;<a href="#installation-%EF%B8%8F"><kbd> <br> Installation <br> </kbd></a>&ensp;
  &ensp;<a href="#overview"><kbd> <br> Overview <br> </kbd></a>&ensp;
  &ensp;<a href="#building-from-source"><kbd> <br> Building <br> </kbd></a>&ensp;

---

## Installation

</div>

Grab `PrimeWine-x86_64.AppImage` from the [latest release](https://github.com/NelloKudo/prime-wine/releases/latest), then:

```
chmod +x PrimeWine-x86_64.AppImage
./PrimeWine-x86_64.AppImage
```

Press `install` and let it do its thing: it downloads Wine and Brave, sets up the prefix with everything needed and adds a **Prime Video** entry to your app menu. After logging in to Prime, accept the Widevine DRM notification you get in the website, wait for the site to reboot, then relaunch Prime Video. Enjoy HD!

From then on:
- Clicking the menu entry (or the AppImage) goes straight to Prime Video.
- Search for **Manage Prime Video settings** in the menu (or run the AppImage with `--manage`) to update Brave, kill Wine, reinstall or uninstall.
- Brave cannot update itself under Wine, so use the `update brave` button every now and then.

The only things needed on your system are `bash`, `tar` and `xz`, which every distro ships anyway (`cabextract` comes bundled in the AppImage).

---

<div align="center">

## Overview

You're probably familiar with the following issue you get while using Prime Video on native Linux browsers:

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

## Building from source

Everything is just plain Rust plus a bash script for packaging:

```
git clone https://github.com/NelloKudo/prime-wine.git
cd prime-wine
./build-appimage.sh
```

The script builds the release binary, compiles a static `cabextract`, and packs everything into `PrimeWine-x86_64.AppImage`.

---

That's pretty much it. Enjoy watching Prime Video at a decent quality, finally!
