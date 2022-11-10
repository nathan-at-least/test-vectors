use std::io::Result;
use std::path::PathBuf;
use target_test_dir::test_with_dir;

#[cfg(unix)]
#[test_with_dir]
fn list_dir_with_symlink(testdir: PathBuf) -> Result<()> {
    // Setup: create a corpus directory with a symlink to a case dir:
    let corpus = testdir.join("corpus");
    std::fs::create_dir(&corpus)?;
    let extcase = testdir.join("external-casedir");
    std::fs::create_dir(&extcase)?;
    std::os::unix::fs::symlink(&extcase, corpus.join("casedir-link"))?;

    // target code:
    let casedirs = crate::listdir::list_dir(&corpus)?;

    // verify the results contain `extcase`:
    assert_eq!(casedirs.len(), 1);

    let foundcase = casedirs.into_iter().next().unwrap();
    assert_eq!(foundcase.as_str(), "casedir-link");
    Ok(())
}
