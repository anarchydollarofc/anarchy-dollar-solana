// Caminho: shared/ausd-derive/src/lib.rs
// Ponto de entrada para macros derive customizadas do aUSD.
// O código das macros será implementado em arquivos separados dentro deste módulo.

// A macro proc_macro_attribute é usada para criar atributos de macro.
#[proc_macro_attribute]
pub fn AusdDeriveMacro(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // Esta é uma macro stub (esboço). A implementação real viria aqui ou seria importada.
    // Por enquanto, apenas retorna o item original para permitir a compilação.
    item
}

// Outras macros derive iriam aqui, se necessário.