# Director

Director is a cross-platform desktop application to calculate directories size and alert you to files/folders that are too big (defined by yourself what "too big" means).

### Important

Due to the cost of calculating the tree of directories, you either do it manually on the targeted directory or you configure it to run on startup to prevent undesired system overload. Anyway the program it's designed to run on `background` so **it must be closed from the system tray**, otherwise (closing the window) it will keep the background program running allowing you to open the window again from the `tray`.

## Stack

Tauri + Svelte + Typescript

## How to run?

Install all `npm` dependencies
```
$ npm i
```

Run tauri develop
```
$ npm run tauri dev
```

## How to build?

Install all `npm` dependencies if you didn't already
```
$ npm i
```

Run tauri build
```
$ npm run tauri build
```

## Features

- Integrated explorer
- Feedback of the calculation process in the orders tab
- Turn explorer into a pie chart
- Create size based alerts for folders, files and both (any)
- Select those directories you want to be queued for calculation on program startup

### Features under consideration (none of this is implemented yet!)

- Integrated search section probiding name and size based search
  - It takes advantage of the already working in memory file index system
  - Could solve an use case slower to perform with the already implemented alerts

- Section for automatic cyclic directories clean up, It's not implemented yet because:
  - it might be harmful in case of not being used properly
  - The project was only intended to provide information

- Show notifications about the alerts and use a badge in the NavBar with the alert count

## Snapshots

![Explorer Snapshot](https://github.com/albertoaer/director/assets/24974091/f7f7c94f-bdbd-4651-9313-0009eba299c4)
![Alerts Snapshot](https://github.com/albertoaer/director/assets/24974091/a2cf9a0c-8780-4deb-8ba0-66f1d34504dc)
![Explorer Chart Snapshot](https://github.com/albertoaer/director/assets/24974091/17b44232-efc4-45b5-8e80-680825d849ae)
![Startup Snapshot](https://github.com/albertoaer/director/assets/24974091/8166fb96-ae39-452c-b943-0dcbceb11494)
![Edit Alerts Snapshot](https://github.com/albertoaer/director/assets/24974091/6f99f3ca-20ec-4808-9497-0e4303399355)

### Example with light theme

![Light Theme Explorer Snapshot](https://github.com/albertoaer/director/assets/24974091/c612f5cf-1a4e-43d9-b696-f44077afd2b7)