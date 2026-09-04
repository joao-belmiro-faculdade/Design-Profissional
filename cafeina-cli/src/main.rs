use std::collections::HashMap;

fn mensagens_erro() -> HashMap<&'static str, &'static str> {
    let mut mensagens = HashMap::new();

    mensagens.insert(
        "CAFEINA_BAIXA",
        "Cafeína insuficiente. Até o compilador perdeu a vontade de trabalhar."
    );

    mensagens.insert(
        "FOCO_404",
        "Erro 404: Foco não encontrado. Tome um expresso."
    );

    mensagens.insert(
        "BUILD_BLOQUEADA",
        "Build bloqueada. Seu café não foi suficiente para convencer o compilador."
    );

    mensagens
}

fn main() {
    let erros = mensagens_erro();

    let cafeina = 35;
    let limite = 51;

    println!("=== Cafeína CLI ===");
    println!("Nível de cafeína: {} mg/L", cafeina);
    println!("Limite mínimo: {} mg/L", limite);

    if cafeina < limite {
        println!();
        println!("ERRO: {}", erros["CAFEINA_BAIXA"]);
        println!("ERRO: {}", erros["FOCO_404"]);
        println!("ERRO: {}", erros["BUILD_BLOQUEADA"]);
        println!();
        println!("A compilação foi recusada.");
    } else {
        println!("Cafeína suficiente. A build está liberada!");
    }
}