use std::io;

fn main() {
    task_options();
}

fn task_options() {
    let _task_types = [
        "Contas",
        "ComprasSuper",
        "ComprasVar",
        "Saude",
        "Trabalho",
        "Estudos",
        "Urgentes",
    ];

    println!(
    "Tipo de tarefas: 
    \n 1) Contas 
    \n 2) Compras (Supermercado) 
    \n 3) Compras (Varejo) 
    \n 4) Saúde 
    \n 5) Trabalho 
    \n 6) Estudos 
    \n 7) Urgentes 
    \n"
    );

    let mut option = String::new();

    io::stdin().read_line(&mut option).expect("Falha ao ler a linha");

    println!("Você escolheu a opção: {}", option);
}

fn tasks_list() {
    println!("Lista de tarefas");
}
