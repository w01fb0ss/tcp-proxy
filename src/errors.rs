use std::{fmt, process::exit};

#[derive(Debug)]
pub enum Errcode {
    #[allow(dead_code)]
    ArgumentInvalid(&'static str),
}

impl Errcode {
    // Translate an Errcode::X into a number to return (the Unix way)
    pub fn get_retcode(&self) -> i32 {
        1 // Everything != 0 will be treated as an error
    }
}

impl fmt::Display for Errcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Define what behaviour for each variant of the enum
        match &self {
            // Message to display when an argument is invalid
            Errcode::ArgumentInvalid(element) => write!(f, "ArgumentInvalid: {}", element),
            // _ => write!(f, "{:?}", self), // For any variant not previously covered
        }
    }
}

#[allow(dead_code)]
pub fn exit_with_retcode(res: Result<(), Errcode>) {
    match res {
        // If it's a success, return 0
        Ok(_) => {
            log::debug!("Exit without any error, returning 0");
            exit(0);
        }

        // If there's an error, print an error message and return the retcode
        Err(e) => {
            let retcode = e.get_retcode();
            log::error!("Error on exit:\n\t{}\n\tReturning {}", e, retcode);
            exit(retcode);
        }
    }
}
