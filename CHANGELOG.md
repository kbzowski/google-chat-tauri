# Changelog

## [0.1.0](https://github.com/kbzowski/google-chat-tauri/compare/google-chat-tauri-v0.0.1...google-chat-tauri-v0.1.0) (2026-05-13)


### Features

* **account-index:** configurable Google Chat account index (FEAT-02) ([28ac1fb](https://github.com/kbzowski/google-chat-tauri/commit/28ac1fb12583dfa47b668a1a77bedf346bb0c592))
* **auto-reload:** heartbeat-based stuck page detection reloads webview ([dd39ed3](https://github.com/kbzowski/google-chat-tauri/commit/dd39ed3aa3af3ec4faaf1072ede1f3df4a240337))
* **badge:** unread count event listener with tray state switching ([6834789](https://github.com/kbzowski/google-chat-tauri/commit/6834789b590f5b68e172c6abe567872a61ff5746))
* **build:** auto-fetch Firefox user agent at build time ([518b7b5](https://github.com/kbzowski/google-chat-tauri/commit/518b7b58fbbaba797dc9fdf010dbe4ab90de9cd7))
* **bundle:** activate MSI/NSIS bundles with generated icon set ([b67a515](https://github.com/kbzowski/google-chat-tauri/commit/b67a5158ac254a6e40f813358c4e0b6a6c45d532))
* **capabilities:** grant default permissions for all registered plugins ([97a5e77](https://github.com/kbzowski/google-chat-tauri/commit/97a5e771d8d834ccafbcc9fbe3e361c58feff856))
* **config:** add Windows bundle targets and installer locales ([bdcf3bf](https://github.com/kbzowski/google-chat-tauri/commit/bdcf3bf1dc6d60a1e7163160697c788904834c5e))
* **config:** typed AppSettings schema with persisted store ([371a831](https://github.com/kbzowski/google-chat-tauri/commit/371a83184f2f7e1ae5859eb0c20d7faf340fb8d6))
* **crash-report:** native dialog on panic with GitHub issue link ([7b30843](https://github.com/kbzowski/google-chat-tauri/commit/7b308438e2d3f5b7630793197fd6edeb2f8bbf8f))
* **deps:** add reqwest, tokio, url runtime crates and mockito for tests ([6534cf8](https://github.com/kbzowski/google-chat-tauri/commit/6534cf848c662756973f9ec1bc6fe92557155805))
* **external-links:** whitelist guard routing external URLs to default browser ([048dc24](https://github.com/kbzowski/google-chat-tauri/commit/048dc24aeb37c88df4b5b4497e42c0ba609b6d53))
* **focus-mode:** Do Not Disturb mode suppressing notifications for configured duration ([7791591](https://github.com/kbzowski/google-chat-tauri/commit/77915919cedc98542c77aa3547c29698671cf360))
* **frontend:** set up Vite + Svelte 5 + TypeScript scaffold for aux windows ([8842582](https://github.com/kbzowski/google-chat-tauri/commit/884258220a6ae8ad9ad5adf9fa5e76c6801fade8))
* **global-shortcut:** register configurable shortcut from settings and re-register on demand ([7e1318e](https://github.com/kbzowski/google-chat-tauri/commit/7e1318eeab1e9fe947de96244558d05fe5677ea1))
* **icons:** generate application icon set for installer bundles ([fa4abd2](https://github.com/kbzowski/google-chat-tauri/commit/fa4abd265276d0e7234691f8d0c3d781d39da2d2))
* **injection:** build pipeline for bundled injection script ([8667e92](https://github.com/kbzowski/google-chat-tauri/commit/8667e92279d572a3fd5cfdc8e00ccded7b057c2b))
* **injection:** custom CSS injection from settings store ([c8de401](https://github.com/kbzowski/google-chat-tauri/commit/c8de401eb4a08d59f2d34b5340cabc03d5ec7a88))
* **injection:** favicon mutation observer emitting state changes ([6cb71aa](https://github.com/kbzowski/google-chat-tauri/commit/6cb71aad0e324ffd01d865ea0d37de398f5ee3a5))
* **injection:** notification override forwarding clicks to Rust ([ec78e73](https://github.com/kbzowski/google-chat-tauri/commit/ec78e73c241314db42493949696f9d05a8b05111))
* **injection:** search shortcut focusing chat search input ([c77d068](https://github.com/kbzowski/google-chat-tauri/commit/c77d0685e6ae7f2e618f52f3eb4db37fd95ea472))
* **injection:** structured logger and uncaught error capture ([a5ff388](https://github.com/kbzowski/google-chat-tauri/commit/a5ff3886fd54ab4b30b32039cf4c5c1347b4c19a))
* **injection:** unread counter watching DOM mutations ([5cdf811](https://github.com/kbzowski/google-chat-tauri/commit/5cdf81114b050c8b934a4201d756ac8301130a24))
* **injection:** zoom controls with persistence ([f7df3af](https://github.com/kbzowski/google-chat-tauri/commit/f7df3af8e593fd0f0587f20934fa44c4e6cef9dc))
* **menu:** application menu with File/Edit/View/History/Preferences/Help ([d6976e6](https://github.com/kbzowski/google-chat-tauri/commit/d6976e6d15359b05ace767ade57b6418dec7253c))
* **notifications:** foreground main window on notification click ([bbb4485](https://github.com/kbzowski/google-chat-tauri/commit/bbb4485a14b02346819c5cd47ac6dbd1cd1837cc))
* **online-watchdog:** periodic connectivity poll with auto offline window ([3dc31b7](https://github.com/kbzowski/google-chat-tauri/commit/3dc31b71035fed34c3ca8a2b30e6c71ba2cde153))
* **online:** Google connectivity ping command with mockable URL ([95481da](https://github.com/kbzowski/google-chat-tauri/commit/95481dafd589136485fb3dc8c383fdca51c873d7))
* **plugins:** register tauri plugins for single-instance, log, window-state, store, autostart, notification, shell, dialog, global-shortcut, opener, updater, process, clipboard-manager ([2d52581](https://github.com/kbzowski/google-chat-tauri/commit/2d52581af7a9b5e7faeb30027f78502138e848ba))
* **taskbar:** Windows progress bar feedback during page load ([32b2726](https://github.com/kbzowski/google-chat-tauri/commit/32b2726feb16937ecc8e1d7fd44fde47f11a428b))
* **theme:** WebView setTheme on startup and live apply-theme listener ([b5725a0](https://github.com/kbzowski/google-chat-tauri/commit/b5725a08b27381dc02d818e0d0294bb1be3ed47f))
* **tray:** system tray with normal/badge/offline icon states ([68dd2fe](https://github.com/kbzowski/google-chat-tauri/commit/68dd2fe9d78c4003cd2132df389c315685ec21b4))
* **ui:** About dialog with debug info and clipboard copy ([ec90685](https://github.com/kbzowski/google-chat-tauri/commit/ec9068517bae47eb962d2406f57bfa1702c9ca26))
* **ui:** App.svelte routes by current window label ([77a2c94](https://github.com/kbzowski/google-chat-tauri/commit/77a2c94003a191063edffaafa0418831408e029d))
* **ui:** live apply settings via emit events and Keyboard Shortcuts dialog ([d00dfec](https://github.com/kbzowski/google-chat-tauri/commit/d00dfecb80605efa9ee9ebeafcc02c1d6205cdf3))
* **ui:** Offline page with auto-retry countdown ([ffc217f](https://github.com/kbzowski/google-chat-tauri/commit/ffc217f10a07b9d58d8c52d0b1d242f981542f7f))
* **ui:** Settings panel with toggles for all config options ([7287fb3](https://github.com/kbzowski/google-chat-tauri/commit/7287fb3588d1c9258dd27a85ae7884d95d2a22cf))
* **ui:** shared settings store, typed IPC wrappers and Toggle component ([0bcc252](https://github.com/kbzowski/google-chat-tauri/commit/0bcc25226629b6f9910fa3551939aa0d8e723500))
* **updater:** auto-check on startup after 5s when enabled in settings ([c1f4652](https://github.com/kbzowski/google-chat-tauri/commit/c1f4652753008c36eaa5452e12fc26da5431567d))
* **updater:** real pubkey, manual check command and Help menu wire-up ([22cfc3d](https://github.com/kbzowski/google-chat-tauri/commit/22cfc3d6c598bf6acbec4a6edf36a889ccceb9b5))
* **window-state:** apply always-on-top and minimize-to-tray from settings ([b42972a](https://github.com/kbzowski/google-chat-tauri/commit/b42972a57e8b92a60dbdbb10be2d4236f2562542))
* **window-title:** show unread count in window title when enabled ([bbb46ee](https://github.com/kbzowski/google-chat-tauri/commit/bbb46ee2ad3edc0bdf0cdfc27e8abaea1571422a))
* **window:** close-to-tray, start-hidden flag and toggle helpers ([01fc123](https://github.com/kbzowski/google-chat-tauri/commit/01fc123dc89dd2e5a2ba371bd3c96ccbdaf04791))


### Bug Fixes

* **config:** add updater plugin placeholder config so dev start does not panic ([b82591f](https://github.com/kbzowski/google-chat-tauri/commit/b82591fe9dc87e8981a7bebd428210b3e428f647))
* **external-links:** allow Google 2FA and reCAPTCHA hosts in webview, log navigation decisions ([b28c3f9](https://github.com/kbzowski/google-chat-tauri/commit/b28c3f915044307ba86f9bad629fffa30f5ca51e))
* **external-links:** allow window.open(_blank) via shell:allow-open and patch Notification.permission setter ([5139d4d](https://github.com/kbzowski/google-chat-tauri/commit/5139d4d999728b0d130d6726266903bd76401494))


### Documentation

* **release:** document release-please workflow and signing key handling ([10295c6](https://github.com/kbzowski/google-chat-tauri/commit/10295c66ce974944bee4b7ca45eaa388163858ef))
