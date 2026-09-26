# O GRANDE LIVRO DO RUST: Tipagem Profunda e Prática Aplicada
**Edição Revisada: Foco em Dados, Memória e 25 Desafios Práticos**

---

## Capítulo 1: O Ecossistema e a Filosofia
Rust é uma linguagem compilada (AOT - Ahead of Time). Isso significa que você compila o código e entrega o binário final para o usuário rodar, sem que ele precise ter o Rust instalado (diferente de Node.js ou Python). O gerenciador oficial é o **Cargo**.

*   `cargo new projeto`: Cria a estrutura base.
*   `cargo check`: Verifica erros de compilação na velocidade da luz (ideal para usar o tempo todo).
*   `cargo build`: Gera o executável de testes.
*   `cargo build --release`: Gera o executável final otimizado, removendo símbolos de debug e rodando até 10x mais rápido.

### 🛠️ Exercícios do Capítulo 1
1. Qual a diferença fundamental de entrega de software entre uma linguagem compilada (Rust) e uma interpretada (JavaScript/Python)?
2. Explique a diferença entre `cargo build` e `cargo check` e quando você usaria cada um no seu dia a dia.
3. Se você fosse enviar um programa em Rust para o seu cliente final rodar, qual comando de compilação exato você utilizaria?
4. O que é o arquivo `Cargo.toml` que é gerado na raiz de todo projeto Rust?
5. Em linguagens C, a falha de segmentação (Segmentation Fault) ocorre quando se acessa memória inválida. Como o Rust tenta evitar isso?

---

## Capítulo 2: Tipos de Dados e Variáveis (Análise Profunda)
Diferente do JavaScript ou Python, o Rust quer saber exatamente o tamanho de cada variável na memória. Variáveis são imutáveis por padrão. O conceito de **Shadowing** permite reaproveitar nomes sobrescrevendo a variável anterior.

### Os Tipos Escalares (Valor Único)

#### 1. Inteiros (Sem casas decimais)
Inteiros são divididos entre **Signed** (com sinal: positivos e negativos, usam a letra `i`) e **Unsigned** (sem sinal: apenas positivos, usam a letra `u`).
*   `i8`, `i16`, `i32`, `i64`, `i128`: Podem ser negativos.
*   `u8`, `u16`, `u32`, `u64`, `u128`: Apenas positivos. Ganham o dobro do limite máximo positivo já que não gastam bit para o sinal de menos.
*   `isize` e `usize`: O tamanho depende do sistema operacional (32 ou 64 bits).

**Qual usar e quando?**
*   **Use `i32` (Padrão do Rust):** Para 90% dos casos. Ele é extremamente rápido nas CPUs modernas de 64 bits.
*   **Use `u8`:** Ao manipular protocolos de rede, leitura de arquivos binários (arquivos de imagem, vídeo) ou criar buffers de memória raw.
*   **Use `usize`:** Sempre que for indexar arrays ou coleções (ex: `lista[indice]`). O tamanho da memória do computador define o limite de itens possíveis, por isso indexação *exige* `usize`.

#### 2. Ponto Flutuante (Decimais)
*   **`f64` (Padrão):** Ocupa 64 bits (Dupla precisão). Em CPUs modernas, o `f64` tem praticamente a mesma velocidade do `f32`, mas com precisão muito maior. Use-o para cálculos financeiros ou científicos.
*   **`f32`:** Ocupa 32 bits (Precisão simples). Use quando a economia de memória for crítica, como no desenvolvimento de *Shaders* gráficos (WebGL/A-Frame/OpenGL) ou jogos.

#### 3. Booleano
*   **`bool`:** `true` ou `false`. Ocupa 1 byte na memória.

#### 4. Caractere (O diferencial do Rust)
*   **`char`:** Usa aspas simples (ex: `'A'`, `'🚀'`). Diferente do C (onde char é 1 byte), no Rust o `char` tem **4 bytes**. Ele suporta nativamente todo o padrão Unicode (emojis, kanjis japoneses, acentos), o que o torna excelente para internacionalização.

