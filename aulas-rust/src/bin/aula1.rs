fn main(){
    //Exer 1
    let mut valor = 10;
    println!("O valor é: {valor}");
    valor = 20;
    println!("O valor Atualizado é: {valor}");

    //Exer 2
    //O valor quando se declara a variavel com uSize n tem um tamanho fico definido podendo ser
    //maior ou menor, impedindo erros de compilamento | Errada!!

    //Resposta Certa:
    //o usize tem um tamanho fixo, mas esse tamanho muda dependendo da arquitetura da máquina (32 bits ou 64 bits).
    //Nós usamos ele porque a memória RAM do computador também é mapeada pela arquitetura.
    //Sendo assim, o usize garante que você sempre tenha o tamanho exato necessário
    //para encontrar qualquer item na memória, evitando o erro de estourar a capacidade do sistema.

    //Exer 3
    let so: (String, u16, bool) = (String::from("linux"), 1, true);
    let (sistema_operacional, porta, ativo) = so;
    println!("Sistema Operacional: {sistema_operacional} | Porta: {porta} | Ativo: {ativo}");

    //Exer 4
    //String com S maisculo e uma cadeia de caracteres que podem ser alteradas com o codigo em execução, já o &str
    //Quando e definida um valor inicial e Quase impossivel alterar a quantidade de caracteres

    //Exer 5
    //Usuario entra com um valor Desconhecido Exe:10
    let entrada = "10";
    //como podemos ver ele entrou com uma string e não com um numero Mas no codigo a gente precisa fazer um calculo
    //E não e possivel usa string para realizar calculos
    //Então usaremos o Shadowing para converter o valor
    let entrada: f64 = entrada.parse().expect("Valor Não é um numero");
    //Assim transformamos em um valor que pode ser utilizado no calculo:
    let soma = entrada + 10.0;
    println!("A conta: {entrada} + 10 = {soma}");

}
