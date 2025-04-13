---
description: Low level utilities with the Bubbaloop server to get system stats and metrics
layout:
  title:
    visible: true
  description:
    visible: true
  tableOfContents:
    visible: true
  outline:
    visible: true
  pagination:
    visible: true
---

# 💊 Stats API

The **Bubbaloop** server provides a comprehensive REST API that allows users to retrieve detailed system information through the `/api/v0/stats` endpoint. This API leverages established Rust libraries to deliver accurate and extensive system data in a structured JSON format.

We expose the following functionality&#x20;

* `whoami` [https://docs.rs/whoami/latest/whoami](https://docs.rs/whoami/latest/whoami/)
* `sysinfo` [https://docss.rs/sysinfo/latest/sysinfo](https://docs.rs/sysinfo/latest/sysinfo/)

## Available API

* `GET /api/v0/stats/whoami`  — Provides detailed information about the system's identity
* `GET /api/v0/stats/sysinfo` — Delivers comprehensive system resource metric

## Usage

### whoami <a href="#whoami" id="whoami"></a>

```
just whoami 0.0.0.0 3000
```

#### **Server terminal**

```bash
[2025-04-13T15:03:20Z DEBUG bubbaloop::api::handles::stats::whoami] 🤖 Received request for whoami
```

#### **Client terminal**

```bash
Result: {
  "arch": "Arm64",
  "desktop_env": "Unknown",
  "device_name": "nvidia-desktop",
  "distro": "Ubuntu 22.04.5 LTS",
  "hostname": "nvidia-desktop",
  "platform": "Linux",
  "realname": "nvidia",
  "username": "nvidia"
}
```

### sysinfo

```
just sysinfo 0.0.0.0 3000
```

#### **Server terminal**

```bash
[2025-04-13T15:03:45Z DEBUG bubbaloop::api::handles::stats::sysinfo] 🤖 Received request for sysinfo
```

#### **Client terminal**

```json
Result: {
  "available_memory": 7011606528,
  "cpus": [
    {
      "brand": "Cortex-A78AE",
      "frequency": 1113,
      "name": "cpu0",
      "usage": 0.0
    },
    {
      "brand": "Cortex-A78AE",
      "frequency": 1113,
      "name": "cpu1",
      "usage": 0.0
    },
    {
      "brand": "Cortex-A78AE",
      "frequency": 1113,
      "name": "cpu2",
      "usage": 0.0
    },
    {
      "brand": "Cortex-A78AE",
      "frequency": 1113,
      "name": "cpu3",
      "usage": 0.0
    },
    {
      "brand": "Cortex-A78AE",
      "frequency": 729,
      "name": "cpu4",
      "usage": 0.0
    },
    {
      "brand": "Cortex-A78AE",
      "frequency": 729,
      "name": "cpu5",
      "usage": 0.0
    }
  ],
  "disks": [
    {
      "available_space": 186810265600,
      "file_system": "ext4",
      "mount_point": "/",
      "name": "/dev/mmcblk0p1",
      "total_space": 250131267584
    },
    {
      "available_space": 65946624,
      "file_system": "vfat",
      "mount_point": "/boot/efi",
      "name": "/dev/mmcblk0p10",
      "total_space": 66059264
    }
  ],
  "free_memory": 4320612352,
  "global_cpu_usage": 18.697363,
  "host_name": "nvidia-desktop",
  "kernel_version": "5.15.148-tegra",
  "name": "Ubuntu",
  "os_version": "22.04",
  "total_memory": 7990116352,
  "total_swap": 3995049984,
  "used_memory": 978509824
}
```