### Tipos Compostos (Múltiplos Valores)

#### 1. Tuplas
Tamanho fixo, tipos diferentes. Excelentes para retornar múltiplos valores de uma função.
```rust
let config: (i32, f64, u8) = (500, 6.4, 1);
let (velocidade, tempo, porta) = config; // Desestruturação
```

#### 2. Matrizes (Arrays)
Tamanho fixo, tipos iguais. São alocados inteiramente na *Stack* (pilha, super rápida).
*   **Quando usar:** Quando você sabe o número exato de elementos que não vai mudar (ex: meses do ano `let meses = ["Jan", "Fev", ...];`).

#### 3. String Dinâmica vs Slice de String (`String` vs `&str`)
Este é o erro número 1 dos iniciantes.
*   **`&str` (String Slice):** É uma visão rápida e imutável de um texto. Vive direto no código binário do programa ou na stack. É muito leve, mas você não pode concatenar ou mudar o tamanho dela facilmente.
*   **`String`:** Vive na *Heap* (memória dinâmica). Pode crescer, diminuir, adicionar textos (ex: `texto.push_str("!")`). Ela tem um custo um pouco maior de performance por exigir alocação de memória no SO.

### 🛠️ Exercícios do Capítulo 2
1. Declare uma variável inteira mutável, inicie com valor 10, mude para 20 e imprima.
2. Explique a diferença de capacidade e uso ideal entre `i32`, `u8` e `usize`.
3. Crie uma Tupla que guarde o nome de um banco de dados (String), a porta (u16) e se ele está ativo (bool). Depois desestruture e imprima as variáveis.
4. Explique com suas palavras por que um Emoji cabe dentro do tipo `char` no Rust, enquanto no C seria um problema.
5. Usando "Shadowing" (redeclaração com `let`), crie uma variável chamada `espacos` contendo "   " (3 espaços), e na linha seguinte transforme essa mesma variável no número `3` usando `espacos.len()`.

---

## Capítulo 3: Ownership e Borrowing (Empréstimo)
A memória principal do Rust é regida por 3 regras de Ownership:
1. Cada valor tem um proprietário (*owner*).
2. Só existe um proprietário por vez.
3. Quando o proprietário sai de escopo (a chave `}` fecha), o valor é jogado fora da memória.

Para não ter que transferir a propriedade de valores o tempo todo, nós "emprestamos" os dados através das referências (`&`).

**Regra de Ouro do Borrowing (Prevenção de Condições de Corrida):**
Em um mesmo escopo, você pode ter:
*   Múltiplas referências de leitura (`&T`).
*   **OU** apenas *uma* referência de escrita/mutável (`&mut T`).
Nunca os dois ao mesmo tempo!

### 🛠️ Exercícios do Capítulo 3
1. Crie uma String "Rust", passe-a para uma função que apenas lê e imprime seu tamanho (usando `&String`), e depois a imprima na `main` para provar que a posse não foi perdida.
2. Crie uma função que receba uma referência mutável `&mut String` e adicione a palavra " é incrível!" no final dela. Chame-a no `main`.
3. Escreva um código que tente criar duas referências mutáveis (`&mut`) para a mesma variável no mesmo escopo, veja o erro que o compilador gera e explique por que o Rust proíbe isso (Concorrência).
4. Crie um Array de números `[10, 20, 30]`. Crie uma função que aceite um empréstimo desse array (um *slice* `&[i32]`) e retorne a soma.
5. Explique a diferença entre passar um dado por valor (move) e passar por referência (borrow) em termos de gerenciamento da memória Heap.

---

## Capítulo 4: Structs, Enums e Pattern Matching

As Structs substituem o conceito clássico de Classes. Os dados vão na `struct`, os métodos vão no bloco `impl`.
O poder real do Rust está nos **Enums**. Enums em C ou C# são só números disfarçados. No Rust, Enums podem encapsular dados dentro deles.

```rust
enum EnderecoIP {
    V4(u8, u8, u8, u8),
    V6(String),
}
```

