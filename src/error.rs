pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Parse(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = Error::Parse("解析失败".to_string());
        assert_eq!(error, Error::Parse("解析失败".to_string()));
    }

    #[test]
    fn test_error_debug() {
        let error = Error::Parse("语法错误".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("Parse"));
        assert!(debug_str.contains("语法错误"));
    }

    #[test]
    fn test_error_clone() {
        let error1 = Error::Parse("原始错误".to_string());
        let error2 = error1.clone();
        assert_eq!(error1, error2);
    }

    #[test]
    fn test_result_ok() {
        let result: Result<i32> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_result_err() {
        let result: Result<i32> = Err(Error::Parse("测试错误".to_string()));
        assert!(result.is_err());

        if let Err(e) = result {
            assert_eq!(e, Error::Parse("测试错误".to_string()));
        }
    }

    #[test]
    fn test_result_with_question_mark() -> Result<()> {
        fn may_fail(fail: bool) -> Result<&'static str> {
            if fail {
                return Err(Error::Parse("操作失败".to_string()));
            }
            Ok("成功")
        }

        assert_eq!(may_fail(false)?, "成功");
        assert!(may_fail(true).is_err());

        Ok(())
    }
}
