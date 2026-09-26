# O GRANDE LIVRO DO RUST: Tipagem Profunda e Prática Aplicada
**Edição Definitiva e Expandida: Teoria Detalhada, Exemplos de Uso e 25 Desafios Práticos**

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

No Rust, variáveis são **imutáveis por padrão**. O compilador precisa saber o tamanho exato de cada tipo de dado na memória para garantir segurança e performance.

### 2.1. Inteiros e o Tratamento de Overflow

Inteiros podem ser **Signed** (com sinal `i`) ou **Unsigned** (sem sinal `u`).
A grande diferença é que os tipos sem sinal (`u8`, `u16`, `u32`, etc.) dobram sua capacidade máxima porque não usam um bit para guardar o sinal negativo.

#### Exemplo 1: Declaração e Inferência
```rust
fn main() {
    // Inferência automática: o Rust assume i32 por padrão
    let padrao = 42; 
    
    // Declaração explícita de tipo para um byte (0 a 255)
    let byte_porta: u8 = 80;
    
    // Uso de underlines para facilitar a leitura humana (o compilador ignora os _)
    let saldo_bancario: i64 = 1_500_000_000;
}
```

#### Exemplo 2: O comportamento de Overflow
O que acontece se tentarmos colocar o número 256 dentro de um `u8` (que só vai até 255)? No modo Debug, o Rust causa um *panic* (crasha o programa para evitar corrupção). No modo Release, ele faz *wrapping* (volta para o zero).
Para controle absoluto, usamos métodos específicos:
```rust
fn main() {
    let a: u8 = 250;
    
    // checked_add: Retorna um Option. Se estourar o limite, retorna None em vez de crashar.
    match a.checked_add(10) {
        Some(valor) => println!("Sucesso: {}", valor),
        None => println!("Erro: O valor excedeu a capacidade de 8 bits!"),
    }
    
    // wrapping_add: Força o estouro controlado. 250 + 10 num limite de 255 volta para o início, resultando em 4.
    let ciclo = a.wrapping_add(10);
    println!("Ciclo (Wrapping): {}", ciclo); // Imprime 4
}
```

#### Exemplo 3: Bases Numéricas
```rust
let hex = 0xff; // Hexadecimal (255)
let octal = 0o77; // Octal (63)
let binario = 0b1111_0000; // Binário (240)
```

### 2.2. O Problema das Strings: `&str` vs `String`

Este é o conceito mais crítico de tipagem para quem vem de outras linguagens. O texto em Rust é tratado de duas formas diferentes de acordo com onde ele vive na memória.

#### Exemplo 1: O Slice Imutável (`&str`)
Um *String Slice* (`&str`) é apenas uma referência. Ele tem tamanho fixo. Pode viver diretamente no arquivo binário final do seu programa (alocação estática) ou na Stack.
```rust
// 'mensagem' é um &str. É super rápido, mas você não pode aumentar ou diminuir seu tamanho.
let mensagem = "Sistema inicializado com sucesso."; 
```

#### Exemplo 2: A String Dinâmica (`String`)
A `String` é um objeto complexo alocado na memória **Heap**. Ela possui três partes na Stack (um ponteiro para a Heap, o tamanho atual (len) e a capacidade total alocada).
```rust
// Criando uma String vazia e crescendo dinamicamente
let mut saudacao = String::new();
saudacao.push_str("Olá"); // Adiciona texto
saudacao.push(',');       // Adiciona um único char
saudacao.push_str(" Rust!");

// Convertendo um &str para String
let usuario = "Alex".to_string(); // ou String::from("Alex");
```

#### Exemplo 3: Manipulação e Concatenação
```rust
let s1 = String::from("Arch ");
let s2 = String::from("Linux");

// O operador + exige a posse da primeira String e a referência da segunda.
// s1 é MOVIDA aqui e não poderá ser usada nas linhas seguintes.
let so = s1 + &s2; 

// Alternativa moderna e limpa: a macro format!
let motor = "V8";
let carro = format!("Motor {} superpotente", motor); // Não rouba a posse de ninguém
```
### O Superpoder da Mutabilidade: A palavra-chave `mut`

No Rust, a segurança é a prioridade máxima. Por isso, por padrão, **todas as variáveis são imutáveis**. O compilador precisa saber o tamanho exato de cada tipo de dado na memória para garantir segurança, performance e evitar alterações acidentais.

