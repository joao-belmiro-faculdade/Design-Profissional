# Comandos da CLI

Referencia dos comandos disponiveis na cafeina-cli.

## cafeina pair

Procura smartwatches por Bluetooth durante 30 segundos e pede confirmacao
no relogio para concluir o pareamento.

## cafeina status

Exibe a ultima leitura registrada, o nivel estimado em mg/L e se a build
esta liberada ou bloqueada.

## cafeina build

Executa a verificacao de nivel e, se aprovada, repassa a chamada para o
cargo build.

## Flags globais

| Flag        | Descricao                                      | Padrao |
| ----------- | ---------------------------------------------- | ------ |
| --limite N  | Sobrescreve o limite minimo em mg/L            | 51     |
| --sem-cache | Ignora a ultima leitura e forca nova consulta  | off    |
| --json      | Retorna a saida em JSON em vez de texto        | off    |
| --verbose   | Exibe os dados brutos recebidos do smartwatch  | off    |
