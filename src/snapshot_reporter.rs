use crate::snapshot_diff::{ChangeKind, SnapshotDiff};
use std::fmt::Write;

/// Formats a list of diffs into a human-readable summary string.
pub fn format_diff_report(diffs: &[SnapshotDiff]) -> String {
    if diffs.is_empty() {
        return "No changes detected.".to_string();
    }

    let mut out = String::new();
    let mut appeared: Vec<&SnapshotDiff> = Vec::new();
    let mut disappeared: Vec<&SnapshotDiff> = Vec::new();
    let mut changed: Vec<&SnapshotDiff> = Vec::new();

    for d in diffs {
        match d.kind {
            ChangeKind::Appeared    => appeared.push(d),
            ChangeKind::Disappeared => disappeared.push(d),
            ChangeKind::Changed     => changed.push(d),
            ChangeKind::Unchanged   => {}
        }
    }

    if !appeared.is_empty() {
        let _ = writeln!(out, "[+] New processes ({}):", appeared.len());
        for d in &appeared {
            let _ = writeln!(out, "    pid={} name={} cpu={:.1}% mem={}KB",
                d.pid, d.name, d.cpu_delta, d.mem_delta_kb);
        }
    }

    if !disappeared.is_empty() {
        let _ = writeln!(out, "[-] Gone processes ({}):", disappeared.len());
        for d in &disappeared {
            let _ = writeln!(out, "    pid={} name={}", d.pid, d.name);
        }
    }

    if !changed.is_empty() {
        let _ = writeln!(out, "[~] Changed processes ({}):", changed.len());
        for d in &changed {
            let sign = if d.cpu_delta >= 0.0 { "+" } else { "" };
            let _ = writeln!(out, "    pid={} name={} cpu={}{:.1}% mem={:+}KB",
                d.pid, d.name, sign, d.cpu_delta, d.mem_delta_kb);
        }
    }

    out.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot_diff::{ChangeKind, SnapshotDiff};

    fn make_diff(pid: u32, kind: ChangeKind, cpu: f64, mem: i64) -> SnapshotDiff {
        SnapshotDiff { pid, name: format!("proc{}", pid), kind, cpu_delta: cpu, mem_delta_kb: mem }
    }

    #[test]
    fn test_empty_report() {
        assert_eq!(format_diff_report(&[]), "No changes detected.");
    }

    #[test]
    fn test_report_contains_sections() {
        let diffs = vec![
            make_diff(1, ChangeKind::Appeared, 3.0, 512),
            make_diff(2, ChangeKind::Disappeared, -1.0, -256),
            make_diff(3, ChangeKind::Changed, 5.5, 128),
        ];
        let report = format_diff_report(&diffs);
        assert!(report.contains("[+] New"));
        assert!(report.contains("[-] Gone"));
        assert!(report.contains("[~] Changed"));
        assert!(report.contains("proc1"));
    }
}
