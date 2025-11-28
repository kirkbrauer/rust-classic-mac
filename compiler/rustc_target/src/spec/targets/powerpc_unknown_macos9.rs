use crate::spec::{
    BinaryFormat, Cc, LinkerFlavor, PanicStrategy, RelocModel, Target, TargetMetadata,
    TargetOptions, cvs,
};
use rustc_abi::Endian;

pub(crate) fn target() -> Target {
    let mut base = TargetOptions {
        os: "none".into(),
        vendor: "unknown".into(),

        // Binary format - LLVM uses ELF as intermediate format for Classic Mac OS.
        // The linker (ld.lld -flavor pef) converts ELF objects to PEF executable.
        binary_format: BinaryFormat::Elf,

        // Linker configuration
        linker_flavor: LinkerFlavor::Unix(Cc::No),
        linker: Some("ld.lld".into()),

        // CFM/PEF characteristics
        dynamic_linking: false,
        has_rpath: false,
        has_thread_local: false,
        position_independent_executables: false,

        // Code generation
        max_atomic_width: Some(32),
        endian: Endian::Big,
        // Use ppc CPU (basic PowerPC G3 compatible) instead of generic
        cpu: "ppc".into(),
        // Disable features not supported on Classic Mac OS
        // Note: CRBits is enabled by default at opt >= Default, which is fine
        features: "-altivec,-vsx".into(),

        // Function sections not beneficial for PEF
        function_sections: false,

        // No standard library initially - use cvs![] for Cow type
        families: cvs![],
        panic_strategy: PanicStrategy::Abort,

        // System libraries - use link_args instead of add_late_link_args method
        late_link_args: TargetOptions::link_args(LinkerFlavor::Unix(Cc::No), &["-lInterfaceLib"]),

        relocation_model: RelocModel::Static,
        ..Default::default()
    };

    // PEF-specific linker arguments
    base.add_pre_link_args(LinkerFlavor::Unix(Cc::No), &["-flavor", "pef", "-e", "__start"]);

    Target {
        llvm_target: "powerpc-apple-classic".into(),
        metadata: TargetMetadata {
            description: Some("PowerPC Classic Mac OS (System 7-9)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 32,
        data_layout: "E-m:e-p:32:32-Fn32-i64:64-n32".into(),
        arch: "powerpc".into(),
        options: base,
    }
}
