// Caminho: shared/ausd-types/src/lib.rs
// Contém definições de tipos, contas e utilitários de bytes/tokens para o projeto aUSD.

// Declaração de módulos internos para esta crate.
pub mod accounts; // Define structs de contas.
pub mod bytes;    // Lida com manipulação de bytes.
pub mod tokens;   // Define tipos e utilitários de tokens.

// Reexporta os módulos para facilitar o acesso por outras crates que dependam de ausd-types.
pub use accounts::*;
pub use bytes::*;
pub use tokens::*;