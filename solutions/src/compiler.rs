//! Executable solutions about the compilation pipeline, its observations, and
//! build decisions. These models do not turn internal IRs into stable APIs.

pub mod c52 {
    use std::collections::BTreeSet;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MirSummary {
        pub basic_blocks: usize,
        pub moves: usize,
        pub drops: usize,
        pub unwind_edges: usize,
    }

    /// Counts textual markers for a small educational MIR sample. This is not
    /// a MIR parser: marker-like text in comments or literals can be counted.
    // SOLUTION: C52-E01
    pub fn summarize_pretty_mir(mir: &str) -> MirSummary {
        MirSummary {
            basic_blocks: mir
                .lines()
                .map(str::trim)
                .filter(|line| line.starts_with("bb") && line.contains('{'))
                .count(),
            moves: mir.matches("move ").count(),
            drops: mir.matches("drop(").count(),
            unwind_edges: mir.matches("unwind").count(),
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum BuildMode {
        Debug,
        Optimized,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AssemblySample<'a> {
        pub function: &'a str,
        pub target: &'a str,
        pub mode: BuildMode,
        pub instruction_lines: usize,
        pub call_sites: usize,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AssemblyComparison {
        pub debug_instruction_lines: usize,
        pub optimized_instruction_lines: usize,
        pub debug_call_sites: usize,
        pub optimized_call_sites: usize,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ComparisonError {
        DifferentFunction,
        DifferentTarget,
        UnexpectedBuildModes,
    }

    // SOLUTION: C52-E02
    pub fn compare_assembly(
        debug: AssemblySample<'_>,
        optimized: AssemblySample<'_>,
    ) -> Result<AssemblyComparison, ComparisonError> {
        if debug.function != optimized.function {
            return Err(ComparisonError::DifferentFunction);
        }
        if debug.target != optimized.target {
            return Err(ComparisonError::DifferentTarget);
        }
        if debug.mode != BuildMode::Debug || optimized.mode != BuildMode::Optimized {
            return Err(ComparisonError::UnexpectedBuildModes);
        }
        Ok(AssemblyComparison {
            debug_instruction_lines: debug.instruction_lines,
            optimized_instruction_lines: optimized.instruction_lines,
            debug_call_sites: debug.call_sites,
            optimized_call_sites: optimized.call_sites,
        })
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TimingSample {
        pub crate_name: String,
        pub fresh: bool,
        pub front_end_ms: u64,
        pub codegen_ms: u64,
        pub build_script_ms: u64,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum BuildPhase {
        FrontEnd,
        Codegen,
        BuildScript,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Bottleneck {
        pub crate_name: String,
        pub dominant_phase: BuildPhase,
        pub total_ms: u128,
    }

    /// Selects the largest sum of supplied, non-overlapping phase durations.
    /// This model does not compute the build graph's critical path.
    // SOLUTION: C52-E03
    pub fn dominant_clean_timing(samples: &[TimingSample]) -> Option<Bottleneck> {
        let sample = samples
            .iter()
            .filter(|sample| !sample.fresh)
            .max_by_key(|sample| {
                u128::from(sample.front_end_ms)
                    + u128::from(sample.codegen_ms)
                    + u128::from(sample.build_script_ms)
            })?;
        let dominant_phase = if sample.build_script_ms >= sample.front_end_ms
            && sample.build_script_ms >= sample.codegen_ms
        {
            BuildPhase::BuildScript
        } else if sample.codegen_ms >= sample.front_end_ms {
            BuildPhase::Codegen
        } else {
            BuildPhase::FrontEnd
        };

        Some(Bottleneck {
            crate_name: sample.crate_name.clone(),
            dominant_phase,
            total_ms: u128::from(sample.front_end_ms)
                + u128::from(sample.codegen_ms)
                + u128::from(sample.build_script_ms),
        })
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum DiagnosticSignal {
        UnexpectedToken,
        MacroExpansionFailure,
        UnresolvedImport,
        TypeMismatch,
        UnsatisfiedTraitBound,
        BorrowAfterMove,
        ConstEvaluationFailure,
        BackendCodegenFailure,
        UndefinedSymbol,
        MissingNativeLibrary,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CompilationPhase {
        Parsing,
        MacroExpansion,
        NameResolution,
        TypeChecking,
        BorrowChecking,
        ConstEvaluation,
        CodegenBackend,
        Linking,
        BuildEnvironment,
    }

    // SOLUTION: C52-E04
    pub const fn diagnostic_phase(signal: DiagnosticSignal) -> CompilationPhase {
        match signal {
            DiagnosticSignal::UnexpectedToken => CompilationPhase::Parsing,
            DiagnosticSignal::MacroExpansionFailure => CompilationPhase::MacroExpansion,
            DiagnosticSignal::UnresolvedImport => CompilationPhase::NameResolution,
            DiagnosticSignal::TypeMismatch | DiagnosticSignal::UnsatisfiedTraitBound => {
                CompilationPhase::TypeChecking
            }
            DiagnosticSignal::BorrowAfterMove => CompilationPhase::BorrowChecking,
            DiagnosticSignal::ConstEvaluationFailure => CompilationPhase::ConstEvaluation,
            DiagnosticSignal::BackendCodegenFailure => CompilationPhase::CodegenBackend,
            DiagnosticSignal::UndefinedSymbol => CompilationPhase::Linking,
            DiagnosticSignal::MissingNativeLibrary => CompilationPhase::BuildEnvironment,
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum StdProvision {
        Prebuilt,
        BuildFromSource,
        NoStd,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ExecutionStrategy {
        LocalRunner,
        RemoteDevice,
        Emulator,
        CompileOnly,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct CrossTargetPlan {
        pub target_spec_available: bool,
        pub std_provision: Option<StdProvision>,
        pub linker_available: bool,
        pub linker_selected: bool,
        pub requires_platform_sdk: bool,
        pub platform_sdk_available: bool,
        pub native_dependencies_resolved: bool,
        pub execution: ExecutionStrategy,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum MissingCrossComponent {
        TargetSpecification,
        RustStandardLibrary,
        Linker,
        LinkerSelection,
        PlatformSdkOrSysroot,
        NativeDependency,
        TestRunner,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct CrossTargetAudit {
        pub build_ready: bool,
        pub test_ready: bool,
        pub missing: Vec<MissingCrossComponent>,
    }

    // SOLUTION: C52-E05
    pub fn audit_cross_target(plan: CrossTargetPlan) -> CrossTargetAudit {
        let mut missing = Vec::new();
        if !plan.target_spec_available {
            missing.push(MissingCrossComponent::TargetSpecification);
        }
        if plan.std_provision.is_none() {
            missing.push(MissingCrossComponent::RustStandardLibrary);
        }
        if !plan.linker_available {
            missing.push(MissingCrossComponent::Linker);
        }
        if plan.linker_available && !plan.linker_selected {
            missing.push(MissingCrossComponent::LinkerSelection);
        }
        if plan.requires_platform_sdk && !plan.platform_sdk_available {
            missing.push(MissingCrossComponent::PlatformSdkOrSysroot);
        }
        if !plan.native_dependencies_resolved {
            missing.push(MissingCrossComponent::NativeDependency);
        }

        let build_ready = missing.is_empty();
        let has_runner = plan.execution != ExecutionStrategy::CompileOnly;
        if build_ready && !has_runner {
            missing.push(MissingCrossComponent::TestRunner);
        }
        CrossTargetAudit {
            build_ready,
            test_ready: build_ready && has_runner,
            missing,
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct QueryEdge<'a> {
        pub query: &'a str,
        pub depends_on: &'a str,
    }

    /// Returns a conservative transitive dependency set. Unlike rustc, it does
    /// not reevaluate queries to determine whether their results really changed.
    // SOLUTION: C52-E06
    pub fn affected_queries(edges: &[QueryEdge<'_>], changed: &[&str]) -> BTreeSet<String> {
        let mut affected = changed
            .iter()
            .map(|name| (*name).to_owned())
            .collect::<BTreeSet<_>>();

        loop {
            let before = affected.len();
            for edge in edges {
                if affected.contains(edge.depends_on) {
                    affected.insert(edge.query.to_owned());
                }
            }
            if affected.len() == before {
                return affected;
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum BuildGoal {
        FastFeedback,
        BalancedRelease,
        RuntimeThroughput,
        SmallBinary,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum OptimizationLevel {
        None,
        Full,
        SizeAggressive,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum LtoMode {
        Off,
        Thin,
        Fat,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum DebugInfo {
        None,
        LineTables,
        Full,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ProfileAdvice {
        pub optimization: OptimizationLevel,
        pub incremental: bool,
        pub codegen_units: u16,
        pub lto: LtoMode,
        pub debug_info: DebugInfo,
    }

    // SOLUTION: C52-E07
    pub const fn profile_for(goal: BuildGoal) -> ProfileAdvice {
        match goal {
            BuildGoal::FastFeedback => ProfileAdvice {
                optimization: OptimizationLevel::None,
                incremental: true,
                codegen_units: 256,
                lto: LtoMode::Off,
                debug_info: DebugInfo::Full,
            },
            BuildGoal::BalancedRelease => ProfileAdvice {
                optimization: OptimizationLevel::Full,
                incremental: false,
                codegen_units: 16,
                lto: LtoMode::Off,
                debug_info: DebugInfo::LineTables,
            },
            BuildGoal::RuntimeThroughput => ProfileAdvice {
                optimization: OptimizationLevel::Full,
                incremental: false,
                codegen_units: 1,
                lto: LtoMode::Fat,
                debug_info: DebugInfo::LineTables,
            },
            BuildGoal::SmallBinary => ProfileAdvice {
                optimization: OptimizationLevel::SizeAggressive,
                incremental: false,
                codegen_units: 1,
                lto: LtoMode::Fat,
                debug_info: DebugInfo::None,
            },
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn mir_summary_locates_control_flow_moves_drops_and_cleanup_edges() {
            let mir = r#"
                bb0: {
                    _2 = move _1;
                    drop(_2) -> [return: bb1, unwind: bb2];
                }
                bb1: { return; }
                bb2 (cleanup): { resume; }
            "#;
            assert_eq!(
                summarize_pretty_mir(mir),
                MirSummary {
                    basic_blocks: 3,
                    moves: 1,
                    drops: 1,
                    unwind_edges: 1,
                },
            );
            assert_eq!(
                summarize_pretty_mir("// no MIR markers in this input"),
                MirSummary {
                    basic_blocks: 0,
                    moves: 0,
                    drops: 0,
                    unwind_edges: 0,
                },
            );
        }

        #[test]
        fn assembly_comparison_requires_matching_context_and_build_modes() {
            let debug = AssemblySample {
                function: "twice_u64",
                target: "x86_64-pc-windows-msvc",
                mode: BuildMode::Debug,
                instruction_lines: 34,
                call_sites: 3,
            };
            let optimized = AssemblySample {
                mode: BuildMode::Optimized,
                instruction_lines: 4,
                call_sites: 0,
                ..debug
            };
            assert_eq!(
                compare_assembly(debug, optimized),
                Ok(AssemblyComparison {
                    debug_instruction_lines: 34,
                    optimized_instruction_lines: 4,
                    debug_call_sites: 3,
                    optimized_call_sites: 0,
                }),
            );
            assert_eq!(
                compare_assembly(
                    debug,
                    AssemblySample {
                        target: "wasm32-unknown-unknown",
                        ..optimized
                    },
                ),
                Err(ComparisonError::DifferentTarget),
            );
            assert_eq!(
                compare_assembly(
                    debug,
                    AssemblySample {
                        function: "another_function",
                        ..optimized
                    },
                ),
                Err(ComparisonError::DifferentFunction),
            );
            assert_eq!(
                compare_assembly(
                    debug,
                    AssemblySample {
                        mode: BuildMode::Debug,
                        ..optimized
                    },
                ),
                Err(ComparisonError::UnexpectedBuildModes),
            );
        }

        #[test]
        fn timing_analysis_ignores_fresh_units_and_names_the_dominant_phase() {
            let bottleneck = dominant_clean_timing(&[
                TimingSample {
                    crate_name: String::from("cached"),
                    fresh: true,
                    front_end_ms: 9_000,
                    codegen_ms: 0,
                    build_script_ms: 0,
                },
                TimingSample {
                    crate_name: String::from("domain"),
                    fresh: false,
                    front_end_ms: 800,
                    codegen_ms: 2_400,
                    build_script_ms: 100,
                },
            ]);
            assert_eq!(
                bottleneck,
                Some(Bottleneck {
                    crate_name: String::from("domain"),
                    dominant_phase: BuildPhase::Codegen,
                    total_ms: 3_300,
                }),
            );

            let front_end = dominant_clean_timing(&[TimingSample {
                crate_name: String::from("types"),
                fresh: false,
                front_end_ms: 500,
                codegen_ms: 100,
                build_script_ms: 0,
            }]);
            assert_eq!(
                front_end.map(|sample| sample.dominant_phase),
                Some(BuildPhase::FrontEnd),
            );

            let build_script = dominant_clean_timing(&[TimingSample {
                crate_name: String::from("native-wrapper"),
                fresh: false,
                front_end_ms: 100,
                codegen_ms: 200,
                build_script_ms: 700,
            }]);
            assert_eq!(
                build_script.map(|sample| sample.dominant_phase),
                Some(BuildPhase::BuildScript),
            );
            assert_eq!(dominant_clean_timing(&[]), None);
            assert_eq!(
                dominant_clean_timing(&[TimingSample {
                    crate_name: String::from("cached"),
                    fresh: true,
                    front_end_ms: 1,
                    codegen_ms: 1,
                    build_script_ms: 1,
                }]),
                None,
            );
        }

        #[test]
        fn ten_diagnostic_signals_map_to_their_first_actionable_phase() {
            let cases = [
                (DiagnosticSignal::UnexpectedToken, CompilationPhase::Parsing),
                (
                    DiagnosticSignal::MacroExpansionFailure,
                    CompilationPhase::MacroExpansion,
                ),
                (
                    DiagnosticSignal::UnresolvedImport,
                    CompilationPhase::NameResolution,
                ),
                (
                    DiagnosticSignal::TypeMismatch,
                    CompilationPhase::TypeChecking,
                ),
                (
                    DiagnosticSignal::UnsatisfiedTraitBound,
                    CompilationPhase::TypeChecking,
                ),
                (
                    DiagnosticSignal::BorrowAfterMove,
                    CompilationPhase::BorrowChecking,
                ),
                (
                    DiagnosticSignal::ConstEvaluationFailure,
                    CompilationPhase::ConstEvaluation,
                ),
                (
                    DiagnosticSignal::BackendCodegenFailure,
                    CompilationPhase::CodegenBackend,
                ),
                (DiagnosticSignal::UndefinedSymbol, CompilationPhase::Linking),
                (
                    DiagnosticSignal::MissingNativeLibrary,
                    CompilationPhase::BuildEnvironment,
                ),
            ];
            for (signal, expected) in cases {
                assert_eq!(diagnostic_phase(signal), expected);
            }
        }

        #[test]
        fn target_std_alone_does_not_make_a_cross_build_or_its_tests_ready() {
            let audit = audit_cross_target(CrossTargetPlan {
                target_spec_available: true,
                std_provision: Some(StdProvision::Prebuilt),
                linker_available: false,
                linker_selected: false,
                requires_platform_sdk: true,
                platform_sdk_available: false,
                native_dependencies_resolved: true,
                execution: ExecutionStrategy::CompileOnly,
            });
            assert!(!audit.build_ready);
            assert!(!audit.test_ready);
            assert_eq!(
                audit.missing,
                [
                    MissingCrossComponent::Linker,
                    MissingCrossComponent::PlatformSdkOrSysroot,
                ],
            );
        }

        #[test]
        fn cross_target_audit_reports_every_build_requirement() {
            let audit = audit_cross_target(CrossTargetPlan {
                target_spec_available: false,
                std_provision: None,
                linker_available: true,
                linker_selected: false,
                requires_platform_sdk: true,
                platform_sdk_available: false,
                native_dependencies_resolved: false,
                execution: ExecutionStrategy::CompileOnly,
            });
            assert!(!audit.build_ready);
            assert!(!audit.test_ready);
            assert_eq!(
                audit.missing,
                [
                    MissingCrossComponent::TargetSpecification,
                    MissingCrossComponent::RustStandardLibrary,
                    MissingCrossComponent::LinkerSelection,
                    MissingCrossComponent::PlatformSdkOrSysroot,
                    MissingCrossComponent::NativeDependency,
                ],
            );
        }

        #[test]
        fn cross_target_audit_distinguishes_building_from_running_tests() {
            let compile_only = CrossTargetPlan {
                target_spec_available: true,
                std_provision: Some(StdProvision::NoStd),
                linker_available: true,
                linker_selected: true,
                requires_platform_sdk: false,
                platform_sdk_available: false,
                native_dependencies_resolved: true,
                execution: ExecutionStrategy::CompileOnly,
            };
            assert_eq!(
                audit_cross_target(compile_only),
                CrossTargetAudit {
                    build_ready: true,
                    test_ready: false,
                    missing: vec![MissingCrossComponent::TestRunner],
                },
            );

            for execution in [
                ExecutionStrategy::LocalRunner,
                ExecutionStrategy::RemoteDevice,
                ExecutionStrategy::Emulator,
            ] {
                let audit = audit_cross_target(CrossTargetPlan {
                    execution,
                    ..compile_only
                });
                assert!(audit.build_ready);
                assert!(audit.test_ready);
                assert!(audit.missing.is_empty());
            }
        }

        #[test]
        fn query_invalidation_propagates_only_through_declared_dependencies() {
            let affected = affected_queries(
                &[
                    QueryEdge {
                        query: "typeck(api)",
                        depends_on: "hir(api)",
                    },
                    QueryEdge {
                        query: "metadata(crate)",
                        depends_on: "typeck(api)",
                    },
                    QueryEdge {
                        query: "codegen(private)",
                        depends_on: "mir(private)",
                    },
                ],
                &["hir(api)"],
            );
            assert_eq!(
                affected,
                ["hir(api)", "metadata(crate)", "typeck(api)"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
            );
        }

        #[test]
        fn query_invalidation_terminates_on_cycles_and_keeps_unrelated_nodes() {
            let affected = affected_queries(
                &[
                    QueryEdge {
                        query: "query-a",
                        depends_on: "query-b",
                    },
                    QueryEdge {
                        query: "query-b",
                        depends_on: "query-a",
                    },
                    QueryEdge {
                        query: "dependent",
                        depends_on: "query-b",
                    },
                    QueryEdge {
                        query: "unrelated",
                        depends_on: "other-input",
                    },
                ],
                &["query-a"],
            );
            assert_eq!(
                affected,
                ["dependent", "query-a", "query-b"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
            );
        }

        #[test]
        fn profile_advice_covers_every_build_goal() {
            assert_eq!(
                profile_for(BuildGoal::FastFeedback),
                ProfileAdvice {
                    optimization: OptimizationLevel::None,
                    incremental: true,
                    codegen_units: 256,
                    lto: LtoMode::Off,
                    debug_info: DebugInfo::Full,
                },
            );
            assert_eq!(
                profile_for(BuildGoal::BalancedRelease),
                ProfileAdvice {
                    optimization: OptimizationLevel::Full,
                    incremental: false,
                    codegen_units: 16,
                    lto: LtoMode::Off,
                    debug_info: DebugInfo::LineTables,
                },
            );
            assert_eq!(
                profile_for(BuildGoal::RuntimeThroughput),
                ProfileAdvice {
                    optimization: OptimizationLevel::Full,
                    incremental: false,
                    codegen_units: 1,
                    lto: LtoMode::Fat,
                    debug_info: DebugInfo::LineTables,
                },
            );
            assert_eq!(
                profile_for(BuildGoal::SmallBinary),
                ProfileAdvice {
                    optimization: OptimizationLevel::SizeAggressive,
                    incremental: false,
                    codegen_units: 1,
                    lto: LtoMode::Fat,
                    debug_info: DebugInfo::None,
                },
            );
        }
    }
}
