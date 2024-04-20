# Firmware Setup via FTP
---

### Prerequisites
- A PS4 with jailbreakable firmware, recommended is 9.00, the latest firmware with a jailbreak.
- 650~ MB of free disk space for Firmware.
- Computer and PS4 connected to the same network.

### Goldhen
This is the most common way to activate your PS4's FTP server as it is built into the jailbreak.

1. Activate FTP Server in the Goldhen Settings

https://github.com/obhq/obliteration/assets/45863583/cc76a763-22cf-42d5-be00-2490df530fd4

   Go to Settings, then Goldhen, Server Settings, and enable Enable FTP Server. You'll be give an IP address and a port number (Usually 2121)

2. Type the IP address and Port into the wizard.

![image](https://github.com/obhq/obliteration/assets/45863583/ab0539f0-bda9-4d36-8f51-7e5290fc9b72)

   Type the it into the format of IP:PORT. As shown above, I'd type in `192.168.86.37:2121` for my PS4.

   **DO NOT ENABLE `Explicit Decryption` FOR GOLDHEN! IT IS AUTOMATICALLY ENABLED.** 

3. Hit next, wait for the files to transfer, then enjoy!

   All files are transferred to a folder called /system within' the folder you selected.
   For example, if you selected your system directory to be `F:/stuff/`, your firmware would be stored inside of `F:/stuff/system`.
