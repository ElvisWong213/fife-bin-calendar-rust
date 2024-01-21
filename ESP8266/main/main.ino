#include <ESP8266WiFi.h>
#include <ESP8266HTTPClient.h>
#include <WiFiClient.h>
#include <chrono>
#include <ArduinoJson.h>
#include <unordered_map>

#ifndef STASSID
#define STASSID "WiFi SSID"
#define STAPSK "WiFi Password"
#endif

const char* ssid = STASSID;
const char* password = STAPSK;

const int whiteLED = 5;
const int greenLED = 4;
const int blueLED = 12;
const int yellowLED = 13;

const double timeout = 10.0; // WiFi connect timeout
const String url = "http://192.168.1.8:8888"; // Servier IP address

void setup() {
  // Setup Led output
  pinMode(whiteLED, OUTPUT);
  pinMode(greenLED, OUTPUT);
  pinMode(blueLED, OUTPUT);
  pinMode(yellowLED, OUTPUT);
  // Connect to WiFi
  if (connectToWifi() == false) {
    WifiConnectFail();
  }
  // Send out network request to server
  networkRequest();
  // Sleep
  Serial.println("Sleep....zzzzz");
  ESP.deepSleep(0);
}

void loop() {
}

// Loading phase LED indicator 
void loadingLED(long ms) {
  digitalWrite(whiteLED, HIGH);
  delay(ms);
  digitalWrite(whiteLED, LOW);
  digitalWrite(greenLED, HIGH);
  delay(ms);
  digitalWrite(greenLED, LOW);
  digitalWrite(blueLED, HIGH);
  delay(ms);
  digitalWrite(blueLED, LOW);
  digitalWrite(yellowLED, HIGH);
  delay(ms);
  digitalWrite(yellowLED, LOW);
}

// Fail to connect to WiFi LED indicator 
void WifiConnectFail() {
  long time = 500;
  while (true) {
    digitalWrite(whiteLED, HIGH);
    digitalWrite(greenLED, HIGH);
    digitalWrite(blueLED, HIGH);
    digitalWrite(yellowLED, HIGH);
    delay(time);
    digitalWrite(whiteLED, LOW);
    digitalWrite(greenLED, LOW);
    digitalWrite(blueLED, LOW);
    digitalWrite(yellowLED, LOW);
    delay(time);
  }
}

// Fail to decode json file LED indicator 
void DecodeJsonError() {
  while (true) {
    long time = 500;
    digitalWrite(whiteLED, HIGH);
    digitalWrite(greenLED, HIGH);
    digitalWrite(blueLED, LOW);
    digitalWrite(yellowLED, LOW);
    delay(time);
    digitalWrite(whiteLED, LOW);
    digitalWrite(greenLED, LOW);
    digitalWrite(blueLED, HIGH);
    digitalWrite(yellowLED, HIGH);
    delay(time);
  }
}

// Connect to WiFi
// return true (WiFi connected)
// return false (Unable to connect to the WiFi)
bool connectToWifi() {
  Serial.begin(115200);
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
    loadingLED(100);
  }
  Serial.println("");
  Serial.println("WiFi connected");
  Serial.println("IP address: ");
  Serial.println(WiFi.localIP());
  return true;
}

// Create LED Hashmap
std::unordered_map<std::string, int> LedHashMap() {
  Serial.println("Start Creating HashMap: ");
  std::unordered_map<std::string, int> hashmap;
  hashmap.insert({"Grey", whiteLED});
  hashmap.insert({"Green", greenLED});
  hashmap.insert({"Blue", blueLED});
  hashmap.insert({"Brown", yellowLED});
  return hashmap;
}

// Decode response to LED Pins array
std::vector<int> decodeNetworkResponseLEDPins(String input) {
  std::vector<int> ledPins = {};

  // If input is empty, return empty array
  if (input.isEmpty()) {
    return ledPins;
  }

  JsonDocument doc;

  DeserializationError error = deserializeJson(doc, input);
  
  if (error) {
    Serial.println("deserializeJson() failed: ");
    Serial.println(error.c_str());
    return ledPins;
  }

  JsonArray colors = doc["colors"];
  const char* update_date = doc["update_date"];

  std::unordered_map<std::string, int> hashmap = LedHashMap();
  Serial.println("Start Decoding: ");
  for (int i = 0; i < colors.size(); i++) {
    ledPins.push_back(hashmap[colors[i].as<std::string>()]);
  }
  Serial.println("Finish Decoding: ");
  return ledPins;
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
      std::vector<int> ledPins = decodeNetworkResponseLEDPins(http.getString());
      // Turn on correspods LED
      for (int ledPin : ledPins) {
        digitalWrite(ledPin, HIGH);
      }
      delay(10000);
      // Turn off correspods LED
      for (int ledPin : ledPins) {
        digitalWrite(ledPin, LOW);
      }
    } else { // Server responded error
      Serial.println("Error code: ");
      Serial.println(httpResponseCode);
    }
    http.end();
  } else {
    Serial.print("WiFi Disconnected"); // WiFi disconnected
    WifiConnectFail();
  }
}
