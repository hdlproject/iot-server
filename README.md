# IoT Server
This Rust project contains all functionalities needed to server IoT nodes such as:
- device management
- sensor data recording
- sensor data inferencing

## ChatGPT Prompting
- System:
```text
Give user a concise and consistent answer format.
Use the following steps to respond the user query:
Step 1. The user will give several sensor data in JSON.
Step 2. Parse the data accordingly and give them the insight related to the condition and what should they do.
```
- User:
```text
{"temperature_in_celsius": 36, "humidity": 20}
```

## Config
Here is the necessary config to run thus project
- OPENAI_API_URL=https://api.openai.com/v1/models
- OPENAI_API_KEY=<open api key>
- POSTGRES_URL=postgres://admin:admin@localhost:5432/iot-manager

## How to Operate
### Migrate Database
```shell
$ cargo run --bin tool migrate-database --dsn=postgres://admin:admin@localhost:5432/iot-manager
```
### Run Web Server
```shell script
$ <set all necessary env vars> cargo run -- bin <package name>
```
### Run Web Server using Docker
```shell script
$ docker build --build-arg APP_NAME=general -t general .
$ docker run \
  -e OPENAI_API_URL="https://api.openai.com" \
  -e OPENAI_API_KEY="<openai api key>" \
  -p 8081:8081 \
  general

$ docker build --build-arg APP_NAME=iot-manager -t iot-manager .
$ docker run \
  -e POSTGRES_URL="postgres://admin:admin@host.docker.internal:5433/iot-manager" \
  -e GENERAL_SERVICE_URL="host.docker.internal:8081" \
  -p 8082:8082 \
  iot-manager
```

## Log
### OpenAI Request Response
```json
{
    "model": "gpt-3.5-turbo",
    "messages": [
      {
        "role": "system",
        "content": "Give user a concise and consistent answer format. Use the following steps to respond the user query: Step 1. The user will give several sensor data in JSON. Step 2. Parse the data accordingly and give them the insight related to the condition and what should they do."
      },
      {
        "role": "user",
        "content": "{\"temperature_in_celsius\": 36, \"humidity\": 20}"
      }
    ]
  }
```
```json
{
    "id": "chatcmpl-ALAN0nYVV1TGS04mACreSQOJ2uG69",
    "object": "chat.completion",
    "created": 1729608114,
    "model": "gpt-3.5-turbo-0125",
    "choices": [
        {
            "index": 0,
            "message": {
                "role": "assistant",
                "content": "Based on the provided sensor data:\n- Temperature: 36°C\n- Humidity: 20%\n\nInsight:\n- The temperature is within the normal range.\n- The humidity is quite low, which may lead to dry air. Consider increasing humidity levels to improve comfort and reduce potential health issues related to dry air.",
                "refusal": null
            },
            "logprobs": null,
            "finish_reason": "stop"
        }
    ],
    "usage": {
        "prompt_tokens": 82,
        "completion_tokens": 64,
        "total_tokens": 146,
        "prompt_tokens_details": {
            "cached_tokens": 0
        },
        "completion_tokens_details": {
            "reasoning_tokens": 0
        }
    },
    "system_fingerprint": null
}
```

## TODO
- 
