# eframe in android

This part of the repo will build an android app using a `cdylib` named `eframe_template_android`. This `cdylib` will be used by the android app to render the UI.

The chain is: `eframe_template` used in `eframe_template_android` used in `eframe_template` (android app)

## Building the android app

```sh
# will compile the cdylib and copy it to the android app
make

make run-on-device
```
