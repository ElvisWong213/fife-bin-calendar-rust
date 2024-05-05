#include <ESP8266WiFi.h>
#include <ESP8266HTTPClient.h>
#include <WiFiClient.h>
#include <chrono>
#include <ArduinoJson.h>
#include <unordered_map>

#include <SPI.h>
#include <Wire.h>
#include <Adafruit_GFX.h>
#include <Adafruit_SSD1306.h>

#ifndef STASSID
#define STASSID "WiFi SSID"
#define STAPSK "WiFi Password"
#endif

#define SCREEN_WIDTH 128 // OLED display width, in pixels
#define SCREEN_HEIGHT 32 // OLED display height, in pixels
#define OLED_RESET     -1 // Reset pin # (or -1 if sharing Arduino reset pin)
#define SCREEN_ADDRESS 0x3C ///< See datasheet for Address; 0x3D for 128x64, 0x3C for 128x32
Adafruit_SSD1306 display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, OLED_RESET);


const char* ssid = STASSID;
const char* password = STAPSK;

const double timeout = 10.0; // WiFi connect timeout
const String url = "http://192.168.1.8:8888"; // Servier IP address

void displayContent(const char* content, uint8_t text_size = 1) {
  display.ssd1306_command(SSD1306_DISPLAYON);
  display.clearDisplay();
  display.setTextSize(text_size);
  display.setTextColor(SSD1306_WHITE);
  display.setCursor(0, 0);
  display.print(content);
  display.display();
}

void displayOff() {
  display.clearDisplay();
  display.ssd1306_command(SSD1306_DISPLAYOFF);
}

void setup() {
  Serial.begin(115200);

  if(!display.begin(SSD1306_SWITCHCAPVCC, SCREEN_ADDRESS)) {
    Serial.println(F("SSD1306 allocation failed"));
    for(;;); // Don't proceed, loop forever
  }
  
  display.display();
  delay(1000);
  display.clearDisplay();
  display.display();
  displayContent("Connecting to WiFi");

  // Connect to WiFi
  if (connectToWifi() == false) {
    Serial.print("Fail to connect WiFi");
    displayContent("Fail to connect WiFi");
  }
  // Send out network request to server
  displayContent("Sending request");
  delay(1000);
  networkRequest();
  // Sleep
  Serial.println("Sleep....zzzzz");
  displayOff();
  ESP.deepSleep(0);
}

void loop() {
}

// Connect to WiFi
// return true (WiFi connected)
// return false (Unable to connect to the WiFi)
bool connectToWifi() {
  Serial.println();
  Serial.print("Connecting WIFI");
  WiFi.mode(WIFI_STA);
  WiFi.begin(ssid, password);
  auto start = std::chrono::system_clock::now();
  while (WiFi.status() != WL_CONNECTED) {
    auto end = std::chrono::system_clock::now();
    std::chrono::duration<double> duration = end - start;
    if (duration.count() > timeout) {
      Serial.print("Unable to connect to the WiFi");
      return false;
    }
    Serial.print(".");
  }
  Serial.println("");
  Serial.println("WiFi connected");
  Serial.println("IP address: ");
  Serial.println(WiFi.localIP());
  return true;
}

// Decode response to LED Pins array
void decodeNetworkResponse(String input) {
  // If input is empty, return empty array
  if (input.isEmpty()) {
    return;
  }

  JsonDocument doc;

  DeserializationError error = deserializeJson(doc, input);
  
  if (error) {
    Serial.println("deserializeJson() failed: ");
    Serial.println(error.c_str());
    return;
  }

  JsonArray colors = doc["colors"];
  const char* update_date = doc["update_date"];
  const char* collect_date = doc["collect_date"];
  std::string display_color = "";

  Serial.println("Start Decoding: ");

  for(JsonVariant color: colors) {
    display_color.append(color.as<std::string>());
    display_color.append(" ");
  }

  Serial.println("Finish Decoding: ");
  if (collect_date == NULL) {
    rotateDisplayContent("Unable to get collect data", display_color.c_str(), 5, 2000);
    return;
  }
  rotateDisplayContent(collect_date, display_color.c_str(), 5, 2000);
}

// Send out network request to server
void networkRequest() {
  // Check WiFi connect status
  if(WiFi.status() == WL_CONNECTED) { // Connected to WiFi
    WiFiClient client;
    HTTPClient http;

    http.begin(client, url);
    int httpResponseCode = http.GET();

    if (httpResponseCode == 200) { // Server responded 200 status code
      Serial.println(httpResponseCode);
      Serial.println(http.getString());
      decodeNetworkResponse(http.getString());
    } else { // Server responded error
      Serial.println("Error code: ");
      Serial.println(httpResponseCode);
    }
    http.end();
  } else {
    Serial.print("WiFi Disconnected"); // WiFi disconnected
  }
}

void rotateDisplayContent(const char* content1, const char* content2, int times, unsigned long delay_time) {
  for(int i = 0; i < times; i++) {
    displayContent(content1, 2);
    delay(delay_time);
    displayContent(content2, 2);
    delay(delay_time);
  }
}

