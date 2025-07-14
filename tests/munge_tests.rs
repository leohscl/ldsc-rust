use std::fs::File;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;

use ldsc_rust::munge::munge;
use tempdir::TempDir;


fn setup_test_default() -> (PathBuf, PathBuf) {
    let temp_dir_path = TempDir::new("example").unwrap().into_path();
    let input_path = temp_dir_path.join("input");
    let output_path = temp_dir_path.join("output");

    let input_file_contents = "snpid	hg18chr	bp	a1	a2	or	se	pval	info	ngt	CEUaf
        rs12565286	1	711153	C	G	0.9887	0.0821	0.8899	0.3693	0	0.0678
        rs11804171	1	713682	A	T	0.9887	0.0821	0.8899	0.3693	0	.
        rs2977670	1	713754	C	G	1.011	0.0822	0.8936	0.3684	0	.";
    let mut input_file = File::create(&input_path).unwrap();
    input_file.write_all(input_file_contents.as_bytes()).unwrap();
    (input_path, output_path)
}

fn get_header(path: &PathBuf) -> String {
    let mut output_file = File::open(path).unwrap();
    let mut output_string = String::new();
    output_file.read_to_string(&mut output_string).unwrap();
    let (header, _) = output_string.split_once(|s| s == '\n').unwrap();
    header.to_string()
}

#[test]
fn test_munge_name_change() {
    let (input_path, output_path) = setup_test_default();
    munge(&input_path, 1, None, &output_path).unwrap();
    let header = get_header(&output_path);
    assert!(header.find("SNP").is_some());
}