Para extrair esses dados com segurança, usamos a estrutura `match`, que o compilador obriga a ser "exaustiva" (você não pode esquecer nenhum caso, evitando bugs em produção).

### 🛠️ Exercícios do Capítulo 4
1. Crie uma Struct `Livro` com campos `titulo`, `autor` e `paginas`. Crie um bloco `impl` com um método `novo` (construtor) e um método `ler` que imprima uma mensagem.
2. Crie um Enum `StatusWeb` com as variantes: `Sucesso(u16)`, `NaoEncontrado`, `ErroInterno(String)`.
3. Crie uma função que receba o Enum `StatusWeb` acima e use um `match` para imprimir mensagens diferentes para cada variante, extraindo os valores (`u16` e `String`) para dentro da impressão.
4. O Rust não tem Null. Explique qual é a solução nativa do Rust (O Enum `Option<T>`) para representar a ausência de um valor de forma segura.
5. Instancie uma variável do tipo `Option<i32>` como `Some(10)`. Escreva um `match` que some 5 se houver valor, e retorne 0 se for `None`.

---

## Capítulo 5: Coleções Dinâmicas e Result (Tratamento de Erros)

Tipos como Arrays têm tamanho fixo na Stack. Se você precisa de listas que crescem dinamicamente, precisa usar coleções alocadas no Heap.

*   **Vetor (`Vec<T>`):** Lista dinâmica. Muito usado para retornos de banco de dados e listas em interfaces.
*   **HashMap<K, V>:** Um dicionário (chave-valor). Muito útil para criar caches em memória ou processar objetos JSON.

**A Ausência de Try/Catch:**
No Rust, funções que podem falhar (como ler um arquivo ou conectar no MongoDB) não lançam exceções invisíveis. Elas retornam um Enum chamado `Result<T, E>`. Você é *obrigado* a tratar o erro ou o sucesso de forma explícita. O operador `?` pode ser usado no final de uma chamada de função para propagar o erro automaticamente se ele falhar.

### 🛠️ Exercícios do Capítulo 5
1. Crie um vetor (`Vec<i32>`), adicione 5 números diferentes usando `.push()`. Faça um loop `for` que multiplique cada número por 2 e imprima.
2. Crie um `HashMap<String, i32>` que armazene nomes de alunos e suas notas. Insira 3 alunos.
3. Usando o `HashMap` criado acima, faça um `.get()` buscando um nome que não existe. Trate o retorno `Option` usando um `match` com mensagem de erro.
4. Escreva uma função que tente dividir dois números de ponto flutuante `f64`. Se o denominador for zero, retorne um `Result::Err("Divisão por zero")`. Se não, retorne `Result::Ok(resultado)`.
5. Na `main`, chame a função de divisão criada acima e trate o sucesso e a falha usando a sintaxe de `match` com o `Result`.

---

## GABARITO DOS EXERCÍCIOS (CÓDIGOS E EXPLICAÇÕES)

