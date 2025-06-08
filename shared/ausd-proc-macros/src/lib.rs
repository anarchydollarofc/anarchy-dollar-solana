// Caminho: shared/ausd-proc-macros/src/lib.rs
// Contém macros de procedimento customizadas para o projeto aUSD.

// Exemplo de uma macro de procedimento simples (apenas para compilação).
#[proc_macro]
pub fn declare_program_id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Esta é uma macro stub (esboço). A implementação real da macro viria aqui.
    // Por exemplo, Elusiv usa isso para injetar o declare_id!
    input // Apenas retorna o TokenStream de entrada.
}

// Outras macros de procedimento (como as que otimizam Compute Units) seriam declaradas aqui.