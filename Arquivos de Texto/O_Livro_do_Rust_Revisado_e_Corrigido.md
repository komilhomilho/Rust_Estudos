# O GRANDE LIVRO DO RUST: Tipagem Profunda e Prática Aplicada
**Edição Definitiva, Corrigida e Expandida: Teoria Sincronizada com 25 Desafios Práticos**

---

## Capítulo 1: O Ecossistema e a Filosofia Aprofundada

Rust é uma linguagem compilada "Ahead of Time" (AOT). Isso significa que, ao contrário de linguagens interpretadas (como Python) ou que rodam em máquinas virtuais (como Java ou C#), o compilador do Rust (`rustc`) traduz todo o seu código diretamente para linguagem de máquina nativa do sistema operacional destino (seja um kernel Linux, um servidor Windows ou um microcontrolador).

O coração da produtividade no Rust é o **Cargo**, seu gerenciador de pacotes e sistema de build. Ele padroniza a forma como projetos são criados, testados e distribuídos.

### O Poder do Cargo na Prática

#### Exemplo 1: Criando e gerenciando dependências
Quando você roda `cargo new meu_projeto`, o Cargo cria uma estrutura com um arquivo `Cargo.toml`. Este é o manifesto do seu projeto.
```toml
# Arquivo: Cargo.toml
[package]
name = "meu_projeto"
version = "0.1.0"
edition = "2021"

[dependencies]
# Adicionando uma biblioteca externa (Crate) para gerar números aleatórios
rand = "0.8.5" 
```

#### Exemplo 2: O fluxo de desenvolvimento diário
No dia a dia, compilar o código inteiro pode ser lento. Por isso, usamos ferramentas específicas para cada momento:
```bash
# 1. Durante a escrita do código (Validação contínua)
cargo check
# Explicação: Verifica a tipagem, borrow checker e sintaxe, mas NÃO gera o arquivo binário. É extremamente rápido e ideal para rodar a cada alteração no editor.

# 2. Para testes locais
cargo run
# Explicação: Compila o projeto em modo "Debug" (com metadados que ajudam a encontrar erros) e executa imediatamente.

# 3. Para enviar para o cliente ou produção
cargo build --release
# Explicação: Ativa otimizações agressivas do compilador. O código demora mais para compilar, mas o binário final pode rodar até 10x mais rápido e ocupar menos espaço em disco.
```

#### Exemplo 3: Ferramentas auxiliares de padronização
O ecossistema Rust brilha por ter ferramentas nativas de qualidade de código:
```bash
# Formata todo o código do projeto no padrão oficial do Rust (indentação, espaços)
cargo fmt

# Roda o Linter oficial (Clippy), que analisa seu código e sugere melhorias de performance e estilo
cargo clippy
```

### 🛠️ Exercícios do Capítulo 1
1. Qual a diferença fundamental de entrega de software entre uma linguagem compilada (Rust) e uma interpretada (JavaScript/Python)?
2. Explique a diferença de velocidade e propósito entre `cargo run` (modo debug) e `cargo build --release`.
3. Você adicionou uma nova biblioteca no seu projeto, mas quer apenas verificar se não quebrou nenhuma regra de memória no resto do código, sem precisar executar o programa. Qual comando você usa?
4. O que é o arquivo `Cargo.toml` e qual a sua função em um projeto Rust?
5. Em linguagens C, a falha de segmentação (Segmentation Fault) ocorre frequentemente em tempo de execução. Qual é o mecanismo do Rust para evitar que o código sequer compile se houver esse risco?

---

## Capítulo 2: Tipos de Dados e Variáveis (Análise Profunda)

No Rust, a segurança é a prioridade máxima. O compilador precisa saber o tamanho exato de cada tipo de dado na memória.

### 2.1. O Superpoder da Mutabilidade (`mut`)
Por padrão, **todas as variáveis são imutáveis**. Isso previne que dados sejam alterados acidentalmente, o que é a maior causa de bugs em sistemas complexos.

#### Exemplo 1: Imutabilidade vs Mutabilidade
```rust
fn main() {
    let imutavel = 10;
    // imutavel = 20; // ISSO GERA ERRO DE COMPILAÇÃO!

    let mut mutavel = 10; // A palavra 'mut' permite a alteração
    mutavel = 20; // Funciona perfeitamente
}
```

### 2.2. Tipos Escalares (Valores Únicos)
O Rust possui tipos primitivos rigorosos. Inteiros podem ser **Signed** (com sinal `i`) ou **Unsigned** (sem sinal `u`). Os tipos sem sinal dobram sua capacidade máxima porque não gastam um bit para guardar o sinal negativo.

#### Exemplo 1: Inteiros e o tipo `usize`
*   `i32`: O padrão para números inteiros (rápido em processadores modernos).
*   `u8`: Vai de 0 a 255. Ideal para manipular bytes puros.
*   `usize`: O tipo mais importante para sistemas. Seu tamanho depende se o seu processador é de 32 ou 64 bits. **Você deve usar `usize` sempre que for indexar listas ou mapear memória**, pois o limite de uma lista depende fisicamente da RAM da máquina.
```rust
let idade: i32 = 25;
let byte_cor: u8 = 255;
let indice_matriz: usize = 0; 
```

#### Exemplo 2: O comportamento de Overflow
O que acontece se somarmos além do limite de um tipo?
```rust
let a: u8 = 250;
    
// checked_add: Retorna sucesso ou vazio em vez de crashar o programa.
match a.checked_add(10) {
    Some(valor) => println!("Sucesso: {}", valor),
    None => println!("Erro: O valor excedeu a capacidade de 8 bits!"),
}
```

### 2.3. Tipos Compostos (Múltiplos Valores)
Grupos de dados que possuem tamanho fixo conhecido em tempo de compilação.

#### Exemplo 1: Tuplas (Tipos diferentes)
Tuplas agrupam valores de **tipos diferentes**. São muito usadas para retornar mais de um valor de uma função.
```rust
// Uma tupla contendo um texto (String), uma porta (u16) e um status (bool)
let config_banco: (String, u16, bool) = (String::from("PostgreSQL"), 5432, true);

// Desestruturação: extraindo os valores para variáveis isoladas
let (nome, porta, ativo) = config_banco;
println!("Conectando no banco {} na porta {}", nome, porta);
```

#### Exemplo 2: Matrizes/Arrays (Mesmo tipo)
Arrays no Rust têm **tamanho fixo** e todos os elementos devem ser do mesmo tipo. Eles são alocados inteiramente na *Stack* (pilha rápida).
```rust
let meses = ["Jan", "Fev", "Mar", "Abr"];
let zeros = [0; 5]; // Cria um array [0, 0, 0, 0, 0] rapidamente
```

### 2.4. O Problema das Strings: `&str` vs `String`

Este é o conceito mais crítico do Rust.
*   **`&str` (String Slice):** É uma visão rápida e imutável. Vive direto no arquivo binário ou aponta para um pedaço da memória. Super rápido, mas não pode crescer.
*   **`String`:** Um objeto complexo alocado na memória dinâmica (**Heap**). Pode crescer e ser modificado.

#### Exemplo 1: Conversão e Mutabilidade
```rust
let fatia: &str = "Texto estático"; 
let mut texto_dinamico: String = String::from("Olá");
texto_dinamico.push_str(", Mundo!"); // O método push_str só existe na String da Heap
```

### 2.5. Shadowing (Sombreamento)
Diferente da mutabilidade (`mut`), o shadowing permite que você reuse o nome de uma variável com um novo `let`. O compilador destrói a antiga e cria uma nova, permitindo mudar até o TIPO do dado, mantendo o código limpo.

#### Exemplo 1: Shadowing na prática
```rust
let palpite = "42"; // É um texto (&str)
// Sombreamos a variável para convertê-la em número, sem precisar criar 'palpite_int'
let palpite: i32 = palpite.parse().expect("Falha ao converter"); 
```

### 🛠️ Exercícios do Capítulo 2
1. Declare uma variável inteira mutável, inicie com valor 10, mude para 20 e imprima.
2. Você está indexando uma matriz de processos no sistema operacional. Por que você deve usar o tipo `usize` em vez de `u32` ou `u64`?
3. Crie uma Tupla que guarde o nome de um banco de dados (String), a porta (u16) e se ele está ativo (bool). Depois desestruture e imprima as variáveis.
4. Explique com suas palavras a diferença na memória entre `String` (com `S` maiúsculo) e `&str`.
5. Mostre um exemplo em código onde o uso do "Shadowing" (`let` repetido) é superior ao uso de uma variável mutável (`mut`).

---

## Capítulo 3: Ownership e Borrowing (Empréstimo de Memória)

O Rust garante segurança de memória sem um Garbage Collector através do **Ownership** (Posse).
1. Cada valor tem um dono.
2. Só existe um dono por vez.
3. Quando o dono sai de escopo (fim da chave `}`), a função `drop` é chamada, e a memória é limpa.

### 3.1. A Transferência de Posse (Move) e o Drop

Quando um dado vive na Heap (como uma `String`), passá-lo para outra variável **move** a posse. A original é invalidada para evitar "Double Free" (duas variáveis tentando dar `drop` na mesma memória).

#### Exemplo 1: O Move
```rust
let servidor_primario = String::from("192.168.0.1");
let servidor_secundario = servidor_primario; // A posse foi MOVIDA!

// println!("{}", servidor_primario); // GERA ERRO! A variável antiga perdeu os direitos.
println!("Ativo: {}", servidor_secundario);
```

### 3.2. Borrowing (O Empréstimo)

Mover variáveis o tempo todo seria exaustivo. Usamos Referências (`&`) para emprestar sem mudar o dono.
**Regra de Ouro:** Você pode ter infinitas referências imutáveis (`&T`) OU uma única referência mutável (`&mut T`) ao mesmo tempo. Nunca as duas juntas.

#### Exemplo 1: Empréstimo Imutável (Leitura)
```rust
fn calcular_tamanho(texto: &String) -> usize {
    texto.len() // Lê o tamanho. Não rouba a posse.
}

fn main() {
    let ip = String::from("127.0.0.1");
    let tam = calcular_tamanho(&ip); // Empresta a leitura
    println!("'{}' tem tamanho {}", ip, tam); // 'ip' continua válido!
}
```

#### Exemplo 2: Prevenção de Data Race (Condição de Corrida)
```rust
let mut arquivo = String::from("dados.txt");

let leitor1 = &arquivo; // OK
let leitor2 = &arquivo; // OK
// let escritor = &mut arquivo; // ERRO! Não pode escrever enquanto tem gente lendo.
```

### 3.3. Slices (Fatias de Coleções)
Podemos emprestar apenas um pedaço de um Array ou String. Um slice de array é representado por `&[T]`.

#### Exemplo 1: Emprestando um Array
```rust
// Função recebe uma fatia genérica de array de inteiros
fn somar_pedaco(numeros: &[i32]) -> i32 {
    let mut soma = 0;
    for n in numeros { soma += n; }
    soma
}

fn main() {
    let matriz = [10, 20, 30, 40, 50];
    let total = somar_pedaco(&matriz); // Empresta a matriz toda
    println!("Soma: {}", total);
}
```

### 🛠️ Exercícios do Capítulo 3
1. Crie uma String "Rust", passe-a para uma função que apenas lê e imprime seu tamanho (usando `&String`), e depois a imprima na `main` para provar que a posse não foi perdida.
2. Crie uma função que receba uma referência mutável `&mut String` e adicione a palavra " é incrível!" no final dela. Chame-a no `main`.
3. Escreva um código que tente criar duas referências mutáveis (`&mut`) para a mesma variável no mesmo escopo, veja o erro que o compilador gera e explique por que o Rust proíbe isso (Data Race).
4. Crie um Array de números `[10, 20, 30]`. Crie uma função que aceite um empréstimo desse array (um *slice* `&[i32]`) e retorne a soma.
5. Explique a diferença profunda entre passar uma `String` por valor (Move) e passar por referência (`&String`) em termos de liberação de memória (função Drop).

---

## Capítulo 4: Structs, Enums e o Fim do Null

Rust rejeita o paradigma clássico de Orientação a Objetos. Ele prefere a **Composição** através de Structs e a representação de estados através de Enums.

### 4.1. Structs e Implementações
Structs armazenam os dados. Os blocos `impl` abrigam as funções.

#### Exemplo 1: Struct Clássica
```rust
struct Retangulo { largura: u32, altura: u32 }

impl Retangulo {
    // Construtor (Não recebe &self)
    fn novo(l: u32, a: u32) -> Retangulo { Retangulo { largura: l, altura: a } }
    
    // Método de instância
    fn area(&self) -> u32 { self.largura * self.altura }
}
```

### 4.2. Enums com Dados e o Controle Exaustivo (`match`)
Enums no Rust podem carregar diferentes tipos de dados dentro de suas variantes.

#### Exemplo 1: Enum poderoso
```rust
enum StatusWeb {
    Sucesso(u16), // Carrega um código HTTP
    NaoEncontrado, // Sem dados
    ErroInterno(String), // Carrega uma mensagem de erro
}

fn processar(status: StatusWeb) {
    // O match obriga a cobrir TODAS as variantes. Impossível esquecer um caso.
    match status {
        StatusWeb::Sucesso(cod) => println!("OK: {}", cod),
        StatusWeb::NaoEncontrado => println!("404"),
        StatusWeb::ErroInterno(msg) => println!("Falha: {}", msg),
    }
}
```

### 4.3. O Fim do Null: A invenção do `Option<T>`
Tony Hoare, o criador da referência nula, chamou o "Null" de seu *Erro de um bilhão de dólares*. Em linguagens normais, tentar ler um dado que é Null quebra o programa inteiro (NullPointerException).
No Rust, **não existe Null**. Se um valor pode não existir, ele deve ser do tipo `Option`.

#### Exemplo 1: Tratando ausência de forma segura
```rust
// O Option já vem nativo no Rust com duas variantes: Some(valor) ou None
let configuracao: Option<i32> = Some(8080);
let sem_config: Option<i32> = None;

// O compilador TE OBRIGA a extrair o valor de forma segura
match configuracao {
    Some(porta) => println!("Rodando na porta {}", porta),
    None => println!("Porta não definida!"),
}
```

#### Exemplo 2: O atalho `if let`
Se você não quiser escrever um `match` inteiro só para testar o sucesso de um Option, use `if let`:
```rust
let valor: Option<i32> = Some(10);

if let Some(v) = valor {
    println!("Temos um valor: {}", v);
} // Ignora o 'None' silenciosamente
```

### 🛠️ Exercícios do Capítulo 4
1. Crie uma Struct `Livro` com campos `titulo`, `autor` e `paginas`. Crie um bloco `impl` com um método `novo` (construtor) e um método `ler` que imprima uma mensagem.
2. Crie o Enum `StatusWeb` com as variantes: `Sucesso(u16)`, `NaoEncontrado`, `ErroInterno(String)`.
3. Crie uma função que receba o Enum `StatusWeb` e use um `match` para imprimir as mensagens (extraindo os valores para a impressão).
4. O Rust não tem "Null". Explique qual é a solução nativa (O Enum `Option<T>`) para representar a ausência de um valor e por que isso é mais seguro.
5. Instancie uma variável do tipo `Option<i32>` como `Some(10)`. Escreva uma estrutura com `if let` para somar 5 a esse número e imprimi-lo, ignorando o caso `None`.

---

## Capítulo 5: Coleções Dinâmicas e Tratamento de Erros Profissional

Enquanto Arrays possuem tamanho rígido na Stack, as Coleções crescem dinamicamente na Heap.

### 5.1. Vetores (`Vec<T>`)
Matrizes flexíveis, essenciais para listas do dia a dia.

#### Exemplo 1: Criação e Iteração Segura
```rust
let mut precos: Vec<f64> = vec![10.50, 25.0];
precos.push(5.0); // Cresce dinamicamente

for preco in &precos {
    println!("R${}", preco);
}
```

### 5.2. Mapas de Hash (`HashMap<K, V>`)
Dicionários baseados em Chave e Valor. O método `.get()` retorna sempre um `Option`, pois a chave pode não existir.

#### Exemplo 1: Inserindo e Lendo
```rust
use std::collections::HashMap;

let mut notas = HashMap::new();
notas.insert(String::from("Alex"), 95);

match notas.get("Alex") {
    Some(nota) => println!("Nota: {}", nota),
    None => println!("Aluno não encontrado!"),
}
```

### 5.3. Tratamento de Erros sem Try/Catch (`Result<T, E>`)
Não existem exceções invisíveis no Rust. Funções que podem falhar retornam o Enum `Result`, com as variantes `Ok(valor)` ou `Err(erro)`.

#### Exemplo 1: Retornando um Erro Matemático
```rust
fn divisao(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Erro fatal: divisão por zero!"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divisao(10.0, 0.0) {
        Ok(res) => println!("Resultado: {}", res),
        Err(msg) => println!("Ops: {}", msg),
    }
}
```

### 🛠️ Exercícios do Capítulo 5
1. Crie um vetor (`Vec<i32>`), adicione 5 números diferentes usando `.push()`. Faça um loop `for` que multiplique cada número por 2 e imprima.
2. Crie um `HashMap<String, i32>` que armazene nomes de alunos e suas notas. Insira 3 alunos.
3. Usando o `HashMap` criado acima, faça um `.get()` buscando um nome que não existe. Trate o retorno `Option` usando um `match` com mensagem de erro amigável.
4. Escreva uma função que tente dividir dois números de ponto flutuante `f64`. Se o denominador for zero, retorne um `Result::Err("Divisão por zero")`. Se não, retorne `Result::Ok(resultado)`.
5. Na função `main`, chame a função de divisão e trate o sucesso e a falha usando a sintaxe de `match`.

---

## GABARITO E EXPLICAÇÕES (Soluções Comentadas)

### Resoluções - Capítulo 1
1. **Compiladas (Rust/C#)** traduzem todo o código para binário nativo de uma só vez, rodando com máxima performance. **Interpretadas (Python/JS)** necessitam de um motor traduzindo linha a linha em tempo real, gerando lentidão.
2. `cargo run` compila rápido, não otimiza variáveis e injeta metadados de debug. `cargo build --release` aplica otimizações matemáticas extremas, demorando mais para compilar, mas gerando um binário super leve e rápido para o cliente final.
3. `cargo check`. Ele ignora a geração do executável, validando apenas as regras de memória e sintaxe.
4. O `Cargo.toml` orquestra metadados do projeto e as bibliotecas externas (Crates).
5. O Rust usa o **Borrow Checker** em tempo de compilação. Ele mapeia toda a vida útil da variável e impede a compilação se houver risco da memória corromper.

### Resoluções - Capítulo 2
```rust
// 1. Variável mutável padrão
let mut num = 10;
num = 20;
println!("{}", num);

// 2. O limite físico da memória RAM disponível para indexar matrizes depende da arquitetura do SO (32 ou 64 bits). Indexar com 'usize' garante que a leitura da memória funcionará independentemente da máquina.

// 3. Tuplas
let bd: (String, u16, bool) = (String::from("PostgreSQL"), 5432, true);
let (nome, porta, ativo) = bd; 
println!("Banco: {}, Porta: {}, Ativo: {}", nome, porta, ativo);

// 4. `String` é dinâmica, alocada na Heap, pode sofrer mutação (.push_str) e pertence à variável. `&str` é estático, leve e imutável; é apenas um ponteiro para leitura.

// 5. Shadowing vs Mutabilidade
let input = "42"; 
// Mutabilidade (mut) não permite mudar o tipo (de texto para int).
// O Shadowing (let) recria a variável, alterando o tipo elegantemente:
let input: i32 = input.parse().unwrap(); 
```

### Resoluções - Capítulo 3
```rust
// 1. Empréstimo Imutável
fn ler_tamanho(s: &String) -> usize { s.len() }
let texto = String::from("Rust");
println!("Tamanho: {}", ler_tamanho(&texto));
println!("Posso usar a variável: {}", texto);

// 2. Empréstimo Mutável
fn adiciona_incrivel(s: &mut String) { s.push_str(" é incrível!"); }
let mut frase = String::from("Rust");
adiciona_incrivel(&mut frase);

// 3. O Rust proíbe múltiplas referências mutáveis para evitar Data Races. Se duas lógicas escrevem na mesma memória juntas, o dado corrompe.
// let r1 = &mut frase; 
// let r2 = &mut frase; // -> ERRO: cannot borrow `frase` as mutable more than once

// 4. Slice de um Array
fn somar_array(numeros: &[i32]) -> i32 {
    let mut total = 0;
    for n in numeros { total += n; }
    total
}
let arr = [10, 20, 30];
println!("Soma: {}", somar_array(&arr));

// 5. Passar por valor (Move) transfere o dono. No fim da função, o Rust chama a função interna `drop()`, limpando a memória Heap; a variável antiga morre. Passar por referência (`&`) só empresta, sem acionar o `drop` no final, preservando a memória.
```

### Resoluções - Capítulo 4
```rust
// 1. Struct Clássica
struct Livro { titulo: String, autor: String, paginas: u16 }
impl Livro {
    fn novo(titulo: String, autor: String, paginas: u16) -> Livro { Livro { titulo, autor, paginas } }
    fn ler(&self) { println!("Lendo {} de {}", self.titulo, self.autor); }
}

// 2 e 3. Enum e Pattern Matching
enum StatusWeb { Sucesso(u16), NaoEncontrado, ErroInterno(String) }

fn analisar_status(status: StatusWeb) {
    match status {
        StatusWeb::Sucesso(codigo) => println!("Tudo OK! HTTP {}", codigo),
        StatusWeb::NaoEncontrado => println!("Página 404"),
        StatusWeb::ErroInterno(msg) => println!("Servidor falhou: {}", msg),
    }
}

// 4. A ausência do Null. O Rust introduz o Enum Option<T>, com Some(valor) e None (vazio). O compilador força (através do match) que o programador preveja o que fazer se for None, eliminando NullPointerExceptions.

// 5. Utilizando if let
let valor: Option<i32> = Some(10);
if let Some(v) = valor {
    println!("Soma: {}", v + 5);
}
```

### Resoluções - Capítulo 5
```rust
// 1. Vector
let mut numeros = Vec::new();
numeros.push(1); numeros.push(2); numeros.push(3); numeros.push(4); numeros.push(5);
for num in &numeros {
    println!("{}", num * 2);
}

// 2 e 3. HashMap e Option Seguro
use std::collections::HashMap;
let mut notas = HashMap::new();
notas.insert(String::from("Alex"), 95); 
notas.insert(String::from("Maria"), 80);
notas.insert(String::from("João"), 70);

match notas.get("Carlos") {
    Some(nota) => println!("Nota: {}", nota),
    None => println!("Erro: Aluno não cadastrado!"),
}

// 4 e 5. Result e Match Seguro
fn divisao_segura(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Operação ilegal: Divisão por zero!"))
    } else {
        Ok(a / b) 
    }
}

match divisao_segura(10.0, 0.0) {
    Ok(resultado) => println!("O resultado é: {:.2}", resultado),
    Err(mensagem) => println!("Falha: {}", mensagem),
}
```
