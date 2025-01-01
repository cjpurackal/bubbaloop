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

### Compute mean and std of a local directory of images

```
just compute-mean-std 0.0.0.0 3000 /path/to/images -n 4
```

### Launch a pipeline

Start and register a pipeline given its id. e.g. `bubbaloop`. will start a recording of the camera and write it to a file as mp4.

```
just pipeline-start 0.0.0.0 3000 bubbaloop
```

To stop the pipeline, use the `pipeline-stop` command:

```
just pipeline-stop 0.0.0.0 3000 bubbaloop
```

To list all the registered pipelines, use the `pipeline-list` command:

```
just pipeline-list 0.0.0.0 3000
```
