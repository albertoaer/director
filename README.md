# Director

Director is cross-platform desktop application to calculate directories size and alert you to files/folders that are too big (defined by yourself what "too big" means).

### Important

Due to the cost of calculating the tree of directories, you either do it manually on the interested directory or you configure it to run on startup. Anyway the program is designed to run on `background` so **it must be closed from the system tray**, otherwise (closing the window) it will keep the background program running.

## Stack

Tauri + Svelte + Typescript

## Features

- Integrated explorer
- Feedback of the calculation process, in the orders tab
- Turn explorer into a pie chart
- Create size based alerts for folders, files and both (any)
- Select those directories you want to be calculated on program startup

## Captures

![Explorer Capture](https://github.com/albertoaer/director/assets/24974091/f7f7c94f-bdbd-4651-9313-0009eba299c4)
![Alerts Capture](https://github.com/albertoaer/director/assets/24974091/a2cf9a0c-8780-4deb-8ba0-66f1d34504dc)
![Explorer Chart Capture](https://github.com/albertoaer/director/assets/24974091/17b44232-efc4-45b5-8e80-680825d849ae)
![Startup Capture](https://github.com/albertoaer/director/assets/24974091/8166fb96-ae39-452c-b943-0dcbceb11494)
![Edit Alerts Capture](https://github.com/albertoaer/director/assets/24974091/6f99f3ca-20ec-4808-9497-0e4303399355)
