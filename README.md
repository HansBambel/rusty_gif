# Create a gif easily from a bunch of pictures in a folder

Images are currently resized to 500 x 500 px to have a reasonable size. This can be removed though.

## Compile it
```
cargo build --release
```

## Run it
Just run `rusty_gif.exe <folder-with-images> <output-folder>`.

This will give you a `output.gif` in the output folder.

The images are read in naming order.

## Next features (soon or never)
- Change resolution (currently hardcoded to 500x500 px) or turn resizing off.
- Change animation speed
- A simple UI
    - Buffer read-in images
    - resize only when values change