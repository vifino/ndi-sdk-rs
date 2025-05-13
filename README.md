# NDI® Software Development Kit bindings for Rust
Unofficial bindings to the [NDI](https://ndi.video/) SDK using vendored headers and dynamic loading.

Dynamic loading ensures easy compilation and upgrading of the NDI runtime.
The NDI Runtime will be loaded on first use of the wrapped NDI functions. If that fails, it'll throw an error, but not panic.
Sucessful usage will require installation of either the redistributables or the SDK on the device.

Redistributables-only (preferred):
- [Windows](http://ndi.link/NDIRedistV6)
- [macOS](http://ndi.link/NDIRedistV6Apple)

Full NDI 6 SDK:
- [Windows](https://downloads.ndi.tv/SDK/NDI_SDK/NDI%206%20SDK.exe)
- [macOS](https://downloads.ndi.tv/SDK/NDI_SDK_Mac/Install_NDI_SDK_v6_Apple.pkg)
- [Linux](https://downloads.ndi.tv/SDK/NDI_SDK_Linux/Install_NDI_SDK_v6_Linux.tar.gz)
- [Android for Linux Host](https://downloads.ndi.tv/SDK/NDI_SDK_Android/Install_NDI_SDK_v6_Android.tar.gz)
- [Android for Windows Host](https://downloads.ndi.tv/SDK/NDI_SDK_Android/NDI%206%20SDK%20%28Android%29.exe)

## Status
See `TODO.md`.

As my (`vifino`) usecase is building an NDI Router, that was the initial target.
Actual Send and Receive is not yet implemented.

## Copyright
NDI® is a registered trademark of Vizrt NDI AB.

The vendored header files (`vendored/include/*`) are MIT Licensed, per the Copyright header on the files and the [Software Distribution](https://web.archive.org/web/20250409115521/https://docs.ndi.video/all/developing-with-ndi/sdk/software-distribution#header-files-ndi_sdk_dir-include-.h) info page.

As per the [licensing page](https://docs.ndi.video/all/developing-with-ndi/sdk/licensing), there are specific restrictions on the resulting product, please confirm your product adheres to those.
The SDK's License Agreement is included at `vendored/NDI SDK License Agreement.pdf` for reference to the terms at the time of inclusion.

