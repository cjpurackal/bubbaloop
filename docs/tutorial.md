# Bubbaloop 101: Turn Your Phone into a Smart Security Camera in 10 Minutes

**Why should you care?**

- **You already own the hardware.** An old iPhone or Android device on your windowsill is now your first smart security feed.
- **Privacy‑first.** Everything stays local on a $249 Jetson Orin Nano (or your laptop) – no cloud fees, no vendor lock‑in.
- **Instant insight.** Live multi‑camera visualization and local video recording with spatial intelligence built in.

This guide walks you through setting up **Bubbaloop**, an open-source camera pipeline built with Rust, to:

- Ingest real-time video from your phone or IP cameras
- Visualize and interact with the results in real-time
- All with high performance on low-cost edge hardware

⏱️ You’ll go from "unopened box" to live feed + local recording in 10–15 minutes.

---

## What You'll Need

### Your Phone or Any Camera

- **iPhone** – use [RTSP Stream](https://apps.apple.com/us/app/rtsp-stream/id6474928937) or Larix Broadcaster
- **Android** – use [WebCamPro](https://play.google.com/store/apps/details?id=com.shenyaocn.android.WebCamPro&hl=en) or IP Webcam / DroidCam
- **Optional**: IP Cam (RTSP compatible) – e.g. TP-Link Tapo TC65 (~£29)

### Hardware

- **Jetson Orin Nano (8GB)** – [Buy here from Seeed](https://www.seeedstudio.com/NVIDIAr-Jetson-Orintm-Nano-Developer-Kit-p-5617.html) (~$249)
- Or your **Linux laptop / PC**

### Software & Tools

- Rust + Cargo
- Kornia-rs – high-performance vision tools in Rust
- Just – command runner
- Rerun for real-time visualization (optional but recommended)
- (Coming soon: Docker image or VirtualBox setup for one-click launch)

---

## Set Up Camera Streaming First

### iPhone (RTSP)
- Download [RTSP Stream](https://apps.apple.com/us/app/rtsp-stream/id6474928937)
- Start a stream and take note of the RTSP URL (e.g. `rtsp://your-ip:8554/live`)

### Android (RTSP or MJPEG)
- Install [WebCamPro](https://play.google.com/store/apps/details?id=com.shenyaocn.android.WebCamPro&hl=en)
- Enable RTSP or MJPEG streaming
- Get your stream URL (e.g. `rtsp://192.168.1.x:8554/live`)

---

## Step-by-Step Setup

### Install Dependencies
```bash
wsl.exe --install Ubuntu-22.04
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install just
```

### Clone the Repo
```bash
git clone https://github.com/kornia/bubbaloop.git
cd bubbaloop
```

### Install System Packages
```bash
just install_deps
```

### Launch the Server
```bash
just serve
```

To run on a specific IP/port:
```bash
just serve 192.168.1.154 3000
```

---

## Start a Camera Pipeline

```bash
just start-pipeline recording 0.0.0.0 3000
```

To stop:
```bash
just stop-pipeline recording 0.0.0.0 3000
```

List all pipelines:
```bash
just pipeline-list 0.0.0.0 3000
```

---

## Configure Your Camera

Edit `src/cu29/pipelines/recording_one_camera.ron`:
```ron
(
    tasks: [
        (
            id: "cam0",
            type: "crate::cu29::tasks::VideoCapture",
            config: {
                "source_type": "rtsp",
                "source_uri": "rtsp://192.168.1.141:8554/live",
                "channel_id": 0,
            }
        ),
    ],
)
```

---

## Visualize with Rerun

```bash
python examples/python-streaming/client.py   --host 0.0.0.0 --port 3000 --cameras 0
```

Or view a recorded `.rrd` file:
```bash
scp your-device:/tmp/1735941642.rrd ./
rerun 1735941642.rrd
```

---

## Running Paligemma for Object Detection

```bash
just start-pipeline paligemma 0.0.0.0 3000
```

Example config:
```ron
(
    tasks: [
        (
            id: "cam0",
            type: "crate::cu29::tasks::VideoCapture",
            config: {
                "source_type": "rtsp",
                "source_uri": "rtsp://192.168.1.141:8554/live",
                "channel_id": 0,
            },
        ),
        (
            id: "detector",
            type: "crate::cu29::tasks::Paligemma",
            config: {
                "prompt": "What objects are in the scene?",
            },
        ),
    ],
)
```

---

## Contribute / Feedback

Join our [Discord server](https://discord.com/invite/HfnywwpBnD) or open issues on [GitHub](https://github.com/kornia/bubbaloop).

💡 *Coming soon*: One-click Docker or VirtualBox setup for a safe, quick start.

