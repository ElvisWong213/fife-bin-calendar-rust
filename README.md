# Fife Bin Calendar Rust

## About this project
Fife Bin Calendar is a project utilizing Rust to host API servers using the Rust Rocket framework. The project also incorporates an ESP8266 development board to connect custom API servers, enabling users to conveniently access and display the scheduled bin collections for the upcoming week. This innovative combination of technologies offers a seamless and efficient solution for managing waste disposal schedules, enhancing user experience, and promoting sustainability.

## Features
- Display of bin types for the next collection day
- Convenient access to waste disposal schedules
- Seamless integration with ESP8266 development board

## Framework Used
- Rocket
- Tokio
- Docker
- Arduino JSON

## Requirements
- Rust 1.75+
- Docker 24.0.7+
- Arduino IDE 2.2.1+
- ESP8266

## Setup
### Setup Rust API Services
- Run on docker
  - Pull the image from docker hub
    ```
    docker pull elviswong213/fife-bin-calendar
    ```
  - Run the image (You can change port 8888 to any port you want)
    ```
    docker run --name fife-bin -it -p 8888:8000 elviswong213/fife-bin-calendar
    ```
  - When the container is running, you can follow the terminal’s prompts to enter your postcode and choose your address. The information will be saved in the ﻿uprn.txt file
  - If you want to chage your address. Stop the container, remove the `uprn.txt` file and run the container again.

### Setup ESP8266
- Follow the circuit design to buid the circuit
  ![Circuit Design](https://github.com/ElvisWong213/fife_bin_calendar_rust/assets/40566101/74339eb1-a6bc-409a-936d-04f0a397d4a8)
- Download and install [Arduino IDE](https://www.arduino.cc/en/software)
- Open the Arduino IDE and navigate to the boards manager to install ﻿esp8266. Additionally, use the library manager to install ﻿ArduinoJson.
- Open `﻿ESP8266/main/main.ino`. Change the `WiFi SSID`, `WiFi password`, and the `URL` to your own configuration. Then, upload it to your ESP8266 development board.
  ![Arduino](https://github.com/ElvisWong213/fife_bin_calendar_rust/assets/40566101/a846ec99-7c51-4a0c-9549-0c429e643c6e)


## Screenshots