### Resoluções - Capítulo 1
1. Compiladas (Rust/C#) convertem o código fonte para linguagem de máquina (binário) antes da execução, otimizando performance. Interpretadas (JS/Python) rodam através de um motor intermediário em tempo real, o que as torna mais lentas.
2. `cargo check` apenas verifica a sintaxe e a tipagem sem gerar binário, rodando em milissegundos (ideal para *hot-reload* mental enquanto digita). `cargo build` processa todo o executável.
3. `cargo build --release`.
4. É o arquivo de manifesto onde ficam as dependências do projeto (bibliotecas/crates externas), configuração do pacote e versão.
5. Pelo *Borrow Checker* em tempo de compilação. O Rust impede referências inválidas, garantindo que uma variável não saia de escopo enquanto outras partes do código ainda apontam para ela.

### Resoluções - Capítulo 2
```rust
// 1. Variável mutável
let mut num = 10;
num = 20;
println!("{}", num);

// 2. Explicação: i32 (inteiro padrão com sinal, rápido e ótimo para uso geral), u8 (0 a 255, perfeito para bytes e buffers), usize (tamanho variável dependendo da RAM/Sistema, obrigatório para indexar listas).

// 3. Tuplas
let bd: (String, u16, bool) = (String::from("PostgreSQL"), 5432, true);
let (nome_banco, porta, ativo) = bd;
println!("Banco: {}, Porta: {}, Ativo: {}", nome_banco, porta, ativo);

// 4. No C, char tem 1 byte, cobrindo apenas a tabela ASCII clássica (letras ocidentais). No Rust, char tem 4 bytes de memória dedicada, o que garante espaço suficiente para mapear o padrão mundial Unicode inteiro, incluindo emojis e alfabetos complexos.

// 5. Shadowing
let espacos = "   ";
let espacos = espacos.len();
println!("Temos {} espaços", espacos);
```

### Resoluções - Capítulo 3
```rust
// 1. Borrowing leitura
fn ler_tamanho(s: &String) -> usize { s.len() }
let texto = String::from("Rust");
println!("Tamanho: {}", ler_tamanho(&texto));
println!("Ainda posso usar: {}", texto);

// 2. Borrowing mutável
fn adiciona_incrivel(s: &mut String) { s.push_str(" é incrível!"); }
let mut frase = String::from("Rust");
adiciona_incrivel(&mut frase);

// 3. A restrição evita o "Data Race". Se duas partes do código (ou duas threads diferentes) pudessem alterar o MESMO espaço de memória no mesmo microssegundo, a memória ficaria corrompida.
// let r1 = &mut frase; 
// let r2 = &mut frase; // COMPILADOR BATE AQUI!

// 4. Slice Empréstimo
fn somar_array(numeros: &[i32]) -> i32 {
    let mut total = 0;
    for n in numeros { total += n; }
    total
}
let arr = [10, 20, 30];
println!("Soma: {}", somar_array(&arr));

// 5. Passar por valor "Move" a posse da Heap, invalidando a variável original para evitar que duas variáveis tentem deletar (Drop) a mesma Heap no fim do bloco (Double Free). Por referência, você empresta o ponteiro Stack sem dar direitos de exclusão da Heap.
```

### Resoluções - Capítulo 4
```rust
// 1. Struct e Impl
struct Livro { titulo: String, autor: String, paginas: u16 }
impl Livro {
    fn novo(titulo: String, autor: String, paginas: u16) -> Livro { Livro { titulo, autor, paginas } }
    fn ler(&self) { println!("Lendo {} de {}", self.titulo, self.autor); }
}

// 2 e 3. Enum avançado e Match Extrator
enum StatusWeb { Sucesso(u16), NaoEncontrado, ErroInterno(String) }

fn analisar_status(status: StatusWeb) {
    match status {
        StatusWeb::Sucesso(codigo) => println!("Tudo OK! HTTP {}", codigo),
        StatusWeb::NaoEncontrado => println!("Página 404"),
        StatusWeb::ErroInterno(msg) => println!("Servidor falhou: {}", msg),
    }
}

// 4. Como não tem Null, o Rust empacota o valor dentro do Enum genérico `Option<T>`, que possui as variantes `Some(valor)` ou `None`. O programador é forçado a usar um `match` para desempacotar, o que zera a chance de explodir um NullPointer Exception.

// 5. Tratando Option
let valor: Option<i32> = Some(10);
let resultado = match valor {
    Some(v) => v + 5,
    None => 0,
};
```

### Resoluções - Capítulo 5
```rust
// 1. Vector e iteração mutável
let mut numeros = Vec::new();
numeros.push(1); numeros.push(2); numeros.push(3); numeros.push(4); numeros.push(5);
for num in &numeros {
    println!("{}", num * 2);
}

// 2 e 3. HashMap e Option Handling
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
        Err(String::from("Divisão por zero detectada!"))
    } else {
        Ok(a / b)
    }
}

match divisao_segura(10.0, 0.0) {
    Ok(resultado) => println!("Resultado da conta: {}", resultado),
    Err(mensagem) => println!("Falha de cálculo: {}", mensagem),
}
```
