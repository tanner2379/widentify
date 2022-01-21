# widentify
Create up to three textboxes with timeouts on your desktop. Useful for identifying workspaces on tiling window managers without a bar.

## Configuration
The default config and an example CSS file can be found in the "examples" directory of this project.

Configure timeouts, window title, window size, and window positioning by creating ~/.config/widentify/widentify.config.
### Note:
Make the window persist by setting "timeout: 0"

Configure window styling by creating ~/.config/widentify/widentify.css. Currently there is not a way to apply different styles to each window.
Windows and the labels within them are created with the gtk library, so not all CSS attributes will work, but the available attributes should
more than cover the use case of this application.
### Note:
If the text inside your window is larger than the window itself, the window will stretch to accommodate the text.

## Running the program
Simply call the program and pass an argument for what text you want each window to display. The program takes 1, 2, or 3 arguments and displays
a window for each.

```bash
widentify Widentify is cool!
```