#### 1. O Comportamento Padrão (Imutabilidade)
Se tentares alterar uma variável normal sem permissão, o compilador vai recusar-se a compilar o código:
```rust
fn main() {
    let pontos = 10;
    // ERRO DE COMPILAÇÃO! Não podes alterar uma variável imutável
    pontos = 20; 
}
```
### 2.3. Shadowing (Sombreamento)
Diferente da mutabilidade (`mut`), o shadowing permite que você reuse o nome de uma variável com a palavra `let`. O compilador destrói a antiga e cria uma nova, permitindo mudar até o TIPO do dado.

#### Exemplo 1: Conversão de tipos sem sujar o código
```rust
// Recebemos um input do usuário como texto
let palpite = "42";

// Sombreamos a variável 'palpite' convertendo-a para número inteiro (i32)
let palpite: i32 = palpite.parse().expect("Falha ao converter número");

// Em outras linguagens, você teria que criar varáveis como 'palpite_str' e 'palpite_int'.
```

### 🛠️ Exercícios do Capítulo 2
1. Declare uma variável inteira mutável, inicie com valor 10, mude para 20 e imprima.
2. Você está indexando uma matriz de processos no sistema operacional. Por que você deve usar o tipo `usize` em vez de `u32` ou `u64`?
3. Crie uma Tupla que guarde o nome de um banco de dados (String), a porta (u16) e se ele está ativo (bool). Depois desestruture e imprima as variáveis.
4. Explique com suas palavras a diferença na memória entre `String` (com `S` maiúsculo) e `&str`.
5. Mostre um exemplo em código onde o uso do "Shadowing" (`let` repetido) é superior ao uso de uma variável mutável (`mut`).

---

## Capítulo 3: Ownership e Borrowing (Empréstimo de Memória)

Para garantir segurança sem a lentidão de um Garbage Collector, o Rust inventou o **Ownership** (Posse).

### 3.1. A Transferência de Posse (Move)

Quando um dado vive na Heap (como uma `String` ou um `Vec`), associá-lo a uma nova variável ou passá-lo para uma função **move** a posse. A variável original é invalidada para evitar "Double Free" (tentar apagar a mesma memória duas vezes).

#### Exemplo 1: O Move em Ação
```rust
let servidor_primario = String::from("192.168.0.1");
let servidor_secundario = servidor_primario; // A posse foi MOVIDA!

// println!("{}", servidor_primario); // ISSO GERA ERRO DE COMPILAÇÃO!
println!("Servidor ativo: {}", servidor_secundario); // Funciona
```

#### Exemplo 2: Clonagem Profunda (Deep Copy)
Se você realmente precisa de duas cópias independentes na Heap, deve clonar explicitamente:
```rust
let a = String::from("Configuração");
let b = a.clone(); // Aloca uma nova área na memória Heap e copia os dados

println!("a: {}, b: {}", a, b); // Ambas funcionam perfeitamente
```

### 3.2. Borrowing (O Empréstimo)

Mover variáveis o tempo todo seria inviável. Por isso, usamos o **Borrowing** (Referências `&`). A referência empresta o dado, mas não altera quem é o dono.

**Regra de Ouro da Concorrência:** Você pode ter infinitas referências imutáveis (`&T`) OU uma única referência mutável (`&mut T`) ao mesmo tempo. Nunca as duas juntas.

#### Exemplo 1: Empréstimo Imutável (Leitura)
```rust
fn calcular_tamanho(texto: &String) -> usize {
    texto.len() // Apenas lê o tamanho. Não rouba a posse.
}

fn main() {
    let ip = String::from("127.0.0.1");
    let tam = calcular_tamanho(&ip); // Passa uma referência
    
    // 'ip' continua válido pois só foi emprestado
    println!("A string '{}' tem tamanho {}", ip, tam);
}
```

#### Exemplo 2: Empréstimo Mutável (Escrita)
```rust
fn adicionar_porta(url: &mut String) {
    url.push_str(":8080"); // Modifica a string original através da referência
}

fn main() {
    let mut endereco = String::from("http://localhost");
    adicionar_porta(&mut endereco); // Passa uma referência mutável
    
    println!("Endereço completo: {}", endereco);
}
```

#### Exemplo 3: Proteção contra Condição de Corrida (Data Race)
O compilador impede que alguém leia um dado enquanto ele está sendo alterado:
```rust
let mut arquivo = String::from("dados.txt");

let leitor1 = &arquivo; // Empréstimo imutável OK
let leitor2 = &arquivo; // Empréstimo imutável OK

// let escritor = &mut arquivo; // ERRO DE COMPILADOR! 
// O Rust aborta porque leitor1 e leitor2 ainda não terminaram de usar a variável.

println!("Lendo: {} e {}", leitor1, leitor2);
// Como leitor1 e leitor2 não são mais usados daqui em diante, o empréstimo mutável agora é permitido!
let escritor = &mut arquivo;
escritor.push_str(".bak");
```

