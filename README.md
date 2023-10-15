# Rinha de Frontend

Este é o repositório da Rinha de Frontend. Esta é uma brincadeira e um desafio de código inspirada pela _"Rinha de Backend"_, uma iniciativa criada por Francisco Franceschi.

O desafio consiste em montar um sistema Frontend com a stack que você quiser, e estressá-lo de acordo com as especificações abaixo, simplesmente pra ver o que acontece. Quem tirar a melhor performance nos critérios aqui estabelecidos, vence.

## Run

- 1 Install Wasm
```sh
rustup target add wasm32-unknown-unknown
```
- 2 Install Trunk
```sh
cargo install --locked trunk
```
- 3 Run Trunk
```sh
trunk serve --open
```

## Build
```sh
trunk serve --release --public-url=/rinha-frontend
```