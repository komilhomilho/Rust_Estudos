#  Rust_Estudos

Bem-vindo ao meu repositório central de estudos na linguagem **Rust**! 

O objetivo deste espaço é documentar a minha evolução contínua, trilhando o caminho do nível **Júnior até a especialização Sênior (e além)**. Este repositório não se limita a um único nicho. Aqui exploro o Rust em sua totalidade: desde a resolução de algoritmos clássicos, até o domínio da programação assíncrona, metaprogramação e arquiteturas de alta performance para sistemas e web.

##  Áreas de Estudo e Foco (Roadmap)

Para atingir a fluência avançada na linguagem, estou aprofundando meus conhecimentos nas seguintes frentes:

* **Algoritmos e Lógica:** Estruturas de dados, análise de complexidade e resolução de *katas* (Codewars, LeetCode) com foco em código idiomático.
* **Domínio da Memória:** Regras de *Ownership*, *Borrowing*, *Lifetimes* explícitos e o ecossistema de *Smart Pointers* (`Box`, `Rc`, `Arc`, `RefCell`, `Cow`).
* **Programação Assíncrona:** Compreensão de *Futures*, uso avançado de `async/await` e integração com runtimes de alta performance como o **Tokio**.
* **Metaprogramação (Macros):** Criação de *Declarative Macros* (`macro_rules!`) para DRY (Don't Repeat Yourself) e *Procedural Macros* (custom derives e atributos).
* **Arquitetura de Software:** *Traits* avançados, *Generics*, tipagem focada em *State-Machines* e Design Patterns aplicados ao modelo seguro do Rust.
* **Sistemas e Interoperabilidade:** *Multithreading* seguro (Fearless Concurrency), ferramentas CLI, manipulação do SO, *Unsafe Rust* e integração FFI (Foreign Function Interface).
* **Fronteiras Modernas:** WebAssembly (Wasm) para exportar a performance do Rust para o navegador e desenvolvimento de backends robustos com frameworks como Axum ou Actix.

##  Projetos e Desafios

Lista de frentes práticas e diretórios presentes neste repositório:

* **Desafios Lógicos (Codewars):** Coleção de algoritmos resolvidos utilizando o poder dos *Iterators* e *Pattern Matching* para máxima eficiência.
* **CLI de Automação (Gerenciador Linux):** Ferramenta de linha de comando integrada ao sistema operacional para leitura de processos e manipulação de partições/arquivos.
* **API RESTful Assíncrona:** Backend de alta concorrência desenvolvido com `Axum` e `SQLx`, estruturado com injeção de dependências e tratamento de erros customizado.
* **Crate de Macros Pessoais:** Biblioteca desenvolvida para encapsular lógicas repetitivas através da criação de macros customizadas.
* **Integração WebAssembly:** Módulo exportando cálculos pesados em Rust para serem consumidos via JavaScript/TypeScript no frontend.

## Como executar os códigos

Este repositório utiliza o **Cargo**, o gerenciador de pacotes e sistema de build oficial do Rust. 

Para testar qualquer projeto ou desafio de lógica presente aqui, navegue até a pasta específica via terminal e utilize:

```bash
# Verifica se o código compila sem gerar o executável final (rápido, ideal para desenvolvimento)
cargo check

# Compila e gera o binário otimizado para produção
cargo build --release

# Compila e executa o código imediatamente
cargo run

# Roda a suíte de testes (essencial para validar as resoluções de algoritmos)
cargo test
