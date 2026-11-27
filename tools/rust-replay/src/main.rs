//! The registry's rust replay gate: every artifact's embedded suite
//! runs green on the native engine, and every corpus accept case
//! parses (reject cases refuse). Generic by construction — no fleet
//! size, no sibling-repo fixtures.

use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .expect("tools/")
        .parent()
        .expect("repo root")
        .to_path_buf();

    let artifacts: Vec<PathBuf> = std::fs::read_dir(root.join("artifacts"))
        .expect("artifacts/")
        .filter_map(|e| {
            let p = e.expect("dir entry").path();
            (p.extension().and_then(|e| e.to_str()) == Some("json"))
                .then_some(p)
        })
        .collect();
    assert!(!artifacts.is_empty(), "no artifacts found");

    let mut checked = 0usize;
    for artifact_path in &artifacts {
        let text = std::fs::read_to_string(artifact_path).expect("artifact reads");
        let artifact = parsanol::PargArtifact::from_json(&text)
            .unwrap_or_else(|e| panic!("{}: envelope rejected: {e}", artifact_path.display()));
        let failures = artifact.run_tests();
        assert!(
            failures.is_empty(),
            "{} embedded suite failures: {failures:?}",
            artifact_path.display()
        );
        checked += 1;
        println!("[replay] {} embedded suite green", artifact_path.display());
    }

    let corpora: Vec<PathBuf> = std::fs::read_dir(root.join("corpora"))
        .expect("corpora/")
        .filter_map(|e| {
            let p = e.expect("dir entry").path().join("corpus.json");
            p.is_file().then_some(p)
        })
        .collect();
    assert!(!corpora.is_empty(), "no corpora found");

    let mut cases = 0usize;
    for corpus_path in corpora {
        let doc: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&corpus_path).expect("corpus reads"))
                .expect("corpus json");
        let flavor = doc["grammar"].as_str().expect("grammar").to_string();
        let artifact_text =
            std::fs::read_to_string(root.join("artifacts").join(format!("{flavor}.json")))
                .unwrap_or_else(|e| panic!("{}: artifact for the corpus: {e}", corpus_path.display()));
        let artifact =
            parsanol::PargArtifact::from_json(&artifact_text).expect("artifact loads");
        let entry = artifact
            .default_entry()
            .unwrap_or_else(|| panic!("{}: the artifact has no default entry", corpus_path.display()))
            .to_string();
        for case in doc["cases"].as_array().expect("cases") {
            let input = case["input"].as_str().expect("input");
            let outcome = artifact.parse_shape(&entry, input).is_ok();
            let wants_accept = case["kind"] == "accept";
            assert_eq!(
                outcome, wants_accept,
                "{}: {:?} parsed={} but the case is {}",
                corpus_path.display(),
                input,
                outcome,
                case["kind"]
            );
            cases += 1;
        }
        println!("[replay] {} corpus replayed", corpus_path.display());
    }

    println!("[replay] {checked} artifacts, {cases} corpus cases — green");
}