### 🛠️ Exercícios do Capítulo 3
1. Crie uma String "Rust", passe-a para uma função que apenas lê e imprime seu tamanho (usando `&String`), e depois a imprima na `main` para provar que a posse não foi perdida.
2. Crie uma função que receba uma referência mutável `&mut String` e adicione a palavra " é incrível!" no final dela. Chame-a no `main`.
3. Escreva um código que tente criar duas referências mutáveis (`&mut`) para a mesma variável no mesmo escopo, veja o erro que o compilador gera e explique por que o Rust proíbe isso (Data Race).
4. Crie um Array de números `[10, 20, 30]`. Crie uma função que aceite um empréstimo desse array (um *slice* `&[i32]`) e retorne a soma.
5. Explique a diferença profunda entre passar uma `String` por valor (Move) e passar por referência (`&String`) em termos de liberação de memória (função Drop).

---

## Capítulo 4: Structs, Enums e Pattern Matching (A Evolução da POO)

Rust rejeita o paradigma clássico de Orientação a Objetos (Classes e Herança). Ele prefere a **Composição** através de Structs e a representação de estados através de Enums incrivelmente poderosos.

### 4.1. Structs e Implementações

Structs armazenam os dados. Os blocos `impl` abrigam as funções atreladas aos dados.

#### Exemplo 1: Modelando um objeto (Struct Clássica)
```rust
struct Retangulo {
    largura: u32,
    altura: u32,
}

impl Retangulo {
    // Método associado (Semelhante a um método estático, não usa &self)
    fn quadrado(tamanho: u32) -> Retangulo {
        Retangulo { largura: tamanho, altura: tamanho }
    }

    // Método de instância (Exige &self para ler os dados da própria struct)
    fn calcular_area(&self) -> u32 {
        self.largura * self.altura
    }
    
    // Método mutável (Pode alterar os dados do próprio objeto)
    fn expandir(&mut self, fator: u32) {
        self.largura *= fator;
        self.altura *= fator;
    }
}

fn main() {
    let mut forma = Retangulo::quadrado(10);
    forma.expandir(2);
    println!("Área final: {}", forma.calcular_area()); // 400
}
```

#### Exemplo 2: Tuple Structs
Úteis quando você quer criar tipos fortes diferentes, mas sem nomear cada campo (muito comum em matemática de jogos ou design de sistemas).
```rust
struct Cor(u8, u8, u8);
struct Ponto3D(f64, f64, f64);

let preto = Cor(0, 0, 0);
let origem = Ponto3D(0.0, 0.0, 0.0);
```

### 4.2. Enums com Dados e o Controle Exaustivo (`match`)

Enums em Rust podem carregar diferentes tipos de dados dentro de suas variantes. Isso é revolucionário para a criação de Máquinas de Estado e tratamento de pacotes de rede.

#### Exemplo 1: Enum carregando múltiplos tipos de dados
```rust
enum PacoteRede {
    Conectar, // Sem dados associados
    Desconectar(String), // Carrega uma String (motivo da desconexão)
    Payload(u32, Vec<u8>), // Carrega o ID do pacote e o binário bruto
}

fn main() {
    let evento = PacoteRede::Desconectar(String::from("Timeout"));
    processar_rede(evento);
}
```

#### Exemplo 2: Desempacotando dados com `match`
O `match` obriga você a tratar **todas** as possibilidades do Enum. Ele impede que você esqueça de tratar um caso e introduza um bug crítico em produção.
```rust
fn processar_rede(pacote: PacoteRede) {
    match pacote {
        PacoteRede::Conectar => println!("Estabelecendo handshake..."),
        PacoteRede::Desconectar(motivo) => println!("Conexão caiu. Motivo: {}", motivo),
        PacoteRede::Payload(id, bytes) => println!("Processando pacote nº {}, tamanho: {} bytes", id, bytes.len()),
    }
}
```

#### Exemplo 3: O atalho `if let`
Se você só se importa com **uma** variante do Enum e quer ignorar as outras, o `match` pode ser verboso. Usamos o `if let`:
```rust
let configuracao = Some(8080); // Um Option contendo um valor

// Em vez de um match gigante, verificamos apenas o caso Some:
if let Some(porta) = configuracao {
    println!("Servidor iniciado na porta {}", porta);
} else {
    println!("Usando porta padrão.");
}
```

