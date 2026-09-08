# Design-Profissional

Uma CLI em Rust que lê o nível de cafeína no seu sangue (via integração com smartwatch) e traduz o seu código em tempo de compilação. Se o nível de cafeína estiver baixo, ela se recusa a compilar e gera mensagens de erro sarcásticas.

Colaboradores:
João Pedro Belmiro
Samuel Curcelli Cordovil
Henrique Dias Nazar
Ygor Luis Fiqueiredo B.
Lucas de Melo dos Anjos

## Modo Turbo-Espresso

A flag `--turbo` ativa o modo Turbo-Espresso da Cafeína CLI.

Nesse modo, a ferramenta considera que o desenvolvedor tomou 3 cafés na última hora e libera a build mesmo quando o nível de cafeína estiver abaixo do limite.

Uso:

```bash
cargo run -- --turbo

## Verificacao de cafeina

A Cafeina CLI verifica o nivel de cafeina antes de liberar a compilacao.
Caso o nivel esteja abaixo do limite configurado, a ferramenta bloqueia a build.
## Limite de cafeina

O limite minimo pode ser configurado pelo usuario utilizando a flag --limite.
Quando o nivel registrado estiver abaixo desse limite, a compilacao sera bloqueada.