use std::{path::PathBuf, fs::{self, DirEntry}, error::Error, rc::Rc, fmt::Display};

type DynError = Box<dyn Error + 'static + Send + Sync>;

/// Lists the files 
/// 
/// Notice that we had to use a hdlr wrapped in an Rc.  Since this is a recursive function, the hdlr
/// gets moved in the for loop when we need to make the recursive call.  But, since hdlr is not a 
/// reference, it is moved.  I could not figure out a way to take a reference to an impl
fn list_dir_helper(path: PathBuf, depth: usize, hdlr: Rc<impl Fn(&DirEntry, usize) -> ()>) -> Result<(), DynError> {
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let ft = entry.file_type()?;
        (*hdlr)(&entry, depth);
        let new_hdlr = hdlr.clone();
        if ft.is_dir() {
            list_dir_helper(entry.path(), depth + 2, new_hdlr)?;
        };
    }

    Ok(())
}

/// Realized I was dense and could do it like this.  Should be faster too
pub fn list_dir_2<T>(path: PathBuf, depth: usize, hdlr: &T) -> Result<(), DynError>
    where T: Fn(&DirEntry, usize) -> Result<(), DynError> {
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let ft = entry.file_type()?;
        hdlr(&entry, depth)?;
        if ft.is_dir() {
            list_dir_2(entry.path(), depth + 2, hdlr)?;
        };
    }

    Ok(())
}

pub fn list_dir(path: PathBuf, hdlr: Rc<impl Fn(&DirEntry, usize) -> ()>) -> Result<(), DynError> {
    list_dir_helper(path, 0, hdlr)
}

pub fn space_string(base: &str, depth: usize) -> String {
    let reserve = base.len() + depth;
    let mut new_str = String::with_capacity(reserve);
    // Doesn't require new allocation, so this should be fast
    for _ in 0..depth {
        new_str.push(' ');
    }
    new_str.push_str(base);
    new_str
}

#[derive(Debug)]
pub struct ListError {

}

impl Display for ListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ListError")
    }
}

impl Error for ListError {

}



/// Lists all the files in a directory, and renames a substring in the filename
fn rename(path: PathBuf, from: &str, to: &str) -> Result<(), DynError> {
    list_dir_2(path, 0, &|entry, _| {
        let fname = entry.file_name();
        let fname = match fname.into_string() {
            Ok(f) => f,
            Err(_) => return Err(Box::new(ListError{})),
        };
        let new_name = fname.replace(from, to);
        let new_path = entry.path()
            .parent()
            .map(|p| { 
                let mut buf = p.to_path_buf();
                buf.push(&new_name);
                buf
            })
            .expect("No parent");
        

        println!("Going to rename {:?} to {:?}", entry.path(), new_path);
        //fs::rename(entry.path(), new_path)?;
        Ok(())
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rename() {
        let res = rename(PathBuf::from(
            "/mnt/chromeos/MyFiles/RPG/Morrow_Project"),
         "TheMorrowProject-", 
         ""
        );
    }

    #[test]
    fn test_spaces() {
        let spaces = space_string("->", 4);
        println!("{spaces}something{spaces}again")
    }

    #[test]
    fn test_list() -> Result<(), DynError> {
        let curr = std::env::current_dir()?;
        let p = curr.parent().unwrap();
        let hdlr = |entry: &DirEntry, depth: usize| {
            let spaces = space_string("-", depth + 2);
            println!("{spaces}{:?}", entry.file_name());
        };
        list_dir(p.to_path_buf(), Rc::new(hdlr))?;
        Ok(())
    }

    #[test]
    fn test_list2() -> Result<(), DynError> {
        let curr = std::env::current_dir()?;
        let p = curr.parent().unwrap();
        list_dir_2(p.to_path_buf(), 0, &|entry: &DirEntry, depth: usize| {
            let spaces = space_string("-", depth + 2);
            println!("{spaces}{:?}", entry.file_name());
            Ok(())
        })?;

        Ok(())
    }
}