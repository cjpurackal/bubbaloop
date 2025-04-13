---
description: Low level utilies with the Bubbaloop server
---

# 💊 Basic Usage

The Bubbaloop server is provided with a REST api that allows the user to query basic information about the system where is running on under the `/api/v0/stats` end point

We expose the following utilities&#x20;

* `whoami` [https://docs.rs/whoami/latest/whoami/](https://docs.rs/whoami/latest/whoami/)

## whoami

```
just whoami 0.0.0.0 3000
```

**Server terminal**

```bash
[2025-04-13T12:34:53Z DEBUG bubbaloop::api::handles::stats] 🤖 Received request for whoami
```

**Client terminal**

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