### 🛠️ Exercícios do Capítulo 4
1. Crie uma Struct `Livro` com campos `titulo`, `autor` e `paginas`. Crie um bloco `impl` com um método `novo` (construtor) e um método `ler` que imprima uma mensagem.
2. Crie um Enum `StatusWeb` com as variantes: `Sucesso(u16)`, `NaoEncontrado`, `ErroInterno(String)`.
3. Crie uma função que receba o Enum `StatusWeb` acima e use um `match` para imprimir mensagens diferentes para cada variante, extraindo os valores (`u16` e `String`) para dentro da impressão.
4. O Rust não tem "Null" ou "Nil". Explique qual é a solução nativa do Rust para representar a ausência de um valor (O Enum `Option<T>`).
5. Instancie uma variável do tipo `Option<i32>` como `Some(10)`. Escreva uma estrutura lógica com `if let` para somar 5 a esse número e imprimi-lo, ignorando silenciosamente o caso em que ele seja `None`.

---

## Capítulo 5: Coleções Dinâmicas e Tratamento de Erros Profissional

Enquanto Arrays e Tuplas possuem tamanho rígido na Stack, as Coleções crescem dinamicamente alocando espaço na memória Heap.

### 5.1. Vetores (`Vec<T>`)

Vetores são matrizes flexíveis, essenciais para praticamente qualquer aplicação.

#### Exemplo 1: Criação e Iteração Segura
```rust
// A macro vec! facilita a criação já populada
let mut precos: Vec<f64> = vec![10.50, 25.0, 9.99];
precos.push(5.0); // Adiciona um elemento no final
precos.pop();     // Remove o último elemento

// Iteração imutável lendo valores
for preco in &precos {
    println!("Valor: R${}", preco);
}
```

#### Exemplo 2: Iteração Mutável (Modificando elementos)
```rust
let mut notas = vec![50, 60, 70];

// Usamos &mut para acessar a referência mutável de cada item da lista
for nota in &mut notas {
    *nota += 10; // O * (desreferência) acessa o valor apontado pela referência para podermos somar
}
// Agora a lista contém [60, 70, 80]
```

### 5.2. Mapas de Hash (`HashMap<K, V>`)

Armazenam dados baseados em Chave e Valor. Extremamente otimizados para buscas rápidas e caching de dados.

#### Exemplo 1: Inserindo e Lendo com Segurança
O método `.get()` retorna um `Option`, pois a chave procurada pode não existir no dicionário.
```rust
use std::collections::HashMap;

let mut servidores = HashMap::new();
servidores.insert(String::from("BancoDeDados"), "10.0.0.10");
servidores.insert(String::from("BackendAPI"), "10.0.0.11");

// Tentando buscar um servidor inexistente
match servidores.get("Frontend") {
    Some(ip) => println!("Conectando ao frontend no IP {}", ip),
    None => println!("ALERTA: Servidor de frontend não localizado!"),
}
```

#### Exemplo 2: Inserção Condicional (Insert if Not Present)
Padrão muito comum para evitar sobrescrever dados existentes:
```rust
let mut pontuacoes = HashMap::new();
pontuacoes.insert(String::from("Equipe_Azul"), 10);

// Só insere o valor 50 se a chave "Equipe_Amarela" NÃO existir
pontuacoes.entry(String::from("Equipe_Amarela")).or_insert(50);
// Não faz nada, pois Equipe_Azul já existe
pontuacoes.entry(String::from("Equipe_Azul")).or_insert(900); 
```

### 5.3. Tratamento de Erros sem Try/Catch (`Result<T, E>`)

No Rust não existem exceções invisíveis estourando e quebrando o programa repentinamente. Toda função que pode falhar retorna o Enum `Result`.
```rust
enum Result<T, E> {
    Ok(T),  // Contém o valor de sucesso
    Err(E), // Contém o erro (geralmente uma String ou um tipo de Erro customizado)
}
```

