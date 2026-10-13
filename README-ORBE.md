# ORBE Launcher 0.3.0 — fonte adaptada

Launcher desktop nativo baseado na arquitetura Rust + egui/eframe do `craft-launcher` 0.2.1 enviado como referência. A interface e o fluxo de catálogo/instalação/atualização foram adaptados para ORBE.

## Catálogo
- ORBE Studio: `SEU_USUARIO/ORBE-Studio`
- ORBE Motion: `SEU_USUARIO/ORBE-Motion`
- Atualização do Launcher: `SEU_USUARIO/ORBE-Launcher`

Antes de distribuir, substitua `SEU_USUARIO` em `crates/core/src/catalog.rs` pelo proprietário real do GitHub e confirme que cada repositório publica Releases com assets compatíveis e checksums. Esses repositórios são placeholders, não foram presumidos como existentes.

## Build Windows x64
Requisitos: Rust stable 1.90+, Visual Studio Build Tools com MSVC e Windows SDK.

```powershell
cargo build --release -p artcraft-launcher --bin orbe-launcher --target x86_64-pc-windows-msvc
```

O executável esperado é `target\x86_64-pc-windows-msvc\release\orbe-launcher.exe`. O workflow `.github/workflows/orbe-windows-build.yml` compila no runner Windows do GitHub Actions e publica o EXE e um ZIP portátil como artifacts. Para gerar o MSI com assistente e escolha da pasta, execute `packaging/windows/package.ps1` em Windows com MSVC, Windows SDK e WiX v5; esse passo precisa ser testado no Windows.

## Recursos
- Janela desktop nativa (não Electron/web wrapper).
- Catálogo em grade/tabela, filtros, status, filas de tarefas, bandeja e fluxo de instalação/atualização da arquitetura-base.
- Marca ORBE azul-escura, ícones próprios e catálogo ORBE Studio/Motion.
- Bundle ID `br.orbe.launcher`.

## Estado de validação
- Alterações de código e recursos incluídas; estrutura ZIP será verificada.
- **Não compilado nem executado neste ambiente**: `cargo` e `rustc` não estão instalados. Build deve ser validado via workflow ou em Windows antes de distribuir.
- Repositórios de Release ainda são placeholders; integração real deve ser testada com instaladores/releases publicados dos aplicativos ORBE.

## Licença
Adaptação do `craft-launcher` 0.2.1 fornecido como referência, licenciado sob MIT OR Apache-2.0. Foram mantidos `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE` e `ATTRIBUTION.md`. Veja `LICENSE-NOTICE-ORBE.md`.
