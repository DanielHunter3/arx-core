pub mod constant;
pub mod errhandle;

pub mod component;
pub mod composition;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = constant::ARX_PATH;
        assert_eq!(result, "/home/user/arx");
    }
}