#### Exemplo 1: O Operador Propagador `?` (The Try Operator)
Se uma operação falhar dentro de uma função, o operador `?` retorna o erro prematuramente para a função superior lidar. É uma sintaxe absurdamente limpa, substituindo aninhamentos complexos de `if`.
```rust
use std::fs::File;
use std::io::{self, Read};

// A função promete retornar uma String de sucesso OU um erro de Input/Output (io::Error)
fn ler_arquivo_config() -> Result<String, io::Error> {
    // Tenta abrir o arquivo. Se falhar, o '?' já retorna Err() automaticamente e sai da função.
    let mut arquivo = File::open("config.json")?;
    
    let mut conteudo = String::new();
    
    // Tenta ler o conteúdo e jogar na variável. Se falhar, o '?' retorna o erro.
    arquivo.read_to_string(&mut conteudo)?;
    
    // Se tudo deu certo até aqui, empacotamos a string no Ok()
    Ok(conteudo)
}

fn main() {
    match ler_arquivo_config() {
        Ok(texto) => println!("Arquivo lido: {}", texto),
        Err(erro) => println!("Falha de sistema: {}", erro),
    }
}
```

### 🛠️ Exercícios do Capítulo 5
1. Crie um vetor (`Vec<i32>`), adicione 5 números diferentes usando `.push()`. Faça um loop `for` que multiplique cada número por 2 e imprima.
2. Crie um `HashMap<String, i32>` que armazene nomes de alunos e suas notas. Insira 3 alunos.
3. Usando o `HashMap` criado acima, faça um `.get()` buscando um nome que não existe. Trate o retorno `Option` usando um `match` com mensagem de erro amigável.
4. Escreva uma função que tente dividir dois números de ponto flutuante `f64`. Se o denominador for zero, retorne um `Result::Err("Divisão por zero")`. Se não, retorne `Result::Ok(resultado)`.
5. Na função `main`, chame a função de divisão criada acima e trate o sucesso e a falha usando a sintaxe de `match` para lidar apropriadamente com o Enum `Result`.

---

## GABARITO E EXPLICAÇÕES (Soluções Comentadas)

