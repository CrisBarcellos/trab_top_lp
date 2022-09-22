use rand::Rng;
use std::fmt::Debug;
use std::io;
use std::str::FromStr;
mod corrida;

use corrida::*;

fn main() {
    let mut escolha: i32 = 1;

    while escolha > 0 {
        println!("Escolha uma função: (Digite 0 para sair)");
        println!("1 - Classificação de Triângulo");
        println!("2 - Corrida"); //Minha intenção aqui era criar uma corrida com N carros. Mas não consegui implementar a tempo.
        escolha = lê("");

        match escolha {
            1 => classificar_triangulo(),
            2 => corrida(),
            _ => println!("Escolha uma função válida"),
        }
    }
}

fn classificar_triangulo() {
    let l1: f32 = lê("Entre com a medida do lado 1 do triangulo: ");
    let l2: f32 = lê("Entre com a medida do lado 2 do triangulo: ");
    let l3: f32 = lê("Entre com a medida do lado 3 do triangulo: ");

    if l1 <= 0.0 || l2 <= 0.0 || l3 <= 0.0 {
        println!("Medida Inválida");
        return;
    }

    if l1 >= l2 + l3 || l2 >= l3 + l1 || l3 >= l1 + l2 {
        println!("Esse triângulo não existe");
        return;
    }

    if l1 == l2 && l2 == l3 {
        println!("Esse triângulo é Equilátero");
    } else if l1 == l2 || l2 == l3 || l3 == l1 {
        println!("Esse triângulo é Isósceles")
    } else {
        println!("Esse triângulo é Escaleno")
    }

    let _escol: i32 = lê("Digite '1' para voltar ao menu");
    print!("{}[2J", 27 as char);
}

fn corrida() {
    println!("Jogo de Corrida. Cadastre dois carros.");

    println!("Cor do Primeiro Carro:");

    let mut carro1 = Carro::novo(lê(""));

    println!("Cor do Segundo Carro:");

    let mut carro2 = Carro::novo(lê(""));

    loop {
        println!("{:?}", carro1);
        println!("{:?}", carro2);

        let num = rand::thread_rng().gen_range(1..3);
        println!("numero{}", num);
        if num == 1 {
            if carro1.correr() == Status::Parado {
                println!("O carro {} é campeão", carro1.cor);
                break;
            }
        } else if num == 2 {
            if carro2.correr() == Status::Parado {
                println!("O carro {} é campeão", carro2.cor);
                break;
            }
        }
    }
}

#[cfg(test)]
mod testes {
    #[test]
    fn problema_transferencia() {
        let a = String::from("s1");
        let _b = a;
        //println!("a string b é:{}", a); essa linha apresenta erro ao tentar utilizar a variável 'a', pois o seu valor já foi movido para a variável 'b'.
    }

    #[test]
    fn problema_emprestimo() {
        let a = String::from("Olá");
        mudar(&a);
    }

    fn mudar(b: &String) {
        //b.push_str(", Rural");//essa linha apresenta erro ao tentar modificar um valor emprestado
    }
}

pub fn lê<T: FromStr>(msg: &str) -> T
where
    <T as FromStr>::Err: Debug,
{
    println!("{}", msg);

    let mut número = String::new();

    io::stdin()
        .read_line(&mut número)
        .expect("Falha na leitura da linha");

    número.trim().parse().unwrap()
}
