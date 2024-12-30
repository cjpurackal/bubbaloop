# Bubbaloop

Bubbaloop is a serving platform for computer vision and AI Robotics.

## Install dependencies

```
just install_deps
```

## Run Bubbaloop locally in Linux

Start the server with in the terminal:

it will listen on `0.0.0.0:3000` by default.

```
just serve
```

Optionally, you can specify the host and port:

```
just serve 0.0.0.0 3000
```

## Use the CLI

```
just help
```

### Request stats

```
just whoami 0.0.0.0 3000
```

### Compute mean and std of an image

```
just compute-mean-std 0.0.0.0 3000 /path/to/images -n 4
```