### Resoluções - Capítulo 1
1. **Compiladas (Rust/C#)** traduzem todo o código para binário nativo da máquina de uma só vez, rodando com máxima performance e sem dependências na máquina destino. **Interpretadas (Python/JS)** necessitam que o usuário tenha um software rodando em segundo plano (interpretador/engine) que traduz linha a linha durante a execução, o que consome mais recursos e gera lentidão.
2. `cargo run` compila rápido, não otimiza variáveis e injeta metadados para você rastrear bugs (ideal para dev). `cargo build --release` demora para compilar porque o compilador reescreve lógicas em Assembly puro, remove funções não utilizadas e otimiza a alocação para velocidade máxima de produção.
3. `cargo check`. Ele ignora a geração do executável, validando apenas as regras de memória, sintaxe e declarações na velocidade da luz.
4. O `Cargo.toml` é o arquivo de manifesto e configuração. Ele orquestra metadados do projeto (nome, autor), versão e, crucialmente, declara todas as bibliotecas externas (Crates) que o seu software precisa baixar para funcionar.
5. Pelo **Borrow Checker** atrelado ao ciclo de vida (Lifetimes). Ele impede em tempo de compilação que referências inválidas existam, bloqueando o software caso haja risco de uma variável sair do escopo e o programa tentar acessá-la (Dangling Pointers).

### Resoluções - Capítulo 2
```rust
// 1. Variável mutável padrão
let mut num = 10;
num = 20;
println!("{}", num);

// 2. O usize se adapta automaticamente à arquitetura do processador (32 bits ou 64 bits). 
// Como o limite físico da memória RAM disponível para matrizes depende da arquitetura do OS, 
// indexar estruturas de dados e ponteiros de SO com 'usize' garante que nunca haverá falha de mapeamento de memória independente de onde rodar.

// 3. Tuplas de configuração
let bd: (String, u16, bool) = (String::from("PostgreSQL"), 5432, true);
let (nome_banco, porta, ativo) = bd; // Desestruturação limpa
println!("Banco: {}, Porta: {}, Ativo: {}", nome_banco, porta, ativo);

// 4. `String` (Heap): É um ponteiro dinâmico, pode crescer, diminuir, sofrer mutação (.push_str) e pertence à variável, sendo liberada automaticamente.
// `&str` (Slice): É estático e imutável. Representa apenas uma "fotografia" na memória (um ponteiro apontando para os dados com um tamanho fixo). É leve e rápido, ideal para leitura.

// 5. Shadowing vs Mutabilidade
let input_usuario = "42"; // É uma String/Slice (texto)
// Mutabilidade não permite mudar o tipo (de texto para int).
// O Shadowing (reusar 'let') destrói a variável texto e cria uma nova numéria com o mesmo nome!
let input_usuario: i32 = input_usuario.parse().unwrap(); 
```

### Resoluções - Capítulo 3
```rust
// 1. Empréstimo Imutável (Apenas leitura)
fn ler_tamanho(s: &String) -> usize { s.len() }
let texto = String::from("Rust");
println!("Tamanho: {}", ler_tamanho(&texto));
println!("Ainda posso usar a variável original: {}", texto);

// 2. Empréstimo Mutável (Escrita delegada)
fn adiciona_incrivel(s: &mut String) { s.push_str(" é incrível!"); }
let mut frase = String::from("Rust");
adiciona_incrivel(&mut frase);

// 3. O Rust proíbe múltiplas referências mutáveis para evitar Data Races em multithreading. 
// Se duas lógicas tentam escrever na mesma área de memória simultaneamente sem controle, os dados corrompem. 
// let r1 = &mut frase; 
// let r2 = &mut frase; // -> ERRO: cannot borrow `frase` as mutable more than once at a time

// 4. Slice de um Array na Stack
fn somar_array(numeros: &[i32]) -> i32 {
    let mut total = 0;
    for n in numeros { total += n; }
    total
}
let arr = [10, 20, 30];
println!("Soma: {}", somar_array(&arr));

// 5. Ao passar por valor (Move), a função de destino assume a propriedade. Ao final do bloco dela, o Rust chama a função 'drop()', 
// deletando a variável da Heap. A variável original fica proibida para impedir Double Free. 
// Passando por '&String', nenhuma propriedade muda; a função apenas observa e nenhuma memória é liberada no final do empréstimo.
```

### Resoluções - Capítulo 4
```rust
// 1. Struct Clássica
struct Livro { titulo: String, autor: String, paginas: u16 }
impl Livro {
    // Construtor
    fn novo(titulo: String, autor: String, paginas: u16) -> Livro { Livro { titulo, autor, paginas } }
    // Método que lê a própria struct
    fn ler(&self) { println!("Lendo {} de {}", self.titulo, self.autor); }
}

// 2 e 3. Enum avançado e Pattern Matching exaustivo
enum StatusWeb { Sucesso(u16), NaoEncontrado, ErroInterno(String) }

fn analisar_status(status: StatusWeb) {
    match status {
        StatusWeb::Sucesso(codigo) => println!("Tudo OK! HTTP {}", codigo),
        StatusWeb::NaoEncontrado => println!("Página 404 - Acesso inválido"),
        StatusWeb::ErroInterno(msg) => println!("Servidor falhou crítico: {}", msg),
    }
}

// 4. A ausência do Null. O Rust introduz o Enum Option<T>, que abriga Some(valor) e None (vazio).
// O compilador força (através do match) que o programador preveja o que o código fará se o resultado for None.
// Isso literalmente elimina a possibilidade de falhas de NullPointerException em produção.

// 5. Utilizando if let (Sintaxe enxuta)
let valor: Option<i32> = Some(10);
if let Some(v) = valor {
    println!("Soma: {}", v + 5);
}
// Não há bloco else, logo o caso None é silenciado elegantemente.
```

### Resoluções - Capítulo 5
```rust
// 1. Vector e Mutação Interna
let mut numeros = Vec::new();
numeros.push(1); numeros.push(2); numeros.push(3); numeros.push(4); numeros.push(5);
// Iterando por referência de forma segura
for num in &numeros {
    println!("{}", num * 2);
}

// 2 e 3. HashMap e Option Seguro
use std::collections::HashMap;
let mut notas = HashMap::new();
notas.insert(String::from("Alex"), 95); // Uma menção especial focada no desenvolvedor
notas.insert(String::from("Maria"), 80);
notas.insert(String::from("João"), 70);

// .get() previne crash se a chave não existir
match notas.get("Carlos") {
    Some(nota) => println!("Nota do aluno: {}", nota),
    None => println!("Erro: Aluno não cadastrado no sistema acadêmico!"),
}

// 4 e 5. Result e Match Seguro (Tratamento Matemático)
fn divisao_segura(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Operação ilegal: Divisão por zero detectada!"))
    } else {
        Ok(a / b) // Retorna o valor de sucesso empacotado no Enum Result
    }
}

match divisao_segura(10.0, 0.0) {
    Ok(resultado) => println!("O resultado da conta é: {:.2}", resultado),
    Err(mensagem) => println!("Falha de cálculo reportada: {}", mensagem),
}
```
