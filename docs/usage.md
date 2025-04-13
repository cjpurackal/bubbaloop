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
  "device_name": "nvidia-nano",
  "distro": "Ubuntu 18.04.6 LTS",
  "hostname": "nvidia-nano",
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
  "available_memory": 47551553536,
  "cpus": [
    {
      "brand": "11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz",
      "frequency": 1272,
      "name": "cpu0",
      "usage": 0.0
    },
    {
      "brand": "11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz",
      "frequency": 1630,
      "name": "cpu1",
      "usage": 0.0
    },
    {
      "brand": "11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz",
      "frequency": 1574,
      "name": "cpu2",
      "usage": 0.0
    },
    {
      "brand": "11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz",
      "frequency": 1320,
      "name": "cpu3",
      "usage": 0.0
    },
    {
      "brand": "11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz",
      "frequency": 1381,
      "name": "cpu4",
      "usage": 0.0
    },
    {
      "brand": "11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz",
      "frequency": 2911,
      "name": "cpu5",
      "usage": 0.0
    },
    {
      "brand": "11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz",
      "frequency": 1274,
      "name": "cpu6",
      "usage": 0.0
    },
    {
      "brand": "11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz",
      "frequency": 1471,
      "name": "cpu7",
      "usage": 0.0
    },
    {
      "brand": "11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz",
      "frequency": 1580,
      "name": "cpu8",
      "usage": 0.0
    },
    {
      "brand": "11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz",
      "frequency": 1507,
      "name": "cpu9",
      "usage": 0.0
    },
    {
      "brand": "11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz",
      "frequency": 1396,
      "name": "cpu10",
      "usage": 0.0
    },
    {
      "brand": "11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz",
      "frequency": 1468,
      "name": "cpu11",
      "usage": 0.0
    },
    {
      "brand": "11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz",
      "frequency": 1426,
      "name": "cpu12",
      "usage": 0.0
    },
    {
      "brand": "11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz",
      "frequency": 1993,
      "name": "cpu13",
      "usage": 0.0
    },
    {
      "brand": "11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz",
      "frequency": 1321,
      "name": "cpu14",
      "usage": 0.0
    },
    {
      "brand": "11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz",
      "frequency": 2195,
      "name": "cpu15",
      "usage": 0.0
    }
  ],
  "disks": [
    {
      "available_space": 76941479936,
      "file_system": "ext4",
      "mount_point": "/",
      "name": "/dev/nvme0n1p2",
      "total_space": 1006450962432
    },
    {
      "available_space": 529432576,
      "file_system": "vfat",
      "mount_point": "/boot/efi",
      "name": "/dev/nvme0n1p1",
      "total_space": 535805952
    },
    {
      "available_space": 0,
      "file_system": "fuse.cursor-0.45.14x86_64.AppImage",
      "mount_point": "/tmp/.mount_cursorO4RPqC",
      "name": "cursor-0.45.14x86_64.AppImage",
      "total_space": 0
    }
  ],
  "free_memory": 32135196672,
  "global_cpu_usage": 5.2112675,
  "host_name": "farmng-TensorBook-late-2021",
  "kernel_version": "5.15.0-130-generic",
  "name": "Ubuntu",
  "os_version": "20.04",
  "total_memory": 67230515200,
  "total_swap": 2147479552,
  "used_memory": 19678961664
}
```
