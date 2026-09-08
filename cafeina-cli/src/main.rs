use std::collections::HashMap;
use std::env;

fn mensagens_erro() -> HashMap<&'static str, &'static str> {
    let mut mensagens = HashMap::new();

    mensagens.insert(
        "CAFEINA_BAIXA",
        "Cafeina insuficiente. Ate o compilador perdeu a vontade de trabalhar."
    );

    mensagens.insert(
        "FOCO_404",
        "Erro 404: Foco nao encontrado. Tome um expresso."
    );

    mensagens.insert(
        "BUILD_BLOQUEADA",
        "Build bloqueada. Seu cafe nao foi suficiente para convencer o compilador."
    );

    mensagens
}

fn main() {
    let erros = mensagens_erro();

    let argumentos: Vec<String> = env::args().collect();
    let turbo = argumentos.iter().any(|arg| arg == "--turbo");

    let cafeina = 35;
    let limite = 51;

    println!("=== Cafeina CLI ===");
    println!("Nivel de cafeina: {} mg/L", cafeina);
    println!("Limite minimo: {} mg/L", limite);

    if turbo {
        println!();
        println!("MODO TURBO-ESPRESSO ATIVADO!");
        println!("Voce tomou 3 cafes na ultima hora.");
        println!("Os avisos de sintaxe serao ignorados.");
        println!("A build esta liberada!");
        return;
    }

    if cafeina < limite {
        println!();
        println!("ERRO: {}", erros["CAFEINA_BAIXA"]);
        println!("ERRO: {}", erros["FOCO_404"]);
        println!("ERRO: {}", erros["BUILD_BLOQUEADA"]);
        println!();
        println!("A compilacao foi recusada.");
    } else {
        println!("Cafeina suficiente. A build esta liberada!");
    }
}